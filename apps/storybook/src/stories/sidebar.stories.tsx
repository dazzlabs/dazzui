import {
  Badge,
  NavItem,
  Sidebar,
  SidebarCard,
  SidebarGroup,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Sidebar> = {
  title: 'Components/Sidebar',
  component: Sidebar,
}

export default meta
type Story = StoryObj<typeof meta>

const PANES = ['Documents', 'Glossaries', 'Settings']

/** A pane beside the column, so the sidebar has something to be a sidebar to. */
function Frame({ children }: { children: React.ReactNode }) {
  return (
    <div
      style={{
        display: 'flex',
        height: '22rem',
        overflow: 'hidden',
        border: 'var(--spacing-px) solid var(--color-border)',
        borderRadius: 'var(--radius-large)',
        background: 'var(--color-surface)',
      }}
    >
      {children}
      <div style={{ flex: 1, minWidth: 0, padding: 'var(--spacing-5)' }} />
    </div>
  )
}

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

export const Overview: Story = {
  render: () => (
    <Page
      title="Sidebar"
      description="The leading workspace column. Its width is a token rather than a layout choice, because the columns of a window have to agree with each other — the header strip is exactly the titlebar's height, or the separator where the two meet steps."
    >
      <Section
        title="Groups and a card"
        description="Rows come in groups; the gap between groups does most of the work and the label only names it. A card pinned to the foot stays put while the rows above it scroll."
      >
        <Frame>
          <Column />
        </Frame>
      </Section>

      <Section
        title="Resizable"
        description="Drag the trailing edge, or focus it with Tab and use the arrow keys — shift for a coarser step. Double-click puts it back where it started. Off by default: a sidebar with no pane to trade width with should not grow a handle that leads nowhere."
      >
        <Frame>
          <Column resizable />
        </Frame>
      </Section>

      <Section
        title="Frame metrics"
        description="The shell's shared scale, which the columns and the window chrome both alias."
      >
        <TokenTable
          tokens={componentTokens('frame')}
          labelOf={token => token.name.slice('frame-'.length)}
          nameHeader="Part"
        />
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('sidebar')}
          labelOf={token => token.name.slice('sidebar-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
