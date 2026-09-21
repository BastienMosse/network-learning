import "./CalloutBlock.css";

import type { CalloutBlock } from "../../../types/course/section/blocks/CalloutBlock";

export function CalloutBlock({ block }: { block: CalloutBlock }) {
  return (
    <aside className={`callout ${block.type}`}>
      <span className="callout-tag">
        {block.type === "info" ? "À retenir" : "Point de vigilance"}
      </span>
      <strong>{block.title}</strong>
      <p>{block.content}</p>
    </aside>
  );
}