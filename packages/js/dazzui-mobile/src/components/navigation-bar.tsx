import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { ChevronLeft } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface NavigationBarProps extends Omit<
  ComponentProps<'header'>,
  'title'
> {
  title: ReactNode
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
 * UINavigationBar and Material's top app bar. iOS sets a large title in
 * the content and keeps its controls in glass; compact, it is a glass bar
 * with the title centred. Material sets the title at the leading edge of
 * a 64dp bar, or drops it under the bar in headline-medium when large.
 */
export function NavigationBar({
  title,
  large = false,
  back,
  leading,
  trailing,
  className,
  ...props
}: NavigationBarProps) {
  const platform = usePlatform()
  return (
    <header
      data-platform={platform}
      data-large={large || undefined}
      className={cx(
        'dzm-nav',
        platform === 'ios' && !large && 'dzm-glass',
        className
      )}
      {...props}
    >
      <div className="dzm-nav__bar">
        <div className="dzm-nav__leading">
          {back ? (
            <button
              type="button"
              className={cx(
                'dzm-nav__back',
                platform === 'ios' && 'dzm-glass',
                'dzm-m3-layer'
              )}
              onClick={back.onClick}
            >
              <ChevronLeft className="dzm-nav__back-glyph" />
              {platform === 'ios' && back.label ? (
                <span className="dzm-nav__back-label">{back.label}</span>
              ) : null}
            </button>
          ) : null}
          {leading}
        </div>
        {!large || platform === 'android' ? (
          <div className="dzm-nav__title" data-inline>
            {large ? null : title}
          </div>
        ) : null}
        <div className="dzm-nav__trailing">{trailing}</div>
      </div>
      {large ? <h1 className="dzm-nav__large-title">{title}</h1> : null}
    </header>
  )
}
