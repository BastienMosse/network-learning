import type { DemoStep } from './demo/DemoStep';
import type { DemoNetwork } from './demo/DemoNetwork';

export type DemoBlock = {
  type: 'demo'
  title: string
  intro?: string
  network?: DemoNetwork
  steps: DemoStep[]
}