import {
  Home20Regular as HomeGlyph,
  PanelRight20Regular as PanelGlyph,
} from '@fluentui/react-icons'
import {
  Icon,
  IconButton,
  SearchField,
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuBadge,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarTrigger,
  Tooltip,
} from '@dazzlabs/dazzui'
import { type CSSProperties, useState } from 'react'

import type { Platforms, Theme } from '../lib/themes'
import { STORIES, matching, storyOf } from '../stories'
import { Brand } from './brand'
import { Desk } from './desk'
import { type Column, Resizer, useColumnWidth } from './resizer'
import { ThemePanel } from './theme-panel'
import './shell.css'

export interface ShellProps {
  /** The story being read; the overview when undefined. */
  story: string | undefined
  open: (story: string) => void
  theme: Theme
  setTheme: (theme: Theme) => void
  platforms: Platforms
  setPlatforms: (platforms: Platforms) => void
}

const LIST: Column = {
  key: 'mobile-example:list-width',
  initial: 272,
  min: 200,
  max: 420,
}

const THEMES: Column = {
  key: 'mobile-example:themes-width',
  initial: 336,
  min: 280,
  max: 560,
}

/**
 * The desktop example's window, put to this use: three full-height columns
 * — the story list, the desk the two phones stand on under its toolbar,
 * and the theme panel. Everything in the window is the desktop kit's;
 * everything on the phones is this package's.
 */
export function Shell({
  story: page,
  open,
  theme,
  setTheme,
  platforms,
  setPlatforms,
}: ShellProps) {
  const [query, setQuery] = useState('')
  const [inspecting, setInspecting] = useState(true)
  const story = storyOf(page) ?? STORIES[0]!
  const [listWidth, setListWidth] = useColumnWidth(LIST)
  const [themesWidth, setThemesWidth] = useColumnWidth(THEMES)
  return (
    <SidebarProvider
      className="gk"
      style={{ '--sidebar-width': `${listWidth}px` } as CSSProperties}
    >
      <Sidebar className="gk-list">
        <SidebarHeader>
          <Brand onClick={() => open('overview')} />
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SearchField
              size="small"
              value={query}
              onValueChange={setQuery}
              placeholder="Search"
              aria-label="Search stories"
            />
          </SidebarGroup>
          <SidebarGroup>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  size="small"
                  icon={<Icon icon={HomeGlyph} />}
                  active={story.id === 'overview'}
                  onClick={() => open('overview')}
                >
                  Overview
                </SidebarMenuButton>
                <SidebarMenuBadge>{STORIES.length - 1}</SidebarMenuBadge>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupLabel>Components</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {matching(query)
                  .filter(item => item.id !== 'overview')
                  .map(item => (
                    <SidebarMenuItem key={item.id}>
                      <SidebarMenuButton
                        size="small"
                        active={item.id === story.id}
                        onClick={() => open(item.id)}
                      >
                        {item.name}
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>

      <SidebarInset className="gk-window">
        <Resizer
          side="left"
          column={LIST}
          width={listWidth}
          setWidth={setListWidth}
          label="Resize the story list"
        />
        <header className="gk-toolbar">
          <Tooltip label="Toggle sidebar  ⌘B" side="bottom">
            <SidebarTrigger />
          </Tooltip>
          <div className="gk-title">
            <strong>{story.name}</strong>
            <span className="gk-quiet">{story.summary}</span>
          </div>
          <span className="gk-space" />
          <Tooltip label="Toggle theme panel" side="bottom">
            <IconButton
              label="Toggle theme panel"
              className="dz-sidebar__trigger"
              aria-expanded={inspecting}
              onClick={() => setInspecting(!inspecting)}
            >
              <Icon icon={PanelGlyph} />
            </IconButton>
          </Tooltip>
        </header>

        <main className="gk-document">
          <Desk story={story} theme={theme} platforms={platforms} />
        </main>
      </SidebarInset>

      {inspecting && (
        <ThemePanel
          theme={theme}
          setTheme={setTheme}
          platforms={platforms}
          setPlatforms={setPlatforms}
          width={themesWidth}
          resizer={
            <Resizer
              side="right"
              column={THEMES}
              width={themesWidth}
              setWidth={setThemesWidth}
              label="Resize the theme panel"
            />
          }
        />
      )}
    </SidebarProvider>
  )
}
