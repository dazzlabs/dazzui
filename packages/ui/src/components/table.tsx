import type { ComponentPropsWithoutRef, KeyboardEvent } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export interface TableProps extends ComponentPropsWithoutRef<'div'> {
  /** Which ramp a selected row is picked out with. */
  tint?: Tint
}

export function Table({ tint = 'primary', className, ...props }: TableProps) {
  return (
    <div
      role="table"
      data-tint={tint}
      className={cx('dz-table', className)}
      {...props}
    />
  )
}

export function TableHead({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return (
    <div role="row" className={cx('dz-table__head', className)} {...props} />
  )
}

export interface TableRowProps extends ComponentPropsWithoutRef<'div'> {
  active?: boolean
  /** Makes the row selectable, and reachable from the keyboard. */
  interactive?: boolean
}

export function TableRow({
  active = false,
  interactive = false,
  className,
  onClick,
  onKeyDown,
  ...props
}: TableRowProps) {
  const handleKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    onKeyDown?.(event)
    if (!interactive || !onClick || event.defaultPrevented) return
    if (event.key !== 'Enter' && event.key !== ' ') return

    // Space would otherwise scroll the list out from under the selection.
    event.preventDefault()
    onClick(event as unknown as Parameters<NonNullable<typeof onClick>>[0])
  }

  return (
    <div
      role="row"
      tabIndex={interactive ? 0 : undefined}
      aria-selected={interactive ? active : undefined}
      data-active={active}
      data-interactive={interactive}
      className={cx('dz-table__row', className)}
      onClick={onClick}
      onKeyDown={handleKeyDown}
      {...props}
    />
  )
}

export interface TableCellProps extends ComponentPropsWithoutRef<'div'> {
  /** Renders as a column heading. */
  head?: boolean
  align?: 'start' | 'end'
}

export function TableCell({
  head = false,
  align = 'start',
  className,
  ...props
}: TableCellProps) {
  return (
    <div
      role={head ? 'columnheader' : 'cell'}
      data-align={align}
      data-head={head}
      className={cx('dz-table__cell', className)}
      {...props}
    />
  )
}
