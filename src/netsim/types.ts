export type DeviceType = "pc" | "hub" | "switch" | "bridge" | "router";

export interface NetInterface {
  id: string;
  name: string;
  ip: string;
  mask: number;
  mtu: number;
  mac: string;
}

export interface NetDevice {
  id: string;
  type: DeviceType;
  name: string;
  x: number;
  y: number;
  interfaces: NetInterface[];
}

export interface NetLink {
  id: string;
  from: { deviceId: string; ifaceId: string };
  to: { deviceId: string; ifaceId: string };
}

export interface Topology {
  devices: NetDevice[];
  links: NetLink[];
}

export const DEVICE_LABELS: Record<DeviceType, string> = {
  pc: "PC",
  hub: "Hub",
  switch: "Switch",
  bridge: "Bridge",
  router: "Router",
};

export const DEVICE_HAS_IP: Record<DeviceType, boolean> = {
  pc: true,
  hub: false,
  switch: false,
  bridge: false,
  router: true,
};

export const MAX_INTERFACES: Record<DeviceType, number> = {
  pc: 4,
  hub: 4,
  switch: 8,
  bridge: 2,
  router: 4,
};

export const DEVICE_COLORS: Record<DeviceType, { bg: string; accent: string }> = {
  pc: { bg: "var(--white)", accent: "var(--ink)" },
  hub: { bg: "var(--yellow)", accent: "#d7d6a6" },
  switch: { bg: "var(--blue)", accent: "#a8c6cf" },
  bridge: { bg: "var(--teal)", accent: "var(--teal-soft)" },
  router: { bg: "var(--coral)", accent: "#f1b59f" },
};
