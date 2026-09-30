import type { ComponentProps } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface SpinnerProps extends ComponentProps<'span'> {
  /** UIKit's medium and large; Material's 24 and 48dp. */
  size?: 'small' | 'large'
  /** Announced to a screen reader. */
  label?: string
}

const SPOKES = Array.from({ length: 8 }, (_, i) => i)

/**
 * Work in flight with no end in sight: UIActivityIndicator's eight fading
 * spokes, and Material's spinning arc. A determinate ring is `Progress`.
 */
export function Spinner({
  size = 'small',
  label = 'Loading',
  className,
  ...props
}: SpinnerProps) {
  const platform = usePlatform()
  return (
    <span
      role="status"
      aria-label={label}
      data-platform={platform}
      data-size={size}
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
              style={{ animationDelay: `${(i - 8) * 0.1}s` }}
            />
          ))}
        </svg>
      ) : (
        <svg viewBox="0 0 48 48" aria-hidden className="dzm-spinner__arc">
          <circle cx="24" cy="24" r="20" />
        </svg>
      )}
    </span>
  )
}
