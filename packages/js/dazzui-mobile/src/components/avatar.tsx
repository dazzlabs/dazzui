import { Avatar as Base } from '@base-ui/react/avatar'
import {
  Children,
  type ComponentProps,
  createContext,
  type ReactNode,
  useContext,
} from 'react'

import { cx } from '../lib/cx'
import { type Platform, usePlatform } from '../lib/theme'

/**
 * `small` is the avatar a bar or a chip carries, `medium` the one a list
 * row leads with — Messages' conversation list, Material's 40dp list
 * avatar — and `large` the one a profile header shows.
 */
export type AvatarSize = 'small' | 'medium' | 'large'

const GroupContext = createContext<AvatarSize | undefined>(undefined)

export interface AvatarProps extends Omit<ComponentProps<'span'>, 'children'> {
  /** The picture. While it loads, or when it fails, the fallback shows. */
  src?: string
  /** What the avatar is of, for a reader. Defaults to `name`. */
  alt?: string
  /** The person's name; the fallback draws its initials. */
  name?: string
  size?: AvatarSize
  /**
   * The glyph the fallback draws when there is no name. Without one, the
   * kit draws the platform's own person silhouette.
   */
  icon?: ReactNode
}

/**
 * A person's picture, or their initials when there is none. iOS draws the
 * fallback as Contacts draws its monogram — `CNAvatarView`: white rounded
 * initials on a grey gradient that darkens toward the bottom — and
 * Material draws it on the primary container, the initials in its ink.
 * Loading is Base UI's: the image only replaces the fallback once it has
 * decoded, so a slow or broken picture never flashes an empty circle.
 */
export function Avatar({
  src,
  alt,
  name,
  size,
  icon,
  className,
  ...props
}: AvatarProps) {
  const platform = usePlatform()
  const group = useContext(GroupContext)
  const label = alt ?? name
  const letters = name ? initials(name) : ''
  return (
    <Base.Root
      role={label ? 'img' : undefined}
      aria-label={label}
      aria-hidden={label ? undefined : true}
      data-platform={platform}
      data-size={size ?? group ?? 'medium'}
      className={cx('dzm-avatar', className)}
      {...props}
    >
      {src ? (
        <Base.Image src={src} alt="" className="dzm-avatar__image" />
      ) : null}
      <Base.Fallback aria-hidden className="dzm-avatar__fallback">
        {letters ? (
          <span className="dzm-avatar__initials">{letters}</span>
        ) : icon ? (
          <span className="dzm-avatar__icon">{icon}</span>
        ) : (
          <Person platform={platform} />
        )}
      </Base.Fallback>
    </Base.Root>
  )
}

export interface AvatarGroupProps extends ComponentProps<'div'> {
  /** The size every avatar in the group takes, unless it sets its own. */
  size?: AvatarSize
  /** How many avatars show; the rest collapse into one "+N" after them. */
  max?: number
}

/**
 * A row of avatars that overlap, each cut out of the one before it by a
 * ring of whatever the row stands on — the stack a shared note or album
 * shows its people in. `max` collapses the rest into a count, drawn as
 * one more monogram.
 */
export function AvatarGroup({
  size = 'medium',
  max,
  className,
  children,
  ...props
}: AvatarGroupProps) {
  const platform = usePlatform()
  const avatars = Children.toArray(children)
  const shown =
    max !== undefined && avatars.length > max ? avatars.slice(0, max) : avatars
  const rest = avatars.length - shown.length
  return (
    <GroupContext.Provider value={size}>
      <div
        role="group"
        data-platform={platform}
        data-size={size}
        className={cx('dzm-avatar-group', className)}
        {...props}
      >
        {shown}
        {rest > 0 ? (
          <span
            role="img"
            aria-label={`${rest} more`}
            data-platform={platform}
            data-size={size}
            className="dzm-avatar"
          >
            <span className="dzm-avatar__fallback">
              <span className="dzm-avatar__initials">+{rest}</span>
            </span>
          </span>
        ) : null}
      </div>
    </GroupContext.Provider>
  )
}

/**
 * The first letter of the first and last names, as Contacts takes them;
 * one letter for a name of one word. Counted in code points, so a name
 * that starts outside the BMP keeps its whole first character.
 */
function initials(name: string): string {
  const words = name.trim().split(/\s+/).filter(Boolean)
  const first = words[0]
  const last = words.length > 1 ? words[words.length - 1] : undefined
  const letter = (word: string | undefined) =>
    word ? (Array.from(word)[0] ?? '') : ''
  return (letter(first) + letter(last)).toLocaleUpperCase()
}

/**
 * The silhouette an avatar with nothing else to show draws. iOS's is
 * `person.crop.circle.fill` — the shoulders run off the circle's foot —
 * and Material's is the `person` glyph, centred at 24dp.
 */
function Person({ platform }: { platform: Platform }) {
  return platform === 'ios' ? (
    <svg
      aria-hidden
      viewBox="0 0 24 24"
      fill="currentColor"
      className="dzm-avatar__person"
    >
      <circle cx="12" cy="9.4" r="4.4" />
      <path d="M2.6 25c0-5.4 4.2-9.2 9.4-9.2s9.4 3.8 9.4 9.2z" />
    </svg>
  ) : (
    <svg
      aria-hidden
      viewBox="0 0 24 24"
      fill="currentColor"
      className="dzm-avatar__person"
    >
      <circle cx="12" cy="8" r="4" />
      <path d="M4 20v-1.2C4 16.1 7.6 14 12 14s8 2.1 8 4.8V20z" />
    </svg>
  )
}
