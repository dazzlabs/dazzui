import type {
  ComponentPropsWithoutRef,
  CSSProperties,
  PointerEvent,
  ReactElement,
  ReactNode,
} from 'react'
import {
  Children,
  cloneElement,
  Fragment,
  isValidElement,
  useId,
  useRef,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import {
  SPLITTER_KEY_COARSE_STEP,
  SPLITTER_KEY_STEP,
} from '../generated/runtime'

/** Which way the panels are laid out, not which way a handle runs. */
export type SplitterOrientation = 'horizontal' | 'vertical'

export interface SplitterPanelProps extends ComponentPropsWithoutRef<'div'> {
  /**
   * Share of the run at rest. The shares are normalised, so three panels
   * written `50`, `25`, `25` and three written `2`, `1`, `1` lay out
   * identically.
   */
  defaultSize?: number
  /** Floor and ceiling, as percentages of the whole run. */
  minSize?: number
  maxSize?: number
}

/**
 * One region of a `Splitter`.
 *
 * It is a marker as much as an element: the splitter counts these to know how
 * many handles to draw and reads their constraints to know what each handle
 * may do, so a panel has to be a direct child. The three sizing props are
 * consumed by the parent and never reach the DOM.
 */
export function SplitterPanel({
  defaultSize: _defaultSize,
  minSize: _minSize,
  maxSize: _maxSize,
  className,
  ...props
}: SplitterPanelProps) {
  return <div className={cx('dz-splitter__panel', className)} {...props} />
}

export interface SplitterProps extends ComponentPropsWithoutRef<'div'> {
  orientation?: SplitterOrientation
  /**
   * The shares after every change, so a host can persist the layout. The
   * splitter owns them either way — there is no controlled mode, because a
   * drag that has to round-trip through a host's state lags the pointer, and
   * a seam that lags the pointer feels broken.
   */
  onSizesChange?: (sizes: number[]) => void
  children?: ReactNode
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max)
}

/**
 * Two or more regions the reader can re-proportion.
 *
 * A desktop window is mostly this — a column beside a document beside an
 * inspector — and which of the three deserves the room is the reader's
 * decision rather than the designer's. Where a column has one right width and
 * the reader only opens or shuts it, that is `Sidebar` instead: it carries
 * its own width token and its own rail, and drawing it with this would offer
 * a choice the design has already made.
 *
 * The handle is drawn *from* the space between two panels rather than added
 * to it: the hit target is wider than the rule, so grabbing a seam does not
 * demand pixel accuracy, while the rule itself stays the hairline every other
 * separator in the system is.
 */
export function Splitter({
  orientation = 'horizontal',
  onSizesChange,
  className,
  style,
  children,
  ...props
}: SplitterProps) {
  const id = useId()
  const panels = Children.toArray(children).filter(
    (child): child is ReactElement<SplitterPanelProps> => isValidElement(child)
  )
  const refs = useRef<(HTMLDivElement | null)[]>([])
  const drag = useRef<
    { start: number; sizes: number[]; total: number } | undefined
  >(undefined)
  /* A drag applies each width on the spot; every other change — an arrow key,
     Home, End, a double-click — is a jump the reader did not make with the
     pointer, and settles over the motion token instead. Both end in the same
     state update, so this is what tells the stylesheet which one a change is. */
  const [dragging, setDragging] = useState(false)
  const [sizes, setSizes] = useState(() => {
    const declared = panels.map(panel => panel.props.defaultSize ?? 1)
    const sum = declared.reduce((a, b) => a + b, 0)
    /* Normalised to a hundred once, here, so everything downstream — the
       clamps, `aria-valuenow`, what a host is handed to persist — speaks one
       unit rather than whatever units the panels happened to be written in. */
    return declared.map(size => (size / sum) * 100)
  })

  const horizontal = orientation === 'horizontal'
  /* Every read of a share goes through here. The array is built from the
     panel list and can only be indexed in range, but saying so once is
     cheaper than saying so at each of the six places that ask. */
  const share = (index: number) => sizes[index] ?? 0
  const measure = (element: HTMLDivElement) =>
    horizontal ? element.offsetWidth : element.offsetHeight

  const bounds = (index: number) => {
    const panel = panels[index]?.props
    return [panel?.minSize ?? 0, panel?.maxSize ?? 100] as const
  }

  /* Both the pointer and the keyboard end here: given the leading panel's new
     share, the trailing one takes what the pair has left over. Nothing else
     in the row moves, which is what stops a drag at one seam from rippling
     through the whole window. */
  const resize = (index: number, nextLeading: number) => {
    setSizes(current => {
      const pair = (current[index] ?? 0) + (current[index + 1] ?? 0)
      const [leadMin, leadMax] = bounds(index)
      const [trailMin, trailMax] = bounds(index + 1)
      const leading = clamp(
        clamp(nextLeading, leadMin, leadMax),
        pair - trailMax,
        pair - trailMin
      )
      const next = [...current]
      next[index] = leading
      next[index + 1] = pair - leading
      onSizesChange?.(next)
      return next
    })
  }

  const onPointerDown = (event: PointerEvent<HTMLDivElement>) => {
    const measured = refs.current.map(element =>
      element ? measure(element) : 0
    )
    const total = measured.reduce((a, b) => a + b, 0)
    if (total === 0) return
    drag.current = {
      start: horizontal ? event.clientX : event.clientY,
      sizes: measured,
      total,
    }
    setDragging(true)
    event.currentTarget.setPointerCapture(event.pointerId)
  }

  const onPointerMove =
    (index: number) => (event: PointerEvent<HTMLDivElement>) => {
      const state = drag.current
      if (!state) return
      const delta = (horizontal ? event.clientX : event.clientY) - state.start
      /* Measured in pixels against the sizes the drag started from, and
         converted once. Accumulating a percentage per frame drifts, and a
         seam that drifts ends up somewhere other than under the pointer. */
      resize(index, (((state.sizes[index] ?? 0) + delta) / state.total) * 100)
    }

  const onPointerUp = (event: PointerEvent<HTMLDivElement>) => {
    if (!drag.current) return
    drag.current = undefined
    setDragging(false)
    event.currentTarget.releasePointerCapture(event.pointerId)
  }

  return (
    <div
      /* The seam takes the accent under the pointer, and every
         `--control-color-*` is declared inside `[data-tint]` — a splitter
         that named no ramp would highlight in a dead value. Fixed rather
         than a prop: a window's seams are furniture, and six of them in six
         colours would be a decision about nothing. */
      data-tint="primary"
      data-orientation={orientation}
      data-dragging={dragging || undefined}
      /* What the handles take off the run, so the panels can divide the rest
         in CSS: a share is a share of the panels' run, not of the box. */
      style={
        {
          '--splitter-seams': Math.max(panels.length - 1, 0),
          ...style,
        } as CSSProperties
      }
      className={cx('dz-splitter', className)}
      {...props}
    >
      {panels.map((panel, index) => {
        const [min, max] = bounds(index)
        const seam = index < panels.length - 1
        return (
          <Fragment key={index}>
            {cloneElement(panel, {
              id: `${id}-panel-${index}`,
              ref: (element: HTMLDivElement | null) => {
                refs.current[index] = element
              },
              /* The share, as a number: the stylesheet turns it into a
                 percentage of the panels' run, which is a length the browser
                 can settle over the motion token — a grow unit is not. */
              style: {
                '--splitter-share': share(index),
                ...panel.props.style,
              } as CSSProperties,
            } as Partial<SplitterPanelProps>)}
            {seam ? (
              <div
                role="separator"
                tabIndex={0}
                aria-orientation={horizontal ? 'vertical' : 'horizontal'}
                aria-controls={`${id}-panel-${index}`}
                aria-valuenow={Math.round(share(index))}
                aria-valuemin={Math.round(min)}
                aria-valuemax={Math.round(max)}
                className="dz-splitter__handle"
                onPointerDown={onPointerDown}
                onPointerMove={onPointerMove(index)}
                onPointerUp={onPointerUp}
                onPointerCancel={onPointerUp}
                onDoubleClick={() =>
                  /* An even split of this pair alone, which is what a reader
                     means by double-clicking one seam. */
                  resize(index, (share(index) + share(index + 1)) / 2)
                }
                onKeyDown={event => {
                  const back = horizontal ? 'ArrowLeft' : 'ArrowUp'
                  const forward = horizontal ? 'ArrowRight' : 'ArrowDown'
                  const step = event.shiftKey
                    ? SPLITTER_KEY_COARSE_STEP
                    : SPLITTER_KEY_STEP
                  if (event.key === back) resize(index, share(index) - step)
                  else if (event.key === forward)
                    resize(index, share(index) + step)
                  else if (event.key === 'Home') resize(index, min)
                  else if (event.key === 'End') resize(index, max)
                  else return
                  event.preventDefault()
                }}
              >
                <span aria-hidden className="dz-splitter__grip" />
              </div>
            ) : null}
          </Fragment>
        )
      })}
    </div>
  )
}
