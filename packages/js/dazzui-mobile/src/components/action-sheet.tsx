import { Dialog as Base } from '@base-ui/react/dialog'
import type { ReactElement, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform, useThemeContainer } from '../lib/theme'

export interface ActionSheetAction {
  label: ReactNode
  /** A leading glyph; Material's rows carry one, iOS's do not. */
  icon?: ReactNode
  role?: 'default' | 'destructive'
  disabled?: boolean
  onClick?: () => void
}

export interface ActionSheetProps extends Omit<Base.Root.Props, 'children'> {
  trigger?: ReactElement<Record<string, unknown>>
  title?: ReactNode
  message?: ReactNode
  actions: readonly ActionSheetAction[]
  /** iOS's separate cancel group; Material dismisses by the scrim alone. */
  cancelLabel?: ReactNode
  className?: string
}

/**
 * UIAlertController's action sheet — the choices in one glass group and
 * cancel in its own — and, on Android, the list Material puts in a modal
 * bottom sheet for the same job.
 */
export function ActionSheet({
  trigger,
  title,
  message,
  actions,
  cancelLabel = 'Cancel',
  className,
  ...root
}: ActionSheetProps) {
  const platform = usePlatform()
  const container = useThemeContainer()
  return (
    <Base.Root {...root}>
      {trigger ? <Base.Trigger render={trigger} /> : null}
      <Base.Portal container={container}>
        <Base.Backdrop
          data-platform={platform}
          className="dzm-scrim dzm-action-sheet-scrim"
        />
        <Base.Viewport
          data-platform={platform}
          className="dzm-overlay dzm-action-sheet-viewport"
        >
          <Base.Popup
            data-platform={platform}
            className={cx('dzm-action-sheet', className)}
            aria-label={title ? undefined : 'Actions'}
          >
            <div
              className={cx(
                'dzm-action-sheet__group',
                platform === 'ios' && 'dzm-glass dzm-glass--thick'
              )}
            >
              {platform === 'android' ? (
                <span className="dzm-action-sheet__grabber" aria-hidden />
              ) : null}
              {title || message ? (
                <div className="dzm-action-sheet__header">
                  {title ? (
                    <Base.Title className="dzm-action-sheet__title">
                      {title}
                    </Base.Title>
                  ) : null}
                  {message ? (
                    <Base.Description className="dzm-action-sheet__message">
                      {message}
                    </Base.Description>
                  ) : null}
                </div>
              ) : null}
              {actions.map((action, index) => (
                <Base.Close
                  key={index}
                  data-role={action.role ?? 'default'}
                  disabled={action.disabled}
                  className="dzm-action-sheet__action dzm-m3-layer"
                  onClick={action.onClick}
                >
                  {action.icon ? (
                    <span className="dzm-action-sheet__icon">
                      {action.icon}
                    </span>
                  ) : null}
                  {action.label}
                </Base.Close>
              ))}
            </div>
            {platform === 'ios' ? (
              <div className="dzm-action-sheet__group dzm-action-sheet__cancel dzm-glass dzm-glass--thick">
                <Base.Close
                  className="dzm-action-sheet__action"
                  data-role="cancel"
                >
                  {cancelLabel}
                </Base.Close>
              </div>
            ) : null}
          </Base.Popup>
        </Base.Viewport>
      </Base.Portal>
    </Base.Root>
  )
}
