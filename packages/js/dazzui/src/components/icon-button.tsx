import { useRender } from '@base-ui/react/use-render'
import type { ComponentProps, ReactElement, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import type { ButtonSize, ButtonVariant } from './button'
import { Tooltip } from './tooltip'

export interface IconButtonProps extends Omit<
  ComponentProps<'button'>,
  'children'
> {
  /** Accessible name, also surfaced as the tooltip. */
  label: string
  size?: ButtonSize
  variant?: ButtonVariant
  tint?: Tint
  /** Render as another element — an `<a>` or router `<Link>` where the button navigates. */
  render?: useRender.RenderProp
  children?: ReactNode
}

/**
 * A `Tooltip` rather than a `title`: the native one is late, unstyled and
 * absent from every keyboard. The button itself is a plain element, which is
 * what lets a `Menu` or a `Dialog` take it as a trigger.
 *
 * It does not latch. A glyph button that stays down is a `Toggle`, which is
 * square at the same control height when its label is a lone glyph — so the
 * two sit in one toolbar without a seam. Keeping an on-state here as well
 * would be a second drawing for one state, and the system draws a selection
 * one way: a tinted wash and a heavier label.
 */
export function IconButton({
  label,
  size = 'tiny',
  variant = 'plain',
  // Neutral by default: a toolbar of these should read as chrome, not as a row
  // of accented actions. Pass a tint where one button genuinely leads.
  tint = 'neutral',
  render,
  ref,
  className,
  children,
  type,
  ...props
}: IconButtonProps) {
  const element = useRender({
    render,
    ref,
    defaultTagName: 'button',
    props: {
      type: render ? type : (type ?? 'button'),
      'aria-label': label,
      'data-control': '',
      'data-size': size,
      'data-tint': tint,
      'data-variant': variant,
      className: cx('dz-icon-button', className),
      ...(children !== undefined ? { children } : {}),
      ...props,
    },
  })

  return (
    <Tooltip label={label}>
      {element as ReactElement<Record<string, unknown>>}
    </Tooltip>
  )
}
