import {
  Badge,
  NavAction,
  NavItem,
  Rail,
  RailGroup,
  SearchField,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { Frame, Pane } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Rail> = {
  title: 'Components/Rail',
  component: Rail,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const GROUPS = [
  { label: undefined, items: ['All documents', 'Recent', 'Shared with me'] },
  { label: 'Status', items: ['In progress', 'Needs review', 'Done'] },
  { label: 'Languages', items: ['Chinese', 'Japanese', 'German'] },
]

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Rail"
      description="The second navigation column: what is inside whatever the sidebar selected. Lighter than the sidebar because it sits a step closer to the content, and narrower because its rows name one thing rather than a whole pane. It scrolls on its own, so a long list does not push the window down."
    >
      <Section
        title="Groups"
        description="The first run usually goes unlabelled — it is what the pane is already called, and naming it twice says nothing. A second run is the one that needs a label, because a gap alone leaves the reader to guess what the rows below have in common. The gaps are tighter than a sidebar's: a narrower column needs less space before its runs read as separate lists. A NavAction at the foot adds to the list the rail is showing."
      >
        <Frame height="24rem" width="100%">
          <Rail>
            {GROUPS.map((group, index) => (
              <RailGroup key={group.label ?? index} label={group.label}>
                {group.items.map(entry => (
                  <NavItem
                    key={entry}
                    active={entry === 'All documents'}
                    size="small"
                  >
                    {entry}
                  </NavItem>
                ))}
              </RailGroup>
            ))}
            <NavAction size="small">＋ New</NavAction>
          </Rail>
          <Pane />
        </Frame>
      </Section>

      <Section title="Frame metrics">
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
  render: () => {
    const [item, setItem] = useState('All documents')
    return (
      <Frame height="20rem" width="40rem">
        <Rail>
          <RailGroup>
            {['All documents', 'Recent', 'Shared with me'].map(entry => (
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
        </Rail>
        <Pane />
      </Frame>
    )
  },
}

export const Groups: Story = {
  render: () => {
    const [item, setItem] = useState('All documents')
    return (
      <Frame height="24rem" width="40rem">
        <Rail>
          {GROUPS.map((group, index) => (
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
        </Rail>
        <Pane />
      </Frame>
    )
  },
}

export const WithAnAction: Story = {
  render: () => (
    <Frame height="20rem" width="40rem">
      <Rail>
        <RailGroup>
          <NavItem active size="small">
            Product terms
          </NavItem>
          <NavItem size="small">Legal</NavItem>
          <NavItem size="small">Marketing</NavItem>
        </RailGroup>
        <NavAction size="small">＋ New glossary</NavAction>
      </Rail>
      <Pane />
    </Frame>
  ),
}

export const WithCounts: Story = {
  render: () => (
    <Frame height="20rem" width="40rem">
      <Rail>
        <RailGroup label="Status">
          {[
            { label: 'Needs review', count: 24, active: true },
            { label: 'In progress', count: 6, active: false },
            { label: 'Done', count: 168, active: false },
          ].map(entry => (
            <NavItem key={entry.label} active={entry.active} size="small">
              {entry.label}
              <Badge
                size="small"
                tint="neutral"
                style={{ marginInlineStart: 'auto' }}
              >
                {entry.count}
              </Badge>
            </NavItem>
          ))}
        </RailGroup>
      </Rail>
      <Pane />
    </Frame>
  ),
}

export const Scrolling: Story = {
  render: () => {
    const [item, setItem] = useState('Document 1')
    const [query, setQuery] = useState('')
    const documents = Array.from(
      { length: 24 },
      (_, index) => `Document ${index + 1}`
    )
    const matched = documents.filter(entry =>
      entry.toLowerCase().includes(query.toLowerCase())
    )

    return (
      <Frame height="22rem" width="40rem">
        <Rail>
          <RailGroup>
            <SearchField
              placeholder="Filter"
              size="small"
              value={query}
              onValueChange={setQuery}
            />
          </RailGroup>
          <RailGroup label="Documents">
            {matched.map(entry => (
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
        </Rail>
        <Pane />
      </Frame>
    )
  },
}
