import {
  Card,
  ContextMenu,
  KeyCap,
  type MenuItem,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof ContextMenu> = {
  title: 'Components/ContextMenu',
  component: ContextMenu,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const ITEMS: MenuItem[] = [
  { label: 'Open', shortcut: <KeyCap>↩</KeyCap> },
  { label: 'Rename', shortcut: <KeyCap>F2</KeyCap> },
  { label: 'Duplicate', shortcut: <KeyCap>⌘D</KeyCap> },
  { label: 'Reveal in Finder' },
  { label: 'Delete', disabled: true },
]

function Target({ children }: { children?: React.ReactNode }) {
  return (
    <Card
      style={{
        display: 'grid',
        placeItems: 'center',
        width: '22rem',
        height: '9rem',
        color: 'var(--color-content-subtle)',
        userSelect: 'none',
      }}
    >
      {children ?? 'Right-click anywhere in here'}
    </Card>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="ContextMenu"
      description="The menu a right-click opens, over the thing that was clicked. It is the same menu a Button opens — Base UI re-exports the popup, the positioner and every row type from Menu, so this shares the drawing and the row renderer rather than resembling them. What is its own is where it opens: at the pointer rather than against a trigger's edge, and on a long press as well as a right-click, so it is reachable on a touch screen."
    >
      <Section
        title="A region that has a menu"
        description="The trigger is a listener on what you give it, not a wrapper around it, so a row, a card or a canvas keeps its own layout. Every affordance in a context menu must also be reachable somewhere a pointer can see — a right-click is a shortcut, never the only door."
      >
        <ContextMenu items={ITEMS}>
          <div>
            <Target />
          </div>
        </ContextMenu>
      </Section>

      <Section
        title="Checked rows"
        description="Set `checked` on every row of a group, including the unchecked ones, so the leading column holds its width and the labels stay on one left edge."
      >
        <ContextMenu
          items={[
            { label: 'Sort by name', checked: true },
            { label: 'Sort by date', checked: false },
            { label: 'Sort by size', checked: false },
          ]}
        >
          <div>
            <Target>Right-click to choose a sort</Target>
          </div>
        </ContextMenu>
      </Section>
    </Page>
  ),
}

export const Bare: Story = {
  render: () => (
    <ContextMenu items={ITEMS}>
      <div>
        <Target />
      </div>
    </ContextMenu>
  ),
}

export const Checked: Story = {
  render: () => {
    const [sort, setSort] = useState('name')
    const keys = ['name', 'date', 'size']

    return (
      <ContextMenu
        items={keys.map(key => ({
          label: `Sort by ${key}`,
          checked: sort === key,
          onSelect: () => setSort(key),
        }))}
      >
        <div>
          <Target>Sorted by {sort}</Target>
        </div>
      </ContextMenu>
    )
  },
}

export const OnARow: Story = {
  render: () => (
    <Col width="22rem" gap="var(--spacing-1)">
      {['main', 'release/4.2', 'fix/token-drift'].map(branch => (
        <ContextMenu
          key={branch}
          items={[
            { label: 'Check out' },
            { label: 'Rename', shortcut: <KeyCap>F2</KeyCap> },
            { label: 'Delete', disabled: branch === 'main' },
          ]}
        >
          <div
            style={{
              padding: 'var(--spacing-2) var(--spacing-3)',
              borderRadius: 'var(--radius-medium)',
              background: 'var(--color-surface-muted)',
              fontFamily: 'var(--base-font-code)',
              fontSize: 'var(--label-small-font-size)',
            }}
          >
            {branch}
          </div>
        </ContextMenu>
      ))}
    </Col>
  ),
}
