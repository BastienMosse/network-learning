import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import "./TextBlock.css";

import type { TextBlock } from "../../../types/course/section/blocks/TextBlock"

export function TextBlock({ block }: { block: TextBlock }) {
  return (
    <section className="text-block">
      {block.title && <h3>{block.title}</h3>}
      <div className="rich-text">
        <ReactMarkdown remarkPlugins={[remarkGfm]}>{block.content}</ReactMarkdown>
      </div>
    </section>
  );
}