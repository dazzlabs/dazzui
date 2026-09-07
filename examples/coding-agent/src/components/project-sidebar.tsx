import {
  Badge,
  Card,
  Icon,
  SearchField,
  SectionLabel,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@dazzlabs/dazzui-react'
import { useMemo, useState } from 'react'

import { BranchGlyph, FolderGlyph, GearGlyph, PlusGlyph } from '../data/icons'
import type { Project } from '../lib/project'

/** `owner/repo` is too wide for the column; the repo is the part that names it. */
function shortName(name: string): string {
  return name.split('/').pop() ?? name
}

export interface ProjectSidebarProps {
  projects: readonly Project[]
  active: Project
  /** How many files the working tree holds, the run's own writes included. */
  changed: number
  /** Marks the settings row while the settings view has the pane. */
  settingsOpen: boolean
  onSelect: (id: string) => void
  onOpenSettings: () => void
}

/**
 * The leading column: the projects themselves. A project is what everything
 * else in the window belongs to — its sessions, its issues, its pull requests
 * and its working tree — so it is what the outermost column lists.
 */
export function ProjectSidebar({
  projects,
  active,
  changed,
  settingsOpen,
  onOpenSettings,
  onSelect,
}: ProjectSidebarProps) {
  const [query, setQuery] = useState('')

  const found = useMemo(() => {
    const needle = query.trim().toLowerCase()
    if (!needle) return projects
    return projects.filter(project =>
      project.name.toLowerCase().includes(needle)
    )
  }, [projects, query])

  return (
    <Sidebar className="ca-sidebar">
      {/* The strip carries nothing of its own, and still has to be here: it
          is held at the titlebar's height, which is what keeps this column's
          rows level with the toolbar beside them — and with the titlebar
          hidden it is where the window's own controls land. */}
      <SidebarHeader>
        <span aria-hidden />
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SearchField
            placeholder="Find a project"
            size="small"
            value={query}
            onValueChange={setQuery}
          />
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Projects</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {found.map(project => (
                <SidebarMenuItem key={project.id}>
                  <SidebarMenuButton
                    active={project.id === active.id}
                    icon={<Icon icon={FolderGlyph} size={14} />}
                    title={`${project.name} — ${project.path}`}
                    onClick={() => onSelect(project.id)}
                  >
                    <span>{shortName(project.name)}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}

              {/* The plain row takes no icon slot — it adds rather than names
                  a pane, so its glyph belongs in the label with it. */}
              <SidebarMenuItem>
                <SidebarMenuButton variant="plain">
                  <Icon icon={PlusGlyph} size={14} />
                  <span>Open a project…</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        {/* Settings belong to the window, not to a project, so the row sits
            under the list rather than in it — and it is a default row rather
            than a plain one because it selects a pane like the rows above. */}
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  active={settingsOpen}
                  icon={<Icon icon={GearGlyph} size={14} />}
                  onClick={onOpenSettings}
                >
                  <span>Settings</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter>
        <Card>
          <SectionLabel>Working tree</SectionLabel>
          <span className="ca-sidebar__branch">
            <Icon icon={BranchGlyph} size={12} />
            <span className="ca-truncate">{active.branch}</span>
          </span>
          <span className="ca-sidebar__stat">
            {changed === 0
              ? 'Nothing to commit'
              : `${changed} ${changed === 1 ? 'file' : 'files'} changed`}
          </span>
          <Badge size="small" tint={changed === 0 ? 'success' : 'warning'}>
            {changed === 0 ? 'clean' : 'uncommitted'}
          </Badge>
        </Card>
      </SidebarFooter>
    </Sidebar>
  )
}
