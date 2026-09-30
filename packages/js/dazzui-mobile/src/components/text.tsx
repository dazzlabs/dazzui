import type { ComponentProps, ElementType } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

/**
 * A type role named as iOS names it and drawn as each OS draws its
 * nearest: `largeTitle` is Dynamic Type's on iOS and `headlineLarge` on
 * Android, `body` is 17pt there and `bodyLarge` here. The mapping is in
 * `text.css`, so a screen written once sets its type as the platform does.
 */
export type TextRole =
  | 'largeTitle'
  | 'title'
  | 'title2'
  | 'title3'
  | 'headline'
  | 'body'
  | 'callout'
  | 'subheadline'
  | 'footnote'
  | 'caption'

/** The label ladder: primary ink, then the two quieter steps. */
export type TextTone = 'primary' | 'secondary' | 'tertiary' | 'tint'

export interface TextProps extends ComponentProps<'span'> {
  role?: TextRole
  tone?: TextTone
  /** The element to render; a `p` or a heading where the document needs one. */
  as?: ElementType
  /** Ends the line with an ellipsis rather than wrapping. */
  truncate?: boolean
}

export function Text({
  role = 'body',
  tone = 'primary',
  as: Tag = 'span',
  truncate = false,
  className,
  ...props
}: TextProps) {
  const platform = usePlatform()
  return (
    <Tag
      data-platform={platform}
      data-role={role}
      data-tone={tone}
      data-truncate={truncate || undefined}
      className={cx('dzm-text', className)}
      {...props}
    />
  )
}
