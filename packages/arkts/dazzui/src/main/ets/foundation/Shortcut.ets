/** Physical HarmonyOS key codes, independent of layout and IME text. */
export interface DazzShortcutEvent {
  keyCode: number
  ctrl: boolean
  alt: boolean
  shift: boolean
  meta: boolean
}

export function shortcutModifiers(event: DazzShortcutEvent): string {
  return (event.ctrl ? '⌃' : '') + (event.alt ? '⌥' : '') +
    (event.shift ? '⇧' : '') + (event.meta ? '⌘' : '')
}

export function isShortcutModifier(code: number): boolean {
  return [2045, 2046, 2047, 2048, 2072, 2073, 2076, 2077].includes(code)
}

/** Keep both physical sides until their release, even on sparse key events. */
export function heldShortcutModifiers(
  event: DazzShortcutEvent,
  pressed: number[],
  releasing: boolean
): string {
  const active = (codes: number[], native: boolean): boolean =>
    codes.some((code: number): boolean => pressed.includes(code)) ||
      (!(releasing && codes.includes(event.keyCode)) && native)
  return shortcutModifiers({
    keyCode: event.keyCode,
    ctrl: active([2072, 2073], event.ctrl),
    alt: active([2045, 2046], event.alt),
    shift: active([2047, 2048], event.shift),
    meta: active([2076, 2077], event.meta),
  })
}

function keyGlyph(code: number): string | null {
  if (code >= 2017 && code <= 2042) return String.fromCharCode(65 + code - 2017)
  if (code >= 2000 && code <= 2009) return String(code - 2000)
  if (code >= 2103 && code <= 2112) return String(code - 2103)
  if (code >= 2090 && code <= 2101) return 'F' + String(code - 2089)
  if (code >= 2816 && code <= 2823) return 'F' + String(code - 2803)
  const glyphs: Record<string, string> = {
    '2050': 'Space', '2054': '↩', '2119': '⌤', '2049': '⇥',
    '2055': '⌫', '2071': '⌦', '2012': '↑', '2013': '↓',
    '2014': '←', '2015': '→', '2081': '↖', '2082': '↘',
    '2068': '⇞', '2069': '⇟', '2057': '-', '2058': '=',
    '2059': '[', '2060': ']', '2061': '\\', '2062': ';',
    '2063': "'", '2043': ',', '2044': '.', '2064': '/', '2056': '`'
  }
  return glyphs[String(code)] ?? null
}

/** React's printed chord: shift alone is typing; F1–F20 may stand alone. */
export function formatShortcut(event: DazzShortcutEvent): string | null {
  const key = keyGlyph(event.keyCode)
  if (!key ||
    (!/^F\d+$/.test(key) && !event.ctrl && !event.alt && !event.meta)) {
    return null
  }
  const modifiers = shortcutModifiers(event)
  return modifiers + (modifiers && key.length > 1 ? ' ' : '') + key
}
