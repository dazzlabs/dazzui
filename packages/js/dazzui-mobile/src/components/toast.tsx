import { Toast as Base } from '@base-ui/react/toast'
import { type ReactNode, useEffect, useState } from 'react'

import { cx } from '../lib/cx'
import { usePlatform, useThemeContainer } from '../lib/theme'

const NOTICE_MS = 4000
const ACTION_MS = 8000

export interface ToastAction {
  label: ReactNode
  onClick: () => void
}

interface ToastData {
  icon?: ReactNode
  action?: ToastAction
}

export interface ToastOptions {
  id?: string
  /** One line. */
  title: ReactNode
  /** A leading glyph; iOS's capsule usually carries one. */
  icon?: ReactNode
  /** Material's snackbar action — undo, retry. */
  action?: ToastAction
  /** Milliseconds before it goes; `0` keeps it until dismissed. */
  timeout?: number
}

/**
 * The queue. Call `add` from anywhere under a `ToastProvider`; Base UI
 * keeps the timers, the limit, the pause on hover and the announcement.
 */
export function useToast() {
  const manager = Base.useToastManager()
  return {
    add: ({ id, title, icon, action, timeout }: ToastOptions) =>
      manager.add({
        id,
        title,
        timeout: timeout ?? (action ? ACTION_MS : NOTICE_MS),
        data: { icon, action } satisfies ToastData,
      }),
    close: manager.close,
  }
}

export interface ToastProviderProps {
  children?: ReactNode
  /** How many are shown at once. Material shows one. */
  limit?: number
}

/**
 * Holds the queue and draws it, portalled into the theme. On iOS a notice
 * is the capsule that drops from the top edge — the one the system uses
 * for AirPods and Silent mode, which iOS has in place of a toast. On
 * Android it is Material's snackbar at the bottom.
 */
export function ToastProvider({ children, limit = 1 }: ToastProviderProps) {
  return (
    <Base.Provider limit={limit}>
      {children}
      <ToastViewport />
    </Base.Provider>
  )
}

function ToastViewport() {
  const platform = usePlatform()
  const container = useThemeContainer()
  const { toasts, close } = Base.useToastManager()
  /*
   * The viewport mounts with the theme, before the theme's element is
   * there to portal into, and Base UI's lighter toast portal reads a ref
   * once. So the element is taken off the ref in an effect and the portal
   * waits for it; a sheet or a menu opens later and never hits this.
   */
  const [node, setNode] = useState<HTMLElement | null>(null)
  useEffect(() => {
    setNode(container?.current ?? null)
  }, [container])
  if (container && !node) return null
  return (
    <Base.Portal container={node ?? undefined}>
      <Base.Viewport data-platform={platform} className="dzm-toast-viewport">
        {toasts.map(toast => {
          const data = (toast.data ?? {}) as ToastData
          return (
            <Base.Root
              key={toast.id}
              toast={toast}
              data-platform={platform}
              className={cx(
                'dzm-toast',
                platform === 'ios' && 'dzm-glass dzm-glass--thick'
              )}
            >
              {data.icon ? (
                <span className="dzm-toast__icon">{data.icon}</span>
              ) : null}
              <Base.Title render={<span />} className="dzm-toast__title" />
              {data.action ? (
                <Base.Action
                  className="dzm-toast__action"
                  onClick={() => {
                    data.action?.onClick()
                    close(toast.id)
                  }}
                >
                  {data.action.label}
                </Base.Action>
              ) : null}
            </Base.Root>
          )
        })}
      </Base.Viewport>
    </Base.Portal>
  )
}
