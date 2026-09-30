import {
  type ComponentProps,
  type ReactNode,
  type RefObject,
  useEffect,
  useRef,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import { ChevronLeft } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface NavigationBarProps extends Omit<
  ComponentProps<'header'>,
  'title'
> {
  title: ReactNode
  /**
   * The line under the title: iOS 26's `navigationSubtitle`, Material
   * Expressive's app bar subtitle.
   */
  subtitle?: ReactNode
  /** iOS's large title; Material's large top app bar. */
  large?: boolean
  /** Draws the back control at the leading edge. */
  back?: { label?: ReactNode; onClick: () => void }
  /** Controls at the leading edge, after any back control. */
  leading?: ReactNode
  /** Controls at the trailing edge — `IconButton`s. */
  trailing?: ReactNode
}

/**
 * UINavigationBar and Material's top app bar, both driven by the scroll
 * of the `Screen` they head.
 *
 * iOS 26 draws the bar clear: its controls float in glass, and once
 * content passes beneath it a soft scroll-edge effect — a blur that fades
 * out downward — keeps the title legible. A large title sits in the
 * content and scrolls away under the bar, and the bar takes the title,
 * centred, as it goes. Material's bar is the surface and turns to the
 * surface container the moment anything scrolls under it; the large bar
 * drops its title under the bar in headline-medium and collapses into the
 * small bar's title-large the same way.
 */
export function NavigationBar({
  title,
  subtitle,
  large = false,
  back,
  leading,
  trailing,
  className,
  ...props
}: NavigationBarProps) {
  const platform = usePlatform()
  const bar = useRef<HTMLElement>(null)
  const heading = useRef<HTMLHeadingElement>(null)
  const { scrolled, collapsed } = useScrollEdge(bar, large ? heading : null)
  const inline = !large || collapsed
  return (
    <>
      <header
        ref={bar}
        data-platform={platform}
        data-large={large || undefined}
        data-scrolled={scrolled || undefined}
        data-collapsed={inline || undefined}
        className={cx('dzm-nav', className)}
        {...props}
      >
        <div className="dzm-nav__bar">
          <div className="dzm-nav__leading">
            {back ? (
              <button
                type="button"
                aria-label={
                  typeof back.label === 'string' ? back.label : 'Back'
                }
                className={cx(
                  'dzm-nav__back',
                  platform === 'ios' && 'dzm-glass',
                  'dzm-m3-layer'
                )}
                onClick={back.onClick}
              >
                <ChevronLeft className="dzm-nav__back-glyph" />
                {platform === 'ios' && back.label ? (
                  <span className="dzm-nav__back-label" aria-hidden>
                    {back.label}
                  </span>
                ) : null}
              </button>
            ) : null}
            {leading}
          </div>
          <div
            className="dzm-nav__title"
            aria-hidden={large || undefined}
            {...(large ? {} : { role: 'heading', 'aria-level': 1 })}
          >
            <span className="dzm-nav__title-text">{title}</span>
            {subtitle ? (
              <span className="dzm-nav__subtitle">{subtitle}</span>
            ) : null}
          </div>
          <div className="dzm-nav__trailing">{trailing}</div>
        </div>
      </header>
      {large ? (
        <div data-platform={platform} className="dzm-nav-large">
          <h1 ref={heading} className="dzm-nav-large__title">
            {title}
          </h1>
          {subtitle ? (
            <p className="dzm-nav-large__subtitle">{subtitle}</p>
          ) : null}
        </div>
      ) : null}
    </>
  )
}

/**
 * Where the bar stands against the scroll beneath it: `scrolled` once any
 * content has passed under it, `collapsed` once the large title has. It
 * reads the scroll of the `Screen` the bar sits in; outside one, both stay
 * false.
 */
function useScrollEdge(
  bar: RefObject<HTMLElement | null>,
  heading: RefObject<HTMLHeadingElement | null> | null
) {
  const [state, setState] = useState({ scrolled: false, collapsed: false })
  const tracksHeading = heading !== null
  useEffect(() => {
    const element = bar.current
    const scroller = element?.closest<HTMLElement>('.dzm-screen__scroll')
    if (!element || !scroller) return
    let frame = 0
    const measure = () => {
      frame = 0
      const scrolled = scroller.scrollTop > 0
      let collapsed = false
      const title = heading?.current
      if (title) {
        // The bar takes the title once the large title's middle has
        // passed under the bar's bottom edge, as UIKit hands it over.
        const edge = element.getBoundingClientRect().bottom
        const box = title.getBoundingClientRect()
        collapsed = box.top + box.height / 2 <= edge
      }
      setState(previous =>
        previous.scrolled === scrolled && previous.collapsed === collapsed
          ? previous
          : { scrolled, collapsed }
      )
    }
    const schedule = () => {
      if (!frame) frame = requestAnimationFrame(measure)
    }
    measure()
    scroller.addEventListener('scroll', schedule, { passive: true })
    const observer = new ResizeObserver(schedule)
    observer.observe(scroller)
    return () => {
      scroller.removeEventListener('scroll', schedule)
      observer.disconnect()
      cancelAnimationFrame(frame)
    }
  }, [bar, heading, tracksHeading])
  return state
}
