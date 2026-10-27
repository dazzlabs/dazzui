export interface SceneValue {
  kind: string
  text?: string
  number?: number
  boolean?: boolean
  name?: string
  address?: number
  items?: SceneValue[]
  fields?: Record<string, SceneValue>
}
export interface Scene {
  id: string
  title: string
  note: string
  content: SceneValue
  initial: Record<string, SceneValue>
  gaps: string[]
}
export interface Story {
  id: string
  name: string
  summary: string
  preview: Scene
  examples: Scene[]
}
export interface ThemeFamily {
  value: string
  title: string
}
export interface ThemeGroup {
  title: string
  families: ThemeFamily[]
}

export class SceneNavigation {
  open: (href: string) => void = () => {}
}
