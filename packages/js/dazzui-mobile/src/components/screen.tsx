import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface ScreenProps extends ComponentProps<'div'> {
  /** The bar across the top — a `NavigationBar`. */
  bar?: ReactNode
  /** The bar across the bottom — a `TabBar`. */
  tabBar?: ReactNode
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
  className,
  children,
  ...props
}: ScreenProps) {
  const platform = usePlatform()
  return (
    <div
      data-platform={platform}
      className={cx('dzm-screen', className)}
      {...props}
    >
      <div className="dzm-screen__scroll">
        {bar}
        <div className="dzm-screen__content">{children}</div>
        {tabBar}
      </div>
    </div>
  )
}
