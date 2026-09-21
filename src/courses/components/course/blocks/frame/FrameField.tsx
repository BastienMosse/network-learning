import "./FrameField.css";

import type { FrameField } from "../../../../types/course/section/blocks/frame/FrameField";

export type FramePart = {
  field: FrameField;
  bits: number;
  offset: number;
};

export function FrameFields({
  fields,
  bitsPerRow,
}: {
  fields: FramePart[];
  bitsPerRow: number;
}) {
  return (
    <div className="frame-row">
      {fields.map(({ field, bits, offset }) => {
        const variable = !!field.variable;
        const continued = offset > 0;

        return (
          <div
            key={`${field.id}-${offset}`}
            className={[
              "frame-cell",
              field.color ?? "blue",
              variable && "frame-variable",
              continued && "frame-continued",
            ]
              .filter(Boolean)
              .join(" ")}
            style={{
              flexBasis: `${(bits / bitsPerRow) * 100}%`,
              ...(variable && {
                backgroundImage:
                  "repeating-linear-gradient(45deg, rgba(0,0,0,0.06) 0 8px, transparent 8px 16px)",
              }),
            }}
            title={field.description}
          >
            <strong>
              {field.label}
              {continued && " (suite)"}
            </strong>

            <small>
              {variable
                ? "longueur variable"
                : field.bits <= bitsPerRow
                  ? `${field.bits} bit${field.bits > 1 ? "s" : ""}`
                  : `${offset}-${offset + bits - 1} / ${field.bits} bits`}
            </small>
          </div>
        );
      })}
    </div>
  );
}