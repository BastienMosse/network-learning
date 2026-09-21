/// Permet de créer un noeud

export type DemoNetworkNode = {
  id: string
  label: string
  kind: 'device' | 'service'
  position?: { x: number; y: number }
}