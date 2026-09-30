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

import { plural } from '../lib/format'
import { across, useIndex } from '../lib/index'
import type { Lens } from '../lib/lens'
import type { Loaded } from '../lib/model'
import { themeId } from '../lib/themes'
import { Brand } from './brand'
import { Document } from './document'
import { LensPanel } from './lens'
import { Overview } from './overview'
import { type Column, Resizer, useColumnWidth } from './resizer'
import './shell.css'

export interface ShellProps {
  loaded: Loaded
  lens: Lens
  page: string | undefined
  open: (page?: string) => void
}

const LIST: Column = {
  key: 'conformance:list-width',
  initial: 272,
  min: 200,
  max: 420,
}
const LENS: Column = {
  key: 'conformance:lens-width',
  initial: 320,
  min: 260,
  max: 520,
}

/**
 * The React example's window, put to a different use: three full-height
 * columns — the list, the document under its toolbar, and the lens — drawn
 * from the kit alone. The stylesheet only places things.
 */
export function Shell({ loaded, lens, page, open }: ShellProps) {
  const [query, setQuery] = useState('')
  const [inspecting, setInspecting] = useState(true)
  const [listWidth, setListWidth] = useColumnWidth(LIST)
  const [lensWidth, setLensWidth] = useColumnWidth(LENS)
  const index = useIndex(loaded, themeId(lens.theme))

  const needle = query.trim().toLowerCase()
  const stories = loaded.scenes.stories.filter(
    s =>
      s.name.toLowerCase().includes(needle) ||
      s.summary.toLowerCase().includes(needle)
  )
  const story = loaded.scenes.stories.find(s => s.id === page)
  const offEverywhere = across(lens.platforms, p => index.total(p))

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
                <SidebarMenuBadge
                  tint={
                    offEverywhere.off + offEverywhere.missing
                      ? 'danger'
                      : 'neutral'
                  }
                >
                  {offEverywhere.off + offEverywhere.missing || '✓'}
                </SidebarMenuBadge>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupLabel>Components</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {stories.map(item => {
                  const t = across(lens.platforms, p => index.story(p, item))
                  const off = t.off + t.missing
                  return (
                    <SidebarMenuItem key={item.id}>
                      <SidebarMenuButton
                        size="small"
                        active={item.id === page}
                        onClick={() => open(item.id)}
                      >
                        {item.name}
                      </SidebarMenuButton>
                      {off > 0 && (
                        <SidebarMenuBadge tint="danger">{off}</SidebarMenuBadge>
                      )}
                    </SidebarMenuItem>
                  )
                })}
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
                ? `${plural(story.scenes.length, 'scene')} · ${plural(lens.platforms.length, 'implementation')}`
                : `${plural(loaded.scenes.stories.length, 'component')} · ${plural(lens.platforms.length, 'implementation')}`}
            </span>
          </div>
          <span className="gk-space" />
          <Tooltip label="Toggle the lens" side="bottom">
            <IconButton
              label="Toggle the lens"
              className="dz-sidebar__trigger"
              aria-expanded={inspecting}
              onClick={() => setInspecting(!inspecting)}
            >
              <Icon icon={PanelGlyph} />
            </IconButton>
          </Tooltip>
        </header>

        <main className="gk-document">
          {story ? (
            <Document
              key={story.id}
              loaded={loaded}
              index={index}
              lens={lens}
              story={story}
            />
          ) : (
            <Overview
              loaded={loaded}
              index={index}
              stories={stories}
              platforms={lens.platforms}
              open={open}
            />
          )}
        </main>
      </SidebarInset>

      {inspecting && (
        <LensPanel
          loaded={loaded}
          index={index}
          lens={lens}
          story={story}
          width={lensWidth}
          resizer={
            <Resizer
              side="right"
              column={LENS}
              width={lensWidth}
              setWidth={setLensWidth}
              label="Resize the lens"
            />
          }
        />
      )}
    </SidebarProvider>
  )
}
