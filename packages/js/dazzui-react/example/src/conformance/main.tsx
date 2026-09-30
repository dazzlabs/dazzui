import { TooltipProvider } from '@dazzlabs/dazzui-react'
import type { ComponentType } from 'react'
import { flushSync } from 'react-dom'
import { createRoot } from 'react-dom/client'
import '@dazzlabs/dazzui-react/style.css'
import '@dazzlabs/dazzui-react/components.css'

import { STORIES } from '../stories'
import {
  applyTheme,
  FAMILIES,
  FAMILY_GROUPS,
  MODES,
  parseTheme,
  themeId,
  type Family,
} from '../lib/themes'
import '../styles/base.css'
import './conformance.css'
import { measure, type Reading } from './measure'
import { walk } from './walk'

/*
 * The page the React collector drives (scripts/conformance.mjs). It draws one
 * scene at a time, alone, in one theme, and reads every specimen in it. The
 * scenes are the stories' own components — the reference is React drawing
 * exactly what the example draws.
 */

const SCENES = new Map<string, ComponentType>()
for (const story of STORIES) {
  SCENES.set(`${story.id}/preview`, story.Preview)
  story.examples.forEach(({ Demo }, index) =>
    SCENES.set(`${story.id}/${index}`, Demo)
  )
}

const container = document.getElementById('root')!
const root = createRoot(container)

const frame = () => new Promise(resolve => requestAnimationFrame(resolve))

export interface Drawn {
  /** The scene's rectangle in the page, in CSS pixels. */
  rect: { x: number; y: number; width: number; height: number }
  specimens: {
    component: string
    reading: Reading | null
    rect: Drawn['rect'] | null
  }[]
}

const box = (rect: DOMRect) => ({
  x: rect.x,
  y: rect.y,
  width: rect.width,
  height: rect.height,
})

async function show(scene: string, theme: string): Promise<Drawn> {
  const Demo = SCENES.get(scene)
  if (!Demo) throw new Error(`No scene ${scene}`)
  applyTheme(document.documentElement, parseTheme(theme))
  flushSync(() =>
    root.render(
      <TooltipProvider>
        <div className="c-scene" key={`${theme}:${scene}`}>
          <Demo />
        </div>
      </TooltipProvider>
    )
  )
  await document.fonts.ready
  await frame()
  await frame()
  const stageNode = container.querySelector<HTMLElement>('.c-scene')!
  return {
    rect: box(stageNode.getBoundingClientRect()),
    specimens: walk(container, Demo).map(({ component, node }) => ({
      component,
      reading: node ? measure(node) : null,
      rect: node ? box(node.getBoundingClientRect()) : null,
    })),
  }
}

/** Crops a PNG of the scene into one PNG per rectangle, as base64. */
async function crop(
  png: string,
  rects: (Drawn['rect'] | null)[],
  scale: number
): Promise<(string | null)[]> {
  const image = new Image()
  image.src = `data:image/png;base64,${png}`
  await image.decode()
  return rects.map(rect => {
    if (!rect || rect.width < 1 || rect.height < 1) return null
    const canvas = document.createElement('canvas')
    canvas.width = Math.round(rect.width * scale)
    canvas.height = Math.round(rect.height * scale)
    canvas
      .getContext('2d')!
      .drawImage(
        image,
        Math.round(rect.x * scale),
        Math.round(rect.y * scale),
        canvas.width,
        canvas.height,
        0,
        0,
        canvas.width,
        canvas.height
      )
    return canvas.toDataURL('image/png').split(',')[1]!
  })
}

const themes = (families: readonly Family[]) =>
  families.flatMap(family =>
    MODES.map(mode => themeId({ family: family.value, mode: mode.value }))
  )

const conformance = {
  scenes: [...SCENES.keys()],
  /** Every theme the example carries, and the twelve DazzUI draws itself. */
  themes: themes(FAMILIES),
  drawn: themes(FAMILY_GROUPS[0]!.families),
  show,
  crop,
}

declare global {
  interface Window {
    conformance?: typeof conformance
  }
}

window.conformance = conformance
