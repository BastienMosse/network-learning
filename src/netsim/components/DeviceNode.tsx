import { useRef } from "react";
import type { NetDevice } from "../types";
import type { ToolMode } from "./Toolbar";
import { DEVICE_LABELS, DEVICE_COLORS } from "../types";
import "./DeviceNode.css";

interface Props {
  device: NetDevice;
  selected: boolean;
  toolMode: ToolMode;
  onSelect: (id: string) => void;
  onMove: (id: string, x: number, y: number) => void;
  onCableClick: (deviceId: string, screenX: number, screenY: number) => void;
  onDelete: (id: string) => void;
}

export function DeviceNode({ device, selected, toolMode, onSelect, onMove, onCableClick, onDelete }: Props) {
  const dragging = useRef(false);
  const moved = useRef(false);
  const offset = useRef({ x: 0, y: 0 });

  const colors = DEVICE_COLORS[device.type];

  const handlePointerDown = (e: React.PointerEvent) => {
    e.stopPropagation();

    if (toolMode !== "select") return;

    onSelect(device.id);
    dragging.current = true;
    moved.current = false;
    offset.current = {
      x: e.clientX - device.x,
      y: e.clientY - device.y,
    };
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
  };

  const handlePointerMove = (e: React.PointerEvent) => {
    if (!dragging.current) return;
    moved.current = true;
    onMove(device.id, e.clientX - offset.current.x, e.clientY - offset.current.y);
  };

  const handlePointerUp = () => {
    dragging.current = false;
  };

  const handleClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (toolMode === "cable") {
      const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
      const canvas = (e.currentTarget as HTMLElement).offsetParent;
      const canvasRect = canvas?.getBoundingClientRect() ?? rect;
      onCableClick(
        device.id,
        rect.left - canvasRect.left + rect.width / 2,
        rect.top - canvasRect.top + rect.height,
      );
    } else if (toolMode === "delete") {
      onDelete(device.id);
    }
  };

  const modeClass =
    toolMode === "cable" ? "ns-device--cable"
    : toolMode === "delete" ? "ns-device--delete"
    : "";

  return (
    <div
      className={`ns-device ${selected ? "ns-device--selected" : ""} ${modeClass}`}
      style={{
        left: device.x,
        top: device.y,
        background: colors.bg,
        boxShadow: `5px 5px 0 ${colors.accent}`,
      }}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onClick={handleClick}
    >
      <span className="ns-device-type">{DEVICE_LABELS[device.type]}</span>
      <span className="ns-device-name">{device.name}</span>
      {device.interfaces.length > 0 && (
        <span className="ns-device-badge">{device.interfaces.length} if</span>
      )}
    </div>
  );
}
