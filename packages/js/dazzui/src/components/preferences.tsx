import {
  type ComponentPropsWithoutRef,
  type CSSProperties,
  type ReactNode,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { SectionLabel } from './section-label'
import { Tooltip } from './tooltip'

export interface PreferencesProps extends ComponentPropsWithoutRef<'div'> {}

/**
 * The settings column, and the root the rest of this file sits in.
 *
 * It owns the two things no group or section can: the run between groups,
 * which is the widest step of the spacing ladder this column climbs, and the
 * width. The column is capped rather than filling its pane — a row puts its
 * label on one edge and its control on the other, and past `preferences.width`
 * the two stop reading as one line.
 */
export function Preferences({ className, ...props }: PreferencesProps) {
  return <div className={cx('dz-preferences', className)} {...props} />
}

export interface PreferenceSectionProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  /** The heading over the rows. Omit for an unlabelled run. */
  label?: ReactNode
  /** A control on the heading line, pushed to the trailing edge. */
  action?: ReactNode
  /** A muted note under the rows, for what the section cannot fit. */
  footer?: ReactNode
}

/** A labelled run of preference rows. */
export function PreferenceSection({
  label,
  action,
  footer,
  className,
  children,
  ...props
}: PreferenceSectionProps) {
  return (
    <div className={cx('dz-preference-section', className)} {...props}>
      {label || action ? (
        <div className="dz-preference-section__heading">
          {label ? <SectionLabel>{label}</SectionLabel> : <span />}
          {action ? (
            <span className="dz-preference__heading-action">{action}</span>
          ) : null}
        </div>
      ) : null}
      <div className="dz-preference-section__rows">{children}</div>
      {footer ? (
        <span className="dz-preference-section__footer">{footer}</span>
      ) : null}
    </div>
  )
}

export interface PreferenceGroupProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  title: ReactNode
  /** The line under the title, for what the group is for. */
  description?: ReactNode
  action?: ReactNode
}

/**
 * A titled run of sections, for a page whose sections come in bunches.
 *
 * Sections stay one size everywhere; the level above them is a title that
 * outranks them, not a section shrunk to fit underneath one. A heading then
 * means the same thing wherever it is met.
 */
export function PreferenceGroup({
  title,
  description,
  action,
  className,
  children,
  ...props
}: PreferenceGroupProps) {
  return (
    <div className={cx('dz-preference-group', className)} {...props}>
      <div className="dz-preference-group__heading">
        <div className="dz-preference-group__titles">
          <span className="dz-preference-group__title">{title}</span>
          {description ? (
            <span className="dz-preference-group__description">
              {description}
            </span>
          ) : null}
        </div>
        {action ? (
          <span className="dz-preference__heading-action">{action}</span>
        ) : null}
      </div>
      <div className="dz-preference-group__sections">{children}</div>
    </div>
  )
}

export interface PreferenceRowProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title' | 'onClick'
> {
  title: ReactNode
  /** The second line, for what the title cannot say in three words. */
  subtitle?: ReactNode
  /** A glyph or avatar before the title, for rows that name a thing. */
  icon?: ReactNode
  /**
   * Makes the row open something. It becomes a button and grows the hover wash
   * that says so; rows that only carry a control leave this off and stay inert.
   */
  onOpen?: () => void
  tint?: Tint
}

/** One preference row: title and optional subtitle left, control right. */
export function PreferenceRow({
  title,
  subtitle,
  icon,
  onOpen,
  tint = 'neutral',
  className,
  children,
  ...props
}: PreferenceRowProps) {
  const body = (
    <>
      {icon ? <span className="dz-preference-row__icon">{icon}</span> : null}
      <div className="dz-preference-row__text">
        <span className="dz-preference-row__title">{title}</span>
        {subtitle ? (
          <span className="dz-preference-row__subtitle">{subtitle}</span>
        ) : null}
      </div>
      {children || onOpen ? (
        <div className="dz-preference-row__trailing">
          {children}
          {onOpen ? <span aria-hidden>›</span> : null}
        </div>
      ) : null}
    </>
  )

  if (onOpen) {
    return (
      <button
        type="button"
        data-interactive="true"
        data-tint={tint}
        className={cx('dz-preference-row', className)}
        onClick={onOpen}
      >
        {body}
      </button>
    )
  }

  return (
    <div
      data-interactive="false"
      data-tint={tint}
      className={cx('dz-preference-row', className)}
      {...props}
    >
      {body}
    </div>
  )
}

export interface PreferenceThemeOption<T extends string = string> {
  value: T
  /** The theme's name, in the tooltip and to a screen reader. */
  label: string
  /**
   * The `data-theme` the swatch is drawn in; `value` when left out. An empty
   * string is the theme `:root` holds, which has no selector of its own.
   */
  theme?: string
}

export interface PreferenceThemeSwatchesProps<
  T extends string = string,
> extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'defaultValue' | 'onChange' | 'children'
> {
  options: readonly PreferenceThemeOption<T>[]
  value?: T
  defaultValue?: T
  onValueChange?: (value: T) => void
}

/**
 * The theme, as a row's control: a run of swatches at the trailing edge of a
 * `PreferenceRow`, each a window in miniature drawn in that theme's own
 * colours — its column, its paper, a line of its ink and its accent. The
 * name is the tooltip's, so a row of six stays the width of a control.
 *
 * A swatch takes its theme by nesting that theme's `data-theme`, which is
 * all a token sheet selects on. The one theme that cannot be asked for that
 * way is the one `:root` holds; its values are read once, off the document
 * with its own theme taken off, and laid on the swatch directly.
 */
export function PreferenceThemeSwatches<T extends string = string>({
  options,
  value,
  defaultValue,
  onValueChange,
  className,
  ...props
}: PreferenceThemeSwatchesProps<T>) {
  const [own, setOwn] = useState(defaultValue)
  const current = value ?? own
  return (
    <div
      role="radiogroup"
      className={cx('dz-preference-themes', className)}
      {...props}
    >
      {options.map(option => {
        const theme = option.theme ?? option.value
        const atRoot = theme === ''
        return (
          <Tooltip key={option.value} label={option.label}>
            <button
              type="button"
              role="radio"
              aria-checked={option.value === current}
              aria-label={option.label}
              className="dz-preference-themes__swatch"
              onClick={() => {
                setOwn(option.value)
                onValueChange?.(option.value)
              }}
            >
              <span
                className="dz-preference-themes__window"
                data-theme={atRoot ? undefined : theme}
                style={atRoot ? readRootTheme() : undefined}
                aria-hidden
              >
                <span className="dz-preference-themes__column" />
                <span className="dz-preference-themes__paper">
                  <span className="dz-preference-themes__ink" />
                  <span className="dz-preference-themes__accent" />
                </span>
              </span>
            </button>
          </Tooltip>
        )
      })}
    </div>
  )
}

/** What a swatch draws with. */
const DRAWN = [
  '--color-surface',
  '--color-surface-column',
  '--color-border-strong',
  '--color-content',
  '--color-primary-500',
] as const

let rootTheme: CSSProperties | undefined

/** The values `:root` holds under no `data-theme`, read once. */
function readRootTheme(): CSSProperties {
  if (rootTheme) return rootTheme
  const root = document.documentElement
  const applied = root.getAttribute('data-theme')
  root.removeAttribute('data-theme')
  const computed = getComputedStyle(root)
  rootTheme = Object.fromEntries(
    DRAWN.map(name => [name, computed.getPropertyValue(name)])
  ) as CSSProperties
  if (applied !== null) root.setAttribute('data-theme', applied)
  return rootTheme
}
