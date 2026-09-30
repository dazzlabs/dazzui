/*
 * The shape of a run, as the viewer and the check read it. Three files:
 * `scenes.json`, the contract codegen writes; `run.json`, which platforms
 * took part; and one `readings.json` per platform. See specs/conformance.md.
 */

/** A kit element inside a scene, at its address. */
export interface Specimen {
  /** `story/scene/index` — the key on every platform. */
  address: string
  /** The kit's name for the component: `button`, `segmented-control`. */
  component: string
  /** The props that pick a recipe. */
  axes: Record<string, string>
  /** What it reads, when it holds one run of text. */
  text?: string
  /** Holds other kit elements: only its box is compared. */
  container?: boolean
}

export interface Scene {
  /** `story/index`; the preview is `story/preview`. */
  id: string
  title: string
  specimens: readonly Specimen[]
}

export interface StoryScenes {
  id: string
  name: string
  summary: string
  scenes: readonly Scene[]
}

export interface Scenes {
  stories: readonly StoryScenes[]
}

export interface Run {
  collected: string
  commit: string
  /** In the order the viewer shows them; the reference first. */
  platforms: readonly string[]
}

/** The property vocabulary. `null` is "could not read". */
export type Values = Record<string, number | string | null>

export interface Reading {
  box: { width: number; height: number }
  values: Values
  image?: string
}

export interface Readings {
  platform: string
  title: string
  role: 'reference' | 'implementation'
  collected: string
  commit: string
  scale: number
  font: string | null
  themes: Record<string, Record<string, Reading>>
  scenes: Record<string, Record<string, string>>
}

export interface Tolerances {
  length: number
  colour: number
  image: number
  kinds: Record<string, 'length' | 'colour' | 'discrete'>
}

/** Everything loaded, keyed for the viewer. */
export interface Loaded {
  scenes: Scenes
  run: Run
  readings: Record<string, Readings>
  tolerances: Tolerances
  reference: string
  implementations: readonly string[]
}
