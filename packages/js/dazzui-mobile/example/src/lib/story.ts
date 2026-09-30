import type { ComponentType } from 'react'

/**
 * A story is one screen, drawn twice: the same `Demo` mounts inside the
 * iPhone and inside the Pixel, and every component in it reads which one
 * it is on from the theme around it.
 */
export interface Story {
  id: string
  name: string
  summary: string
  Demo: ComponentType
}
