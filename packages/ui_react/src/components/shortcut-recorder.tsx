import { DismissCircle12Filled } from '@fluentui/react-icons'
import {
  type KeyboardEvent as ReactKeyboardEvent,
  useEffect,
  useRef,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

/*
 * A shortcut is kept as the glyph string a settings page prints — `⌥⇧2`,
 * `⌥ Space` — rather than as a key-code tuple, because that is what every
 * other place showing a shortcut already carries. A host that stores
 * `Option+Shift+2` is one mapping away.
 */

/** Modifiers in the order macOS prints them: ⌃ ⌥ ⇧ ⌘. */
const MODIFIER_KEYS = new Set(['Control', 'Alt', 'Shift', 'Meta'])

/**
 * Keys named by their physical position (`event.code`), not by the character
 * they produce: with ⌥ held, `event.key` for the T key is `†` on a US layout,
 * and the recorded shortcut would print as gibberish.
 */
const KEY_GLYPHS: Record<string, string> = {
  Space: 'Space',
  Enter: '↩',
  NumpadEnter: '⌤',
  Tab: '⇥',
  Backspace: '⌫',
  Delete: '⌦',
  ArrowUp: '↑',
  ArrowDown: '↓',
  ArrowLeft: '←',
  ArrowRight: '→',
  Home: '↖',
  End: '↘',
  PageUp: '⇞',
  PageDown: '⇟',
  Minus: '-',
  Equal: '=',
  BracketLeft: '[',
  BracketRight: ']',
  Backslash: '\\',
  Semicolon: ';',
  Quote: "'",
  Comma: ',',
  Period: '.',
  Slash: '/',
  Backquote: '`',
}

type ShortcutEvent = {
  code: string
  key: string
  ctrlKey: boolean
  altKey: boolean
  shiftKey: boolean
  metaKey: boolean
}

function keyGlyph(code: string, key: string): string | null {
  if (KEY_GLYPHS[code]) return KEY_GLYPHS[code]
  if (/^Key[A-Z]$/.test(code)) return code.slice(3)
  if (/^(Digit|Numpad)\d$/.test(code)) return code.slice(-1)
  if (/^F([1-9]|1\d|20)$/.test(code)) return code

  // No position — a synthetic event, an on-screen keyboard — so fall back to
  // the character, which is only trustworthy while it is plain ASCII.
  if (code === '') {
    if (key === ' ') return 'Space'
    if (KEY_GLYPHS[key]) return KEY_GLYPHS[key]
    if (/^F([1-9]|1\d|20)$/.test(key)) return key
    if (/^[\x21-\x7e]$/.test(key)) return key.toUpperCase()
  }
  return null
}

function modifierGlyphs(event: Omit<ShortcutEvent, 'code' | 'key'>): string {
  return [
    event.ctrlKey && '⌃',
    event.altKey && '⌥',
    event.shiftKey && '⇧',
    event.metaKey && '⌘',
  ]
    .filter(Boolean)
    .join('')
}

/**
 * The glyph string for a key event, or `null` when the event is not something
 * a shortcut can be made of.
 *
 * A key on its own is typing, not a shortcut, so it has to carry ⌃, ⌥ or ⌘ —
 * ⇧ alone is still typing — unless it is a function key, which is a shortcut
 * by itself. Word keys (`Space`, `F5`) get a space after the modifiers so
 * `⌥ Space` reads as two things and `⌥⇧2` as one.
 */
export function formatShortcut(event: ShortcutEvent): string | null {
  const key = keyGlyph(event.code, event.key)
  if (!key) return null

  const isFunctionKey = /^F\d+$/.test(key)
  if (!isFunctionKey && !event.ctrlKey && !event.altKey && !event.metaKey) {
    return null
  }

  const mods = modifierGlyphs(event)
  return key.length > 1 ? `${mods}${mods ? ' ' : ''}${key}` : `${mods}${key}`
}

export interface ShortcutRecorderProps {
  /** The bound keys as printed — `⌥⇧2`, `⌥ Space`. Empty means unbound. */
  value: string
  onValueChange: (value: string) => void
  /** Called on clear or ⌫ while recording; defaults to `onValueChange('')`. */
  onClear?: () => void
  /** Shown while idle and unbound. */
  placeholder?: string
  /** Shown while the control waits for a key. */
  recordingLabel?: string
  /** Accessible name of the clear button. */
  clearLabel?: string
  /** `error` for a binding the page cannot accept — one already taken. */
  state?: 'default' | 'error'
  disabled?: boolean
  tint?: Tint
  'aria-label'?: string
  className?: string
}

/**
 * The control a settings row uses to rebind a shortcut. Click it and it
 * listens; press a combination and it keeps it. Escape backs out, ⌫ clears,
 * and clicking away cancels.
 *
 * Held modifiers show live while it listens, so pressing ⌥⇧ and then wondering
 * which key to add is not done blind.
 */
export function ShortcutRecorder({
  value,
  onValueChange,
  onClear,
  placeholder = 'Record shortcut',
  recordingLabel = 'Press keys…',
  clearLabel = 'Clear shortcut',
  state = 'default',
  disabled = false,
  tint = 'primary',
  'aria-label': ariaLabel,
  className,
}: ShortcutRecorderProps) {
  const button = useRef<HTMLButtonElement>(null)
  const [recording, setRecording] = useState(false)
  /** Modifiers held right now, printed while recording. */
  const [held, setHeld] = useState('')
  /** Set for a beat after a press the recorder could not take. */
  const [rejected, setRejected] = useState(false)

  useEffect(() => {
    if (!rejected) return
    const timer = window.setTimeout(() => setRejected(false), 360)
    return () => window.clearTimeout(timer)
  }, [rejected])

  const stop = () => {
    setRecording(false)
    setHeld('')
  }

  const clear = () => {
    stop()
    if (onClear) onClear()
    else onValueChange('')
  }

  const onKeyDown = (event: ReactKeyboardEvent<HTMLButtonElement>) => {
    if (!recording) return

    // Nothing typed here may reach the page: no dialog closes on this Escape,
    // no ⌘F opens a search, no Space clicks the button under the pointer.
    event.preventDefault()
    event.stopPropagation()

    if (event.key === 'Escape') {
      stop()
      return
    }
    if (MODIFIER_KEYS.has(event.key)) {
      setHeld(modifierGlyphs(event))
      return
    }

    const bare =
      !event.ctrlKey && !event.altKey && !event.metaKey && !event.shiftKey
    if (bare && (event.key === 'Backspace' || event.key === 'Delete')) {
      clear()
      return
    }

    const next = formatShortcut(event)
    if (!next) {
      setRejected(true)
      return
    }
    stop()
    onValueChange(next)
  }

  const onKeyUp = (event: ReactKeyboardEvent<HTMLButtonElement>) => {
    if (recording && MODIFIER_KEYS.has(event.key)) {
      setHeld(modifierGlyphs(event))
    }
  }

  const bound = value !== ''
  const label = recording ? held || recordingLabel : bound ? value : placeholder
  const showsGlyphs = recording ? held !== '' : bound
  const clearable = bound && !recording && !disabled

  return (
    <span data-tint={tint} className={cx('dz-shortcut-recorder', className)}>
      <button
        ref={button}
        type="button"
        disabled={disabled}
        aria-label={ariaLabel}
        aria-pressed={recording}
        data-clearable={clearable}
        data-glyphs={showsGlyphs}
        data-recording={recording}
        data-rejected={rejected}
        data-state={state}
        className="dz-shortcut-recorder__button"
        onBlur={stop}
        onClick={() => {
          if (recording) return
          setRecording(true)
          // Safari leaves a clicked button unfocused, and the keys would go to
          // whatever had focus before.
          button.current?.focus()
        }}
        onKeyDown={onKeyDown}
        onKeyUp={onKeyUp}
      >
        {label}
      </button>
      {clearable ? (
        <button
          type="button"
          aria-label={clearLabel}
          className="dz-shortcut-recorder__clear"
          onClick={clear}
        >
          <DismissCircle12Filled />
        </button>
      ) : null}
    </span>
  )
}
