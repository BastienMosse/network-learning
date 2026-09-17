import "./DemoStepPacket.css";

import type { DemoStepPacket } from "../../../../../types/course/section/blocks/demo/demo_step/DemoStepPacket"

export function DemoStepPacket ({
  packets,
  stepIndex,
  nodePosition,
}: {
  packets?: DemoStepPacket[][],
  stepIndex: number,
  nodePosition: (nodeId: string) => { x: number; y: number } | null,
}) {
  return (
    <>
      {packets?.map((group, groupIndex) => 
        group.map((packet, packetIndex) => {
          const packetFrom = nodePosition(packet.from);
          const packetTo = nodePosition(packet.to);
          if (!packetFrom || !packetTo) return null;
          return (
            <div
              key={`${stepIndex}-${groupIndex}-${packetIndex}-${packet.from}-${packet.to}`}
              className={`moving-packet ${packet.keep ? "packet-keep" : "packet-hide"}`}
              style={
                {
                  "--packet-from-x": `${packetFrom.x}%`,
                  "--packet-from-y": `${packetFrom.y}%`,
                  "--packet-to-x": `${packetTo.x}%`,
                  "--packet-to-y": `${packetTo.y}%`,
                  "--packet-delay": `${groupIndex * 950}ms`,
                } as React.CSSProperties
              }
            >
              <span/>
            </div>
          );
        })
      )}
    </>
  )
}