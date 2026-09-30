import { Slider as Base } from '@base-ui/react/slider'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface SliderProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  className?: string
  /** Glyphs at either end — a small and a large speaker, say. */
  leading?: ReactNode
  trailing?: ReactNode
}

/**
 * UISlider and Material Expressive's slider. iOS draws a thin groove and a
 * round white thumb that the track thickens under; Material draws a thick
 * track in two colours with a gap around a narrow handle bar and a stop
 * dot at the far end. Base UI owns the drag, the keyboard and the value.
 */
export function Slider({
  className,
  leading,
  trailing,
  ...props
}: SliderProps) {
  const platform = usePlatform()
  return (
    <Base.Root
      data-platform={platform}
      className={cx('dzm-slider', className)}
      {...props}
    >
      {leading ? <span className="dzm-slider__end">{leading}</span> : null}
      <Base.Control className="dzm-slider__control">
        <Base.Track className="dzm-slider__track">
          <Base.Indicator className="dzm-slider__indicator" />
          <Base.Thumb className="dzm-slider__thumb" />
          <span className="dzm-slider__stop" aria-hidden />
        </Base.Track>
      </Base.Control>
      {trailing ? <span className="dzm-slider__end">{trailing}</span> : null}
    </Base.Root>
  )
}
