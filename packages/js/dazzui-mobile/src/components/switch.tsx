import { Switch as Base } from '@base-ui/react/switch'

import { cx } from '../lib/cx'
import { Check } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface SwitchProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  className?: string
  /**
   * Material draws a check in the thumb when it is on; iOS's thumb is
   * always plain. Off by default, because a switch beside a label already
   * says what it is.
   */
  icon?: boolean
}

/**
 * UISwitch and Material's Switch: the same state machine — Base UI's —
 * under two drawings. Both slide a thumb along a track; Material's thumb
 * also grows as it turns on, and its track keeps an outline while off.
 */
export function Switch({ icon = false, className, ...props }: SwitchProps) {
  const platform = usePlatform()
  return (
    <Base.Root
      data-platform={platform}
      data-icon={icon || undefined}
      className={cx('dzm-switch', className)}
      {...props}
    >
      <Base.Thumb className="dzm-switch__thumb">
        {platform === 'android' && icon ? (
          <Check className="dzm-switch__check" />
        ) : null}
      </Base.Thumb>
    </Base.Root>
  )
}
