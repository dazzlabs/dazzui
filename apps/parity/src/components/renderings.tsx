import { SectionLabel } from '@dazzlabs/dazzui-react'
import { useEffect, useRef, useState } from 'react'

import {
  renderName,
  type Platform,
  type Specimen,
  type ThemeId,
} from '../lib/model'

export interface RenderingsProps {
  specimen: Specimen
  platforms: readonly Platform[]
  theme: ThemeId
  /** Whether the extractor has drawn the React side. */
  react: boolean
}

/**
 * The pictures, side by side: what React drew and what each port drew of the
 * same specimen under the same theme.
 *
 * This is the half of the comparison the value table cannot reach. Only one
 * of the three frameworks will say what colour a view painted or what face it
 * set a label in — a SwiftUI `View` is a value rather than a tree, and GPUI's
 * scene is gone by the time a test could look at it — so a table of theirs is
 * a box and little else. Every one of them will *draw*, though, and a drawing
 * carries every decision at once.
 *
 * The number under each port is how much of its drawing differs from
 * React's. It is never zero and is not meant to be: two rasterisers do not
 * agree pixel for pixel about the edge of a letter or the inside of a
 * corner, and a small drawing is mostly edges — a badge the eye cannot tell
 * apart from React's comes out around a fifth different. What the number is
 * good for is the size of a difference, not its presence: a wrong colour
 * fills the whole drawing and a wrong box misses most of it, and both land
 * far above where two renderers' anti-aliasing ever does.
 */
export function Renderings({
  specimen,
  platforms,
  theme,
  react,
}: RenderingsProps) {
  const name = renderName(specimen.key)
  const sides = [
    ...(react ? [{ id: 'react', label: 'React' }] : []),
    ...platforms.filter(platform => platform.renders),
  ]
  if (sides.length === 0) return null

  return (
    <div className="pv-renders">
      {sides.map(side => (
        <Rendering
          key={side.id}
          label={side.label}
          src={`/renders/${side.id}/${theme}/${name}.png`}
          reference={
            side.id === 'react' || specimen.composed
              ? null
              : `/renders/react/${theme}/${name}.png`
          }
          note={
            side.id !== 'react' && specimen.composed
              ? 'holds the story'
              : undefined
          }
        />
      ))}
    </div>
  )
}

/**
 * Where a difference stops being two rasterisers and starts being two
 * drawings. Anti-aliasing on the edges of a small specimen's letters and
 * corners runs to something like a fifth of it; a colour, a box or a corner
 * that is actually wrong runs far higher.
 */
const STRUCTURAL = 35

function Rendering({
  label,
  src,
  reference,
  note,
}: {
  label: string
  src: string
  /** What to measure this against, or null when this *is* the reference. */
  reference: string | null
  /** Why there is no number, when there is none. */
  note?: string
}) {
  const [state, setState] = useState<'loading' | 'shown' | 'missing'>('loading')
  const [difference, setDifference] = useState<number | null>(null)
  const image = useRef<HTMLImageElement>(null)

  useEffect(() => {
    if (!reference || state !== 'shown') return
    let cancelled = false
    void compare(src, reference).then(value => {
      if (!cancelled) setDifference(value)
    })
    return () => {
      cancelled = true
    }
  }, [src, reference, state])

  return (
    <div className="pv-render" data-state={state}>
      <SectionLabel faint>{label}</SectionLabel>
      <div className="pv-render__well">
        <img
          ref={image}
          className="pv-render__image"
          src={src}
          alt={`${label} rendering`}
          onLoad={() => setState('shown')}
          onError={() => setState('missing')}
        />
        {state === 'missing' && (
          <span className="pv-render__absent">not drawn</span>
        )}
      </div>
      {note !== undefined && (
        <span
          className="pv-render__difference"
          title="A specimen carries its axes and its text, not the elements a story put inside it — so what this holds is not what React's held, and the two pictures cannot be counted against each other."
        >
          {note}
        </span>
      )}
      {difference !== null && (
        <span
          className="pv-render__difference pv-mono"
          data-tint="danger"
          data-differs={difference >= STRUCTURAL ? 'true' : 'false'}
          title={
            difference >= STRUCTURAL
              ? 'More than two rasterisers disagreeing about edges — look at the two drawings.'
              : 'Within what two rasterisers differ by on the edges of letters and corners.'
          }
        >
          {`${difference.toFixed(1)}% differs`}
        </span>
      )}
    </div>
  )
}

/**
 * How much of one drawing differs from another, as a percentage of the pixels
 * they have in common.
 *
 * Two drawings of the same specimen are the same size when the two platforms
 * agree about its box; where they do not, the overlap is compared and the
 * rest counts as difference, since a picture that is the wrong size differs
 * everywhere it is missing. A channel is called equal within a step of eight,
 * which is where two renderers' anti-aliasing of the same edge lands.
 */
async function compare(a: string, b: string): Promise<number | null> {
  const [first, second] = await Promise.all([load(a), load(b)])
  if (!first || !second) return null

  const width = Math.max(first.width, second.width)
  const height = Math.max(first.height, second.height)
  const overlapWidth = Math.min(first.width, second.width)
  const overlapHeight = Math.min(first.height, second.height)
  if (width === 0 || height === 0) return null

  const one = pixels(first, overlapWidth, overlapHeight)
  const two = pixels(second, overlapWidth, overlapHeight)
  if (!one || !two) return null

  let differing = width * height - overlapWidth * overlapHeight
  for (let i = 0; i < one.length; i += 4) {
    const close =
      Math.abs(one[i] - two[i]) <= 8 &&
      Math.abs(one[i + 1] - two[i + 1]) <= 8 &&
      Math.abs(one[i + 2] - two[i + 2]) <= 8 &&
      Math.abs(one[i + 3] - two[i + 3]) <= 8
    if (!close) differing += 1
  }
  return (differing / (width * height)) * 100
}

function load(src: string): Promise<HTMLImageElement | null> {
  return new Promise(resolve => {
    const image = new Image()
    image.onload = () => resolve(image)
    image.onerror = () => resolve(null)
    image.src = src
  })
}

function pixels(
  image: HTMLImageElement,
  width: number,
  height: number
): Uint8ClampedArray | null {
  const canvas = document.createElement('canvas')
  canvas.width = width
  canvas.height = height
  const context = canvas.getContext('2d', { willReadFrequently: true })
  if (!context) return null
  context.drawImage(image, 0, 0)
  return context.getImageData(0, 0, width, height).data
}
