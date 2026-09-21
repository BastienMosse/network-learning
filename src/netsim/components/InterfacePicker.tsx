import type { NetDevice, NetLink } from "../types";
import { DEVICE_LABELS } from "../types";
import "./InterfacePicker.css";

interface Props {
  device: NetDevice;
  links: NetLink[];
  position: { x: number; y: number };
  onPick: (deviceId: string, ifaceId: string) => void;
  onClose: () => void;
}

export function InterfacePicker({ device, links, position, onPick, onClose }: Props) {
  const isLinked = (ifaceId: string) =>
    links.some(
      l =>
        (l.from.deviceId === device.id && l.from.ifaceId === ifaceId) ||
        (l.to.deviceId === device.id && l.to.ifaceId === ifaceId),
    );

  const available = device.interfaces.filter(i => !isLinked(i.id));

  return (
    <>
      <div className="ns-picker-overlay" onClick={onClose} />
      <div
        className="ns-picker"
        style={{ left: position.x, top: position.y }}
      >
        <div className="ns-picker-header">
          <span>{DEVICE_LABELS[device.type]}</span>
          <strong>{device.name}</strong>
        </div>

        {available.length === 0 ? (
          <p className="ns-picker-empty">
            {device.interfaces.length === 0
              ? "Aucune interface"
              : "Toutes les interfaces sont liées"}
          </p>
        ) : (
          <div className="ns-picker-list">
            {available.map((iface) => (
              <button
                key={iface.id}
                className="ns-picker-item"
                onClick={() => onPick(device.id, iface.id)}
              >
                <strong>{iface.name}</strong>
                {iface.ip && <span>{iface.ip}/{iface.mask}</span>}
              </button>
            ))}
          </div>
        )}
      </div>
    </>
  );
}
