import "./DemoNetworkSegment.css";

import type { DemoNetworkSegment } from "../../../../../types/course/section/blocks/demo/demo_network/DemoNetworkSegment"

export function DemoNetworkSegment({ segments }: { segments: DemoNetworkSegment[] | undefined }) {
  return (
    <>
      {segments?.map((segment) => (
        <div
          key={segment.id}
          className={`demo-segment ${segment.color || "blue"}`}
          style={{
            left: `${segment.x}%`,
            top: `${segment.y}%`,
            width: `${segment.width}%`,
            height: `${segment.height}%`,
          }}
        >
          <span>{segment.label}</span>
        </div>
      ))}
    </>
  )
}