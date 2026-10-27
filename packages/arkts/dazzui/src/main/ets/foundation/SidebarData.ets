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
