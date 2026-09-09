import { useCallback, useState } from 'react'

import type { Project } from './project'
import type { Appearance, Family } from './theme'

export const MODELS = [
  { value: 'claude-sonnet', label: 'Claude Sonnet' },
  { value: 'claude-opus', label: 'Claude Opus' },
  { value: 'custom', label: 'Custom…' },
]

/**
 * When the agent stops for a person. `always` confirms every tool call;
 * `reads` lets reads and searches through and gates everything that changes
 * something; `auto` gates nothing.
 */
export type Approval = 'always' | 'reads' | 'auto'

export const APPROVALS: { value: Approval; label: string; hint: string }[] = [
  {
    value: 'always',
    label: 'Ask every time',
    hint: 'Every tool call is confirmed before it runs.',
  },
  {
    value: 'reads',
    label: 'Allow reads',
    hint: 'Reading and searching run freely; anything that changes something asks.',
  },
  {
    value: 'auto',
    label: 'Fully automatic',
    hint: 'Nothing asks. The agent runs to the end of its plan.',
  },
]

/** How many sessions may run at once — one worktree each. */
export const PARALLEL = [1, 2, 4]

/**
 * How far a message lets the agent go. `ask` answers and writes nothing;
 * `plan` drafts the steps and waits for a go; `agent` carries the plan out
 * and stops for the permission card; `auto` runs to a ready pull request
 * before coming back. It is the composer's own switch, because it is decided
 * per message — the approval policy above is decided per window.
 */
export type Mode = 'ask' | 'plan' | 'agent' | 'auto'

export const MODES: { value: Mode; label: string; hint: string }[] = [
  {
    value: 'ask',
    label: 'Ask',
    hint: 'Reads and answers. Nothing is written.',
  },
  {
    value: 'plan',
    label: 'Plan',
    hint: 'Drafts the plan, then waits to be told to go.',
  },
  {
    value: 'agent',
    label: 'Agent',
    hint: 'Edits and runs commands; anything risky stops for the card.',
  },
  {
    value: 'auto',
    label: 'Auto',
    hint: 'Runs to a ready pull request before coming back to you.',
  },
]

export const MERGE_METHODS = [
  { value: 'squash', label: 'Squash' },
  { value: 'merge', label: 'Merge' },
  { value: 'rebase', label: 'Rebase' },
]

/**
 * The bindings the settings sheet lists. They are not rebindable here — it
 * is a table of what the window already answers to, and a recorder would
 * promise a persistence this prototype has not got.
 */
export const SHORTCUTS = [
  { keys: '⌘K', label: 'Command palette' },
  { keys: '⌘P', label: 'Switch project' },
  { keys: '⌘N', label: 'New session' },
  {
    keys: '⌘1–7',
    label: 'Switch page (chat, inbox, review, Git, tests, GitHub, release)',
  },
  { keys: '⌘, / ⌘9', label: 'Settings' },
  { keys: '⌘B', label: 'Hide or show the sidebar' },
  { keys: '⏎', label: 'Send the message' },
  { keys: 'J / K', label: 'Review: next and previous file' },
  { keys: 'A / X / C', label: 'Review: accept, revert, comment' },
] as const

export interface Settings {
  /** The look: which of the six families the window is drawn in. */
  family: Family
  /** And whether on paper, on a dark ground, or whichever the system says. */
  appearance: Appearance
  model: string
  /** What a new message starts in. The composer can change it per message. */
  mode: Mode
  approval: Approval
  /** How many sessions may run at once. */
  parallel: number
  mergeMethod: string
  /** Whether the dock — changes, terminal, pull request — is drawn beside the thread. */
  dock: boolean
  /**
   * Which projects are bound to a provider, overriding the fixture. Absent
   * means the project's own `provider` decides.
   */
  connected: Record<string, boolean>
}

export const DEFAULTS: Settings = {
  family: 'nocturne',
  appearance: 'light',
  model: MODELS[0].value,
  mode: 'agent',
  approval: 'always',
  parallel: 2,
  mergeMethod: 'squash',
  dock: true,
  connected: {},
}

/** Whether a project has a provider to list issues and pull requests from. */
export function isConnected(settings: Settings, project: Project): boolean {
  return settings.connected[project.id] ?? project.provider !== null
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
 * settings page, the composer and the titlebar edit the same values — a model
 * picker in two places that disagreed would be two settings wearing one name.
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
