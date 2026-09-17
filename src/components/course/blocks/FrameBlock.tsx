import "./FrameBlock.css";

import type { FrameField } from "../../../types/course/section/blocks/frame/FrameField";
import type { FrameBlock as FrameBlockData } from "../../../types/course/section/blocks/FrameBlock";
import { FrameFields } from "./frame/FrameField";

type FramePart = {
  field: FrameField;
  bits: number;
  offset: number;
};

function layoutFrameRows(
  fields: FrameField[],
  bitsPerRow: number,
): FramePart[][] {
  const rows: FramePart[][] = [];
  let row: FramePart[] = [];
  let used = 0;

  const addRow = () => {
    if (!row.length) return;

    const remaining = bitsPerRow - used;
    const variables = row.filter((part) => part.field.variable);

    if (remaining > 0 && variables.length) {
      variables.forEach((part, i) => {
        const share =
          i === variables.length - 1
            ? remaining -
              variables
                .slice(0, i)
                .reduce((sum, p) => sum + (p.bits - p.field.bits), 0)
            : Math.floor(remaining / variables.length);

        part.bits += share;
      });
    }

    rows.push(row);
    row = [];
    used = 0;
  };

  for (const field of fields) {
    let remaining = field.bits;
    let offset = 0;

    if (field.variable && remaining === 0) {
      row.push({ field, bits: 0, offset: 0 });
      continue;
    }

    while (remaining > 0) {
      const bits = Math.min(bitsPerRow - used, remaining);

      row.push({ field, bits, offset });

      used += bits;
      offset += bits;
      remaining -= bits;

      if (used === bitsPerRow) addRow();
    }
  }

  addRow();

  return rows;
}

export function FrameBlock({
  block,
}: {
  block: FrameBlockData;
}) {
  const bitsPerRow = block.bitsPerRow ?? 32;
  const rows = layoutFrameRows(block.fields, bitsPerRow);

  return (
    <figure className="frame-block">
        {block.title && <figcaption>{block.title}</figcaption>}
        <div className="frame-ruler">
            {Array.from(
                { length: bitsPerRow / 8 + 1 }, (_, i) => i * 8
            ).map((bit) => (
                <span key={bit} style={{left: `${(bit / bitsPerRow) * 100}%`,}}>
                    {bit}
                </span>
            ))}
        </div>
        {rows.map((fields, i) => (
            <FrameFields key={i} fields={fields} bitsPerRow={bitsPerRow} />
        ))}
    </figure>
  );
}