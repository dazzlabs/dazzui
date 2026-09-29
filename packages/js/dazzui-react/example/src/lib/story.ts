import type { ComponentType } from 'react'

export interface Example {
  title: string
  note?: string
  Demo: ComponentType
}

/**
 * A component, as the shell reads it. `Preview` is the one specimen that
 * stands for the component in the overview; `examples` are what its page
 * shows.
 */
export interface Story {
  /** The kit's own file name for the component, and its place in the address. */
  id: string
  name: string
  summary: string
  Preview: ComponentType
  examples: readonly Example[]
}
