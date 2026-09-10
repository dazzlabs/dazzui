import { ArrowUpRight12Regular } from '@fluentui/react-icons'
import type { ComponentProps } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

/**
 * When the rule under the label is drawn. `hover` is the default because a
 * page of prose with an underline on every link reads as struck through;
 * `always` is for a link that has to be findable without a pointer, and
 * `none` for one whose surroundings already say it is a link — a breadcrumb,
 * a row that is entirely a target.
 */
export type LinkUnderline = 'hover' | 'always' | 'none'

export interface LinkProps extends ComponentProps<'a'> {
  tint?: Tint
  underline?: LinkUnderline
  /**
   * Opens away from this window. It sets the target and the `rel` that keeps
   * the opener out of the new page's reach, and draws the arrow that says so
   * before the reader commits.
   */
  external?: boolean
}

/**
 * Text that goes somewhere.
 *
 * It is the one interactive thing in this package that is genuinely inline,
 * which is why it exists beside `Button variant="plain"` rather than inside
 * it: a button keeps its box at every variant, and a box is exactly what a
 * link in a sentence must not have. So this takes the shared state machine
 * for its colour — `plain`'s content recipe, a grade darker on hover — and
 * none of its geometry.
 */
export function Link({
  tint = 'primary',
  underline = 'hover',
  external = false,
  className,
  children,
  target,
  rel,
  ...props
}: LinkProps) {
  return (
    <a
      data-control=""
      data-tint={tint}
      data-underline={underline}
      data-variant="plain"
      target={target ?? (external ? '_blank' : undefined)}
      rel={rel ?? (external ? 'noreferrer noopener' : undefined)}
      className={cx('dz-link', className)}
      {...props}
    >
      {children}
      {external ? (
        <span aria-hidden className="dz-link__external">
          <ArrowUpRight12Regular />
        </span>
      ) : null}
    </a>
  )
}
