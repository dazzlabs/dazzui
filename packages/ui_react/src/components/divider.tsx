import { Separator } from '@base-ui/react/separator'

import { cx } from '../lib/cx'

export type DividerTone = 'default' | 'muted'

export type DividerOrientation = 'horizontal' | 'vertical'

export interface DividerProps extends Omit<
  Separator.Props,
  'className' | 'render' | 'children'
> {
  tone?: DividerTone
  className?: string
}

export function Divider({
  tone = 'default',
  orientation = 'horizontal',
  className,
  ...props
}: DividerProps) {
  return (
    <Separator
      orientation={orientation}
      data-tone={tone}
      className={cx('dz-divider', className)}
      {...props}
    />
  )
}
