import { useCallback, useEffect, useRef, useState } from "react";
import type { DeviceType, NetDevice, NetLink, NetInterface } from "../../netsim/types";
import { DEVICE_LABELS, DEVICE_HAS_IP, MAX_INTERFACES } from "../../netsim/types";
import type { ToolMode } from "../../netsim/components/Toolbar";
import { Toolbar } from "../../netsim/components/Toolbar";
import { Canvas } from "../../netsim/components/Canvas";
import { DevicePanel } from "../../netsim/components/DevicePanel";
import { Console } from "../../netsim/components/Console";
import type { ConsoleEntry } from "../../netsim/components/Console";
import "./Playground.css";

const isTauri = "__TAURI_INTERNALS__" in window;

let nextId = 1;
const uid = () => `n${nextId++}`;

const CANVAS_CENTER_X = 500;
const CANVAS_CENTER_Y = 300;

interface CableState {
  from?: { deviceId: string; ifaceId: string };
}

interface PickerState {
  deviceId: string;
  position: { x: number; y: number };
}

interface StepInfo {
  device: string;
  event: string;
  tables: string[];
}

// ── Persist state across page navigation ──

interface SavedState {
  devices: NetDevice[];
  links: NetLink[];
  consoleEntries: ConsoleEntry[];
  running: boolean;
  stepMode: boolean;
  selectedId: string | null;
  nextId: number;
}

let savedState: SavedState | null = null;

async function tauriInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(cmd, args);
}

async function tauriListen<T>(event: string, handler: (e: { payload: T }) => void) {
  const { listen } = await import("@tauri-apps/api/event");
  return listen<T>(event, handler);
}

export function Playground() {
  if (!isTauri) {
    return (
      <main className="ns-page">
        <div className="ns-web-notice">
          <h2>Simulateur réseau</h2>
          <p>
            Le simulateur réseau nécessite l'application desktop pour fonctionner.
          </p>
          <p>
            Téléchargez l'application pour accéder à cette fonctionnalité.
          </p>
        </div>
      </main>
    );
  }

  return <PlaygroundInner />;
}

function PlaygroundInner() {
  const [init] = useState<SavedState | null>(() => {
    if (savedState) {
      const s = savedState;
      nextId = s.nextId;
      savedState = null;
      return s;
    }
    return null;
  });

  const [devices, setDevices] = useState<NetDevice[]>(init?.devices ?? []);
  const [links, setLinks] = useState<NetLink[]>(init?.links ?? []);
  const [selectedId, setSelectedId] = useState<string | null>(init?.selectedId ?? null);

  const [toolMode, setToolMode] = useState<ToolMode>("select");
  const [cableState, setCableState] = useState<CableState>({});
  const [pickerState, setPickerState] = useState<PickerState | null>(null);

  const [running, setRunning] = useState(init?.running ?? false);
  const [stepMode, setStepMode] = useState(init?.stepMode ?? false);
  const [waitingStep, setWaitingStep] = useState(false);
  const [consoleEntries, setConsoleEntries] = useState<ConsoleEntry[]>(init?.consoleEntries ?? []);

  const stateRef = useRef({ devices, links, consoleEntries, running, stepMode, selectedId });
  useEffect(() => {
    stateRef.current = { devices, links, consoleEntries, running, stepMode, selectedId };
  });
  useEffect(() => {
    return () => {
      savedState = { ...stateRef.current, nextId };
    };
  }, []);

  useEffect(() => {
    const unlisten = tauriListen<{ device: string; event: string; tables: string[] }>(
      "sim-step",
      (event) => {
        const { device, event: evt } = event.payload;
        setConsoleEntries(prev => [
          ...prev,
          { type: "step" as const, text: `  [${device}] ${evt}` },
        ]);
      },
    );
    return () => { unlisten.then(fn => fn()); };
  }, []);

  const logConsole = useCallback((type: ConsoleEntry["type"], text: string) => {
    setConsoleEntries(prev => [...prev, { type, text }]);
  }, []);

  const resetTool = () => {
    setCableState({});
    setPickerState(null);
  };

  const setTool = useCallback((mode: ToolMode) => {
    resetTool();
    setToolMode(mode);
  }, []);

  // ── Device CRUD ──

  const addDevice = useCallback((type: DeviceType) => {
    if (running) return;
    const id = uid();
    const count = devices.filter(d => d.type === type).length + 1;
    const name = `${DEVICE_LABELS[type]}${count}`;
    const spread = devices.length * 40;
    const device: NetDevice = {
      id,
      type,
      name,
      x: CANVAS_CENTER_X + (spread % 300) - 150,
      y: CANVAS_CENTER_Y + Math.floor(spread / 300) * 80 - 40,
      interfaces: [],
    };

    tauriInvoke("sim_create_device", { name, deviceType: type }).catch(err => {
      logConsole("err", `Erreur création ${name}: ${err}`);
    });

    setDevices(prev => [...prev, device]);
    setSelectedId(id);
  }, [devices, running, logConsole]);

  const moveDevice = useCallback((id: string, x: number, y: number) => {
    setDevices(prev =>
      prev.map(d => (d.id === id ? { ...d, x, y } : d)),
    );
  }, []);

  const renameDevice = useCallback((deviceId: string, newName: string) => {
    if (running) return;
    const device = devices.find(d => d.id === deviceId);
    if (!device || device.name === newName) return;

    tauriInvoke("sim_rename_device", { oldName: device.name, newName }).catch(err => {
      logConsole("err", `Erreur renommage: ${err}`);
    });

    setDevices(prev =>
      prev.map(d => (d.id === deviceId ? { ...d, name: newName } : d)),
    );
  }, [devices, running, logConsole]);

  const deleteDevice = useCallback((id: string) => {
    if (running) return;
    const device = devices.find(d => d.id === id);
    if (device) {
      tauriInvoke("sim_remove_device", { name: device.name }).catch(err => {
        logConsole("err", `Erreur suppression: ${err}`);
      });
    }
    setDevices(prev => prev.filter(d => d.id !== id));
    setLinks(prev =>
      prev.filter(l => l.from.deviceId !== id && l.to.deviceId !== id),
    );
    if (selectedId === id) setSelectedId(null);
  }, [selectedId, devices, running, logConsole]);

  // ── Interfaces ──

  const addInterface = useCallback(
    async (deviceId: string, name: string, ip: string, mask: number, mtu: number) => {
      const device = devices.find(d => d.id === deviceId);
      if (!device) return;

      if (device.interfaces.length >= MAX_INTERFACES[device.type]) {
        logConsole("err", `${device.name}: limite de ${MAX_INTERFACES[device.type]} interfaces atteinte`);
        return;
      }

      const backendIp = DEVICE_HAS_IP[device.type] ? ip : "0.0.0.0";
      const backendMask = DEVICE_HAS_IP[device.type] ? mask : 0;

      try {
        const result = await tauriInvoke<{ id: number; mac: string }>("sim_add_interface", {
          deviceName: device.name,
          ifaceName: name,
          ip: backendIp,
          mask: backendMask,
          mtu,
        });

        const iface: NetInterface = { id: uid(), name, ip, mask, mtu, mac: result.mac };
        setDevices(prev =>
          prev.map(d =>
            d.id === deviceId
              ? { ...d, interfaces: [...d.interfaces, iface] }
              : d,
          ),
        );
      } catch (err) {
        logConsole("err", `Erreur interface: ${err}`);
      }
    },
    [devices, logConsole],
  );

  const editInterface = useCallback(
    (deviceId: string, ifaceId: string, name: string, ip: string, mask: number, mtu: number) => {
      const device = devices.find(d => d.id === deviceId);
      if (!device) return;
      const oldIface = device.interfaces.find(i => i.id === ifaceId);
      if (!oldIface) return;

      const backendIp = DEVICE_HAS_IP[device.type] ? ip : undefined;
      const backendMask = DEVICE_HAS_IP[device.type] ? mask : undefined;

      tauriInvoke("sim_edit_interface", {
        deviceName: device.name,
        oldIfaceName: oldIface.name,
        newName: name !== oldIface.name ? name : null,
        ip: backendIp !== undefined && backendIp !== oldIface.ip ? backendIp : null,
        mask: backendMask !== undefined && backendMask !== oldIface.mask ? backendMask : null,
        mtu: mtu !== oldIface.mtu ? mtu : null,
      }).catch(err => {
        logConsole("err", `Erreur modification: ${err}`);
      });

      setDevices(prev =>
        prev.map(d =>
          d.id === deviceId
            ? { ...d, interfaces: d.interfaces.map(i => i.id === ifaceId ? { ...i, name, ip, mask, mtu } : i) }
            : d,
        ),
      );
    },
    [devices, logConsole],
  );

  // ── Cable linking ──

  const handleDeviceClick = useCallback((deviceId: string, x: number, y: number) => {
    if (toolMode !== "cable") return;
    if (cableState.from && cableState.from.deviceId === deviceId) {
      setPickerState(null);
      return;
    }
    setPickerState({ deviceId, position: { x, y } });
  }, [toolMode, cableState]);

  const handlePickInterface = useCallback((deviceId: string, ifaceId: string) => {
    setPickerState(null);

    if (!cableState.from) {
      setCableState({ from: { deviceId, ifaceId } });
      return;
    }

    if (cableState.from.deviceId === deviceId) {
      setCableState({});
      return;
    }

    const fromDevice = devices.find(d => d.id === cableState.from!.deviceId);
    const toDevice = devices.find(d => d.id === deviceId);
    const fromIface = fromDevice?.interfaces.find(i => i.id === cableState.from!.ifaceId);
    const toIface = toDevice?.interfaces.find(i => i.id === ifaceId);

    if (fromDevice && toDevice && fromIface && toIface) {
      tauriInvoke("sim_link_interfaces", {
        devA: fromDevice.name,
        ifaceA: fromIface.name,
        devB: toDevice.name,
        ifaceB: toIface.name,
      }).catch(err => {
        logConsole("err", `Erreur liaison: ${err}`);
      });
    }

    setLinks(prev => [
      ...prev,
      { id: uid(), from: cableState.from!, to: { deviceId, ifaceId } },
    ]);
    setCableState({});
  }, [cableState, devices, logConsole]);

  const closePicker = useCallback(() => {
    setPickerState(null);
  }, []);

  // ── Simulation controls ──

  const handleStart = useCallback(async () => {
    try {
      if (stepMode) {
        await tauriInvoke("sim_set_step_mode", { enabled: true });
      }
      await tauriInvoke("sim_start");
      setRunning(true);
      logConsole("info", `Simulation démarrée${stepMode ? " (mode étape par étape)" : ""}`);
    } catch (err) {
      logConsole("err", `Erreur: ${err}`);
    }
  }, [logConsole, stepMode]);

  const handleStop = useCallback(async () => {
    try {
      await tauriInvoke("sim_stop");
      setRunning(false);
      setWaitingStep(false);
      logConsole("info", "Simulation arrêtée");
    } catch (err) {
      logConsole("err", `Erreur: ${err}`);
    }
  }, [logConsole]);

  const handleClear = useCallback(() => {
    setConsoleEntries([]);
  }, []);

  const handleReset = useCallback(async () => {
    try {
      await tauriInvoke("sim_reset");
      setRunning(false);
      setWaitingStep(false);
      setDevices([]);
      setLinks([]);
      setSelectedId(null);
      setConsoleEntries([]);
      resetTool();
    } catch (err) {
      logConsole("err", `Erreur: ${err}`);
    }
  }, [logConsole]);

  const handleNextStep = useCallback(async () => {
    if (waitingStep) return;
    setWaitingStep(true);
    try {
      const step = await tauriInvoke<StepInfo>("sim_next_step");
      logConsole("info", `[${step.device}] ${step.event}`);
      for (const table of step.tables) {
        logConsole("out", table);
      }
    } catch (err) {
      logConsole("err", `${err}`);
    }
    setWaitingStep(false);
  }, [waitingStep, logConsole]);

  const handleExec = useCallback(async (deviceName: string, command: string) => {
    logConsole("cmd", `${deviceName}> ${command}`);
    try {
      const result = await tauriInvoke<string>("sim_exec", { deviceName, input: command });
      if (result) logConsole("out", result);
      if (stepMode && running) {
        handleNextStep();
      }
    } catch (err) {
      logConsole("err", `${err}`);
    }
  }, [logConsole, stepMode, running, handleNextStep]);

  const selected = devices.find(d => d.id === selectedId) ?? null;

  return (
    <main className="ns-page">
      <div className="ns-topbar">
        <h1>Playground</h1>
        <Toolbar
          onAdd={addDevice}
          toolMode={toolMode}
          onSetTool={setTool}
          running={running}
          onStart={handleStart}
          onStop={handleStop}
          onReset={handleReset}
        />
      </div>

      <div className="ns-workspace">
        <Canvas
          devices={devices}
          links={links}
          selectedId={selectedId}
          toolMode={toolMode}
          pickerState={pickerState}
          onSelect={setSelectedId}
          onDeselect={() => setSelectedId(null)}
          onMove={moveDevice}
          onDeviceClick={handleDeviceClick}
          onPickInterface={handlePickInterface}
          onClosePicker={closePicker}
          onDeleteDevice={deleteDevice}
        />

        {selected && toolMode === "select" && (
          <DevicePanel
            device={selected}
            links={links}
            allDevices={devices}
            linkMode={{ active: false }}
            onAddInterface={addInterface}
            onEditInterface={editInterface}
            onRenameDevice={renameDevice}
            onDeleteDevice={deleteDevice}
            onStartLink={() => {}}
            onClose={() => setSelectedId(null)}
          />
        )}
      </div>

      <Console
        devices={devices}
        running={running}
        onExec={handleExec}
        onClear={handleClear}
        entries={consoleEntries}
        stepMode={stepMode}
        onSetStepMode={setStepMode}
        onNextStep={handleNextStep}
        waitingStep={waitingStep}
      />
    </main>
  );
}
