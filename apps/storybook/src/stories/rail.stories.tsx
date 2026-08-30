import { NavAction, NavItem, Rail, RailGroup } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Rail> = {
  title: 'Components/Rail',
  component: Rail,
}

export default meta
type Story = StoryObj<typeof meta>

function Column() {
  const [item, setItem] = useState('All documents')

  const groups = [
    { label: undefined, items: ['All documents', 'Recent', 'Shared with me'] },
    { label: 'Status', items: ['In progress', 'Needs review', 'Done'] },
  ]

  return (
    <div
      style={{
        display: 'flex',
        height: '20rem',
        overflow: 'hidden',
        border: 'var(--spacing-px) solid var(--color-border)',
        borderRadius: 'var(--radius-large)',
        background: 'var(--color-surface)',
      }}
    >
      <Rail>
        {groups.map((group, index) => (
          <RailGroup key={group.label ?? index} label={group.label}>
            {group.items.map(entry => (
              <NavItem
                key={entry}
                active={item === entry}
                size="small"
                onClick={() => setItem(entry)}
              >
                {entry}
              </NavItem>
            ))}
          </RailGroup>
        ))}
        <NavAction size="small">＋ New</NavAction>
      </Rail>
      <div style={{ flex: 1, minWidth: 0, padding: 'var(--spacing-5)' }} />
    </div>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="Rail"
      description="The second navigation column: what is inside whatever the sidebar selected. Lighter than the sidebar because it sits a step closer to the content, and narrower because its rows name one thing rather than a whole pane."
    >
      <Section
        title="Groups"
        description="The first run usually goes unlabelled — it is what the pane is already called, and naming it twice says nothing. A second run is the one that needs a label, because a gap alone leaves the reader to guess what the rows below have in common. The gaps are tighter than a sidebar's: a narrower column needs less space before its runs read as separate lists."
      >
        <Column />
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('rail')}
          labelOf={token => token.name.slice('rail-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
