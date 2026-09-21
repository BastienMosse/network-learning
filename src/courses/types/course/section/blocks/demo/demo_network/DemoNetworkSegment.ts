/// Permet de dessiner une zone de segment

export type DemoNetworkSegment = {
  id: string
  label: string
  x: number
  y: number
  width: number
  height: number
  color?: 'coral' | 'blue' | 'green' | 'yellow' | 'teal'
}