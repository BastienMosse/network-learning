import "./DemoStepMedia.css";

import type { DemoStepMedia } from "../../../../../types/course/section/blocks/demo/demo_step/DemoStepMedia";

export function DemoStepMedia ({ media }: { media?: DemoStepMedia }) {
  return (
    <>
      {media && ( 
        <div className="demo-image-stage">
          <figure className="demo-image-figure">
            <img src={media.src} alt={media.alt} />
            {media.caption && <figcaption>{media.caption}</figcaption>}
          </figure>
        </div>
      )}
    </>
  )
}