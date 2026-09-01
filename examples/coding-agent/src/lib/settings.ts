import { useCallback, useState } from 'react'

import type { Project } from './project'
import type { Theme } from './theme'

/** What a message is sent under: reading and explaining, or writing. */
export type Mode = 'ask' | 'build'

export const MODELS = ['forge-1-pro', 'forge-1', 'forge-1-mini']

/**
 * The bindings the settings page rebinds.
 *
 * A shortcut is kept as the glyph string it prints, which is what the design
 * system's recorder hands back — a host that stores `Command+K` is one mapping
 * away, and this prototype has nothing to map for.
 */
export const SHORTCUTS = [
  { id: 'send', label: 'Send the message', where: 'In the composer' },
  { id: 'stop', label: 'Stop the run' },
  { id: 'session', label: 'New session' },
  { id: 'sidebar', label: 'Show or hide projects' },
  { id: 'changes', label: 'Go to changes' },
  { id: 'palette', label: 'Command palette' },
] as const

export type ShortcutId = (typeof SHORTCUTS)[number]['id']

export interface Settings {
  theme: Theme
  model: string
  mode: Mode
  /** Ask before the first write of a session. Off, and nothing asks at all. */
  askBeforeWrite: boolean
  runCommands: boolean
  fetchWeb: boolean
  /** How many tool calls the agent may have in flight. */
  parallel: number
  /** Keep going when a command comes back non-zero, or stop the run there. */
  continueOnFailure: boolean
  /** Commands allowed to run without asking, one per line. */
  allowlist: string
  shortcuts: Record<ShortcutId, string>
  /**
   * Which projects are bound to a provider, overriding the fixture. Absent
   * means the project's own `provider` decides.
   */
  connected: Record<string, boolean>
}

export const DEFAULTS: Settings = {
  theme: 'studio-light',
  model: MODELS[0],
  mode: 'build',
  askBeforeWrite: true,
  runCommands: true,
  fetchWeb: false,
  parallel: 4,
  continueOnFailure: false,
  allowlist: 'pnpm typecheck\npnpm lint\ngit status',
  shortcuts: {
    send: '↩',
    stop: '⌘.',
    session: '⌘N',
    sidebar: '⌃⌘S',
    changes: '⇧⌘C',
    palette: '⌘K',
  },
  connected: {},
}

/** Whether a project has a provider to list issues and pull requests from. */
export function isConnected(settings: Settings, project: Project): boolean {
  return settings.connected[project.id] ?? project.provider !== null
}

/** Two rows bound to the same keys; the recorder shows both as erroring. */
export function clashes(shortcuts: Settings['shortcuts']): Set<string> {
  const seen = new Map<string, number>()
  for (const value of Object.values(shortcuts)) {
    if (!value) continue
    seen.set(value, (seen.get(value) ?? 0) + 1)
  }
  return new Set([...seen].filter(([, count]) => count > 1).map(([key]) => key))
}

export interface SettingsStore {
  settings: Settings
  update: (patch: Partial<Settings>) => void
  reset: () => void
}

/**
 * The window's preferences, in one place.
 *
 * They live here rather than beside the controls that read them because the
 * settings page and the toolbar edit the same values — a theme menu and an
 * appearance row that disagreed would be two settings wearing one name.
 */
export function useSettings(): SettingsStore {
  const [settings, setSettings] = useState<Settings>(DEFAULTS)

  const update = useCallback(
    (patch: Partial<Settings>) =>
      setSettings(current => ({ ...current, ...patch })),
    []
  )

  const reset = useCallback(() => setSettings(DEFAULTS), [])

  return { settings, update, reset }
}
