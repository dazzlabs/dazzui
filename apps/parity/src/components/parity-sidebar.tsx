import {
  NavItem,
  SearchField,
  Sidebar,
  SidebarGroup,
} from '@dazzlabs/dazzui-react'

export interface ParitySidebarProps {
  classNames: readonly string[]
  /** Drifting checks per component, under the platform scope in force. */
  counts: ReadonlyMap<string, number>
  query: string
  onQueryChange: (query: string) => void
  selected: string | null
  onSelectOverview: () => void
  onSelect: (className: string) => void
}

/**
 * The leading column: one row per component, with how many of its checks
 * drift. The filter narrows the rows by name, and the same query narrows
 * the specimens on the open page by their axes, so typing `danger` shows
 * the danger-tinted specimens of whichever component is open.
 */
export function ParitySidebar({
  classNames,
  counts,
  query,
  onQueryChange,
  selected,
  onSelectOverview,
  onSelect,
}: ParitySidebarProps) {
  const needle = query.trim().toLowerCase()
  const rows = classNames.filter(name => needle === '' || name.includes(needle))

  return (
    <Sidebar
      className="pv-sidebar"
      header={
        <div className="pv-sidebar__search">
          <SearchField
            value={query}
            onValueChange={onQueryChange}
            placeholder="Filter"
            size="small"
            aria-label="Filter components and specimens"
          />
        </div>
      }
    >
      <SidebarGroup>
        <NavItem active={selected === null} onClick={onSelectOverview}>
          Overview
        </NavItem>
      </SidebarGroup>
      <SidebarGroup label="Components">
        {rows.map(name => {
          const count = counts.get(name) ?? 0
          return (
            <NavItem
              key={name}
              active={selected === name}
              onClick={() => onSelect(name)}
              tint={count > 0 ? 'primary' : 'neutral'}
            >
              <span className="pv-truncate">{name.replace(/^ff-/, '')}</span>
              {count > 0 && <span className="pv-count">{count}</span>}
            </NavItem>
          )
        })}
      </SidebarGroup>
    </Sidebar>
  )
}
