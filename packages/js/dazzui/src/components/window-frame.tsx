import type { ComponentPropsWithoutRef, CSSProperties, ReactNode } from 'react'

import { cx } from '../lib/cx'

/**
 * Whose window chrome is drawn. `macos` is macOS 27's Liquid Glass lights,
 * `macos15` the flat ones before it; `gnome` is libadwaita's headerbar with
 * its default layout of Close alone, `ubuntu` the same headerbar with all
 * three; `kde` is Breeze; `omarchy` is Hyprland, which draws no titlebar.
 */
export type WindowFramePlatform =
  | 'macos'
  | 'macos15'
  | 'windows'
  | 'gnome'
  | 'ubuntu'
  | 'kde'
  | 'omarchy'

export interface WindowFrameControls {
  /** `false` draws the control disabled, or leaves it out where the platform does. */
  close?: boolean
  minimize?: boolean
  maximize?: boolean
}

export interface WindowFrameProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  /** The window's title. Kept as its accessible name even when not drawn. */
  title: string
  platform?: WindowFramePlatform
  /** macOS only: a transparent titlebar with the title hidden. */
  titleHidden?: boolean
  controls?: WindowFrameControls
  /** Not the key window: the controls and the title grey out. */
  inactive?: boolean
  /** Content width, as the window's content rect states it. */
  width?: number | string
  /** Content height; left out, the window sizes to its content. */
  height?: number | string
  onClose?: () => void
  onMinimize?: () => void
  onMaximize?: () => void
  children?: ReactNode
}

const ALL: WindowFrameControls = { close: true, minimize: true, maximize: true }

const length = (value: number | string | undefined) =>
  typeof value === 'number' ? `${value}px` : value

/**
 * A top-level window: the titlebar with the platform's own controls, and the
 * content below it. The colours are the theme's; the chrome — where the
 * controls sit, their glyphs and their sizes — is the platform's, transcribed
 * rather than decided here, so a window reads as the desktop it names under
 * any theme.
 */
export function WindowFrame({
  title,
  platform = 'macos',
  titleHidden = false,
  controls = ALL,
  inactive = false,
  width,
  height,
  onClose,
  onMinimize,
  onMaximize,
  className,
  style,
  children,
  ...props
}: WindowFrameProps) {
  const handlers = { onClose, onMinimize, onMaximize }
  return (
    <div
      role="group"
      aria-label={title}
      data-platform={platform}
      data-inactive={inactive ? '' : undefined}
      className={cx('dz-window-frame', className)}
      style={
        {
          '--dz-window-width': length(width),
          '--dz-window-height': length(height),
          ...style,
        } as CSSProperties
      }
      {...props}
    >
      {platform !== 'omarchy' && (
        <div
          className="dz-window-frame__titlebar"
          data-title-hidden={titleHidden ? '' : undefined}
        >
          <span className="dz-window-frame__title">{title}</span>
          {platform === 'macos' || platform === 'macos15' ? (
            <TrafficLights controls={controls} {...handlers} />
          ) : platform === 'windows' ? (
            <CaptionButtons controls={controls} {...handlers} />
          ) : (
            <HeaderbarButtons
              controls={
                platform === 'gnome' ? { close: controls.close } : controls
              }
              {...handlers}
            />
          )}
        </div>
      )}
      <div className="dz-window-frame__content">{children}</div>
    </div>
  )
}

interface ButtonsProps {
  controls: WindowFrameControls
  onClose?: () => void
  onMinimize?: () => void
  onMaximize?: () => void
}

interface ControlProps {
  kind: 'close' | 'minimize' | 'maximize'
  enabled: boolean | undefined
  onClick?: () => void
  children: ReactNode
}

const LABELS = { close: 'Close', minimize: 'Minimize', maximize: 'Maximize' }

/** One window control. Out of the tab order, as the platforms keep them. */
function Control({ kind, enabled, onClick, children }: ControlProps) {
  return (
    <button
      type="button"
      tabIndex={-1}
      aria-label={LABELS[kind]}
      data-control-kind={kind}
      disabled={!enabled}
      onClick={onClick}
    >
      {children}
    </button>
  )
}

/** The traffic lights. Hovering the group shows each enabled light's glyph, as AppKit does. */
function TrafficLights({
  controls,
  onClose,
  onMinimize,
  onMaximize,
}: ButtonsProps) {
  return (
    <div className="dz-window-frame__traffic">
      <Control kind="close" enabled={controls.close} onClick={onClose}>
        <svg viewBox="0 0 12 12" aria-hidden>
          <path d="M3.5 3.5l5 5M8.5 3.5l-5 5" />
        </svg>
      </Control>
      <Control kind="minimize" enabled={controls.minimize} onClick={onMinimize}>
        <svg viewBox="0 0 12 12" aria-hidden>
          <path d="M2.8 6h6.4" />
        </svg>
      </Control>
      <Control kind="maximize" enabled={controls.maximize} onClick={onMaximize}>
        <svg viewBox="0 0 12 12" aria-hidden>
          <path d="M6 2.8v6.4M2.8 6h6.4" />
        </svg>
      </Control>
    </div>
  )
}

/** Windows caption buttons. A window that can neither minimise nor maximise shows Close alone. */
function CaptionButtons({
  controls,
  onClose,
  onMinimize,
  onMaximize,
}: ButtonsProps) {
  const sizing = controls.minimize || controls.maximize
  return (
    <div className="dz-window-frame__caption">
      {sizing && (
        <>
          <Control
            kind="minimize"
            enabled={controls.minimize}
            onClick={onMinimize}
          >
            <svg viewBox="0 0 10 10" aria-hidden>
              <path d="M0 5.5h10" />
            </svg>
          </Control>
          <Control
            kind="maximize"
            enabled={controls.maximize}
            onClick={onMaximize}
          >
            <svg viewBox="0 0 10 10" aria-hidden>
              <rect x="0.5" y="0.5" width="9" height="9" rx="1" />
            </svg>
          </Control>
        </>
      )}
      <Control kind="close" enabled={controls.close} onClick={onClose}>
        <svg viewBox="0 0 10 10" aria-hidden>
          <path d="M0.5 0.5l9 9M9.5 0.5l-9 9" />
        </svg>
      </Control>
    </div>
  )
}

/**
 * Linux headerbar buttons: libadwaita's round ones on GNOME and Ubuntu,
 * Breeze's flat chevrons on KDE. The glyph set is picked in the stylesheet.
 */
function HeaderbarButtons({
  controls,
  onClose,
  onMinimize,
  onMaximize,
}: ButtonsProps) {
  return (
    <div className="dz-window-frame__headerbar">
      {controls.minimize && (
        <Control kind="minimize" enabled onClick={onMinimize}>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="adwaita">
            <path d="M4.5 11.5h7" />
          </svg>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="breeze">
            <path d="M4 6l4 4 4-4" />
          </svg>
        </Control>
      )}
      {controls.maximize && (
        <Control kind="maximize" enabled onClick={onMaximize}>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="adwaita">
            <rect x="4.5" y="4.5" width="7" height="7" rx="1" />
          </svg>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="breeze">
            <path d="M4 10l4-4 4 4" />
          </svg>
        </Control>
      )}
      <Control kind="close" enabled={controls.close} onClick={onClose}>
        <svg viewBox="0 0 16 16" aria-hidden data-glyph="adwaita">
          <path d="M5 5l6 6M11 5l-6 6" />
        </svg>
        <svg viewBox="0 0 16 16" aria-hidden data-glyph="breeze">
          <path d="M4.5 4.5l7 7M11.5 4.5l-7 7" />
        </svg>
      </Control>
    </div>
  )
}
