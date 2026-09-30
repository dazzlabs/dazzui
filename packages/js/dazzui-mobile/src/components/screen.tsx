import {
  type ComponentProps,
  type ReactNode,
  type RefObject,
  useEffect,
  useRef,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import { type Platform, usePlatform } from '../lib/theme'
import { Spinner } from './spinner'

export interface ScreenProps extends ComponentProps<'div'> {
  /** The bar across the top — a `NavigationBar`. */
  bar?: ReactNode
  /** The bar across the bottom — a `TabBar` or a `Toolbar`. */
  tabBar?: ReactNode
  /**
   * Pull to refresh: dragging the content down from its top calls this,
   * and the indicator spins until the promise it returns settles.
   */
  onRefresh?: () => unknown
}

/**
 * One screen: a bar, the content that scrolls under it, and a tab bar the
 * content also scrolls under. The bars stick to the edges of the scroll
 * and paint over what passes beneath them — glass on iOS, a surface
 * container on Android — which is what makes them read as the OS's.
 */
export function Screen({
  bar,
  tabBar,
  onRefresh,
  className,
  children,
  ...props
}: ScreenProps) {
  const platform = usePlatform()
  const scroll = useRef<HTMLDivElement>(null)
  const { pull, refreshing, pulling } = usePullToRefresh(
    scroll,
    platform,
    onRefresh
  )
  const threshold = THRESHOLD[platform]
  return (
    <div
      data-platform={platform}
      className={cx('dzm-screen', className)}
      {...props}
    >
      <div
        ref={scroll}
        className="dzm-screen__scroll"
        data-pulling={pulling || undefined}
      >
        {bar}
        {onRefresh ? (
          <div
            data-platform={platform}
            data-refreshing={refreshing || undefined}
            data-pulling={pulling || undefined}
            data-idle={(pull === 0 && !refreshing) || undefined}
            className="dzm-refresh"
            style={{ '--dzm-refresh-pull': `${pull}px` } as never}
          >
            <span className="dzm-refresh__indicator">
              {pull > 0 || refreshing ? (
                <Spinner
                  label="Refreshing"
                  size="small"
                  progress={refreshing ? undefined : pull / threshold}
                />
              ) : null}
            </span>
          </div>
        ) : null}
        <div className="dzm-screen__content">{children}</div>
        {tabBar}
      </div>
    </div>
  )
}

/**
 * How far the content has to come down to refresh, and where the
 * indicator holds while it does: UIRefreshControl's, and the distances
 * Compose's `PullToRefreshBox` uses.
 */
const THRESHOLD: Record<Platform, number> = { ios: 88, android: 80 }
const HOLD: Record<Platform, number> = { ios: 60, android: 80 }
/** The pull follows the finger at half its travel, as a rubber band does. */
const RESISTANCE = 0.5

/**
 * The gesture. It starts only with the content at its top and a drag
 * downward, from a finger or a mouse; anything else is the scroll's.
 */
function usePullToRefresh(
  scroll: RefObject<HTMLDivElement | null>,
  platform: Platform,
  onRefresh: (() => unknown) | undefined
) {
  const [pull, setPull] = useState(0)
  const [pulling, setPulling] = useState(false)
  const [refreshing, setRefreshing] = useState(false)
  const handler = useRef(onRefresh)
  handler.current = onRefresh
  const enabled = Boolean(onRefresh)

  useEffect(() => {
    const element = scroll.current
    if (!element || !enabled) return
    let start: number | null = null
    let distance = 0
    let busy = false

    const begin = (y: number) => {
      if (busy || element.scrollTop > 0) return
      start = y
      distance = 0
    }
    const follow = (y: number, event: Event) => {
      if (start === null) return
      const next = Math.max(0, y - start)
      if (next === 0 && distance === 0) return
      if (element.scrollTop > 0) {
        start = null
        return
      }
      event.preventDefault()
      distance = next
      setPulling(true)
      setPull(Math.min(distance * RESISTANCE, THRESHOLD[platform] * 1.6))
    }
    const end = async () => {
      if (start === null) return
      start = null
      setPulling(false)
      if (distance * RESISTANCE < THRESHOLD[platform]) {
        setPull(0)
        return
      }
      busy = true
      setRefreshing(true)
      setPull(HOLD[platform])
      try {
        await handler.current?.()
      } finally {
        busy = false
        setRefreshing(false)
        setPull(0)
      }
    }

    const onTouchStart = (event: TouchEvent) => begin(event.touches[0]!.clientY)
    const onTouchMove = (event: TouchEvent) =>
      follow(event.touches[0]!.clientY, event)
    const onPointerDown = (event: PointerEvent) => {
      if (event.pointerType !== 'mouse' || event.button !== 0) return
      begin(event.clientY)
      const move = (next: PointerEvent) => follow(next.clientY, next)
      const up = () => {
        window.removeEventListener('pointermove', move)
        window.removeEventListener('pointerup', up)
        // A pull that started on a row must not also press it.
        if (distance > 0) {
          window.addEventListener(
            'click',
            event => {
              event.preventDefault()
              event.stopPropagation()
            },
            { capture: true, once: true }
          )
        }
        void end()
      }
      window.addEventListener('pointermove', move)
      window.addEventListener('pointerup', up)
    }
    const onTouchEnd = () => void end()

    element.addEventListener('touchstart', onTouchStart, { passive: true })
    element.addEventListener('touchmove', onTouchMove, { passive: false })
    element.addEventListener('touchend', onTouchEnd)
    element.addEventListener('touchcancel', onTouchEnd)
    element.addEventListener('pointerdown', onPointerDown)
    return () => {
      element.removeEventListener('touchstart', onTouchStart)
      element.removeEventListener('touchmove', onTouchMove)
      element.removeEventListener('touchend', onTouchEnd)
      element.removeEventListener('touchcancel', onTouchEnd)
      element.removeEventListener('pointerdown', onPointerDown)
    }
  }, [scroll, platform, enabled])

  return { pull, pulling, refreshing }
}
