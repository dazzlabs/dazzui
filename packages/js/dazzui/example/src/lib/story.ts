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

/**
 * Examples outside shared conformance, added to a compiled story's page. A file in
 * `stories/react/` exports one under the compiled story's `id`; codegen never
 * reads it only for the platforms explicitly listed below.
 */
export interface StoryExtension {
  id: string
  /** Native mappings that also compile these examples, outside conformance. */
  platforms?: readonly ('flutter' | 'swiftui' | 'winui' | 'gpui')[]
  examples: readonly Example[]
}
