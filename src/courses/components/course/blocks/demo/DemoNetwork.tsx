import "./DemoNetwork.css";

import { DemoNetworkSegment } from "./demo_network/DemoNetworkSegment";
import { DemoNetworkNode } from "./demo_network/DemoNetworkNode";
import { DemoNetworkConnection } from "./demo_network/DemoNetworkConnection";
import type { DemoNetwork } from "../../../../types/course/section/blocks/demo/DemoNetwork";

export function DemoNetwork ({
  network,
  active,
  nodePosition,
  children,
}: {
  network?: DemoNetwork,
  active?: string[],
  nodePosition: (nodeId: string) => { x: number; y: number } | null,
  children?: React.ReactNode,
}) {
  return (
    <>
      {network && (
        <div className="demo-network">
          <DemoNetworkSegment segments={network.segments} />
          <DemoNetworkNode nodes={network.nodes ?? []} active={active} nodePosition={nodePosition} />
          <DemoNetworkConnection connections={network.connections ?? []} nodePosition={nodePosition} />
          {children}
        </div>
      )}
    </>
  )
}