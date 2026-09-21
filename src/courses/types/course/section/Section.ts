import type { ContentBlock } from './blocks/ContentBlock'

export type Section = {
  id: string
  title: string
  kicker?: string
  blocks: ContentBlock[]
}