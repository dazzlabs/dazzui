import { Switch as Base } from '@base-ui/react/switch'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type SwitchSize = 'small' | 'medium' | 'large'

export interface SwitchProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  size?: SwitchSize
  tint?: Tint
  className?: string
}

export function Switch({
  size = 'medium',
  tint = 'primary',
  className,
  ...props
}: SwitchProps) {
  return (
    <Base.Root
      data-size={size}
      data-tint={tint}
      className={cx('dz-switch', className)}
      {...props}
    >
      <Base.Thumb className="dz-switch__thumb" />
    </Base.Root>
  )
}
