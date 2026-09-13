import type { ComponentType, ReactNode } from 'react'

export type Layout = 'centered' | 'fullscreen' | 'padded'
export interface Parameters {
  layout?: Layout
}
export type Decorator = (Story: ComponentType) => ReactNode
// The generic retains existing story declarations while the scene migration
// proceeds. Components are rendered explicitly; the host never infers props.
export interface Meta<T = unknown> {
  title: string
  component?: T
  parameters?: Parameters
  decorators?: Decorator[]
}
export interface StoryObj<_T = unknown> {
  name?: string
  parameters?: Parameters
  render: () => ReactNode
  decorators?: Decorator[]
}
export interface StoryModule {
  default: Meta
  [name: string]: StoryObj | Meta
}
