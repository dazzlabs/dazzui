import {
  Icon,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@dazzlabs/dazzui-react'

import { DRAWING, POINTERS, type Tool, type ToolId } from '../data/tools'

export interface ToolRailProps {
  tool: ToolId
  onPick: (tool: ToolId) => void
}

/**
 * The second column: what the pointer does next. It is the same `Sidebar`
 * as the document column at the rail's own width, and it never collapses.
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
  const rows = (tools: readonly Tool[]) => (
    <SidebarMenu>
      {tools.map(entry => (
        <SidebarMenuItem key={entry.id}>
          <SidebarMenuButton
            active={tool === entry.id}
            icon={<Icon icon={entry.icon} size={14} />}
            size="small"
            onClick={() => onPick(entry.id)}
          >
            <span className="cv-truncate">{entry.label}</span>
            <span className="cv-key">{entry.key}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      ))}
    </SidebarMenu>
  )

  return (
    <Sidebar collapsible="none" className="cv-rail">
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>{rows(POINTERS)}</SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Insert</SidebarGroupLabel>
          <SidebarGroupContent>{rows(DRAWING)}</SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter>
        <p className="cv-rail__hint">
          A tool draws once, then hands the pointer back to Select.
        </p>
      </SidebarFooter>
    </Sidebar>
  )
}
