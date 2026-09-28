import { Collapsible as Base } from '@base-ui/react/collapsible'
import { ChevronRight12Regular } from '@fluentui/react-icons'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'

export interface CollapsibleProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children' | 'title'
> {
  /** The header row's label — what the panel under it is a disclosure of. */
  title: ReactNode
  /** A glyph before the title. */
  icon?: ReactNode
  /** A trailing aside on the header — a count, a duration, a `Badge`. */
  meta?: ReactNode
  /**
   * Keeps the panel in the DOM while it is closed, so the page can be
   * searched and a form inside it still submits.
   */
  keepMounted?: boolean
  className?: string
  children?: ReactNode
}

/**
 * One thing that opens. The row is the affordance and the panel is what it
 * discloses; Base UI owns the part that is easy to get wrong — the height the
 * panel animates to, `aria-expanded` and `aria-controls`, and keeping the
 * panel out of the tab order while it is shut.
 *
 * It is the single disclosure. Several of them that know about each other are
 * an `Accordion`, which wears this same drawing.
 */
export function Collapsible({
  title,
  icon,
  meta,
  keepMounted,
  className,
  children,
  ...props
}: CollapsibleProps) {
  return (
    <Base.Root
      /* The row's hover comes off the `normal` recipe, and every
         `--control-color-*` is declared inside `[data-tint]` — a disclosure
         that named no ramp would inherit a dead value. It is fixed rather
         than a prop: a disclosure never re-tints, it is furniture. */
      data-tint="neutral"
      className={cx('dz-collapsible', className)}
      {...props}
    >
      <Base.Trigger className="dz-disclosure__trigger">
        <span aria-hidden className="dz-disclosure__chevron">
          <ChevronRight12Regular />
        </span>
        {icon ? <span className="dz-disclosure__icon">{icon}</span> : null}
        <span className="dz-disclosure__title">{title}</span>
        {meta ? <span className="dz-disclosure__meta">{meta}</span> : null}
      </Base.Trigger>
      <Base.Panel keepMounted={keepMounted} className="dz-disclosure__panel">
        <div className="dz-disclosure__content">{children}</div>
      </Base.Panel>
    </Base.Root>
  )
}
