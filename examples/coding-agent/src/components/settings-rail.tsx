import {
  Icon,
  type IconGlyph,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
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
    <Sidebar collapsible="none" className="ca-rail">
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              {SECTIONS.map(entry => (
                <SidebarMenuItem key={entry.id}>
                  <SidebarMenuButton
                    active={entry.id === section}
                    icon={<Icon icon={entry.glyph} size={14} />}
                    size="small"
                    onClick={() => onOpen(entry.id)}
                  >
                    <span>{entry.label}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      {/* One word: the rail is 150px by token, and a label that wraps turns a
          row into two. The chevron says where "back" goes. */}
      <SidebarFooter>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="small" variant="plain" onClick={onClose}>
              <Icon icon={ChevronGlyph} size={14} className="ca-rail__back" />
              <span>Back</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  )
}
