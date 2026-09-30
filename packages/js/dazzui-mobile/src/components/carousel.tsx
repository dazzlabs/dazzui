import {
  Children,
  type ComponentProps,
  createContext,
  isValidElement,
  type KeyboardEvent,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'
import { PageControl } from './page-control'

export interface CarouselProps extends Omit<
  ComponentProps<'div'>,
  'defaultValue' | 'onChange'
> {
  /** `CarouselItem`s. */
  children: ReactNode
  /** The item in view; scrolling reports it, setting it scrolls there. */
  value?: number
  onValueChange?: (value: number) => void
  /**
   * The page control under the items. iOS draws one by default, as a
   * paging scroll view is always shown with its dots; Android only when
   * asked.
   */
  showPageControl?: boolean
  'aria-label'?: string
}

interface Slot {
  index: number
  count: number
}

const SlotContext = createContext<Slot>({ index: 0, count: 1 })

/**
 * A row of items that scrolls sideways and snaps. On iOS it is a paging
 * UIScrollView: one page at a time, each a full-width card inset 16pt at
 * the grouped corner, with a UIPageControl beneath. On Android it is
 * Material's carousel in its hero arrangement: items at the extra-large
 * corner, 8dp apart, snapping to a 16dp leading edge with the next one
 * peeking in. The strip is a focusable region; the arrow keys move it one
 * item at a time.
 */
export function Carousel({
  value,
  onValueChange,
  showPageControl,
  'aria-label': label,
  className,
  children,
  ...props
}: CarouselProps) {
  const platform = usePlatform()
  const track = useRef<HTMLDivElement>(null)
  const [index, setIndex] = useState(0)
  const reported = useRef(0)
  const mounted = useRef(false)
  const onChange = useRef(onValueChange)
  onChange.current = onValueChange
  const items = Children.toArray(children).filter(isValidElement)
  const count = items.length

  const scrollTo = useCallback((next: number, animated = true) => {
    const strip = track.current
    const item = strip?.children[next] as HTMLElement | undefined
    if (!strip || !item) return
    const padding = parseFloat(getComputedStyle(strip).scrollPaddingLeft) || 0
    const still = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    strip.scrollTo({
      left: item.offsetLeft - padding,
      behavior: animated && !still ? 'smooth' : 'auto',
    })
  }, [])

  // The item in view is the one whose snap edge is nearest the scroll
  // position; at the end of the strip, the last one.
  useEffect(() => {
    const strip = track.current
    if (!strip) return
    let frame = 0
    const measure = () => {
      frame = 0
      const padding = parseFloat(getComputedStyle(strip).scrollPaddingLeft) || 0
      const slots = Array.from(strip.children) as HTMLElement[]
      let nearest = 0
      if (strip.scrollLeft >= strip.scrollWidth - strip.clientWidth - 1) {
        nearest = slots.length - 1
      } else {
        let distance = Infinity
        slots.forEach((slot, i) => {
          const d = Math.abs(slot.offsetLeft - padding - strip.scrollLeft)
          if (d < distance) {
            distance = d
            nearest = i
          }
        })
      }
      if (nearest === reported.current) return
      reported.current = nearest
      setIndex(nearest)
      onChange.current?.(nearest)
    }
    const onScroll = () => {
      if (!frame) frame = requestAnimationFrame(measure)
    }
    strip.addEventListener('scroll', onScroll, { passive: true })
    return () => {
      strip.removeEventListener('scroll', onScroll)
      cancelAnimationFrame(frame)
    }
  }, [])

  // A value set from outside that the scroll has not reached scrolls there:
  // at once on mount, animated after.
  useEffect(() => {
    const first = !mounted.current
    mounted.current = true
    if (value === undefined || value === reported.current) return
    scrollTo(value, !first)
  }, [value, scrollTo])

  const onKeyDown = (event: KeyboardEvent) => {
    const target =
      event.key === 'ArrowRight'
        ? index + 1
        : event.key === 'ArrowLeft'
          ? index - 1
          : event.key === 'Home'
            ? 0
            : event.key === 'End'
              ? count - 1
              : null
    if (target === null || target < 0 || target >= count) return
    event.preventDefault()
    scrollTo(target)
  }

  const dots = showPageControl ?? platform === 'ios'

  return (
    <div
      data-platform={platform}
      className={cx('dzm-carousel', className)}
      {...props}
    >
      <div
        ref={track}
        role="region"
        aria-roledescription="carousel"
        aria-label={label}
        tabIndex={0}
        className="dzm-carousel__track"
        onKeyDown={onKeyDown}
      >
        {items.map((item, i) => (
          <SlotContext.Provider key={item.key ?? i} value={{ index: i, count }}>
            {item}
          </SlotContext.Provider>
        ))}
      </div>
      {dots && count > 1 ? (
        <PageControl
          className="dzm-carousel__pages"
          count={count}
          value={index}
          onValueChange={scrollTo}
          aria-label={label}
        />
      ) : null}
    </div>
  )
}

export interface CarouselItemProps extends ComponentProps<'div'> {
  /**
   * A title laid over the item's foot: a caption on a scrim on iOS, the
   * label Material draws on a carousel item on Android.
   */
  label?: ReactNode
}

/**
 * One item of a `Carousel`: whatever it holds — an image, a card — clipped
 * to the platform's corner, with an optional label over its foot.
 */
export function CarouselItem({
  label,
  className,
  children,
  ...props
}: CarouselItemProps) {
  const platform = usePlatform()
  const { index, count } = useContext(SlotContext)
  return (
    <div
      role="group"
      aria-roledescription="slide"
      aria-label={`${index + 1} of ${count}`}
      data-platform={platform}
      className={cx('dzm-carousel__item', className)}
      {...props}
    >
      <div className="dzm-carousel__card">
        {children}
        {label ? <div className="dzm-carousel__label">{label}</div> : null}
      </div>
    </div>
  )
}
