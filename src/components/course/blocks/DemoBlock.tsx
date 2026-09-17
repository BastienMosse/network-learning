import { useEffect, useState } from "react";
import "./DemoBlock.css";

import type { DemoBlock } from "../../../types/course/section/blocks/DemoBlock";
import { ArrowLeft, ArrowRight, Check, Pause, Play, RotateCcw } from "lucide-react";
import { DemoNetwork } from "./demo/DemoNetwork";
import { DemoStep } from "./demo/DemoStep";


export function DemoBlock({ block }: { block: DemoBlock }) {
  const [stepIndex, setStepIndex] = useState(0);
  const [playing, setPlaying] = useState(false);

  const step = block.steps[stepIndex];
  const nodes = block.network?.nodes ?? [];

  const nodePosition = (nodeId: string) => {
    const nodeIndex = nodes.findIndex((node) => node.id === nodeId);
    if (nodeIndex < 0) return null;
    const configuredPosition = nodes[nodeIndex].position;
    return configuredPosition || {
      x: ((nodeIndex * 2 + 1) / (nodes.length * 2)) * 100,
      y: 50,
    };
  };


  useEffect(() => {
    if (!playing) return;
    const timer = window.setTimeout(() => {
      if (stepIndex >= block.steps.length - 1) setPlaying(false);
      else setStepIndex(stepIndex + 1);
    }, 3000);
    return () => window.clearTimeout(timer);
  }, [playing, stepIndex, block.steps.length]);
  
  return (
    <section className="demo-block">
      <div className="demo-title">
        <div>
          <span className="demo-kicker">
            <span className="live-dot" /> Démo interactive
          </span>
          <h3>{block.title}</h3>
          <p>{block.intro}</p>
        </div>
        <button
          className="reset-button"
          onClick={() => {
            setStepIndex(0);
            setPlaying(false);
          }}
        >
          <RotateCcw size={15} /> Réinitialiser
        </button>
      </div>
      <DemoStep step={step} stepIndex={stepIndex} stepNumber={block.steps.length} nodePosition={nodePosition}>
        <DemoNetwork network={block.network} active={step.active} nodePosition={nodePosition} />
      </DemoStep>
      <div className="demo-controls">
        <div className="step-dots">
          {block.steps.map((item, index) => (
            <button
              key={item.title}
              className={
                index === stepIndex
                  ? "current"
                  : index < stepIndex
                    ? "done"
                    : ""
              }
              onClick={() => {
                setStepIndex(index);
                setPlaying(false);
              }}
              aria-label={`Aller à l'étape ${index + 1}`}
            >
              <span>{index < stepIndex ? <Check size={11} /> : index + 1}</span>
            </button>
          ))}
        </div>
        <div className="demo-actions">
          <button
            onClick={() => setStepIndex(Math.max(0, stepIndex - 1))}
            disabled={stepIndex === 0}
          >
            <ArrowLeft size={14} />
          </button>
          <button className="play-button" onClick={() => setPlaying(!playing)}>
            {playing ? (
              <>
                <Pause size={13} fill="currentColor" /> Pause
              </>
            ) : (
              <>
                <Play size={13} fill="currentColor" /> Lire
              </>
            )}
          </button>
          <button
            onClick={() =>
              setStepIndex(Math.min(block.steps.length - 1, stepIndex + 1))
            }
            disabled={stepIndex === block.steps.length - 1}
          >
            <ArrowRight size={14} />
          </button>
        </div>
      </div>
    </section>
  );
}