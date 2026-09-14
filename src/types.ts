export type Theme = {
  id: string
  title: string
  eyebrow: string
  description: string
  accent: string
  courseIds: string[]
}

export type CourseSummary = {
  id: string
  slug: string
  title: string
  description: string
  level: string
  duration: string
  themeId: string
  color: string
}

export type TextBlock = {
  type: 'text'
  title?: string
  content: string
}

export type ImageBlock = {
  type: 'image'
  title?: string
  src: string
  alt?: string
  caption?: string
}

export type CalloutBlock = {
  type: 'info' | 'warning'
  title: string
  content: string
}

export type CodeBlock = {
  type: 'code'
  language?: string
  code: string
}

export type MermaidBlock = {
  type: 'mermaid'
  title?: string
  diagram: string
  width?: string
  height?: string
}

export type DemoNode = {
  id: string
  label: string
  kind: 'device' | 'service'
  position?: { x: number; y: number }
}

export type DemoConnection = {
  from: string
  to: string
}

export type DemoSegment = {
  id: string
  label: string
  x: number
  y: number
  width: number
  height: number
  color?: 'coral' | 'blue' | 'green' | 'yellow' | 'teal'
}

export type DemoMedia = {
  src: string
  alt: string
  caption?: string
}

export type DemoTable = {
  headers: string[]
  rows: string[][]
}

export type DemoPacket = {
  from: string
  to: string
}

export type DemoStep = {
  title: string
  description: string
  active?: string[]
  packet?: DemoPacket
  packets?: DemoPacket[]
  packetGroups?: DemoPacket[][]
  sequential?: boolean
  panel?: string
  media?: DemoMedia
  table?: DemoTable
}

export type DemoBlock = {
  type: 'demo'
  title: string
  intro?: string
  nodes?: DemoNode[]
  connections?: DemoConnection[]
  segments?: DemoSegment[]
  steps: DemoStep[]
}

export type FrameField = {
  id: string
  label: string
  bits: number
  color?: 'coral' | 'blue' | 'green' | 'yellow' | 'teal'
  description?: string
  variable?: boolean;
}

export type FrameBlock = {
  type: 'frame'
  title?: string
  bitsPerRow?: number
  fields: FrameField[]
}

export type ContentBlock = TextBlock | ImageBlock | CalloutBlock | CodeBlock | MermaidBlock | DemoBlock | FrameBlock

export type CourseSection = {
  id: string
  title: string
  kicker?: string
  blocks: ContentBlock[]
}

export type Course = CourseSummary & {
  objectives: string[]
  sections: CourseSection[]
}

export type CourseManifest = {
  themes: Theme[]
  courses: CourseSummary[]
  courseFiles: Record<string, string>
}