import {
  Icon,
  type IconGlyph,
  NavAction,
  NavItem,
  Rail,
  RailGroup,
} from '@dazzlabs/dazzui-react'

import {
  BranchGlyph,
  ChevronGlyph,
  GearGlyph,
  KeyboardGlyph,
  ShieldGlyph,
  SparkleGlyph,
} from '../data/icons'
import type { SettingsSection } from '../lib/project'

const SECTIONS: { id: SettingsSection; label: string; glyph: IconGlyph }[] = [
  { id: 'general', label: 'General', glyph: GearGlyph },
  { id: 'agent', label: 'Agent', glyph: SparkleGlyph },
  { id: 'permissions', label: 'Permissions', glyph: ShieldGlyph },
  { id: 'integrations', label: 'Integrations', glyph: BranchGlyph },
  { id: 'shortcuts', label: 'Shortcuts', glyph: KeyboardGlyph },
]

export interface SettingsRailProps {
  section: SettingsSection
  onOpen: (section: SettingsSection) => void
  onClose: () => void
}

/**
 * The settings pages, in the same column the project's panes use.
 *
 * Settings belong to the window rather than to any one project, so they take
 * the rail over instead of sitting in it — one navigation column that changes
 * what it lists, rather than a second one that appears.
 */
export function SettingsRail({ section, onOpen, onClose }: SettingsRailProps) {
  return (
    <Rail className="ca-rail">
      <RailGroup>
        {SECTIONS.map(entry => (
          <NavItem
            key={entry.id}
            active={entry.id === section}
            icon={<Icon icon={entry.glyph} size={14} />}
            size="small"
            onClick={() => onOpen(entry.id)}
          >
            <span className="ca-truncate">{entry.label}</span>
          </NavItem>
        ))}
      </RailGroup>

      {/* One word: the rail is 150px by token, and a label that wraps turns a
          row into two. The chevron says where "back" goes. */}
      <NavAction size="small" onClick={onClose}>
        <Icon icon={ChevronGlyph} size={14} className="ca-rail__back" />
        <span className="ca-truncate">Back</span>
      </NavAction>
    </Rail>
  )
}
