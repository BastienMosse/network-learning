import "./ImageBlock.css";

import type { ImageBlock } from "../../../types/course/section/blocks/ImageBlock";

export function ImageBlock({ block }: { block: ImageBlock }) {
  return (
    <figure className="image-block">
      {block.title && <h3>{block.title}</h3>}
      <img src={block.src} alt={block.alt || block.title || "Illustration du cours"} />
      <figcaption>{block.caption}</figcaption>
    </figure>
  );
}