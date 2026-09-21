import { useEffect, useId, useState } from "react";
import "./MermaidBlock.css";

import type { MermaidBlock } from "../../../types/course/section/blocks/MermaidBlock";

export function MermaidBlock({ block }: { block: MermaidBlock }) {
  const diagramId = `mermaid-${useId().replace(/:/g, "")}`;
  const [svg, setSvg] = useState("");
  const [error, setError] = useState("");

  useEffect(() => {
    let cancelled = false;
    import("mermaid").then(({ default: mermaid }) => {
      mermaid.initialize({ startOnLoad: false, securityLevel: "strict", theme: "base", flowchart: { useMaxWidth: true } });
      return mermaid.parse(block.diagram).then(() => mermaid.render(diagramId, block.diagram));
    }).then(({ svg: renderedSvg }) => {
      if (!cancelled) {
        setSvg(renderedSvg);
        setError("");
      }
    }).catch(() => {
      if (!cancelled) {
        setSvg("");
        setError("Ce diagramme Mermaid contient une erreur de syntaxe.");
      }
    });
    return () => { cancelled = true; };
  }, [block.diagram, diagramId]);

  return (
    <figure className="mermaid-block">
      {block.title && <figcaption>{block.title}</figcaption>}
      {error ? <p className="mermaid-error">{error}</p> : svg ? <div className="mermaid-diagram" style={{ width: block.width || "100%", ...(block.height ? { height: block.height } : {}) }} dangerouslySetInnerHTML={{ __html: svg }} /> : <p className="mermaid-loading">Génération du diagramme...</p>}
    </figure>
  );
}