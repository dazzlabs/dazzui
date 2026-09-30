import {
  OptionCard,
  SegmentedControl,
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
} from '@dazzlabs/dazzui'
import { type CSSProperties, type ReactNode, useEffect } from 'react'

import {
  FAMILY_GROUPS,
  MODES,
  PLATFORMS,
  type Platforms,
  type Theme,
  useSystemDark,
} from '../lib/themes'
import { ThemePreview } from './theme-preview'

interface Props {
  theme: Theme
  setTheme: (theme: Theme) => void
  platforms: Platforms
  setPlatforms: (platforms: Platforms) => void
  width: number
  /** The panel's own edge, placed by the shell that keeps its width. */
  resizer: ReactNode
}

/**
 * The theme, and which devices show it: the mode first — with `System`
 * beside light and dark, since that is the half a phone picks on its own —
 * then the devices, then the family.
 */
export function ThemePanel({
  theme,
  setTheme,
  platforms,
  setPlatforms,
  width,
  resizer,
}: Props) {
  const systemDark = useSystemDark()
  const shownMode =
    theme.mode === 'system' ? (systemDark ? 'dark' : 'light') : theme.mode

  /* The chosen theme may be thirty rows down; open on it. */
  useEffect(() => {
    document
      .querySelector('.gk-themes [aria-checked="true"]')
      ?.scrollIntoView({ block: 'center' })
  }, [])

  return (
    <Sidebar
      side="right"
      collapsible="none"
      className="gk-themes"
      style={{ '--sidebar-width': `${width}px` } as CSSProperties}
    >
      {resizer}
      <SidebarHeader className="gk-themes__head">
        <strong>Theme</strong>
        <SegmentedControl
          size="small"
          aria-label="Appearance"
          items={MODES}
          value={theme.mode}
          onValueChange={mode => setTheme({ ...theme, mode })}
        />
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Devices</SidebarGroupLabel>
          <SidebarGroupContent>
            <SegmentedControl
              size="small"
              stretch
              aria-label="Devices"
              items={PLATFORMS}
              value={platforms}
              onValueChange={setPlatforms}
            />
          </SidebarGroupContent>
        </SidebarGroup>
        {FAMILY_GROUPS.map(({ title, families }) => (
          <SidebarGroup key={title}>
            <SidebarGroupLabel>{title}</SidebarGroupLabel>
            <SidebarGroupContent
              className="gk-themes__grid"
              role="radiogroup"
              aria-label={title}
            >
              {families.map(family => {
                const active = family.value === theme.family
                return (
                  <OptionCard
                    key={family.value}
                    className="gk-themes__tile"
                    selected={active}
                    title={
                      <>
                        <ThemePreview family={family.value} mode={shownMode} />
                        <span className="gk-themes__name">{family.title}</span>
                      </>
                    }
                    aria-label={family.title}
                    onClick={() => setTheme({ ...theme, family: family.value })}
                  />
                )
              })}
            </SidebarGroupContent>
          </SidebarGroup>
        ))}
      </SidebarContent>
    </Sidebar>
  )
}
