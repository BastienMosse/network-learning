import type { DemoNetworkConnection } from "./demo_network/DemoNetworkConnection"
import type { DemoNetworkNode } from "./demo_network/DemoNetworkNode"
import type { DemoNetworkSegment } from "./demo_network/DemoNetworkSegment"


export type DemoNetwork = {
    nodes?: DemoNetworkNode[]
    connections?: DemoNetworkConnection[]
    segments?: DemoNetworkSegment[]
}