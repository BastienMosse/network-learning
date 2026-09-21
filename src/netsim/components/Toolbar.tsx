import { Monitor, Network, GitFork, Unplug, Router, Cable, Trash2, Play, Square, RotateCcw } from "lucide-react";
import type { DeviceType } from "../types";
import "./Toolbar.css";

const ITEMS: { type: DeviceType; label: string; icon: React.ReactNode }[] = [
  { type: "pc", label: "PC", icon: <Monitor size={18} /> },
  { type: "hub", label: "Hub", icon: <Unplug size={18} /> },
  { type: "bridge", label: "Bridge", icon: <GitFork size={18} /> },
  { type: "switch", label: "Switch", icon: <Network size={18} /> },
  { type: "router", label: "Router", icon: <Router size={18} /> },
];

export type ToolMode = "select" | "cable" | "delete";

interface Props {
  onAdd: (type: DeviceType) => void;
  toolMode: ToolMode;
  onSetTool: (mode: ToolMode) => void;
  running: boolean;
  onStart: () => void;
  onStop: () => void;
  onReset: () => void;
}

export function Toolbar({ onAdd, toolMode, onSetTool, running, onStart, onStop, onReset }: Props) {
  const toggle = (mode: ToolMode) =>
    onSetTool(toolMode === mode ? "select" : mode);

  return (
    <div className="ns-toolbar">
      <span className="ns-toolbar-label">Ajouter</span>
      {ITEMS.map((item) => (
        <button
          key={item.type}
          className="ns-toolbar-btn"
          onClick={() => onAdd(item.type)}
          title={item.label}
          disabled={running}
        >
          {item.icon}
          <span>{item.label}</span>
        </button>
      ))}

      <span className="ns-toolbar-sep" />

      <button
        className={`ns-toolbar-btn ns-toolbar-cable ${toolMode === "cable" ? "active" : ""}`}
        onClick={() => toggle("cable")}
        title="Relier deux interfaces"
        disabled={running}
      >
        <Cable size={18} />
        <span>Cable</span>
      </button>

      <button
        className={`ns-toolbar-btn ns-toolbar-delete ${toolMode === "delete" ? "active" : ""}`}
        onClick={() => toggle("delete")}
        title="Supprimer un élément"
        disabled={running}
      >
        <Trash2 size={18} />
        <span>Supprimer</span>
      </button>

      <span className="ns-toolbar-sep" />

      {!running ? (
        <button
          className="ns-toolbar-btn ns-toolbar-start"
          onClick={onStart}
          title="Démarrer la simulation"
        >
          <Play size={18} />
          <span>Start</span>
        </button>
      ) : (
        <button
          className="ns-toolbar-btn ns-toolbar-stop"
          onClick={onStop}
          title="Arrêter la simulation"
        >
          <Square size={18} />
          <span>Stop</span>
        </button>
      )}

      <button
        className="ns-toolbar-btn"
        onClick={onReset}
        title="Réinitialiser"
      >
        <RotateCcw size={18} />
        <span>Reset</span>
      </button>
    </div>
  );
}
