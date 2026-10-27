import { isSidebarShortcut, toggleWindowSidebars } from './SidebarData'

export function dispatchSidebarKey(windowName: string, event: KeyEvent): boolean {
  if (event.type !== KeyType.Down || !isSidebarShortcut(
    event.keyCode, event.unicode ?? 0,
    event.getModifierKeyState?.(['Ctrl']) ?? false, event.metaKey !== 0,
    event.getModifierKeyState?.(['Shift']) ?? false,
    event.getModifierKeyState?.(['Alt']) ?? false, event.isCapsLockOn ?? false
  )) return false
  if (!toggleWindowSidebars(windowName)) return false
  event.stopPropagation()
  return true
}
