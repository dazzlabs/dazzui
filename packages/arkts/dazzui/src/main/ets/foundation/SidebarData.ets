import { DazzTint, DazzVariant } from './TokenTypes'

/** A fresh snapshot is provided after every accepted state change. */
export class DazzSidebarContext {
  open: boolean = true
  inset: boolean = false
  insetCollapsed: boolean = false
  toggle: () => void = () => {}
  column: (id: string, variant: string, collapsible: string) => void = () => {}
  removeColumn: (id: string) => void = () => {}
}

export class DazzSidebarColumnContext {
  icon: boolean = false
  offcanvas: boolean = false
  expandedWidth: number = 0
}

export class DazzSidebarRowContext {
  owned: boolean = false
  active: boolean = false
  hovered: boolean = false
  disabled: boolean = false
  variant: DazzVariant = 'filled'
  tint: DazzTint = 'primary'
  badge: boolean = false
}

export interface DazzSidebarColumnData {
  id: string
  variant: string
  collapsible: string
}

interface SidebarShortcut {
  window: string
  owner: string
  toggle: () => void
}
let shortcuts: SidebarShortcut[] = []
let shortcutObservers: (() => void)[] = []

export function observeSidebarShortcuts(observer: () => void): void {
  if (!shortcutObservers.includes(observer)) shortcutObservers.push(observer)
}
export function unobserveSidebarShortcuts(observer: () => void): void {
  shortcutObservers = shortcutObservers.filter(item => item !== observer)
}
function shortcutsChanged(): void { shortcutObservers.slice().forEach(observer => observer()) }

export function hasWindowSidebars(window: string): boolean {
  return shortcuts.some(item => item.window === window)
}

export function registerSidebarShortcut(
  window: string, owner: string, toggle: () => void
): void {
  unregisterSidebarShortcut(owner)
  shortcuts.push({ window, owner, toggle })
  shortcutsChanged()
}

export function unregisterSidebarShortcut(owner: string): void {
  const had = shortcuts.some(item => item.owner === owner)
  shortcuts = shortcuts.filter(item => item.owner !== owner)
  if (had) shortcutsChanged()
}

/** React installs one window key listener for each mounted provider. */
export function toggleWindowSidebars(window: string): boolean {
  const targets = shortcuts.filter(item => item.window === window)
  targets.forEach(item => item.toggle())
  return targets.length > 0
}

export function isSidebarShortcut(
  key: number, unicode: number, ctrl: boolean, meta: boolean,
  shift: boolean = false, alt: boolean = false, caps: boolean = false
): boolean {
  if (!ctrl && !meta) return false
  // React matches the printed, lower-case key, not an arbitrary modified B.
  return unicode > 0 ? unicode === 98
    : key === 2018 && !shift && !alt && !caps
}

export const DEFAULT_SIDEBAR_MIN_WIDTH: number = 120
export const DEFAULT_SIDEBAR_MAX_WIDTH: number = 480
export const SIDEBAR_KEY_STEP: number = 1
export const SIDEBAR_KEY_BIG_STEP: number = 10

export interface DazzSidebarBounds {
  min: number
  max: number
}

export function clampSidebarWidth(
  width: number, minWidth?: number, maxWidth?: number
): number {
  const safe = Number.isFinite(width) ? width : DEFAULT_SIDEBAR_MIN_WIDTH
  let low = minWidth !== undefined && Number.isFinite(minWidth) ? minWidth : 0
  let high = maxWidth !== undefined && Number.isFinite(maxWidth)
    ? maxWidth : Number.POSITIVE_INFINITY
  if (low > high) {
    const swap = low
    low = high
    high = swap
  }
  return Math.max(low, Math.min(high, safe))
}

export function resolveSidebarBounds(
  fallback: number, defaultWidth?: number, minWidth?: number, maxWidth?: number
): DazzSidebarBounds {
  const anchor = defaultWidth !== undefined && Number.isFinite(defaultWidth)
    ? defaultWidth : fallback
  let min = minWidth !== undefined && Number.isFinite(minWidth)
    ? minWidth : Math.min(DEFAULT_SIDEBAR_MIN_WIDTH, anchor)
  let max = maxWidth !== undefined && Number.isFinite(maxWidth)
    ? maxWidth : Math.max(DEFAULT_SIDEBAR_MAX_WIDTH, anchor)
  if (min > max) {
    const swap = min
    min = max
    max = swap
  }
  return { min, max }
}

export function resolveSidebarWidth(
  fallback: number,
  controlled?: number,
  local?: number,
  defaultWidth?: number,
  minWidth?: number,
  maxWidth?: number
): number {
  const raw = controlled !== undefined && Number.isFinite(controlled)
    ? controlled
    : local !== undefined && Number.isFinite(local)
      ? local
      : defaultWidth !== undefined && Number.isFinite(defaultWidth)
        ? defaultWidth
        : fallback
  return clampSidebarWidth(raw, minWidth, maxWidth)
}

export function dragSidebarWidth(
  startWidth: number, offsetX: number, side: string,
  minWidth?: number, maxWidth?: number
): number {
  const delta = side === 'right' ? -offsetX : offsetX
  return clampSidebarWidth(startWidth + delta, minWidth, maxWidth)
}

export function resetSidebarWidth(
  fallback: number, defaultWidth?: number,
  minWidth?: number, maxWidth?: number
): number {
  const target = defaultWidth !== undefined && Number.isFinite(defaultWidth)
    ? defaultWidth : fallback
  return clampSidebarWidth(target, minWidth, maxWidth)
}

export function stepSidebarWidth(
  currentWidth: number, keyCode: number, side: string,
  shift: boolean = false, minWidth?: number, maxWidth?: number
): number | undefined {
  const step = shift ? SIDEBAR_KEY_BIG_STEP : SIDEBAR_KEY_STEP
  if (keyCode === 2014) {
    const delta = side === 'right' ? step : -step
    return clampSidebarWidth(currentWidth + delta, minWidth, maxWidth)
  }
  if (keyCode === 2015) {
    const delta = side === 'right' ? -step : step
    return clampSidebarWidth(currentWidth + delta, minWidth, maxWidth)
  }
  if (keyCode === 2081 && minWidth !== undefined && Number.isFinite(minWidth)) {
    return clampSidebarWidth(minWidth, minWidth, maxWidth)
  }
  if (keyCode === 2082 && maxWidth !== undefined && Number.isFinite(maxWidth)) {
    return clampSidebarWidth(maxWidth, minWidth, maxWidth)
  }
  return undefined
}
