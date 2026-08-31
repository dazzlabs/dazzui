import { Document20Regular as Doc } from '@fluentui/react-icons'
import {
  Badge,
  Icon,
  NavAction,
  NavItem,
  Progress,
  SearchField,
  Sidebar,
  SidebarCard,
  SidebarGroup,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { Frame, Pane } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Sidebar> = {
  title: 'Components/Sidebar',
  component: Sidebar,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const PANES = ['Documents', 'Glossaries', 'Settings']

function Column({ resizable }: { resizable?: boolean }) {
  const [pane, setPane] = useState('Documents')

  return (
    <Sidebar resizable={resizable}>
      <SidebarGroup>
        {PANES.map(entry => (
          <NavItem
            key={entry}
            active={pane === entry}
            onClick={() => setPane(entry)}
          >
            {entry}
          </NavItem>
        ))}
      </SidebarGroup>

      <SidebarGroup label="Library">
        <NavItem>Translation memory</NavItem>
        <NavItem>Archive</NavItem>
      </SidebarGroup>

      <SidebarCard label="Today">
        <span style={{ fontSize: 'var(--body-small-font-size)' }}>
          148 segments translated
        </span>
        <Badge size="small" tint="success">
          All providers up
        </Badge>
      </SidebarCard>
    </Sidebar>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Sidebar"
      description="The leading workspace column. Its width is a token rather than a layout choice, because the columns of a window have to agree with each other — the header strip is exactly the titlebar's height, or the separator where the two meet steps."
    >
      <Section
        title="Groups and a card"
        description="Rows come in groups; the gap between groups does most of the work and the label only names it, and the first run usually goes unlabelled because the window already names it. A card pinned to the foot stays put while the rows above it scroll, which is what separates it from one more group at the bottom of the list."
      >
        <Frame width="100%">
          <Column />
          <Pane />
        </Frame>
      </Section>

      <Section
        title="Resizable"
        description="Drag the trailing edge, or focus it with Tab and use the arrow keys — shift for a coarser step. Double-click puts it back where it started. Off by default: a sidebar with no pane to trade width with should not grow a handle that leads nowhere. Drag past the floor and it collapses rather than shrinking, but only where `onCollapse` is passed — leave it out and the drag simply stops at `minWidth`."
      >
        <Frame width="100%">
          <Column resizable />
          <Pane />
        </Frame>
      </Section>

      <Section
        title="Frame metrics"
        description="The shell's shared scale, which the columns and the window chrome both alias. The sidebar owns none of its own: a column that sized itself would agree with the titlebar only by luck."
      >
        <TokenTable
          tokens={componentTokens('frame')}
          labelOf={token => token.name.slice('frame-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const AColumn: Story = {
  render: () => (
    <Frame width="44rem">
      <Column />
      <Pane />
    </Frame>
  ),
}

export const Groups: Story = {
  render: () => (
    <Frame height="24rem" width="44rem">
      <Sidebar>
        <SidebarGroup>
          <NavItem active icon={<Icon icon={Doc} size={14} />}>
            Documents
          </NavItem>
          <NavItem icon={<Icon icon={Doc} size={14} />}>Glossaries</NavItem>
        </SidebarGroup>
        <SidebarGroup label="Library">
          <NavItem>Translation memory</NavItem>
          <NavItem>Archive</NavItem>
        </SidebarGroup>
        <SidebarGroup label="Shared">
          <NavItem>Team glossary</NavItem>
          <NavItem>Style guide</NavItem>
        </SidebarGroup>
      </Sidebar>
      <Pane />
    </Frame>
  ),
}

export const WithAHeader: Story = {
  render: () => {
    const [query, setQuery] = useState('')
    return (
      <Frame width="44rem">
        <Sidebar
          header={
            <SearchField
              placeholder="Search"
              size="small"
              value={query}
              onValueChange={setQuery}
            />
          }
        >
          <SidebarGroup>
            <NavItem active>Documents</NavItem>
            <NavItem>Glossaries</NavItem>
            <NavItem>Settings</NavItem>
          </SidebarGroup>
        </Sidebar>
        <Pane />
      </Frame>
    )
  },
}

export const WithACard: Story = {
  render: () => (
    <Frame height="24rem" width="44rem">
      <Sidebar>
        <SidebarGroup>
          {Array.from({ length: 10 }, (_, index) => (
            <NavItem key={index} active={index === 0}>
              Document {index + 1}
            </NavItem>
          ))}
        </SidebarGroup>
        <SidebarCard label="Today">
          <span style={{ fontSize: 'var(--body-small-font-size)' }}>
            148 of 392 segments
          </span>
          <Progress size="small" value={38} />
        </SidebarCard>
      </Sidebar>
      <Pane />
    </Frame>
  ),
}

export const WithAnAction: Story = {
  render: () => (
    <Frame width="44rem">
      <Sidebar>
        <SidebarGroup>
          <NavItem active>Documents</NavItem>
          <NavItem>Glossaries</NavItem>
        </SidebarGroup>
        <SidebarGroup label="Workspaces">
          <NavItem>Personal</NavItem>
          <NavItem>Acme Ltd</NavItem>
          <NavAction>＋ New workspace</NavAction>
        </SidebarGroup>
      </Sidebar>
      <Pane />
    </Frame>
  ),
}

export const Resizable: Story = {
  render: () => (
    <Frame width="44rem">
      <Column resizable />
      <Pane />
    </Frame>
  ),
}

export const Collapsing: Story = {
  render: () => {
    const [width, setWidth] = useState(220)
    const [collapsed, setCollapsed] = useState(false)

    return (
      <Frame width="44rem">
        {collapsed ? null : (
          <Sidebar
            resizable
            width={width}
            onWidthChange={setWidth}
            onCollapse={() => setCollapsed(true)}
          >
            <SidebarGroup>
              <NavItem active>Documents</NavItem>
              <NavItem>Glossaries</NavItem>
              <NavItem>Settings</NavItem>
            </SidebarGroup>
          </Sidebar>
        )}
        <Pane>
          {collapsed ? (
            <NavAction
              style={{ width: 'auto' }}
              onClick={() => setCollapsed(false)}
            >
              Bring the sidebar back
            </NavAction>
          ) : null}
        </Pane>
      </Frame>
    )
  },
}
