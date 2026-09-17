import type { ContentBlock } from "../../types/course/section/blocks/ContentBlock";
import { CalloutBlock } from "./blocks/CalloutBlock";
import { CodeBlock } from "./blocks/CodeBlock";
import { ImageBlock } from "./blocks/ImageBlock";
import { MermaidBlock } from "./blocks/MermaidBlock";
import { TextBlock } from "./blocks/TextBlock";
import { DemoBlock } from "./blocks/DemoBlock";
import { FrameBlock } from "./blocks/FrameBlock";

export function BlockRenderer({ block }: { block: ContentBlock }) {
    switch (block.type) {
        case "text": return <TextBlock block={block} />;
        case "info": return <CalloutBlock block={block} />;
        case "warning" : return <CalloutBlock block={block} />;
        case "code": return <CodeBlock block={block} />;
        case "mermaid": return <MermaidBlock block={block} />;
        case "image": return <ImageBlock block={block} />;
        case "demo": return <DemoBlock block={block} />;
        case "frame": return <FrameBlock block={block} />;
        default: return null
    }
}