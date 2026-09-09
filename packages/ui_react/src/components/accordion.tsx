import { Accordion as Base } from '@base-ui/react/accordion'
import { ChevronRight12Regular } from '@fluentui/react-icons'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'

export interface AccordionProps extends Omit<
  Base.Root.Props,
  'className' | 'render'
> {
  /**
   * Draws a hairline between the sections, for a run that has to read as one
   * list rather than as a stack of separate disclosures.
   */
  divided?: boolean
  className?: string
}

/**
 * Several disclosures that know about each other: the arrow keys move between
 * the headers, Home and End reach the ends, and by default opening one shuts
 * the last — `multiple` is what lets a reader hold two open at once.
 *
 * The row and the panel are the same drawing a `Collapsible` wears
 * (collapsible.css). What is the accordion's own is the run between the
 * sections, which is the thing a single disclosure has no opinion about.
 */
export function Accordion({
  divided = false,
  className,
  ...props
}: AccordionProps) {
  return (
    <Base.Root
      data-divided={divided}
      data-tint="neutral"
      className={cx('dz-accordion', className)}
      {...props}
    />
  )
}

export interface AccordionItemProps extends Omit<
  Base.Item.Props,
  'className' | 'render' | 'title'
> {
  /** The header row's label. */
  title: ReactNode
  /** A glyph before the title. */
  icon?: ReactNode
  /** A trailing aside on the header — a count, a duration, a `Badge`. */
  meta?: ReactNode
  className?: string
}

/** One section: the header that opens it, and everything it discloses. */
export function AccordionItem({
  title,
  icon,
  meta,
  className,
  children,
  ...props
}: AccordionItemProps) {
  return (
    <Base.Item className={cx('dz-accordion__item', className)} {...props}>
      <Base.Header className="dz-accordion__header">
        <Base.Trigger className="dz-disclosure__trigger">
          <span aria-hidden className="dz-disclosure__chevron">
            <ChevronRight12Regular />
          </span>
          {icon ? <span className="dz-disclosure__icon">{icon}</span> : null}
          <span className="dz-disclosure__title">{title}</span>
          {meta ? <span className="dz-disclosure__meta">{meta}</span> : null}
        </Base.Trigger>
      </Base.Header>
      <Base.Panel className="dz-disclosure__panel">
        <div className="dz-disclosure__content">{children}</div>
      </Base.Panel>
    </Base.Item>
  )
}
