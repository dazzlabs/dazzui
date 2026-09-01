import type { Tint } from '@dazzlabs/dazzui-react'
import { useCallback, useMemo, useReducer, useState } from 'react'

import { contains, type Rect, union } from './geometry'

/** The five things this canvas can draw. */
export type ShapeKind = 'frame' | 'rect' | 'ellipse' | 'text' | 'note'

/**
 * How a shape is painted, in the design system's own words.
 *
 * The canvas has no palette of its own: a shape carries a `tint` and a
 * `variant`, exactly as a `Badge` does, and the token build resolves the pair
 * into a colour against whichever ramp the tint names. That is the whole
 * reason the drawing follows the theme menu — nothing here is a colour, so
 * there is nothing to be wrong in the other three themes.
 */
export type ShapeVariant = 'filled' | 'tinted' | 'outlined'

export interface Shape extends Rect {
  id: string
  kind: ShapeKind
  /** What the layer list calls it. Renamed in place, never derived again. */
  name: string
  tint: Tint
  variant: ShapeVariant
  /** Corner radius in scene units; an ellipse and a text have no use for it. */
  radius: number
  /**
   * Type size in scene units, for the two kinds that are mostly words. It is
   * a scene measure rather than a token step because it is part of the
   * drawing: zoom in and the type grows with everything around it.
   */
  size: number
  /** 0–100, the way an inspector prints it. */
  opacity: number
  /** The words a text, a note or a frame carries. */
  text: string
  /** Locked shapes are still drawn and still listed — they just cannot be hit. */
  locked: boolean
  hidden: boolean
}

export interface Page {
  id: string
  name: string
  /** Back to front: the last shape is the one on top. */
  shapes: Shape[]
}

export interface Doc {
  /** What the toolbar calls it. A canvas is a file, even a fictional one. */
  name: string
  pages: Page[]
}

/**
 * The document's grid, in scene units.
 *
 * Not a token. The spacing scale measures the interface; this measures the
 * drawing, and a drawing is entitled to its own ruler — Figma's nudge grid is
 * not its padding scale either.
 */
export const GRID = 8

/** How many steps back the history keeps. Old enough is far enough. */
const HISTORY_LIMIT = 64

/** What a shape is called before anyone renames it. */
const NAMES: Record<ShapeKind, string> = {
  frame: 'Frame',
  rect: 'Rectangle',
  ellipse: 'Ellipse',
  text: 'Text',
  note: 'Note',
}

/** The box a shape takes when it is clicked into place rather than dragged. */
const EXTENTS: Record<ShapeKind, { width: number; height: number }> = {
  frame: { width: 320, height: 240 },
  rect: { width: 160, height: 112 },
  ellipse: { width: 128, height: 128 },
  text: { width: 176, height: 32 },
  note: { width: 160, height: 120 },
}

/**
 * The type each kind's words are set in. A frame is the exception: its label
 * is chrome rather than part of the drawing, so it is set in the interface's
 * own face and never scales with the scene.
 */
const TYPE: Record<ShapeKind, number> = {
  frame: 0,
  rect: 14,
  ellipse: 14,
  text: 18,
  note: 13,
}

const RADIUS: Record<ShapeKind, number> = {
  frame: 8,
  rect: 8,
  ellipse: 0,
  text: 0,
  note: 8,
}

const VARIANTS: Record<ShapeKind, ShapeVariant> = {
  frame: 'outlined',
  rect: 'filled',
  ellipse: 'tinted',
  text: 'tinted',
  note: 'tinted',
}

let seq = 0
const nextId = () => `s${++seq}`

let pageSeq = 0
const nextPageId = () => `p${++pageSeq}`

/** `Page 2` — the first number the document has not used. */
function nextPageName(doc: Doc): string {
  const used = new Set(doc.pages.map(page => page.name))
  let index = 1
  while (used.has(`Page ${index}`)) index += 1
  return `Page ${index}`
}

/** The page a document is open at, falling back to the first one it has. */
export function pageOf(doc: Doc, id: string): Page {
  return doc.pages.find(page => page.id === id) ?? doc.pages[0]
}

/**
 * The topmost shape under a point, or nothing.
 *
 * Walked from the front, because that is the one the pointer is pointing at.
 * Hidden and locked shapes are skipped: both are ways of saying *not this
 * one*, and a click that selects what you locked is the reason you locked it.
 */
export function shapeAt(page: Page, point: { x: number; y: number }) {
  for (let index = page.shapes.length - 1; index >= 0; index -= 1) {
    const shape = page.shapes[index]
    if (shape.hidden || shape.locked) continue
    if (contains(shape, point)) return shape
  }
  return null
}

/** `Rectangle 3` — the first number this page has not used for that kind. */
export function nextName(page: Page, kind: ShapeKind): string {
  const base = NAMES[kind]
  const used = new Set(page.shapes.map(shape => shape.name))
  let index = 1
  while (used.has(`${base} ${index}`)) index += 1
  return `${base} ${index}`
}

export function create(
  page: Page,
  kind: ShapeKind,
  rect: Rect,
  tint: Tint
): Shape {
  const name = nextName(page, kind)
  return {
    id: nextId(),
    kind,
    name,
    tint,
    variant: VARIANTS[kind],
    radius: RADIUS[kind],
    size: TYPE[kind],
    opacity: 100,
    // A frame's text is its label, which is the name it is listed under; a
    // note and a text carry words instead, and start with something to see.
    text: kind === 'frame' ? name : kind === 'text' ? 'Type something' : '',
    locked: false,
    hidden: false,
    ...rect,
  }
}

/** The box a click-placed shape of this kind gets, centred on the click. */
export function placed(kind: ShapeKind, at: { x: number; y: number }): Rect {
  const { width, height } = EXTENTS[kind]
  return { x: at.x - width / 2, y: at.y - height / 2, width, height }
}

/* Recipes -----------------------------------------------------------------
 * A recipe is a pure page-to-page function. The store takes them rather than
 * a set of verbs of its own, so an edit and the live frames of a drag share
 * one path through the history — see `useDocument` below.
 */

export type Recipe = (page: Page) => Page

export function insert(shapes: readonly Shape[]): Recipe {
  return page => ({ ...page, shapes: [...page.shapes, ...shapes] })
}

export function remove(ids: readonly string[]): Recipe {
  const gone = new Set(ids)
  return page => ({
    ...page,
    shapes: page.shapes.filter(shape => !gone.has(shape.id)),
  })
}

export function patch(
  ids: readonly string[],
  change: Partial<Shape> | ((shape: Shape) => Partial<Shape>)
): Recipe {
  const touched = new Set(ids)
  return page => ({
    ...page,
    shapes: page.shapes.map(shape =>
      touched.has(shape.id)
        ? {
            ...shape,
            ...(typeof change === 'function' ? change(shape) : change),
          }
        : shape
    ),
  })
}

/**
 * One step forward or back in the paint order.
 *
 * The moved shapes keep their order among themselves, which is what stops a
 * multi-selection from shuffling itself every time it is nudged forward.
 */
export function reorder(ids: readonly string[], direction: 1 | -1): Recipe {
  const moving = new Set(ids)
  return page => {
    const shapes = [...page.shapes]
    const indexes = shapes
      .map((shape, index) => (moving.has(shape.id) ? index : -1))
      .filter(index => index >= 0)

    // Walk from the end the shapes are heading towards, so each one lands
    // before the next is asked to move.
    const ordered = direction === 1 ? [...indexes].reverse() : indexes
    for (const index of ordered) {
      const target = index + direction
      if (target < 0 || target >= shapes.length) continue
      if (moving.has(shapes[target].id)) continue
      const [shape] = shapes.splice(index, 1)
      shapes.splice(target, 0, shape)
    }
    return { ...page, shapes }
  }
}

export type Edge = 'left' | 'center' | 'right' | 'top' | 'middle' | 'bottom'

/** Line the selection up on its own bounding box — never on the canvas. */
export function align(ids: readonly string[], edge: Edge): Recipe {
  const moving = new Set(ids)
  return page => {
    const box = union(page.shapes.filter(shape => moving.has(shape.id)))
    if (!box) return page

    return {
      ...page,
      shapes: page.shapes.map(shape => {
        if (!moving.has(shape.id)) return shape
        switch (edge) {
          case 'left':
            return { ...shape, x: box.x }
          case 'center':
            return { ...shape, x: box.x + (box.width - shape.width) / 2 }
          case 'right':
            return { ...shape, x: box.x + box.width - shape.width }
          case 'top':
            return { ...shape, y: box.y }
          case 'middle':
            return { ...shape, y: box.y + (box.height - shape.height) / 2 }
          case 'bottom':
            return { ...shape, y: box.y + box.height - shape.height }
        }
      }),
    }
  }
}

/** How far a duplicate lands from the shape it came from. */
const OFFSET = GRID * 2

/** Copies, offset and renamed, with ids the caller can select afterwards. */
export function copies(page: Page, ids: readonly string[]): Shape[] {
  const wanted = new Set(ids)
  return page.shapes
    .filter(shape => wanted.has(shape.id))
    .map(shape => ({
      ...shape,
      id: nextId(),
      name: `${shape.name} copy`,
      x: shape.x + OFFSET,
      y: shape.y + OFFSET,
    }))
}

/* The store ---------------------------------------------------------------- */

interface History {
  past: Doc[]
  present: Doc
  future: Doc[]
  /**
   * What the document looked like when a gesture began. A drag writes dozens
   * of frames and is worth exactly one undo step, so the frames go straight
   * into `present` and this is what gets pushed when the hand comes off.
   */
  held: Doc | null
}

type Action =
  | { kind: 'edit'; pageId: string; recipe: Recipe }
  | { kind: 'document'; recipe: (doc: Doc) => Doc }
  | { kind: 'draft'; pageId: string; recipe: Recipe }
  | { kind: 'begin' }
  | { kind: 'settle' }
  | { kind: 'undo' }
  | { kind: 'redo' }

function apply(doc: Doc, pageId: string, recipe: Recipe): Doc {
  return {
    ...doc,
    pages: doc.pages.map(page => (page.id === pageId ? recipe(page) : page)),
  }
}

function push(past: Doc[], doc: Doc): Doc[] {
  return [...past, doc].slice(-HISTORY_LIMIT)
}

function reduce(state: History, action: Action): History {
  switch (action.kind) {
    case 'edit': {
      const present = apply(state.present, action.pageId, action.recipe)
      if (present === state.present) return state
      return {
        past: push(state.past, state.present),
        present,
        future: [],
        held: null,
      }
    }
    // A change to the document rather than to a page in it — adding a page.
    // There is no gesture that does this, so it is always one whole step.
    case 'document':
      return {
        past: push(state.past, state.present),
        present: action.recipe(state.present),
        future: [],
        held: null,
      }
    case 'draft':
      return {
        ...state,
        present: apply(state.present, action.pageId, action.recipe),
      }
    case 'begin':
      return { ...state, held: state.present }
    case 'settle': {
      // A gesture that changed nothing — a click that missed, a drag of zero
      // pixels — leaves no step behind to undo.
      if (!state.held || state.held === state.present) {
        return { ...state, held: null }
      }
      return {
        past: push(state.past, state.held),
        present: state.present,
        future: [],
        held: null,
      }
    }
    case 'undo': {
      const previous = state.past.at(-1)
      if (!previous) return state
      return {
        past: state.past.slice(0, -1),
        present: previous,
        future: [state.present, ...state.future],
        held: null,
      }
    }
    case 'redo': {
      const [next, ...rest] = state.future
      if (!next) return state
      return {
        past: push(state.past, state.present),
        present: next,
        future: rest,
        held: null,
      }
    }
  }
}

export interface DocumentStore {
  /** What the document is called, and every page it holds. */
  name: string
  pages: readonly Page[]
  /** The open page. */
  page: Page
  pageId: string
  openPage: (id: string) => void
  addPage: () => void
  /** The ids that are selected *and* still on the page. */
  selection: string[]
  selected: Shape[]
  select: (ids: readonly string[]) => void
  /** One indivisible change: what stood before it becomes the undo step. */
  edit: (recipe: Recipe) => void
  /** Open a gesture. Everything drafted until `settle` is one step. */
  begin: () => void
  draft: (recipe: Recipe) => void
  settle: () => void
  canUndo: boolean
  canRedo: boolean
  undo: () => void
  redo: () => void
}

/**
 * The document, its history, and where the eye is in it.
 *
 * Selection and the open page sit outside the history on purpose: neither is
 * a fact about the drawing, and an undo that moved you somewhere else to show
 * you what it did would be undoing something you never asked for. The cost is
 * that undoing a delete brings the shapes back unselected, which is the right
 * side of that trade — `selection` is filtered against the page on every read,
 * so an id that no longer exists is simply gone rather than a stale reference
 * every caller has to remember to check.
 */
export function useDocument(initial: Doc): DocumentStore {
  const [history, dispatch] = useReducer(reduce, {
    past: [],
    present: initial,
    future: [],
    held: null,
  })
  const [pageId, setPageId] = useState(initial.pages[0].id)
  const [ids, setIds] = useState<readonly string[]>([])

  const doc = history.present
  const page = pageOf(doc, pageId)

  const selected = useMemo(
    () => page.shapes.filter(shape => ids.includes(shape.id)),
    [ids, page.shapes]
  )

  const edit = useCallback(
    (recipe: Recipe) => dispatch({ kind: 'edit', pageId, recipe }),
    [pageId]
  )
  const draft = useCallback(
    (recipe: Recipe) => dispatch({ kind: 'draft', pageId, recipe }),
    [pageId]
  )
  const begin = useCallback(() => dispatch({ kind: 'begin' }), [])
  const settle = useCallback(() => dispatch({ kind: 'settle' }), [])
  const undo = useCallback(() => dispatch({ kind: 'undo' }), [])
  const redo = useCallback(() => dispatch({ kind: 'redo' }), [])

  const select = useCallback((next: readonly string[]) => setIds(next), [])

  const openPage = useCallback((id: string) => {
    setPageId(id)
    setIds([])
  }, [])

  const addPage = useCallback(() => {
    // The id is minted out here because the reducer has to stay pure and the
    // new page has to be opened, which is not something a recipe can do.
    const id = nextPageId()
    dispatch({
      kind: 'document',
      recipe: current => ({
        ...current,
        pages: [
          ...current.pages,
          { id, name: nextPageName(current), shapes: [] },
        ],
      }),
    })
    setPageId(id)
    setIds([])
  }, [])

  return {
    name: doc.name,
    pages: doc.pages,
    page,
    pageId,
    openPage,
    addPage,
    selection: selected.map(shape => shape.id),
    selected,
    select,
    edit,
    begin,
    draft,
    settle,
    canUndo: history.past.length > 0,
    canRedo: history.future.length > 0,
    undo,
    redo,
  }
}
