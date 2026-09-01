/**
 * The keys that are not a tool.
 *
 * One list, read by the inspector when nothing is selected — a canvas is a
 * keyboard instrument, and a prototype that hides its keys is only half of
 * one. The tools' own letters live beside them in `tools.tsx`, printed on the
 * rows they belong to.
 *
 * They are the glyphs a Mac prints rather than key codes: this is a panel, not
 * a binding table, and `app.tsx` is where the events are actually read.
 */
export interface Shortcut {
  keys: string
  label: string
}

export const SHORTCUTS: Shortcut[] = [
  { keys: '⌘Z', label: 'Undo' },
  { keys: '⇧⌘Z', label: 'Redo' },
  { keys: '⌘D', label: 'Duplicate' },
  { keys: '⌫', label: 'Delete' },
  { keys: '⌘A', label: 'Select everything' },
  { keys: '⎋', label: 'Deselect' },
  { keys: '⌘0', label: 'Zoom to 100%' },
  { keys: '⇧1', label: 'Zoom to fit' },
  { keys: '↑↓←→', label: 'Nudge by a unit' },
  { keys: '⇧↑', label: 'Nudge by the grid' },
  { keys: 'Space', label: 'Hold to pan' },
  { keys: '⌘', label: 'Hold to zoom on scroll' },
]
