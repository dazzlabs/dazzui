import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'

export type DialogTone = 'default' | 'danger'

export interface DialogProps extends ComponentPropsWithoutRef<'div'> {
  /** Overrides the token width for a sheet that needs more or less room. */
  width?: number | string
  tone?: DialogTone
}

export function Dialog({
  width,
  tone = 'default',
  className,
  style,
  ...props
}: DialogProps) {
  return (
    <div
      role="dialog"
      aria-modal="true"
      data-tone={tone}
      className={cx('dz-dialog', className)}
      style={width === undefined ? style : { width, ...style }}
      {...props}
    />
  )
}

export interface DialogHeaderProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  title: ReactNode
  subtitle?: ReactNode
}

export function DialogHeader({
  title,
  subtitle,
  className,
  children,
  ...props
}: DialogHeaderProps) {
  return (
    <div className={cx('dz-dialog__header', className)} {...props}>
      <span className="dz-dialog__title">{title}</span>
      {subtitle ? (
        <span className="dz-dialog__subtitle">{subtitle}</span>
      ) : null}
      {children}
    </div>
  )
}

export function DialogBody({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-dialog__body', className)} {...props} />
}

export function DialogFooter({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-dialog__footer', className)} {...props} />
}

/** The dimmed backdrop a `Dialog` is centred in. */
export function DialogScrim({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-dialog-scrim', className)} {...props} />
}
