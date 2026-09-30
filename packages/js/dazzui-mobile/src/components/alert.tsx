import { AlertDialog as Base } from '@base-ui/react/alert-dialog'
import type { ReactElement, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform, useThemeContainer } from '../lib/theme'
import { Button } from './button'

export interface AlertAction {
  label: ReactNode
  /**
   * `cancel` sets in the bold weight on iOS and dismisses; `destructive`
   * sets in the danger colour. Both close the alert when pressed.
   */
  role?: 'default' | 'cancel' | 'destructive'
  onClick?: () => void
}

export interface AlertProps extends Omit<Base.Root.Props, 'children'> {
  trigger?: ReactElement<Record<string, unknown>>
  title: ReactNode
  message?: ReactNode
  /** Material's optional glyph above the headline. */
  icon?: ReactNode
  /**
   * In order. iOS sets two side by side with cancel on the left, and
   * stacks three or more with cancel at the bottom; Material sets them at
   * the trailing edge, the confirming action last.
   */
  actions: readonly AlertAction[]
  className?: string
}

/**
 * UIAlertController's alert and Material's basic dialog: the modal that
 * asks one question. Neither is dismissed by a press outside it, which is
 * why it is Base UI's alert dialog rather than its dialog.
 */
export function Alert({
  trigger,
  title,
  message,
  icon,
  actions,
  className,
  ...root
}: AlertProps) {
  const platform = usePlatform()
  const container = useThemeContainer()
  const stacked = platform === 'ios' && actions.length > 2
  /*
   * iOS places cancel where the thumb expects it: on the left of a pair,
   * and at the bottom of a stack. Material keeps the order it is given,
   * the confirming action last.
   */
  const rank = (action: AlertAction) =>
    action.role === 'cancel' ? (stacked ? 1 : -1) : 0
  const ordered =
    platform === 'ios'
      ? [...actions].sort((a, b) => rank(a) - rank(b))
      : actions
  return (
    <Base.Root {...root}>
      {trigger ? <Base.Trigger render={trigger} /> : null}
      <Base.Portal container={container}>
        <Base.Backdrop
          data-platform={platform}
          className="dzm-scrim dzm-alert-scrim"
        />
        <Base.Viewport
          data-platform={platform}
          className="dzm-overlay dzm-alert-viewport"
        >
          <Base.Popup
            data-platform={platform}
            data-stacked={stacked || undefined}
            className={cx(
              'dzm-alert',
              platform === 'ios' && 'dzm-glass dzm-glass--thick',
              className
            )}
          >
            <div className="dzm-alert__content">
              {icon && platform === 'android' ? (
                <span className="dzm-alert__icon">{icon}</span>
              ) : null}
              <Base.Title className="dzm-alert__title">{title}</Base.Title>
              {message ? (
                <Base.Description className="dzm-alert__message">
                  {message}
                </Base.Description>
              ) : null}
            </div>
            <div className="dzm-alert__actions">
              {ordered.map((action, index) =>
                platform === 'ios' ? (
                  <Base.Close
                    key={index}
                    data-role={action.role ?? 'default'}
                    className="dzm-alert__action"
                    onClick={action.onClick}
                  >
                    {action.label}
                  </Base.Close>
                ) : (
                  <Base.Close
                    key={index}
                    render={
                      <Button
                        variant="plain"
                        size="small"
                        tone={
                          action.role === 'destructive'
                            ? 'destructive'
                            : 'default'
                        }
                      />
                    }
                    onClick={action.onClick}
                  >
                    {action.label}
                  </Base.Close>
                )
              )}
            </div>
          </Base.Popup>
        </Base.Viewport>
      </Base.Portal>
    </Base.Root>
  )
}
