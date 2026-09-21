import type { DemoStepPacket } from './demo_step/DemoStepPacket';
import type { DemoStepMedia } from './demo_step/DemoStepMedia';
import type { DemoStepTable } from './demo_step/DemoStepTable';

export type DemoStep = {
  title: string
  description: string
  active?: string[]
  packets?: DemoStepPacket[][]
  panel?: string
  media?: DemoStepMedia
  table?: DemoStepTable
}