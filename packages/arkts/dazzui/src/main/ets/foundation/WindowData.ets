import { DazzTokens } from './Tokens'

/** Resolved logical typography supplied to a rich title builder. */
export class DazzWindowTitleStyle {
  active: boolean = false
  size: number = 0
  leading: number = 0
  weight: number = 0
  color: string = ''
}

export type DazzWindowPlatform = 'macos' | 'macos15' | 'windows' | 'gnome' | 'ubuntu' | 'kde' | 'omarchy'
export type DazzWindowControlState = 'enabled' | 'disabled' | 'hidden'
export type DazzWindowControl = DazzWindowControlState | boolean
export type DazzWindowControlKind = 'close' | 'minimize' | 'maximize'
export interface DazzWindowControls {
  close?: DazzWindowControl
  minimize?: DazzWindowControl
  maximize?: DazzWindowControl
}
export interface DazzWindowControlItem { kind: DazzWindowControlKind; state: DazzWindowControlState }
export interface DazzWindowChrome {
  radius: number
  height: number
  button: number
  gap: number
  inset: number
  titleInset: number
  trailingInset: number
  typography: string
  titleSize?: number
  titleWeight?: number
}

/** Platform chrome, transcribed from window-frame.css, not a ds recipe.
 * Theme colours, spacing, toolbar height and shadows still come from ds. */
export function windowChrome(platform: DazzWindowPlatform): DazzWindowChrome {
  if (platform === 'macos') return { radius: 16, height: 32, button: 14, gap: 9, inset: 0, titleInset: 80, trailingInset: 0, typography: 'labelLarge' }
  if (platform === 'macos15') return { radius: 10, height: 28, button: 12, gap: 8, inset: 0, titleInset: 80, trailingInset: 0, typography: 'labelLarge' }
  if (platform === 'windows') return { radius: 8, height: 32, button: 46, gap: 0, inset: 0, titleInset: 0, trailingInset: 0, typography: 'captionMedium', titleSize: 12 }
  if (platform === 'gnome' || platform === 'ubuntu') return { radius: 15, height: 47, button: 24, gap: 12, inset: 12, titleInset: 96, trailingInset: 108, typography: 'titleSmall', titleWeight: 700 }
  if (platform === 'kde') return { radius: 5, height: 30, button: 18, gap: 6, inset: 6, titleInset: 96, trailingInset: 72, typography: 'bodyMedium' }
  return { radius: 0, height: 0, button: 0, gap: 0, inset: 0, titleInset: 0, trailingInset: 0, typography: 'bodyMedium' }
}
export function windowMac(platform: DazzWindowPlatform): boolean { return platform === 'macos' || platform === 'macos15' }
export function windowTitleStyle(tokens: DazzTokens, platform: DazzWindowPlatform, inactive: boolean): DazzWindowTitleStyle {
  const chrome = windowChrome(platform)
  const title = tokens.type(chrome.typography)
  const system = tokens.systemStyle
  const result = new DazzWindowTitleStyle()
  result.active = true
  result.size = chrome.titleSize === undefined ? title.size : system ? chrome.titleSize * system.lengthScale : chrome.titleSize
  result.leading = tokens.type('bodyMedium').leading
  result.weight = chrome.titleWeight ?? title.weight
  result.color = tokens.color(inactive ? 'content-subtle' : 'content')
  return result
}
export function windowControlState(value: DazzWindowControl | undefined): DazzWindowControlState {
  return value === true ? 'enabled' : value === false || value === undefined ? 'disabled' : value
}
export function windowControls(platform: DazzWindowPlatform, controls: DazzWindowControls | false | undefined): DazzWindowControls | false {
  if (controls === false) return false
  const resolved: DazzWindowControls = controls ?? { close: true, minimize: true, maximize: true }
  return platform === 'gnome' ? { close: resolved.close, minimize: resolved.minimize ?? 'hidden', maximize: resolved.maximize ?? 'hidden' } : resolved
}
export function windowControlItems(platform: DazzWindowPlatform, controls: DazzWindowControls | false): DazzWindowControlItem[] {
  if (controls === false || platform === 'omarchy') return []
  const order: DazzWindowControlKind[] = windowMac(platform) ? ['close', 'minimize', 'maximize'] : ['minimize', 'maximize', 'close']
  return order.map((kind: DazzWindowControlKind): DazzWindowControlItem => ({ kind, state: windowControlState(kind === 'close' ? controls.close : kind === 'minimize' ? controls.minimize : controls.maximize) }))
    .filter((item: DazzWindowControlItem): boolean => windowMac(platform) ? item.state !== 'hidden' : item.state === 'enabled')
}
export function windowGlyph(platform: DazzWindowPlatform, kind: DazzWindowControlKind): string {
  if (windowMac(platform)) return kind === 'close' ? 'M3.5 3.5l5 5M8.5 3.5l-5 5' : kind === 'minimize' ? 'M2.8 6h6.4' : 'M6 2.8v6.4M2.8 6h6.4'
  if (platform === 'windows') return kind === 'close' ? 'M0.5 0.5l9 9M9.5 0.5l-9 9' : kind === 'minimize' ? 'M0 5.5h10' : 'M1.5 .5h7a1 1 0 0 1 1 1v7a1 1 0 0 1-1 1h-7a1 1 0 0 1-1-1v-7a1 1 0 0 1 1-1Z'
  if (platform === 'kde') return kind === 'close' ? 'M4.5 4.5l7 7M11.5 4.5l-7 7' : kind === 'minimize' ? 'M4 6l4 4 4-4' : 'M4 10l4-4 4 4'
  return kind === 'close' ? 'M5 5l6 6M11 5l-6 6' : kind === 'minimize' ? 'M4.5 11.5h7' : 'M5.5 4.5h5a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1h-5a1 1 0 0 1-1-1v-5a1 1 0 0 1 1-1Z'
}

export class DazzWindowContext {
  owned: boolean = false
  platform: DazzWindowPlatform = 'macos'
  controls: DazzWindowControls | false = false
  rootBand: boolean = false
  inactive: boolean = false
  fullSizeContent: boolean = false
  onClose: () => void = () => {}
  onMinimize: () => void = () => {}
  onMaximize: () => void = () => {}
  pane: (mounted: boolean) => void = () => {}
}

export function windowHandoff(context: DazzWindowContext, controls: DazzWindowControls | false | undefined): boolean {
  return controls === undefined && context.owned && !context.rootBand && !windowMac(context.platform) && context.controls !== false
}
export function windowTitlebarControls(context: DazzWindowContext, controls: DazzWindowControls | false | undefined): DazzWindowControls | false {
  if (controls !== undefined) return windowControls(context.platform, controls)
  if (windowHandoff(context, controls)) return context.controls
  return context.owned && (context.controls !== false || context.platform === 'omarchy') ? false : windowControls(context.platform, undefined)
}
