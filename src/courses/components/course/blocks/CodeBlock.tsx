import { TerminalSquare } from "lucide-react";
import "./CodeBlock.css";

import type { CodeBlock } from "../../../types/course/section/blocks/CodeBlock";

export function CodeBlock ({ block }: { block: CodeBlock }) {
  return (
    <div className="code-block">
      <div>
        <TerminalSquare size={16} />
        <span>{block.language || "exemple"}</span>
      </div>
      <pre>{block.code}</pre>
    </div>
  );
}