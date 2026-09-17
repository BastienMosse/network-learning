import "./DemoStepTable.css";

import type { DemoStepTable } from "../../../../../types/course/section/blocks/demo/demo_step/DemoStepTable";

export function DemoStepTable ({ table }: { table?: DemoStepTable }) {
  return (
    <>
      {table && (
        <div className="demo-table-wrap">
          <table className="demo-table">
            <thead>
              <tr>
                {table.headers.map((header) => <th key={header}>{header}</th>)}
              </tr>
            </thead>
            <tbody>
              {table.rows.map((row, rowIndex) => (
                <tr key={`row-${rowIndex}`}>
                  {row.map((cell, cellIndex) => <td key={`cell-${rowIndex}-${cellIndex}`}>{cell}</td>)}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </>
  )
}