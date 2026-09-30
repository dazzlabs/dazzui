import type { ComponentProps } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface SpinnerProps extends ComponentProps<'span'> {
  /** UIKit's medium and large; Material's 24 and 48dp. */
  size?: 'small' | 'large'
  /** Announced to a screen reader. */
  label?: string
  /**
   * Draws it still and partly there, from 0 to 1 — what a pull to refresh
   * shows before it lets go: UIRefreshControl lays its spokes down one by
   * one, Material's indicator draws out its arc. Leave it unset to spin.
   */
  progress?: number
}

const SPOKES = Array.from({ length: 8 }, (_, i) => i)
/** The arc's circumference, at the 20 radius it is drawn at. */
const ARC = 2 * Math.PI * 20

/**
 * Work in flight with no end in sight: UIActivityIndicator's eight fading
 * spokes, and Material's spinning arc. A determinate ring is `Progress`.
 */
export function Spinner({
  size = 'small',
  label = 'Loading',
  progress,
  className,
  ...props
}: SpinnerProps) {
  const platform = usePlatform()
  const still = progress !== undefined
  const amount = Math.max(0, Math.min(1, progress ?? 0))
  return (
    <span
      role="status"
      aria-label={label}
      data-platform={platform}
      data-size={size}
      data-still={still || undefined}
      className={cx('dzm-spinner', className)}
      {...props}
    >
      {platform === 'ios' ? (
        <svg viewBox="0 0 24 24" aria-hidden className="dzm-spinner__spokes">
          {SPOKES.map(i => (
            <rect
              key={i}
              className="dzm-spinner__spoke"
              x="11"
              y="2"
              width="2"
              height="6"
              rx="1"
              transform={`rotate(${i * 45} 12 12)`}
              style={
                still
                  ? { opacity: i < Math.ceil(amount * 8) ? 1 : 0 }
                  : { animationDelay: `${(i - 8) * 0.1}s` }
              }
            />
          ))}
        </svg>
      ) : (
        <svg
          viewBox="0 0 48 48"
          aria-hidden
          className="dzm-spinner__arc"
          style={
            still ? { transform: `rotate(${amount * 270 - 90}deg)` } : undefined
          }
        >
          <circle
            cx="24"
            cy="24"
            r="20"
            style={
              still
                ? { strokeDasharray: `${amount * 0.8 * ARC} ${ARC}` }
                : undefined
            }
          />
        </svg>
      )}
    </span>
  )
}
