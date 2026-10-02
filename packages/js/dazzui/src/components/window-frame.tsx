import {
  createContext,
  useContext,
  type ComponentPropsWithoutRef,
  type CSSProperties,
  type ReactNode,
} from 'react'

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

/**
 * A window control's state. `disabled` is "the window cannot do this", which
 * each platform draws its own way — AppKit greys the light, WinUI and
 * libadwaita leave the button out. `hidden` leaves it out everywhere, macOS
 * included, which is what a window with no such control at all wants.
 */
export type WindowControlState = 'enabled' | 'disabled' | 'hidden'

/**
 * A control's state, or `true` for `enabled` and `false` for `disabled`.
 * A control left out of the set is one the window does not have, which is
 * `disabled` too — so `controls={{ close: true }}` is the About panel that
 * can only be closed.
 */
export type WindowControl = WindowControlState | boolean

export interface WindowFrameControls {
  close?: WindowControl
  minimize?: WindowControl
  maximize?: WindowControl
}

/**
 * The titlebar's slots, named after where each one sits. `toolbar` is the
 * band's own content; a window that carries one gets the design system's
 * 52px titlebar rather than the platform's shorter chrome, so the band lines
 * up with the `SidebarHeader` beside it.
 */
export interface WindowTitlebarSlots {
  /** Past the window controls, before the title: where AppKit puts the sidebar toggle. */
  leading?: ReactNode
  /** The band's toolbar content. */
  toolbar?: ReactNode
  /** At the trailing edge, inside the window controls. */
  trailing?: ReactNode
  /** The band's own centre, where a macOS titlebar accessory view sits. */
  accessory?: ReactNode
}

export interface WindowFrameProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  /** The window's title. Drawn in the titlebar, and its accessible name when it is a string. */
  title?: ReactNode
  /** De-emphasised context after the title — a section or a file name. */
  subtitle?: ReactNode
  platform?: WindowFramePlatform
  /** A transparent titlebar: the title is kept as the accessible name only. */
  titleHidden?: boolean
  /**
   * The titlebar's slots. There is no titlebar without a window: `false`
   * leaves it out, so the content runs from the window's top edge and the
   * controls, if any, float over it.
   */
  titlebar?: WindowTitlebarSlots | false
  /**
   * The content fills the window and the chrome floats over it, the way
   * macOS's `fullSizeContentView` does: a sidebar reaches the window's top
   * edge, and the band is the design system's 52px — the height a
   * `SidebarHeader` is — rather than the platform's own chrome.
   */
  fullSizeContent?: boolean
  /** Which window controls the window carries. `false` carries none. */
  controls?: WindowFrameControls | false
  /** Not the key window: the controls and the title grey out. */
  inactive?: boolean
  /** The window's width; left out, `--frame-window-width`. */
  width?: number | string
  /** The window's height, chrome included; left out, it sizes to its content. */
  height?: number | string
  /** The narrowest the window may be drawn, whatever `width` says. */
  minWidth?: number | string
  /** The widest it may be drawn; a window is never wider than what holds it. */
  maxWidth?: number | string
  minHeight?: number | string
  maxHeight?: number | string
  onClose?: () => void
  onMinimize?: () => void
  onMaximize?: () => void
  children?: ReactNode
}

const ALL: WindowFrameControls = { close: true, minimize: true, maximize: true }

const length = (value: number | string | undefined) =>
  typeof value === 'number' ? `${value}px` : value

/** A control's state: only `true` is live; a control not named is not had. */
function stateOf(control: WindowControl | undefined): WindowControlState {
  if (control === true) return 'enabled'
  if (control === false || control === undefined) return 'disabled'
  return control
}

/** Each control's state, with the platform's own default layout. */
function controlsOf(
  platform: WindowFramePlatform,
  controls: WindowFrameControls
): WindowFrameControls {
  // libadwaita's default layout is Close alone; the other two are drawn when
  // the window carries them, which is the whole difference from Ubuntu.
  if (platform !== 'gnome') return controls
  return {
    ...controls,
    minimize: controls.minimize ?? 'hidden',
    maximize: controls.maximize ?? 'hidden',
  }
}

interface FrameContextValue {
  platform: WindowFramePlatform
  /** Whether the frame draws the window's controls itself. */
  controls: boolean
}

const FrameContext = createContext<FrameContextValue | null>(null)

/**
 * A top-level window: the titlebar with the platform's own controls, and the
 * content below it. The colours are the theme's; the chrome — where the
 * controls sit, their glyphs and their sizes — is the platform's, transcribed
 * rather than decided here, so a window reads as the desktop it names under
 * any theme.
 *
 * An application window is this frame holding a `WindowBody`: the sidebar
 * runs from the window's top edge to its foot, the band belongs to the pane
 * beside it (`WindowMain` holds a `WindowTitlebar`, a `WindowContent` and a
 * `WindowFooter`), and `fullSizeContent` puts the controls over the sidebar's
 * head the way Finder and System Settings draw them.
 */
export function WindowFrame({
  title,
  subtitle,
  platform = 'macos',
  titleHidden = false,
  titlebar = {},
  fullSizeContent = false,
  controls,
  inactive = false,
  width,
  height,
  minWidth,
  maxWidth,
  minHeight,
  maxHeight,
  onClose,
  onMinimize,
  onMaximize,
  className,
  style,
  children,
  ...props
}: WindowFrameProps) {
  // Hyprland draws no chrome at all, so Omarchy is a window without a band
  // whatever the slots say.
  const band = platform === 'omarchy' || titlebar === false ? null : titlebar
  const resolved =
    controls === false ? false : controlsOf(platform, controls ?? ALL)
  const handlers = { onClose, onMinimize, onMaximize }
  // A band inside a pane is the pane's toolbar: the frame has the window's
  // controls already — in its own band, or floating over the content when it
  // draws no band — and a window with neither has none.
  const draws = resolved !== false && (band !== null || platform !== 'omarchy')
  return (
    <FrameContext.Provider value={{ platform, controls: draws }}>
      <div
        role="group"
        aria-label={typeof title === 'string' ? title : undefined}
        data-platform={platform}
        data-inactive={inactive ? '' : undefined}
        data-full-size-content={fullSizeContent ? '' : undefined}
        className={cx('dz-window-frame', className)}
        style={
          {
            '--dz-window-width': length(width),
            '--dz-window-height': length(height),
            '--dz-window-min-width': length(minWidth),
            '--dz-window-max-width': length(maxWidth),
            '--dz-window-min-height': length(minHeight),
            '--dz-window-max-height': length(maxHeight),
            ...style,
          } as CSSProperties
        }
        {...props}
      >
        {band ? (
          <WindowTitlebar
            title={title}
            subtitle={subtitle}
            titleHidden={titleHidden}
            controls={resolved}
            {...band}
            {...handlers}
          />
        ) : (
          platform !== 'omarchy' &&
          resolved !== false && (
            <div className="dz-window-frame__chrome">
              <Controls platform={platform} controls={resolved} {...handlers} />
            </div>
          )
        )}
        <div className="dz-window-frame__content">{children}</div>
      </div>
    </FrameContext.Provider>
  )
}

export interface WindowTitlebarProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title' | 'children'
> {
  title?: ReactNode
  /** De-emphasised context after the title — a section or a file name. */
  subtitle?: ReactNode
  /** Past the window controls, before the title: where AppKit puts the sidebar toggle. */
  leading?: ReactNode
  /** The band's toolbar content; carrying one makes the band 52px. */
  toolbar?: ReactNode
  /** At the trailing edge, inside the window controls. */
  trailing?: ReactNode
  /** The band's own centre, where a macOS titlebar accessory view sits. */
  accessory?: ReactNode
  /** A transparent band: the title is kept as the accessible name only. */
  titleHidden?: boolean
  /**
   * Which window controls the band carries. A band inside a `WindowMain` is
   * the pane's toolbar, and the frame has the window's controls already, so
   * it carries none unless the frame was told to draw none itself.
   */
  controls?: WindowFrameControls | false
  onClose?: () => void
  onMinimize?: () => void
  onMaximize?: () => void
}

/**
 * The window's band: the title, the toolbar content and the platform's own
 * controls, in one row. Its height is fixed rather than derived from its
 * contents, so a view that parks a panel here is no taller than one showing
 * only a title, and it lines up with the sidebar's header strip.
 */
export function WindowTitlebar({
  title,
  subtitle,
  leading,
  toolbar,
  trailing,
  accessory,
  titleHidden = false,
  controls,
  onClose,
  onMinimize,
  onMaximize,
  className,
  ...props
}: WindowTitlebarProps) {
  const frame = useContext(FrameContext)
  const platform = frame?.platform ?? 'macos'
  const carrying = controls ?? (frame?.controls ? false : ALL)
  const resolved = carrying === false ? false : controlsOf(platform, carrying)
  return (
    <div
      className={cx('dz-window-frame__titlebar', className)}
      data-title-hidden={titleHidden ? '' : undefined}
      data-toolbar={toolbar != null || accessory != null ? '' : undefined}
      {...props}
    >
      {leading != null && (
        <div className="dz-window-frame__leading">{leading}</div>
      )}
      {title != null && (
        <span className="dz-window-frame__title">
          <span className="dz-window-frame__title-text">{title}</span>
          {subtitle != null && (
            <span className="dz-window-frame__subtitle">{subtitle}</span>
          )}
        </span>
      )}
      {toolbar != null && (
        <div className="dz-window-frame__toolbar">{toolbar}</div>
      )}
      {trailing != null && (
        <div className="dz-window-frame__trailing">{trailing}</div>
      )}
      {accessory != null && (
        <div className="dz-window-frame__accessory">{accessory}</div>
      )}
      {resolved !== false && (
        <Controls
          platform={platform}
          controls={resolved}
          onClose={onClose}
          onMinimize={onMinimize}
          onMaximize={onMaximize}
        />
      )}
    </div>
  )
}

/**
 * The window's body: the full-height column beside the pane, and whatever
 * they sit in. It clips rather than grows, so with a fixed-height window the
 * panes scroll inside it.
 */
export function WindowBody({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-window-frame__body', className)} {...props} />
}

/**
 * The pane beside a full-height sidebar. It owns its own band, so the
 * sidebar can run from the window's top edge to its foot the way Finder,
 * Mail and System Settings draw it.
 */
export function WindowMain({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-window-frame__main', className)} {...props} />
}

/**
 * The row the views sit in, under a `WindowMain`'s band: a rail, a main
 * column and an aside as siblings.
 */
export function WindowContent({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-window-frame__pane', className)} {...props} />
}

/** The strip at the window's or a dialog's foot. */
export function WindowFooter({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-window-frame__footer', className)} {...props} />
}

interface ControlsProps {
  platform: WindowFramePlatform
  controls: WindowFrameControls
  onClose?: () => void
  onMinimize?: () => void
  onMaximize?: () => void
}

/** The platform's own controls, in the platform's own corner. */
function Controls({
  platform,
  controls,
  onClose,
  onMinimize,
  onMaximize,
}: ControlsProps) {
  const handlers = { onClose, onMinimize, onMaximize }
  if (platform === 'macos' || platform === 'macos15')
    return <TrafficLights controls={controls} {...handlers} />
  if (platform === 'windows')
    return <CaptionButtons controls={controls} {...handlers} />
  return <HeaderbarButtons controls={controls} {...handlers} />
}

interface ButtonsProps {
  controls: WindowFrameControls
  onClose?: () => void
  onMinimize?: () => void
  onMaximize?: () => void
}

interface ControlProps {
  kind: 'close' | 'minimize' | 'maximize'
  state: WindowControlState
  onClick?: () => void
  children: ReactNode
}

const LABELS = { close: 'Close', minimize: 'Minimize', maximize: 'Maximize' }

/** One window control. Out of the tab order, as the platforms keep them. */
function Control({ kind, state, onClick, children }: ControlProps) {
  return (
    <button
      type="button"
      tabIndex={-1}
      aria-label={LABELS[kind]}
      data-control-kind={kind}
      data-state={state}
      disabled={state === 'disabled'}
      onClick={onClick}
    >
      {children}
    </button>
  )
}

/**
 * The traffic lights. AppKit always draws three, so one the window cannot do
 * is drawn greyed rather than left out; hovering the group shows each enabled
 * light's glyph.
 */
function TrafficLights({
  controls,
  onClose,
  onMinimize,
  onMaximize,
}: ButtonsProps) {
  const close = stateOf(controls.close)
  const minimize = stateOf(controls.minimize)
  const maximize = stateOf(controls.maximize)
  if (close === 'hidden' && minimize === 'hidden' && maximize === 'hidden')
    return null
  return (
    <div className="dz-window-frame__traffic">
      {close !== 'hidden' && (
        <Control kind="close" state={close} onClick={onClose}>
          <svg viewBox="0 0 12 12" aria-hidden>
            <path d="M3.5 3.5l5 5M8.5 3.5l-5 5" />
          </svg>
        </Control>
      )}
      {minimize !== 'hidden' && (
        <Control kind="minimize" state={minimize} onClick={onMinimize}>
          <svg viewBox="0 0 12 12" aria-hidden>
            <path d="M2.8 6h6.4" />
          </svg>
        </Control>
      )}
      {maximize !== 'hidden' && (
        <Control kind="maximize" state={maximize} onClick={onMaximize}>
          <svg viewBox="0 0 12 12" aria-hidden>
            <path d="M6 2.8v6.4M2.8 6h6.4" />
          </svg>
        </Control>
      )}
    </div>
  )
}

/**
 * Windows caption buttons. WinUI leaves a button the window cannot use out
 * rather than drawing it inert, so a window that can neither minimise nor
 * maximise shows Close alone — and one that cannot close shows nothing.
 */
function CaptionButtons({
  controls,
  onClose,
  onMinimize,
  onMaximize,
}: ButtonsProps) {
  const close = stateOf(controls.close) === 'enabled'
  const minimize = stateOf(controls.minimize) === 'enabled'
  const maximize = stateOf(controls.maximize) === 'enabled'
  if (!close && !minimize && !maximize) return null
  return (
    <div className="dz-window-frame__caption">
      {minimize && (
        <Control kind="minimize" state="enabled" onClick={onMinimize}>
          <svg viewBox="0 0 10 10" aria-hidden>
            <path d="M0 5.5h10" />
          </svg>
        </Control>
      )}
      {maximize && (
        <Control kind="maximize" state="enabled" onClick={onMaximize}>
          <svg viewBox="0 0 10 10" aria-hidden>
            <rect x="0.5" y="0.5" width="9" height="9" rx="1" />
          </svg>
        </Control>
      )}
      {close && (
        <Control kind="close" state="enabled" onClick={onClose}>
          <svg viewBox="0 0 10 10" aria-hidden>
            <path d="M0.5 0.5l9 9M9.5 0.5l-9 9" />
          </svg>
        </Control>
      )}
    </div>
  )
}

/**
 * Linux headerbar buttons: libadwaita's round ones on GNOME and Ubuntu,
 * Breeze's flat chevrons on KDE. The glyph set is picked in the stylesheet.
 * An action the window cannot take is left out of the headerbar, as CSD does.
 */
function HeaderbarButtons({
  controls,
  onClose,
  onMinimize,
  onMaximize,
}: ButtonsProps) {
  const close = stateOf(controls.close) === 'enabled'
  const minimize = stateOf(controls.minimize) === 'enabled'
  const maximize = stateOf(controls.maximize) === 'enabled'
  if (!close && !minimize && !maximize) return null
  return (
    <div className="dz-window-frame__headerbar">
      {minimize && (
        <Control kind="minimize" state="enabled" onClick={onMinimize}>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="adwaita">
            <path d="M4.5 11.5h7" />
          </svg>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="breeze">
            <path d="M4 6l4 4 4-4" />
          </svg>
        </Control>
      )}
      {maximize && (
        <Control kind="maximize" state="enabled" onClick={onMaximize}>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="adwaita">
            <rect x="4.5" y="4.5" width="7" height="7" rx="1" />
          </svg>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="breeze">
            <path d="M4 10l4-4 4 4" />
          </svg>
        </Control>
      )}
      {close && (
        <Control kind="close" state="enabled" onClick={onClose}>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="adwaita">
            <path d="M5 5l6 6M11 5l-6 6" />
          </svg>
          <svg viewBox="0 0 16 16" aria-hidden data-glyph="breeze">
            <path d="M4.5 4.5l7 7M11.5 4.5l-7 7" />
          </svg>
        </Control>
      )}
    </div>
  )
}
