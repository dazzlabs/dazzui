import { Icon, NavItem, Rail, RailGroup } from '@dazzlabs/dazzui-react'

import { DRAWING, POINTERS, type ToolId } from '../data/tools'

export interface ToolRailProps {
  tool: ToolId
  onPick: (tool: ToolId) => void
}

/**
 * The second column: what the pointer does next.
 *
 * The tools are rows with names rather than a strip of unlabelled glyphs. A
 * gallery can afford the glyph on its own because the tooltip is one hover
 * away; a column that already exists at a token width can afford the word, and
 * the word is what makes the key beside it worth printing.
 *
 * The first group goes unlabelled — the two tools that only look are what the
 * column is already about — and the five that draw get the heading.
 */
export function ToolRail({ tool, onPick }: ToolRailProps) {
  return (
    <Rail className="cv-rail">
      <RailGroup>
        {POINTERS.map(entry => (
          <NavItem
            key={entry.id}
            active={tool === entry.id}
            icon={<Icon icon={entry.icon} size={14} />}
            size="small"
            onClick={() => onPick(entry.id)}
          >
            <span className="cv-truncate">{entry.label}</span>
            <span className="cv-key">{entry.key}</span>
          </NavItem>
        ))}
      </RailGroup>

      <RailGroup label="Insert">
        {DRAWING.map(entry => (
          <NavItem
            key={entry.id}
            active={tool === entry.id}
            icon={<Icon icon={entry.icon} size={14} />}
            size="small"
            onClick={() => onPick(entry.id)}
          >
            <span className="cv-truncate">{entry.label}</span>
            <span className="cv-key">{entry.key}</span>
          </NavItem>
        ))}
      </RailGroup>

      <p className="cv-rail__hint">
        A tool draws once, then hands the pointer back to Select.
      </p>
    </Rail>
  )
}
