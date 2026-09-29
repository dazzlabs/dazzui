import {
  Home20Regular as HomeGlyph,
  PanelRight20Regular as PanelGlyph,
} from '@fluentui/react-icons'
import {
  Button,
  Card,
  Icon,
  IconButton,
  SearchField,
  SectionLabel,
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
} from '@dazzlabs/dazzui-react'
import { type CSSProperties, useState } from 'react'

import { STORIES, storyOf, matching } from '../stories'
import type { Story } from '../lib/story'
import type { Theme } from '../lib/themes'
import { Brand } from './brand'
import { type Column, Resizer, useColumnWidth } from './resizer'
import { ThemePanel } from './theme-panel'
import './shell.css'

/** Everything the shell is handed. It owns the layout and nothing else. */
export interface ShellProps {
  /** The component being read, or undefined on the overview. */
  page: string | undefined
  open: (page?: string) => void
  theme: Theme
  setTheme: (theme: Theme) => void
}

const LIST: Column = {
  key: 'example:list-width',
  initial: 272,
  min: 200,
  max: 420,
}

const THEMES: Column = {
  key: 'example:themes-width',
  initial: 336,
  min: 280,
  max: 560,
}

/**
 * A desktop application: three full-height columns — the source list, the
 * document under its toolbar, and the theme panel.
 * The shell is drawn from the kit alone and the stylesheet only places
 * things, so a port builds the same window out of its own components.
 */
export function Shell({ page, open, theme, setTheme }: ShellProps) {
  const [query, setQuery] = useState('')
  const [inspecting, setInspecting] = useState(true)
  const story = storyOf(page)
  const [listWidth, setListWidth] = useColumnWidth(LIST)
  const [themesWidth, setThemesWidth] = useColumnWidth(THEMES)
  return (
    <SidebarProvider
      className="gk"
      style={{ '--sidebar-width': `${listWidth}px` } as CSSProperties}
    >
      <Sidebar className="gk-list">
        <SidebarHeader>
          <Brand onClick={() => open()} />
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SearchField
              size="small"
              value={query}
              onValueChange={setQuery}
              placeholder="Search"
              aria-label="Search components"
            />
          </SidebarGroup>
          <SidebarGroup>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  size="small"
                  icon={<Icon icon={HomeGlyph} />}
                  active={!story}
                  onClick={() => open()}
                >
                  Overview
                </SidebarMenuButton>
                <SidebarMenuBadge>{STORIES.length}</SidebarMenuBadge>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupLabel>Components</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {matching(query).map(item => (
                  <SidebarMenuItem key={item.id}>
                    <SidebarMenuButton
                      size="small"
                      active={item.id === page}
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
          label="Resize the component list"
        />
        <header className="gk-toolbar">
          <Tooltip label="Toggle sidebar  ⌘B" side="bottom">
            <SidebarTrigger />
          </Tooltip>
          <div className="gk-title">
            <strong>{story ? story.name : 'Overview'}</strong>
            <span className="gk-quiet">
              {story
                ? `${story.examples.length} examples`
                : `${STORIES.length} components`}
            </span>
          </div>
          <span className="gk-space" />
          <Tooltip label="Toggle theme panel" side="bottom">
            <IconButton
              label="Toggle theme panel"
              className="dz-sidebar__trigger"
              aria-expanded={inspecting}
              onClick={() => setInspecting(!inspecting)}
            >
              <Icon icon={PanelGlyph} size={16} />
            </IconButton>
          </Tooltip>
        </header>

        <main className="gk-document">
          {story ? (
            <Document key={story.id} story={story} />
          ) : (
            <Overview query={query} open={open} />
          )}
        </main>
      </SidebarInset>

      {inspecting && (
        <ThemePanel
          theme={theme}
          setTheme={setTheme}
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

function Overview({
  query,
  open,
}: { query: string } & Pick<ShellProps, 'open'>) {
  return (
    <div className="gk-page">
      <div className="gk-grid">
        {matching(query).map(({ id, name, Preview }) => (
          <Card key={id} variant="outlined" className="gk-tile">
            <div className="gk-stage">
              <Preview />
            </div>
            <Button variant="plain" fullWidth onClick={() => open(id)}>
              {name}
            </Button>
          </Card>
        ))}
      </div>
    </div>
  )
}

function Document({ story }: { story: Story }) {
  return (
    <div className="gk-page">
      <p className="gk-lead">{story.summary}</p>
      {story.examples.map(({ title, note, Demo }) => (
        <section key={title} className="gk-section">
          <SectionLabel>{title}</SectionLabel>
          <Card variant="outlined" size="large" className="gk-stage">
            <Demo />
          </Card>
          {note && <p className="gk-quiet">{note}</p>}
        </section>
      ))}
    </div>
  )
}
