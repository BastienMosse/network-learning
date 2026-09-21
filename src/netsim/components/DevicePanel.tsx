import { useState } from "react";
import { Plus, Trash2, Link, Pencil, Check, X } from "lucide-react";
import type { NetDevice, NetLink } from "../types";
import { DEVICE_LABELS, DEVICE_HAS_IP, MAX_INTERFACES } from "../types";
import "./DevicePanel.css";

interface Props {
  device: NetDevice;
  links: NetLink[];
  allDevices: NetDevice[];
  linkMode: { active: boolean; from?: { deviceId: string; ifaceId: string } };
  onAddInterface: (deviceId: string, name: string, ip: string, mask: number, mtu: number) => void;
  onEditInterface: (deviceId: string, ifaceId: string, name: string, ip: string, mask: number, mtu: number) => void;
  onRenameDevice: (deviceId: string, newName: string) => void;
  onDeleteDevice: (deviceId: string) => void;
  onStartLink: (deviceId: string, ifaceId: string) => void;
  onClose: () => void;
}

function isValidIp(ip: string): boolean {
  const parts = ip.split(".");
  if (parts.length !== 4) return false;
  return parts.every(p => {
    const n = parseInt(p);
    return !isNaN(n) && n >= 0 && n <= 255 && String(n) === p;
  });
}

function isValidMask(mask: string): boolean {
  const n = parseInt(mask);
  return !isNaN(n) && n >= 0 && n <= 32;
}

function isValidMtu(mtu: string): boolean {
  const n = parseInt(mtu);
  return !isNaN(n) && n >= 68 && n <= 9000;
}

function isValidName(name: string): boolean {
  return name.trim().length > 0 && /^[a-zA-Z0-9_-]+$/.test(name.trim());
}

export function DevicePanel({
  device, links, allDevices, linkMode,
  onAddInterface, onEditInterface, onRenameDevice, onDeleteDevice, onStartLink, onClose,
}: Props) {
  const [ifName, setIfName] = useState("");
  const [ifIp, setIfIp] = useState("");
  const [ifMask, setIfMask] = useState("24");
  const [ifMtu, setIfMtu] = useState("1500");
  const [addError, setAddError] = useState("");

  const [editingName, setEditingName] = useState(false);
  const [deviceName, setDeviceName] = useState(device.name);
  const [nameError, setNameError] = useState("");

  const [editingId, setEditingId] = useState<string | null>(null);
  const [editName, setEditName] = useState("");
  const [editIp, setEditIp] = useState("");
  const [editMask, setEditMask] = useState("");
  const [editMtu, setEditMtu] = useState("");
  const [editError, setEditError] = useState("");

  const hasIp = DEVICE_HAS_IP[device.type];
  const maxIfaces = MAX_INTERFACES[device.type];
  const atLimit = device.interfaces.length >= maxIfaces;

  const handleAdd = () => {
    setAddError("");
    const name = ifName.trim();
    if (!isValidName(name)) {
      setAddError("Nom invalide (lettres, chiffres, - ou _)");
      return;
    }
    if (device.interfaces.some(i => i.name === name)) {
      setAddError("Ce nom d'interface existe déjà");
      return;
    }
    if (hasIp) {
      if (!isValidIp(ifIp)) {
        setAddError("Adresse IP invalide");
        return;
      }
      if (!isValidMask(ifMask)) {
        setAddError("Masque invalide (0-32)");
        return;
      }
    }
    if (!isValidMtu(ifMtu)) {
      setAddError("MTU invalide (68-9000)");
      return;
    }
    onAddInterface(device.id, name, hasIp ? ifIp : "", hasIp ? (parseInt(ifMask) || 24) : 0, parseInt(ifMtu) || 1500);
    setIfName("");
    setIfIp("");
    setAddError("");
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") handleAdd();
  };

  const startEdit = (iface: { id: string; name: string; ip: string; mask: number; mtu: number }) => {
    setEditingId(iface.id);
    setEditName(iface.name);
    setEditIp(iface.ip);
    setEditMask(String(iface.mask));
    setEditMtu(String(iface.mtu));
    setEditError("");
  };

  const cancelEdit = () => {
    setEditingId(null);
    setEditError("");
  };

  const saveEdit = () => {
    setEditError("");
    const name = editName.trim();
    if (!isValidName(name)) {
      setEditError("Nom invalide");
      return;
    }
    const existing = device.interfaces.find(i => i.name === name && i.id !== editingId);
    if (existing) {
      setEditError("Ce nom existe déjà");
      return;
    }
    if (hasIp && !isValidIp(editIp)) {
      setEditError("IP invalide");
      return;
    }
    if (hasIp && !isValidMask(editMask)) {
      setEditError("Masque invalide");
      return;
    }
    if (!isValidMtu(editMtu)) {
      setEditError("MTU invalide");
      return;
    }
    onEditInterface(
      device.id,
      editingId!,
      name,
      hasIp ? editIp : "",
      hasIp ? (parseInt(editMask) || 0) : 0,
      parseInt(editMtu) || 1500,
    );
    setEditingId(null);
  };

  const handleEditKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") saveEdit();
    if (e.key === "Escape") cancelEdit();
  };

  const findPeer = (deviceId: string, ifaceId: string) => {
    for (const link of links) {
      if (link.from.deviceId === deviceId && link.from.ifaceId === ifaceId) {
        const peer = allDevices.find(d => d.id === link.to.deviceId);
        const peerIf = peer?.interfaces.find(i => i.id === link.to.ifaceId);
        return peer && peerIf ? `${peer.name}:${peerIf.name}` : null;
      }
      if (link.to.deviceId === deviceId && link.to.ifaceId === ifaceId) {
        const peer = allDevices.find(d => d.id === link.from.deviceId);
        const peerIf = peer?.interfaces.find(i => i.id === link.from.ifaceId);
        return peer && peerIf ? `${peer.name}:${peerIf.name}` : null;
      }
    }
    return null;
  };

  const isIfaceLinked = (ifaceId: string) => {
    return links.some(
      l =>
        (l.from.deviceId === device.id && l.from.ifaceId === ifaceId) ||
        (l.to.deviceId === device.id && l.to.ifaceId === ifaceId),
    );
  };

  const isLinkSource = (ifaceId: string) =>
    linkMode.active &&
    linkMode.from?.deviceId === device.id &&
    linkMode.from?.ifaceId === ifaceId;

  return (
    <aside className="ns-panel">
      <div className="ns-panel-header">
        <div>
          <span className="eyebrow">{DEVICE_LABELS[device.type]}</span>
          {editingName ? (
            <div className="ns-panel-name-edit">
              <input
                value={deviceName}
                onChange={e => setDeviceName(e.target.value)}
                onKeyDown={e => {
                  if (e.key === "Enter") {
                    const n = deviceName.trim();
                    if (!isValidName(n)) {
                      setNameError("Nom invalide");
                      return;
                    }
                    if (n !== device.name && allDevices.some(d => d.name === n)) {
                      setNameError("Ce nom existe déjà");
                      return;
                    }
                    onRenameDevice(device.id, n);
                    setEditingName(false);
                    setNameError("");
                  }
                  if (e.key === "Escape") {
                    setDeviceName(device.name);
                    setEditingName(false);
                    setNameError("");
                  }
                }}
                autoFocus
              />
              <button className="ns-iface-edit-btn save" onClick={() => {
                const n = deviceName.trim();
                if (!isValidName(n)) { setNameError("Nom invalide"); return; }
                if (n !== device.name && allDevices.some(d => d.name === n)) { setNameError("Ce nom existe déjà"); return; }
                onRenameDevice(device.id, n);
                setEditingName(false);
                setNameError("");
              }}><Check size={13} /></button>
              <button className="ns-iface-edit-btn cancel" onClick={() => {
                setDeviceName(device.name);
                setEditingName(false);
                setNameError("");
              }}><X size={13} /></button>
            </div>
          ) : (
            <div className="ns-panel-name-row">
              <h3>{device.name}</h3>
              <button className="ns-iface-edit-btn" onClick={() => { setDeviceName(device.name); setEditingName(true); }} title="Renommer">
                <Pencil size={13} />
              </button>
            </div>
          )}
          {nameError && <span className="ns-iface-error">{nameError}</span>}
        </div>
        <button className="ns-panel-close" onClick={onClose}>&times;</button>
      </div>

      <div className="ns-panel-section">
        <span className="ns-panel-label">Interfaces</span>
        {device.interfaces.length === 0 && (
          <p className="ns-panel-empty">Aucune interface</p>
        )}
        {device.interfaces.map((iface) => {
          const peer = findPeer(device.id, iface.id);
          const linked = isIfaceLinked(iface.id);

          if (editingId === iface.id) {
            return (
              <div key={iface.id} className="ns-iface-edit">
                <div className="ns-iface-edit-row">
                  <input
                    value={editName}
                    onChange={e => setEditName(e.target.value)}
                    onKeyDown={handleEditKeyDown}
                    placeholder="Nom"
                    style={{ width: "80px" }}
                  />
                  {hasIp && (
                    <>
                      <input
                        value={editIp}
                        onChange={e => setEditIp(e.target.value)}
                        onKeyDown={handleEditKeyDown}
                        placeholder="IP"
                        style={{ width: "110px" }}
                      />
                      <input
                        value={editMask}
                        onChange={e => setEditMask(e.target.value)}
                        onKeyDown={handleEditKeyDown}
                        placeholder="CIDR"
                        style={{ width: "40px" }}
                      />
                    </>
                  )}
                  <input
                    value={editMtu}
                    onChange={e => setEditMtu(e.target.value)}
                    onKeyDown={handleEditKeyDown}
                    placeholder="MTU"
                    style={{ width: "55px" }}
                  />
                  <button className="ns-iface-edit-btn save" onClick={saveEdit} title="Sauvegarder">
                    <Check size={13} />
                  </button>
                  <button className="ns-iface-edit-btn cancel" onClick={cancelEdit} title="Annuler">
                    <X size={13} />
                  </button>
                </div>
                {editError && <span className="ns-iface-error">{editError}</span>}
              </div>
            );
          }

          return (
            <div key={iface.id} className="ns-iface-row">
              <div className="ns-iface-info">
                <strong>{iface.name}</strong>
                {iface.ip && <span>{iface.ip}/{iface.mask}</span>}
                <span className="ns-iface-mtu">MTU {iface.mtu}</span>
              </div>
              {peer && <span className="ns-iface-peer">{peer}</span>}
              <button
                className="ns-iface-edit-btn"
                onClick={() => startEdit(iface)}
                title="Modifier"
              >
                <Pencil size={13} />
              </button>
              {!linked && (
                <button
                  className={`ns-iface-link-btn ${isLinkSource(iface.id) ? "active" : ""}`}
                  onClick={() => onStartLink(device.id, iface.id)}
                  title="Lier cette interface"
                >
                  <Link size={13} />
                </button>
              )}
            </div>
          );
        })}
      </div>

      <div className="ns-panel-section">
        <span className="ns-panel-label">
          Ajouter une interface ({device.interfaces.length}/{maxIfaces})
        </span>
        {atLimit ? (
          <p className="ns-panel-empty">Limite d'interfaces atteinte</p>
        ) : (
        <div className="ns-panel-form">
          <input
            placeholder={hasIp ? "Nom (eth0)" : "Nom (port1)"}
            value={ifName}
            onChange={(e) => setIfName(e.target.value)}
            onKeyDown={handleKeyDown}
          />
          {hasIp && (
            <>
              <input
                placeholder="IP (192.168.1.1)"
                value={ifIp}
                onChange={(e) => setIfIp(e.target.value)}
                onKeyDown={handleKeyDown}
              />
              <div className="ns-panel-form-row">
                <input
                  placeholder="Masque"
                  value={ifMask}
                  onChange={(e) => setIfMask(e.target.value)}
                  onKeyDown={handleKeyDown}
                  style={{ width: "60px" }}
                />
                <input
                  placeholder="MTU"
                  value={ifMtu}
                  onChange={(e) => setIfMtu(e.target.value)}
                  onKeyDown={handleKeyDown}
                  style={{ width: "80px" }}
                />
                <button className="ns-panel-add-btn" onClick={handleAdd}>
                  <Plus size={14} />
                </button>
              </div>
            </>
          )}
          {!hasIp && (
            <div className="ns-panel-form-row">
              <input
                placeholder="MTU"
                value={ifMtu}
                onChange={(e) => setIfMtu(e.target.value)}
                onKeyDown={handleKeyDown}
                style={{ width: "80px" }}
              />
              <button className="ns-panel-add-btn" onClick={handleAdd}>
                <Plus size={14} />
              </button>
            </div>
          )}
          {addError && <span className="ns-iface-error">{addError}</span>}
        </div>
        )}
      </div>

      <button className="ns-panel-delete" onClick={() => onDeleteDevice(device.id)}>
        <Trash2 size={14} /> Supprimer
      </button>
    </aside>
  );
}
