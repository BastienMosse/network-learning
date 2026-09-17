import "./DemoNetworkConnection.css";

import type { DemoNetworkConnection } from "../../../../../types/course/section/blocks/demo/demo_network/DemoNetworkConnection";

export function DemoNetworkConnection ({
  connections,
  nodePosition,
}: {
  connections: DemoNetworkConnection[],
  nodePosition: (nodeId: string) => { x: number; y: number } | null,
}) {
  return (
    <svg className="demo-connections" viewBox="0 0 100 100" preserveAspectRatio="none" aria-hidden="true">
      {connections.map((connection) => {
        const from = nodePosition(connection.from);
        const to = nodePosition(connection.to);
        if (!from || !to) return null;
        return <line key={`${connection.from}-${connection.to}`} x1={from.x} y1={from.y} x2={to.x} y2={to.y} />;
      })}
    </svg>
  )
}