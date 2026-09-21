import type { NetDevice, NetLink } from "../types";
import type { ToolMode } from "./Toolbar";
import { DeviceNode } from "./DeviceNode";
import { InterfacePicker } from "./InterfacePicker";
import "./Canvas.css";

interface PickerState {
  deviceId: string;
  position: { x: number; y: number };
}

interface Props {
  devices: NetDevice[];
  links: NetLink[];
  selectedId: string | null;
  toolMode: ToolMode;
  pickerState: PickerState | null;
  onSelect: (id: string) => void;
  onDeselect: () => void;
  onMove: (id: string, x: number, y: number) => void;
  onDeviceClick: (deviceId: string, screenX: number, screenY: number) => void;
  onPickInterface: (deviceId: string, ifaceId: string) => void;
  onClosePicker: () => void;
  onDeleteDevice: (id: string) => void;
}

export function Canvas({
  devices, links, selectedId, toolMode, pickerState,
  onSelect, onDeselect, onMove,
  onDeviceClick, onPickInterface, onClosePicker, onDeleteDevice,
}: Props) {
  const getDeviceCenter = (deviceId: string) => {
    const device = devices.find(d => d.id === deviceId);
    if (!device) return null;
    return { x: device.x, y: device.y };
  };

  const pickerDevice = pickerState
    ? devices.find(d => d.id === pickerState.deviceId) ?? null
    : null;

  return (
    <div className="ns-canvas" onPointerDown={(e) => {
      if (e.target === e.currentTarget) {
        onDeselect();
        onClosePicker();
      }
    }}>
      <svg className="ns-canvas-svg">
        {links.map((link) => {
          const from = getDeviceCenter(link.from.deviceId);
          const to = getDeviceCenter(link.to.deviceId);
          if (!from || !to) return null;
          return (
            <line
              key={link.id}
              x1={from.x} y1={from.y}
              x2={to.x} y2={to.y}
              className="ns-link-line"
            />
          );
        })}
      </svg>

      {devices.map((device) => (
        <DeviceNode
          key={device.id}
          device={device}
          selected={device.id === selectedId}
          toolMode={toolMode}
          onSelect={onSelect}
          onMove={onMove}
          onCableClick={onDeviceClick}
          onDelete={onDeleteDevice}
        />
      ))}

      {pickerDevice && pickerState && (
        <InterfacePicker
          device={pickerDevice}
          links={links}
          position={pickerState.position}
          onPick={onPickInterface}
          onClose={onClosePicker}
        />
      )}

      {devices.length === 0 && (
        <div className="ns-canvas-empty">
          Ajoutez des machines avec la barre d'outils ci-dessus
        </div>
      )}
    </div>
  );
}
