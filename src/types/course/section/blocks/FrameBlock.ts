import type { FrameField } from './frame/FrameField'

export type FrameBlock = {
  type: 'frame'
  title?: string
  bitsPerRow?: number
  fields: FrameField[]
}