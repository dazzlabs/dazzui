import {
  Badge,
  Icon,
  NavAction,
  NavItem,
  SearchField,
  Sidebar,
  SidebarCard,
  SidebarGroup,
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
    <Sidebar
      className="ca-sidebar"
      // The strip carries nothing of its own, and still has to be here: it is
      // held at the titlebar's height, which is what keeps this column's rows
      // level with the toolbar beside them — and with the titlebar hidden it
      // is where the window's own controls land.
      header={<span aria-hidden />}
    >
      <div className="ca-sidebar__search">
        <SearchField
          placeholder="Find a project"
          size="small"
          value={query}
          onValueChange={setQuery}
        />
      </div>

      <SidebarGroup label="Projects">
        {found.map(project => (
          <NavItem
            key={project.id}
            active={project.id === active.id}
            icon={<Icon icon={FolderGlyph} size={14} />}
            title={`${project.name} — ${project.path}`}
            onClick={() => onSelect(project.id)}
          >
            <span className="ca-truncate">{shortName(project.name)}</span>
          </NavItem>
        ))}
      </SidebarGroup>

      {/* A nav action takes no icon slot — it adds rather than names a pane, so
          its glyph belongs in the label with it. */}
      <NavAction className="ca-sidebar__new">
        <Icon icon={PlusGlyph} size={14} />
        Open a project…
      </NavAction>

      {/* Settings belong to the window, not to a project, so the row sits
          under the list rather than in it — and it is a NavItem rather than a
          NavAction because it selects a pane like the rows above it do. */}
      <NavItem
        active={settingsOpen}
        className="ca-sidebar__settings"
        icon={<Icon icon={GearGlyph} size={14} />}
        onClick={onOpenSettings}
      >
        <span className="ca-truncate">Settings</span>
      </NavItem>

      <SidebarCard label="Working tree">
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
      </SidebarCard>
    </Sidebar>
  )
}
