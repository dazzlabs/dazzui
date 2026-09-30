import {
  type ComponentProps,
  type PointerEvent,
  type ReactNode,
  useEffect,
  useRef,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import { ChevronRight } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface ListProps extends ComponentProps<'div'> {}

/**
 * The scrolling column of a Settings screen: inset grouped sections on iOS
 * — the cells a card at the 26pt corner, the header above and the footer
 * below in the footnote style — and Material's list, flat on the surface
 * with a subheader per section and a divider between items.
 */
export function List({ className, ...props }: ListProps) {
  const platform = usePlatform()
  return (
    <div
      data-platform={platform}
      className={cx('dzm-list', className)}
      {...props}
    />
  )
}

export interface ListSectionProps extends ComponentProps<'section'> {
  header?: ReactNode
  footer?: ReactNode
}

export function ListSection({
  header,
  footer,
  className,
  children,
  ...props
}: ListSectionProps) {
  const platform = usePlatform()
  return (
    <section
      data-platform={platform}
      className={cx('dzm-list-section', className)}
      {...props}
    >
      {header ? <div className="dzm-list-section__header">{header}</div> : null}
      <div className="dzm-list-section__group" role="group">
        {children}
      </div>
      {footer ? <div className="dzm-list-section__footer">{footer}</div> : null}
    </section>
  )
}

export interface ListItemProps extends Omit<ComponentProps<'div'>, 'title'> {
  /** The primary line. */
  title: ReactNode
  /** The line beneath it. */
  subtitle?: ReactNode
  /** A glyph at the leading edge. */
  icon?: ReactNode
  /**
   * iOS paints a leading glyph on a coloured rounded square, as Settings
   * does; pass the colour. Material draws the glyph alone.
   */
  iconColor?: string
  /** A value at the trailing edge — the current setting, in the secondary label. */
  value?: ReactNode
  /** A control at the trailing edge — a `Switch`, a `Badge`. */
  trailing?: ReactNode
  /** Draws the disclosure chevron and makes the row press. */
  href?: string
  onClick?: () => void
  /** The row is a choice in the destructive colour. */
  tone?: 'default' | 'destructive'
  disabled?: boolean
  /**
   * Actions a swipe from the leading edge uncovers, the first outermost.
   * A long swipe performs the first.
   */
  leadingActions?: readonly SwipeAction[]
  /** Actions a swipe from the trailing edge uncovers, the first outermost. */
  trailingActions?: readonly SwipeAction[]
}

/**
 * One of a row's swipe actions: SwiftUI's `.swipeActions` button, and
 * the action behind Material's swipe-to-dismiss.
 */
export interface SwipeAction {
  label: string
  icon?: ReactNode
  /**
   * `destructive` is the danger accent, `accent` the tint and `default`
   * the grey iOS gives an action with no role.
   */
  tone?: 'default' | 'accent' | 'destructive'
  /** Any colour, in place of the tone's. */
  color?: string
  onAction: () => void
}

export function ListItem({
  title,
  subtitle,
  icon,
  iconColor,
  value,
  trailing,
  href,
  onClick,
  tone = 'default',
  disabled = false,
  leadingActions,
  trailingActions,
  className,
  ...props
}: ListItemProps) {
  const platform = usePlatform()
  const interactive = Boolean(onClick || href)
  const chevron = platform === 'ios' && interactive
  const content = (
    <>
      {icon ? (
        <span
          className="dzm-list-item__icon"
          style={
            iconColor
              ? ({ '--dzm-list-icon-fill': iconColor } as never)
              : undefined
          }
        >
          {icon}
        </span>
      ) : null}
      <span className="dzm-list-item__text">
        <span className="dzm-list-item__title">{title}</span>
        {subtitle ? (
          <span className="dzm-list-item__subtitle">{subtitle}</span>
        ) : null}
      </span>
      {value !== undefined ? (
        <span className="dzm-list-item__value">{value}</span>
      ) : null}
      {trailing ? (
        <span className="dzm-list-item__trailing">{trailing}</span>
      ) : null}
      {chevron ? (
        <span className="dzm-list-item__chevron">
          <ChevronRight />
        </span>
      ) : null}
    </>
  )
  const shared = {
    'data-platform': platform,
    'data-tone': tone,
    'data-interactive': interactive || undefined,
    'data-two-line': subtitle ? '' : undefined,
    className: cx('dzm-list-item', interactive && 'dzm-m3-layer', className),
  }
  const row = href ? (
    <a href={href} aria-disabled={disabled || undefined} {...shared}>
      {content}
    </a>
  ) : onClick ? (
    <button type="button" onClick={onClick} disabled={disabled} {...shared}>
      {content}
    </button>
  ) : (
    <div {...shared} {...props}>
      {content}
    </div>
  )
  if (!leadingActions?.length && !trailingActions?.length) return row
  return (
    <SwipeRow leading={leadingActions ?? []} trailing={trailingActions ?? []}>
      {row}
    </SwipeRow>
  )
}

/** Past this much of the row's width a swipe performs the outermost action. */
const COMMIT = { ios: 0.6, android: 0.4 }
/** The width iOS gives each uncovered action. */
const ACTION_WIDTH = 74
/** How far a finger moves before the row decides it is being swiped. */
const SLOP = 8

type Side = 'leading' | 'trailing'

/**
 * The row under a swipe. iOS slides it aside to uncover its actions, each
 * a full-height button, and a long swipe stretches the outermost across
 * the row and performs it. Material has no uncovered buttons: the row
 * slides over a background that takes the action's colour once the swipe
 * has gone far enough to perform it — swipe-to-dismiss, as Gmail archives.
 * A vertical drag is left to the scroll; a tap on an open row closes it.
 */
function SwipeRow({
  leading,
  trailing,
  children,
}: {
  leading: readonly SwipeAction[]
  trailing: readonly SwipeAction[]
  children: ReactNode
}) {
  const platform = usePlatform()
  const root = useRef<HTMLDivElement>(null)
  const [offset, setOffset] = useState(0)
  const [dragging, setDragging] = useState(false)
  const [performing, setPerforming] = useState(false)
  const latest = useRef(0)
  const suppressClick = useRef(false)
  const drag = useRef<{
    x: number
    y: number
    start: number
    width: number
    active: boolean
  } | null>(null)

  const move = (next: number) => {
    latest.current = next
    setOffset(next)
  }

  const side: Side | null =
    offset > 0 ? 'leading' : offset < 0 ? 'trailing' : null
  const actions = side === 'leading' ? leading : trailing
  const width = root.current?.offsetWidth ?? 0
  // Only a finger, or the action it let go on, arms the row: an iOS row
  // left open can be wider than the commit point without meaning it.
  const armed =
    (dragging || performing) &&
    side !== null &&
    width > 0 &&
    Math.abs(offset) > width * COMMIT[platform]

  // An open row closes when anything else is pressed.
  useEffect(() => {
    if (offset === 0 || dragging) return
    const close = (event: globalThis.PointerEvent) => {
      if (!root.current?.contains(event.target as Node)) move(0)
    }
    document.addEventListener('pointerdown', close, true)
    return () => document.removeEventListener('pointerdown', close, true)
  }, [offset, dragging])

  const perform = (from: Side, action: SwipeAction | undefined) => {
    if (!action) return move(0)
    const full = root.current?.offsetWidth ?? 0
    setPerforming(true)
    move(from === 'leading' ? full : -full)
    action.onAction()
    // If the row is still here once it has slid away, it comes back.
    window.setTimeout(() => {
      setPerforming(false)
      move(0)
    }, 450)
  }

  const onPointerDown = (event: PointerEvent<HTMLDivElement>) => {
    if (event.button !== 0 || !root.current) return
    drag.current = {
      x: event.clientX,
      y: event.clientY,
      start: latest.current,
      width: root.current.offsetWidth,
      active: false,
    }
  }

  const onPointerMove = (event: PointerEvent<HTMLDivElement>) => {
    const current = drag.current
    if (!current) return
    const dx = event.clientX - current.x
    const dy = event.clientY - current.y
    if (!current.active) {
      if (Math.abs(dy) > SLOP && Math.abs(dy) > Math.abs(dx)) {
        drag.current = null
        return
      }
      if (Math.abs(dx) < SLOP) return
      current.active = true
      setDragging(true)
      try {
        root.current?.setPointerCapture(event.pointerId)
      } catch {
        // A pointer that is already gone cannot be captured; the drag
        // still follows it while it stays over the row.
      }
    }
    let next = current.start + dx
    if (!leading.length) next = Math.min(next, 0)
    if (!trailing.length) next = Math.max(next, 0)
    move(Math.max(-current.width, Math.min(current.width, next)))
  }

  const onPointerUp = () => {
    const current = drag.current
    drag.current = null
    if (!current?.active) return
    setDragging(false)
    suppressClick.current = true
    const x = latest.current
    const from: Side = x > 0 ? 'leading' : 'trailing'
    const list = from === 'leading' ? leading : trailing
    if (Math.abs(x) > current.width * COMMIT[platform]) {
      return perform(from, list[0])
    }
    if (platform === 'ios') {
      const open = list.length * ACTION_WIDTH
      return move(Math.abs(x) > open / 2 ? Math.sign(x) * open : 0)
    }
    move(0)
  }

  return (
    <div
      ref={root}
      data-platform={platform}
      data-side={side ?? undefined}
      data-armed={armed || undefined}
      data-dragging={dragging || undefined}
      className="dzm-list-swipe"
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerCancel={onPointerUp}
      onClickCapture={event => {
        if (suppressClick.current || (latest.current !== 0 && !dragging)) {
          suppressClick.current = false
          if (!(event.target as Element).closest('.dzm-list-swipe__action')) {
            event.preventDefault()
            event.stopPropagation()
            move(0)
          }
        }
      }}
    >
      {side ? (
        <div
          className="dzm-list-swipe__actions"
          data-side={side}
          style={
            {
              width: Math.abs(offset),
              ...(actions[0]?.color
                ? { '--dzm-swipe-fill': actions[0].color }
                : {}),
            } as never
          }
        >
          {(side === 'leading' ? actions : [...actions].reverse()).map(
            action => (
              <button
                key={action.label}
                type="button"
                tabIndex={-1}
                data-tone={action.tone ?? 'default'}
                data-outermost={action === actions[0] || undefined}
                className="dzm-list-swipe__action"
                style={
                  action.color
                    ? ({ '--dzm-swipe-fill': action.color } as never)
                    : undefined
                }
                onClick={() => perform(side, action)}
              >
                {action.icon ? (
                  <span className="dzm-list-swipe__icon">{action.icon}</span>
                ) : null}
                <span className="dzm-list-swipe__label">{action.label}</span>
              </button>
            )
          )}
        </div>
      ) : null}
      <div
        className="dzm-list-swipe__row"
        style={{ transform: offset ? `translateX(${offset}px)` : undefined }}
      >
        {children}
      </div>
    </div>
  )
}
