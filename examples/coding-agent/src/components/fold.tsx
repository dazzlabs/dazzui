import { Icon } from '@dazzlabs/dazzui-react'
import type { ReactNode } from 'react'

import { ChevronGlyph } from '../data/icons'

export interface FoldProps {
  /** The row that is always visible. Laid out as a flex line. */
  summary: ReactNode
  open: boolean
  /** Left out, the row discloses nothing and stops offering to. */
  onToggle?: () => void
  /**
   * A control on the trailing edge of the row. It sits beside the disclosure
   * button rather than inside it: a button within a button is not a thing the
   * platform has, and a checkbox that also folds the row is not one either.
   */
  action?: ReactNode
  children?: ReactNode
}

/**
 * A disclosure row: a line saying what something is, with the thing itself
 * folded away underneath.
 *
 * It is drawn from tokens rather than from `Card`, because it has to hold a
 * full-bleed hover wash and a separator that reaches both edges — a card's
 * padding is the one thing a disclosure row cannot have.
 */
export function Fold({ summary, open, onToggle, action, children }: FoldProps) {
  const foldable = Boolean(children) && Boolean(onToggle)

  return (
    <div data-open={open} className="ca-fold">
      <div className="ca-fold__row">
        <button
          type="button"
          aria-expanded={foldable ? open : undefined}
          disabled={!foldable}
          className="ca-fold__head"
          onClick={onToggle}
        >
          {summary}
          {foldable ? (
            <Icon icon={ChevronGlyph} size={14} className="ca-fold__chevron" />
          ) : null}
        </button>
        {action ? <span className="ca-fold__action">{action}</span> : null}
      </div>

      {open && children ? (
        <div className="ca-fold__body">{children}</div>
      ) : null}
    </div>
  )
}
