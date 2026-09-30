import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { ChevronRight } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface ListProps extends ComponentProps<'div'> {}

/**
 * The scrolling column of a Settings screen: inset grouped sections on iOS
 * — the cells a card at the 26pt corner, the header above and the footer
 * below in the footnote style — and Material's list, flat on the surface
 * with a subheader per section and a divider between items.
 */
export function List({ className, ...props }: ListProps) {
  const platform = usePlatform()
  return (
    <div
      data-platform={platform}
      className={cx('dzm-list', className)}
      {...props}
    />
  )
}

export interface ListSectionProps extends ComponentProps<'section'> {
  header?: ReactNode
  footer?: ReactNode
}

export function ListSection({
  header,
  footer,
  className,
  children,
  ...props
}: ListSectionProps) {
  const platform = usePlatform()
  return (
    <section
      data-platform={platform}
      className={cx('dzm-list-section', className)}
      {...props}
    >
      {header ? <div className="dzm-list-section__header">{header}</div> : null}
      <div className="dzm-list-section__group" role="group">
        {children}
      </div>
      {footer ? <div className="dzm-list-section__footer">{footer}</div> : null}
    </section>
  )
}

export interface ListItemProps extends Omit<ComponentProps<'div'>, 'title'> {
  /** The primary line. */
  title: ReactNode
  /** The line beneath it. */
  subtitle?: ReactNode
  /** A glyph at the leading edge. */
  icon?: ReactNode
  /**
   * iOS paints a leading glyph on a coloured rounded square, as Settings
   * does; pass the colour. Material draws the glyph alone.
   */
  iconColor?: string
  /** A value at the trailing edge — the current setting, in the secondary label. */
  value?: ReactNode
  /** A control at the trailing edge — a `Switch`, a `Badge`. */
  trailing?: ReactNode
  /** Draws the disclosure chevron and makes the row press. */
  href?: string
  onClick?: () => void
  /** The row is a choice in the destructive colour. */
  tone?: 'default' | 'destructive'
  disabled?: boolean
}

export function ListItem({
  title,
  subtitle,
  icon,
  iconColor,
  value,
  trailing,
  href,
  onClick,
  tone = 'default',
  disabled = false,
  className,
  ...props
}: ListItemProps) {
  const platform = usePlatform()
  const interactive = Boolean(onClick || href)
  const chevron = platform === 'ios' && interactive
  const content = (
    <>
      {icon ? (
        <span
          className="dzm-list-item__icon"
          style={
            iconColor
              ? ({ '--dzm-list-icon-fill': iconColor } as never)
              : undefined
          }
        >
          {icon}
        </span>
      ) : null}
      <span className="dzm-list-item__text">
        <span className="dzm-list-item__title">{title}</span>
        {subtitle ? (
          <span className="dzm-list-item__subtitle">{subtitle}</span>
        ) : null}
      </span>
      {value !== undefined ? (
        <span className="dzm-list-item__value">{value}</span>
      ) : null}
      {trailing ? (
        <span className="dzm-list-item__trailing">{trailing}</span>
      ) : null}
      {chevron ? (
        <span className="dzm-list-item__chevron">
          <ChevronRight />
        </span>
      ) : null}
    </>
  )
  const shared = {
    'data-platform': platform,
    'data-tone': tone,
    'data-interactive': interactive || undefined,
    'data-two-line': subtitle ? '' : undefined,
    className: cx('dzm-list-item', interactive && 'dzm-m3-layer', className),
  }
  if (href) {
    return (
      <a href={href} aria-disabled={disabled || undefined} {...shared}>
        {content}
      </a>
    )
  }
  if (onClick) {
    return (
      <button type="button" onClick={onClick} disabled={disabled} {...shared}>
        {content}
      </button>
    )
  }
  return (
    <div {...shared} {...props}>
      {content}
    </div>
  )
}
