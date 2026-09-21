import { useState, useRef, useEffect, useCallback } from "react";
import type { NetDevice } from "../types";
import "./Console.css";

export interface ConsoleEntry {
  type: "cmd" | "out" | "err" | "info" | "step";
  text: string;
}

interface Props {
  devices: NetDevice[];
  running: boolean;
  onExec: (deviceName: string, command: string) => void;
  onClear: () => void;
  entries: ConsoleEntry[];
  stepMode: boolean;
  onSetStepMode: (enabled: boolean) => void;
  onNextStep: () => void;
  waitingStep: boolean;
}

const MIN_HEIGHT = 100;
const MAX_HEIGHT = 600;

export function Console({ devices, running, onExec, onClear, entries, stepMode, onSetStepMode, onNextStep, waitingStep }: Props) {
  const [selectedDevice, setSelectedDevice] = useState("");
  const [input, setInput] = useState("");
  const [height, setHeight] = useState(220);
  const outputRef = useRef<HTMLDivElement>(null);
  const dragging = useRef(false);

  useEffect(() => {
    if (devices.length > 0 && !devices.find(d => d.name === selectedDevice)) {
      setSelectedDevice(devices[0].name);
    }
  }, [devices]);

  useEffect(() => {
    if (outputRef.current) {
      outputRef.current.scrollTop = outputRef.current.scrollHeight;
    }
  }, [entries]);

  const handleSubmit = (e: React.KeyboardEvent) => {
    if (e.key !== "Enter" || !input.trim() || !selectedDevice) return;
    const cmd = input.trim();
    if (cmd === "clear") {
      onClear();
      setInput("");
      return;
    }
    onExec(selectedDevice, cmd);
    setInput("");
  };

  const onDragStart = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    dragging.current = true;
    const startY = e.clientY;
    const startH = height;

    const onMove = (ev: MouseEvent) => {
      if (!dragging.current) return;
      const delta = startY - ev.clientY;
      setHeight(Math.min(MAX_HEIGHT, Math.max(MIN_HEIGHT, startH + delta)));
    };

    const onUp = () => {
      dragging.current = false;
      document.removeEventListener("mousemove", onMove);
      document.removeEventListener("mouseup", onUp);
    };

    document.addEventListener("mousemove", onMove);
    document.addEventListener("mouseup", onUp);
  }, [height]);

  return (
    <div className="ns-console" style={{ height }}>
      <div className="ns-console-resize" onMouseDown={onDragStart} />
      <div className="ns-console-header">
        <span>Console</span>
        {devices.length > 0 && (
          <select
            value={selectedDevice}
            onChange={e => setSelectedDevice(e.target.value)}
          >
            {devices.map(d => (
              <option key={d.id} value={d.name}>{d.name}</option>
            ))}
          </select>
        )}
        <button className="ns-console-clear" onClick={onClear} title="Effacer la console">
          clear
        </button>
        <div className="ns-console-right">
          {!running && <span className="ns-console-stopped">Simulation arrêtée</span>}
          <label className="ns-step-toggle">
            <input
              type="checkbox"
              checked={stepMode}
              onChange={e => onSetStepMode(e.target.checked)}
              disabled={running}
            />
            <span>Pas à pas</span>
          </label>
          {stepMode && running && (
            <button className="ns-step-btn" onClick={onNextStep} disabled={waitingStep}>
              {waitingStep ? "..." : "Suivant →"}
            </button>
          )}
        </div>
      </div>

      <div className="ns-console-output" ref={outputRef}>
        {entries.map((entry, i) => (
          <div key={i} className={`console-${entry.type}`}>{entry.text}</div>
        ))}
      </div>

      <div className="ns-console-input">
        <span className="ns-console-prompt">{selectedDevice || "..."}&gt;</span>
        <input
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={handleSubmit}
          placeholder={running ? "ping 10.0.1.2, arping 10.0.1.2, arp, ifconfig, clear, help..." : "Démarrez la simulation d'abord"}
          disabled={!running || devices.length === 0}
        />
      </div>
    </div>
  );
}
