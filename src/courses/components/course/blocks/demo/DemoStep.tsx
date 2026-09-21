import React from "react";
import "./DemoStep.css";

import type { DemoStep } from "../../../../types/course/section/blocks/demo/DemoStep";
import { DemoStepMedia } from "./demo_step/DemoStepMedia";
import { DemoStepPacket } from "./demo_step/DemoStepPacket";
import { DemoStepTable } from "./demo_step/DemoStepTable";
import { DemoNetwork as DemoNetworkElement} from "./DemoNetwork";

export function DemoStep({
  step,
  stepIndex,
  stepNumber,
  nodePosition,
  children,
}: {
  step: DemoStep,
  stepIndex: number,
  stepNumber: number,
  nodePosition: (nodeId: string) => { x: number; y: number } | null,
  children: React.ReactElement<React.ComponentProps<typeof DemoNetworkElement>>,
}) {
  const packets = (
    <DemoStepPacket stepIndex={stepIndex} nodePosition={nodePosition} packets={step.packets} />
  );

  const network = React.cloneElement(children, {
    children: packets,
  });
  
  return (
    <div className="demo-stage">
      <DemoStepMedia media={step.media} />
      {network}
      <div className="demo-panel">
        <span className="panel-label">État actuel</span>
        <strong>{step.panel}</strong>
        <p>{step.description}</p>
        <DemoStepTable table={step.table} />
        <span className="panel-step">
          0{stepIndex + 1} <i>/</i> 0{stepNumber}
        </span>
      </div>
    </div>
  );
}