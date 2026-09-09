import { Avatar as Base } from '@base-ui/react/avatar'
import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type AvatarSize = 'small' | 'medium' | 'large'

export type AvatarShape = 'circle' | 'square'

export interface AvatarProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  /** The picture. While it loads, and if it never arrives, the fallback shows. */
  src?: string
  /**
   * Who or what this is. It names the avatar to a screen reader and, unless
   * `fallback` says otherwise, it is where the initials come from.
   */
  name?: string
  /** Drawn in place of the picture. Defaults to the initials of `name`. */
  fallback?: ReactNode
  size?: AvatarSize
  /** A person is a circle and a thing is a square — a project, a repository. */
  shape?: AvatarShape
  tint?: Tint
  className?: string
}

/**
 * Initials for the fallback: the first letter of the first word and of the
 * last, which is the pair a reader actually recognises. One word gives one
 * letter rather than two off the same word, since "Ac" for Acme reads as a
 * truncation rather than as a monogram.
 */
function initialsOf(name: string): string {
  const words = name.trim().split(/\s+/).filter(Boolean)
  if (words.length === 0) return ''
  const first = words[0]?.[0] ?? ''
  const last = words.length > 1 ? (words.at(-1)?.[0] ?? '') : ''
  return (first + last).toUpperCase()
}

/**
 * The picture that stands for a person or a project.
 *
 * Base UI owns the part that is invisible until it fails: the image's loading
 * status, so the fallback is what is drawn until the picture has actually
 * decoded rather than flashing empty behind a broken one.
 *
 * It is not interactive and so stays off the shared control state machine —
 * it reads the `tinted` recipe's resting colours directly. An avatar that
 * lit up under the pointer would read as something you can press.
 */
export function Avatar({
  src,
  name,
  fallback,
  size = 'medium',
  shape = 'circle',
  tint = 'primary',
  className,
  ...props
}: AvatarProps) {
  return (
    <Base.Root
      role="img"
      aria-label={name}
      data-shape={shape}
      data-size={size}
      data-tint={tint}
      className={cx('dz-avatar', className)}
      {...props}
    >
      {src ? (
        <Base.Image src={src} alt={name} className="dz-avatar__image" />
      ) : null}
      <Base.Fallback className="dz-avatar__fallback">
        {fallback ?? (name ? initialsOf(name) : null)}
      </Base.Fallback>
    </Base.Root>
  )
}

export interface AvatarGroupProps extends ComponentPropsWithoutRef<'div'> {
  /**
   * How many to draw before the rest become a count. Omit to draw every
   * child.
   */
  max?: number
  /**
   * The size the avatars inside are drawn at. The group does not set it on
   * them — each one carries its own — but the overlap and the count chip are
   * measured from it, and they have no other way to know.
   */
  size?: AvatarSize
}

/**
 * A run of avatars, overlapped. Each one is cut out of the one before it, so
 * the stack reads as a group rather than as a smear. The cut is the paper
 * colour, so a group sitting on a card re-points `--avatar-ring` to the
 * card's own surface — the same move a second navigation column makes with
 * `--sidebar-width`.
 */
export function AvatarGroup({
  max,
  size = 'medium',
  className,
  children,
  ...props
}: AvatarGroupProps) {
  const all = Array.isArray(children) ? children.flat() : [children]
  const shown = max === undefined ? all : all.slice(0, max)
  const hidden = all.length - shown.length

  return (
    <div
      data-size={size}
      className={cx('dz-avatar-group', className)}
      {...props}
    >
      {shown}
      {hidden > 0 ? (
        <span aria-hidden className="dz-avatar-group__rest">
          +{hidden}
        </span>
      ) : null}
    </div>
  )
}
