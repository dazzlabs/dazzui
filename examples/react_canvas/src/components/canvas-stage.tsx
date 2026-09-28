import { EmptyState, type Tint } from '@dazzlabs/dazzui-react'
import {
  type CSSProperties,
  type PointerEvent as ReactPointerEvent,
  useEffect,
  useRef,
  useState,
} from 'react'

import { toolOf, type ToolId } from '../data/tools'
import {
  CURSORS,
  type Handle,
  HANDLES,
  MIN_SIZE,
  type Point,
  type Rect,
  grip,
  overlaps,
  rectFrom,
  resize,
  snap,
  union,
} from '../lib/geometry'
import { isTyping } from '../lib/keys'
import {
  create,
  type DocumentStore,
  GRID,
  insert,
  type Page,
  patch,
  placed,
  type Shape,
  shapeAt,
} from '../lib/scene'
import { toScene, type ViewportStore } from '../lib/viewport'

/**
 * How coarse the dots are, in grid steps. Every step would be a grey wash at
 * anything but the closest zoom; every fourth reads as paper.
 */
const GRID_EVERY = 4

/** Below this the dots crowd into a texture, so the grid stops drawing. */
const GRID_FLOOR = 0.4

/**
 * What the pointer is in the middle of.
 *
 * A gesture lives in a ref rather than in state: it is written on every
 * pointer move and read on the next one, and nothing on screen is drawn from
 * it — what the drag *does* goes into the document, which is what re-renders.
 * The one exception is the marquee, which is not in the document because it is
 * not part of the drawing.
 */
type Gesture =
  | { type: 'pan'; last: Point }
  | { type: 'marquee'; from: Point; additive: boolean; base: string[] }
  | { type: 'draw'; id: string; from: Point; kind: Shape['kind'] }
  | { type: 'move'; from: Point; before: Shape[] }
  | { type: 'resize'; handle: Handle; from: Point; before: Shape[] }

export interface CanvasStageProps {
  page: Page
  selection: readonly string[]
  tool: ToolId
  /** A drawing tool hands the pointer back once it has drawn. */
  onPickTool: (tool: ToolId) => void
  /** The tint a new shape is drawn in. */
  tint: Tint
  grid: boolean
  snapping: boolean
  view: ViewportStore
  doc: DocumentStore
}

export function CanvasStage({
  page,
  selection,
  tool,
  onPickTool,
  tint,
  grid,
  snapping,
  view,
  doc,
}: CanvasStageProps) {
  const stage = useRef<HTMLDivElement>(null)
  const gesture = useRef<Gesture | null>(null)
  const [marquee, setMarquee] = useState<Rect | null>(null)
  const [space, setSpace] = useState(false)
  const [dragging, setDragging] = useState(false)
  const [editing, setEditing] = useState<string | null>(null)

  const { viewport } = view
  const { zoom } = viewport
  const { measure, panBy, zoomBy } = view

  /*
   * The stage is the only thing that knows how big it is, and the zoom menu,
   * the fit button and the keyboard all need to.
   *
   * The observer reports a size as soon as the element has one, which is not
   * necessarily on mount: a tab that has never been painted has no layout to
   * measure, and its notification arrives when it is first shown. Everything
   * downstream is written for that — `fit` does nothing until there is a stage
   * to fit to, and the first frame of the drawing lands the moment there is.
   */
  useEffect(() => {
    const element = stage.current
    if (!element) return

    const observer = new ResizeObserver(([entry]) => {
      measure({
        width: entry.contentRect.width,
        height: entry.contentRect.height,
      })
    })
    observer.observe(element)
    return () => observer.disconnect()
  }, [measure])

  /*
   * The wheel is bound by hand because React's own `onWheel` is passive: it
   * cannot call `preventDefault`, and without that the browser answers a
   * two-finger swipe by scrolling the page the canvas is in.
   */
  useEffect(() => {
    const element = stage.current
    if (!element) return

    const onWheel = (event: WheelEvent) => {
      event.preventDefault()
      const box = element.getBoundingClientRect()
      const at = { x: event.clientX - box.left, y: event.clientY - box.top }

      // A trackpad pinch arrives as a wheel with ctrl held down — the browser
      // says so whether or not the key is — and ⌘ is the mouse equivalent.
      if (event.ctrlKey || event.metaKey) {
        zoomBy(Math.exp(-event.deltaY / 160), at)
        return
      }
      panBy(-event.deltaX, -event.deltaY)
    }

    element.addEventListener('wheel', onWheel, { passive: false })
    return () => element.removeEventListener('wheel', onWheel)
  }, [panBy, zoomBy])

  /* Space pans, whatever the tool is. It is held rather than pressed, so it is
     the one key the canvas reads for itself. */
  useEffect(() => {
    const down = (event: KeyboardEvent) => {
      if (event.code === 'Space' && !isTyping(event.target)) setSpace(true)
    }
    const up = (event: KeyboardEvent) => {
      if (event.code === 'Space') setSpace(false)
    }
    window.addEventListener('keydown', down)
    window.addEventListener('keyup', up)
    return () => {
      window.removeEventListener('keydown', down)
      window.removeEventListener('keyup', up)
    }
  }, [])

  const drawing = toolOf(tool).kind
  const panning = tool === 'hand' || space

  const at = (event: { clientX: number; clientY: number }): Point => {
    const box = stage.current?.getBoundingClientRect()
    return {
      x: event.clientX - (box?.left ?? 0),
      y: event.clientY - (box?.top ?? 0),
    }
  }

  /** Both corners on the grid — for a shape being dragged out of nothing. */
  const snapped = (rect: Rect): Rect => {
    if (!snapping) return rect
    const x = snap(rect.x, GRID)
    const y = snap(rect.y, GRID)
    return {
      x,
      y,
      width: Math.max(MIN_SIZE, snap(rect.x + rect.width, GRID) - x),
      height: Math.max(MIN_SIZE, snap(rect.y + rect.height, GRID) - y),
    }
  }

  /**
   * Only the edges the grip is actually moving. Snapping all four would drag
   * the opposite corner along whenever a shape did not already start on the
   * grid, which reads as the shape sliding out from under the hand.
   */
  const snappedEdges = (rect: Rect, handle: Handle): Rect => {
    if (!snapping) return rect
    let { x, y, width, height } = rect

    if (handle.includes('w')) {
      const edge = snap(x, GRID)
      width = Math.max(MIN_SIZE, width + x - edge)
      x = edge
    }
    if (handle.includes('e'))
      width = Math.max(MIN_SIZE, snap(x + width, GRID) - x)
    if (handle.includes('n')) {
      const edge = snap(y, GRID)
      height = Math.max(MIN_SIZE, height + y - edge)
      y = edge
    }
    if (handle.includes('s'))
      height = Math.max(MIN_SIZE, snap(y + height, GRID) - y)

    return { x, y, width, height }
  }

  const onPointerDown = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.button !== 0 && event.button !== 1) return
    const element = stage.current
    if (!element) return

    // Captured on the stage rather than on whatever was clicked, so a drag
    // that starts on a grip still reports its moves here.
    element.setPointerCapture(event.pointerId)
    setDragging(true)

    // A click anywhere else closes an edit in progress, and closes it here
    // rather than leaving it to the field's own blur: the textarea is about to
    // be unmounted, and an element removed while it has the focus never fires
    // one. Settling twice is harmless — the second finds nothing held.
    if (editing) {
      doc.settle()
      setEditing(null)
    }

    const screen = at(event)
    const point = toScene(viewport, screen)

    if (panning || event.button === 1) {
      gesture.current = { type: 'pan', last: screen }
      return
    }

    if (drawing) {
      const shape = create(
        page,
        drawing,
        { x: point.x, y: point.y, width: 0, height: 0 },
        tint
      )
      doc.begin()
      doc.draft(insert([shape]))
      doc.select([shape.id])
      gesture.current = {
        type: 'draw',
        id: shape.id,
        from: point,
        kind: drawing,
      }
      return
    }

    const hit = shapeAt(page, point)
    const additive = event.shiftKey || event.metaKey

    if (!hit) {
      if (!additive) doc.select([])
      gesture.current = {
        type: 'marquee',
        from: screen,
        additive,
        base: additive ? [...selection] : [],
      }
      setMarquee({ x: screen.x, y: screen.y, width: 0, height: 0 })
      return
    }

    const ids = additive
      ? selection.includes(hit.id)
        ? selection.filter(id => id !== hit.id)
        : [...selection, hit.id]
      : selection.includes(hit.id)
        ? selection
        : [hit.id]
    doc.select(ids)

    // A shift-click that took the shape *out* of the selection is not the
    // start of a drag of it.
    if (!ids.includes(hit.id)) return

    doc.begin()
    gesture.current = {
      type: 'move',
      from: point,
      before: page.shapes.filter(shape => ids.includes(shape.id)),
    }
  }

  const startResize = (
    event: ReactPointerEvent<HTMLDivElement>,
    handle: Handle
  ) => {
    if (event.button !== 0) return
    event.stopPropagation()
    const element = stage.current
    if (!element) return
    element.setPointerCapture(event.pointerId)
    setDragging(true)
    doc.begin()
    gesture.current = {
      type: 'resize',
      handle,
      from: toScene(viewport, at(event)),
      before: doc.selected,
    }
  }

  const onPointerMove = (event: ReactPointerEvent<HTMLDivElement>) => {
    const current = gesture.current
    if (!current) return
    const screen = at(event)
    const point = toScene(viewport, screen)

    switch (current.type) {
      case 'pan': {
        panBy(screen.x - current.last.x, screen.y - current.last.y)
        current.last = screen
        return
      }

      case 'marquee': {
        setMarquee(rectFrom(current.from, screen))
        return
      }

      case 'draw': {
        doc.draft(patch([current.id], snapped(rectFrom(current.from, point))))
        return
      }

      case 'move': {
        const anchor = current.before[0]
        let dx = point.x - current.from.x
        let dy = point.y - current.from.y

        // The whole selection moves by one delta, taken off the first shape.
        // Snapping each shape to the grid on its own would pull a group apart
        // the first time it was nudged.
        if (snapping && anchor) {
          dx = snap(anchor.x + dx, GRID) - anchor.x
          dy = snap(anchor.y + dy, GRID) - anchor.y
        }

        const ids = current.before.map(shape => shape.id)
        doc.draft(
          patch(ids, shape => {
            const before = current.before.find(entry => entry.id === shape.id)
            if (!before) return shape
            return { x: before.x + dx, y: before.y + dy }
          })
        )
        return
      }

      case 'resize': {
        const box = union(current.before)
        if (!box) return

        // One shape or ten, the grip drags the selection's own box and the
        // shapes inside it keep their places in it proportionally. A single
        // selection is the same code with a box the size of the shape.
        const next = snappedEdges(
          resize(
            box,
            current.handle,
            point.x - current.from.x,
            point.y - current.from.y
          ),
          current.handle
        )
        const sx = box.width === 0 ? 1 : next.width / box.width
        const sy = box.height === 0 ? 1 : next.height / box.height

        const ids = current.before.map(shape => shape.id)
        doc.draft(
          patch(ids, shape => {
            const before = current.before.find(entry => entry.id === shape.id)
            if (!before) return shape
            return {
              x: next.x + (before.x - box.x) * sx,
              y: next.y + (before.y - box.y) * sy,
              width: Math.max(MIN_SIZE, before.width * sx),
              height: Math.max(MIN_SIZE, before.height * sy),
            }
          })
        )
        return
      }
    }
  }

  const onPointerUp = (event: ReactPointerEvent<HTMLDivElement>) => {
    const current = gesture.current
    gesture.current = null
    setDragging(false)
    stage.current?.releasePointerCapture(event.pointerId)
    if (!current) return

    if (current.type === 'marquee') {
      const box = rectFrom(
        toScene(viewport, current.from),
        toScene(viewport, at(event))
      )
      const inside = page.shapes
        .filter(shape => !shape.hidden && !shape.locked && overlaps(shape, box))
        .map(shape => shape.id)
      doc.select(
        current.additive ? [...new Set([...current.base, ...inside])] : inside
      )
      setMarquee(null)
      return
    }

    if (current.type === 'draw') {
      const box = rectFrom(current.from, toScene(viewport, at(event)))
      // A click rather than a drag: give the shape the box its kind starts at,
      // centred where the click landed.
      if (box.width < MIN_SIZE || box.height < MIN_SIZE) {
        doc.draft(patch([current.id], placed(current.kind, current.from)))
      }
      doc.settle()
      onPickTool('select')
      return
    }

    doc.settle()
  }

  const onDoubleClick = (event: ReactPointerEvent<HTMLDivElement>) => {
    const hit = shapeAt(page, toScene(viewport, at(event)))
    // A frame's label is its name, which the inspector edits — there is
    // nothing on the frame itself to put a caret in.
    if (!hit || hit.kind === 'frame') return
    doc.select([hit.id])
    setEditing(hit.id)
  }

  const box = union(doc.selected)
  const empty = page.shapes.length === 0

  return (
    <div
      ref={stage}
      className="cv-stage"
      data-cursor={
        dragging && panning
          ? 'grabbing'
          : panning
            ? 'grab'
            : drawing
              ? 'draw'
              : 'pick'
      }
      data-grid={grid && zoom >= GRID_FLOOR}
      style={
        {
          '--cv-zoom': zoom,
          '--cv-grid': `${GRID * GRID_EVERY * zoom}px`,
          '--cv-grid-x': `${viewport.x}px`,
          '--cv-grid-y': `${viewport.y}px`,
        } as CSSProperties
      }
      onDoubleClick={onDoubleClick}
      onPointerCancel={onPointerUp}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
    >
      <div
        className="cv-scene"
        style={{
          transform: `translate(${viewport.x}px, ${viewport.y}px) scale(${zoom})`,
        }}
      >
        {page.shapes
          .filter(shape => !shape.hidden)
          .map(shape => (
            <ShapeView
              key={shape.id}
              doc={doc}
              editing={editing === shape.id}
              shape={shape}
              onEdited={() => setEditing(null)}
            />
          ))}
      </div>

      {/*
       * The selection is drawn over the scene rather than in it, in screen
       * units: a grip is the same size to the hand however far away the
       * drawing is being held.
       */}
      <div className="cv-overlay">
        {box ? (
          <div
            className="cv-selection"
            style={{
              left: box.x * zoom + viewport.x,
              top: box.y * zoom + viewport.y,
              width: box.width * zoom,
              height: box.height * zoom,
            }}
          >
            {HANDLES.map(handle => {
              const point = grip(
                {
                  x: 0,
                  y: 0,
                  width: box.width * zoom,
                  height: box.height * zoom,
                },
                handle
              )
              return (
                <div
                  key={handle}
                  className="cv-handle"
                  style={{
                    left: point.x,
                    top: point.y,
                    cursor: CURSORS[handle],
                  }}
                  onPointerDown={event => startResize(event, handle)}
                />
              )
            })}
          </div>
        ) : null}

        {marquee ? (
          <div
            className="cv-marquee"
            style={{
              left: marquee.x,
              top: marquee.y,
              width: marquee.width,
              height: marquee.height,
            }}
          />
        ) : null}
      </div>

      {empty ? (
        <div className="cv-stage__empty">
          {/* One line and no explainer under it, which is what the component
              is drawn for — so the way out goes in the line. */}
          <EmptyState title="Nothing on this page. Pick a shape from the tools, or press R." />
        </div>
      ) : null}
    </div>
  )
}

interface ShapeViewProps {
  shape: Shape
  editing: boolean
  doc: DocumentStore
  onEdited: () => void
}

/**
 * One shape.
 *
 * Its colour is a `data-tint` and a `data-variant` and nothing else — the same
 * two attributes a `Badge` carries — so the stylesheet names a role and the
 * token build decides what colour that is in the theme the window happens to
 * be in.
 */
function ShapeView({ shape, editing, doc, onEdited }: ShapeViewProps) {
  return (
    <div
      className="cv-shape"
      data-kind={shape.kind}
      data-locked={shape.locked}
      data-tint={shape.tint}
      data-variant={shape.variant}
      style={{
        left: shape.x,
        top: shape.y,
        width: shape.width,
        height: shape.height,
        // An ellipse is a rectangle with the corner taken all the way, so the
        // kind decides the radius rather than the field the inspector edits.
        borderRadius: shape.kind === 'ellipse' ? '50%' : shape.radius,
        fontSize: shape.size,
        opacity: shape.opacity / 100,
      }}
    >
      {shape.kind === 'frame' ? (
        <span className="cv-shape__label">{shape.text}</span>
      ) : editing ? (
        <textarea
          autoFocus
          className="cv-shape__editor"
          defaultValue={shape.text}
          // One undo step for the whole edit: the document is drafted on every
          // keystroke so the canvas keeps up, and the step is recorded when the
          // caret leaves.
          onBlur={() => {
            doc.settle()
            onEdited()
          }}
          onChange={event =>
            doc.draft(patch([shape.id], { text: event.target.value }))
          }
          onFocus={() => doc.begin()}
          onKeyDown={event => {
            if (event.key === 'Escape') event.currentTarget.blur()
          }}
          onPointerDown={event => event.stopPropagation()}
        />
      ) : shape.text ? (
        <span className="cv-shape__text">{shape.text}</span>
      ) : null}
    </div>
  )
}
