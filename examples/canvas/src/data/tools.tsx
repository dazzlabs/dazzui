import type { IconGlyph } from '@dazzlabs/dazzui-react'
import {
  Circle20Regular,
  Cursor20Regular,
  Frame20Regular,
  HandRight20Regular,
  Note20Regular,
  Square20Regular,
  TextT20Regular,
} from '@fluentui/react-icons'

import type { ShapeKind } from '../lib/scene'

/**
 * The glyphs come from Fluent rather than being drawn here.
 *
 * `ui_react` depends on the set already — a checkbox's tick and a select's
 * chevron are Fluent's — so a second hand-drawn hand beside them would read as
 * two icon sets on one screen, which is the thing an example is supposed to
 * catch. `Icon` still takes any component that renders an svg; this app simply
 * has no reason to pass a different one.
 */
export type ToolId = 'select' | 'hand' | ShapeKind

export interface Tool {
  id: ToolId
  label: string
  /** The one key that reaches it. Printed in the rail and bound in `app.tsx`. */
  key: string
  icon: IconGlyph
  /** What it draws, or nothing for the two tools that only look. */
  kind: ShapeKind | null
}

export const TOOLS: Tool[] = [
  {
    id: 'select',
    label: 'Select',
    key: 'V',
    icon: Cursor20Regular,
    kind: null,
  },
  { id: 'hand', label: 'Hand', key: 'H', icon: HandRight20Regular, kind: null },
  {
    id: 'frame',
    label: 'Frame',
    key: 'F',
    icon: Frame20Regular,
    kind: 'frame',
  },
  {
    id: 'rect',
    label: 'Rectangle',
    key: 'R',
    icon: Square20Regular,
    kind: 'rect',
  },
  {
    id: 'ellipse',
    label: 'Ellipse',
    key: 'O',
    icon: Circle20Regular,
    kind: 'ellipse',
  },
  { id: 'text', label: 'Text', key: 'T', icon: TextT20Regular, kind: 'text' },
  { id: 'note', label: 'Note', key: 'N', icon: Note20Regular, kind: 'note' },
]

/** The two that only look, and the five that draw. */
export const POINTERS = TOOLS.filter(tool => tool.kind === null)
export const DRAWING = TOOLS.filter(tool => tool.kind !== null)

export function toolOf(id: ToolId): Tool {
  return TOOLS.find(tool => tool.id === id) ?? TOOLS[0]
}
