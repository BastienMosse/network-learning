import "./DemoNetworkNode.css";

import type { DemoNetworkNode } from "../../../../../types/course/section/blocks/demo/demo_network/DemoNetworkNode"

export function DemoNetworkNode({
  nodes,
  active = [],
  nodePosition,
}: {
  nodes: DemoNetworkNode[],
  active?: string[],
  nodePosition: (nodeId: string) => { x: number; y: number } | null,
}) {
  return (
    <>
      {nodes.map((node) => (
        <div
          key={node.id}
          className={`demo-node ${node.kind} ${active.includes(node.id) ? "active" : ""}`}
          style={
            {
              left: `${nodePosition(node.id)?.x ?? 50}%`,
              top: `${nodePosition(node.id)?.y ?? 50}%`,
            } as React.CSSProperties
          }
        >
          <span className="node-symbol">
            {node.kind === "service" ? "◆" : "□"}
          </span>
          <strong>{node.label}</strong>
          <small>
            {node.kind === "service" ? "service" : "équipement"}
          </small>
        </div>
      ))}
    </>
  )
}