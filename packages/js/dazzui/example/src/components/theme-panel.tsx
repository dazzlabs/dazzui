import {
  WeatherMoon20Regular as Moon,
  WeatherSunny20Regular as Sun,
} from '@fluentui/react-icons'
import {
  Icon,
  OptionCard,
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  Toggle,
} from '@dazzlabs/dazzui'
import { type CSSProperties, type ReactNode, useEffect } from 'react'

import { FAMILY_GROUPS, type Theme } from '../lib/themes'
import { ThemePreview } from './theme-preview'

interface Props {
  theme: Theme
  setTheme: (theme: Theme) => void
  width: number
  /** The panel's own edge, placed by the shell that keeps its width. */
  resizer: ReactNode
}

/** The theme, and nothing else: which half of it, then which family. */
export function ThemePanel({ theme, setTheme, width, resizer }: Props) {
  const dark = theme.mode === 'dark'

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
        <Toggle
          label={dark ? 'Switch to light' : 'Switch to dark'}
          pressed={dark}
          onPressedChange={pressed =>
            setTheme({ ...theme, mode: pressed ? 'dark' : 'light' })
          }
        >
          <Icon icon={dark ? Moon : Sun} />
        </Toggle>
      </SidebarHeader>
      <SidebarContent>
        {FAMILY_GROUPS.map(({ title, families }) => (
          <SidebarGroup key={title}>
            <SidebarGroupLabel>{title}</SidebarGroupLabel>
            <SidebarGroupContent
              className="gk-themes__grid"
              role="radiogroup"
              aria-label={title}
            >
              {families.map(family => {
                const row = { family: family.value, mode: theme.mode }
                const active = family.value === theme.family
                return (
                  <OptionCard
                    key={family.value}
                    className="gk-themes__tile"
                    selected={active}
                    title={
                      <>
                        <ThemePreview theme={row} />
                        <span className="gk-themes__name">{family.title}</span>
                      </>
                    }
                    aria-label={family.title}
                    onClick={() => setTheme(row)}
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
