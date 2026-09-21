import { useState, useRef, useEffect } from "react";
import type { NetDevice } from "../types";
import "./Console.css";

export interface ConsoleEntry {
  type: "cmd" | "out" | "err" | "info";
  text: string;
}

interface Props {
  devices: NetDevice[];
  running: boolean;
  onExec: (deviceName: string, command: string) => void;
  entries: ConsoleEntry[];
}

export function Console({ devices, running, onExec, entries }: Props) {
  const [selectedDevice, setSelectedDevice] = useState("");
  const [input, setInput] = useState("");
  const outputRef = useRef<HTMLDivElement>(null);

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
    onExec(selectedDevice, input.trim());
    setInput("");
  };

  return (
    <div className="ns-console">
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
        {!running && <span style={{ marginLeft: "auto", fontSize: 11, color: "var(--coral)" }}>Simulation arrêtée</span>}
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
          placeholder={running ? "ping 10.0.1.2, arp, ifconfig, help..." : "Démarrez la simulation d'abord"}
          disabled={!running || devices.length === 0}
        />
      </div>
    </div>
  );
}
