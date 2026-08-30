import {
  ActionBar,
  Card,
  PillTabs,
  SearchField,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof PillTabs> = {
  title: 'Components/PillTabs',
  component: PillTabs,
}

export default meta
type Story = StoryObj<typeof meta>

const FILTERS = [
  { value: 'all', label: 'All history' },
  { value: 'starred', label: 'Starred', count: 64 },
  { value: 'edited', label: 'Edited by me', count: 18 },
  { value: 'shared', label: 'Shared', disabled: true },
]

const PLAIN = [
  { value: 'all', label: 'All' },
  { value: 'open', label: 'Open' },
  { value: 'done', label: 'Done' },
]

export const Overview: Story = {
  render: () => {
    const [filter, setFilter] = useState('all')
    return (
      <Page
        title="PillTabs"
        description="Free-standing filter pills. Unlike a segmented control there is no track holding them together, so each pill needs its own fill to be a target at all — which also means they can wrap, and a filter row usually has to."
      >
        <Section
          title="Filtering"
          description="A count rides inside the pill, dimmed against the label it follows rather than set beside it as a badge."
        >
          <PillTabs
            aria-label="History filter"
            items={FILTERS}
            value={filter}
            onValueChange={setFilter}
          />
        </Section>
      </Page>
    )
  },
}

export const WithCounts: Story = {
  render: () => {
    const [filter, setFilter] = useState('needs-review')
    return (
      <Page
        title="With counts"
        description="The count is part of the pill, not a badge beside it: a badge would be a second thing to read, and the number here is an attribute of the filter rather than a state of its own."
      >
        <Section title="Counted and uncounted">
          <Stack>
            <Specimen label="counted">
              <PillTabs
                aria-label="Segment filter"
                items={[
                  { value: 'all', label: 'All', count: 392 },
                  { value: 'needs-review', label: 'Needs review', count: 24 },
                  { value: 'done', label: 'Done', count: 168 },
                  { value: 'empty', label: 'Untranslated', count: 0 },
                ]}
                value={filter}
                onValueChange={setFilter}
              />
            </Specimen>
            <Specimen label="plain">
              <PillTabs aria-label="Plain filter" items={PLAIN} value="all" />
            </Specimen>
          </Stack>
        </Section>
      </Page>
    )
  },
}

export const Disabled: Story = {
  render: () => (
    <Page
      title="Disabled"
      description="A filter with nothing behind it yet stays in the row rather than disappearing, so the set does not reflow every time a permission or a connection changes."
    >
      <Section title="One unavailable">
        <PillTabs aria-label="History filter" items={FILTERS} value="all" />
      </Section>
    </Page>
  ),
}

export const Tints: Story = {
  render: () => (
    <Page
      title="Tints"
      description="The selected pill is the filled recipe, so the ramp is what picks it out. A filter row keeps one tint: the colour means “this one is on”, and varying it per pill would make it mean something about the pill."
    >
      <Section title="Every ramp">
        <Stack>
          {TINTS.map(tint => (
            <Specimen key={tint} label={tint}>
              <PillTabs
                aria-label={tint}
                items={PLAIN}
                tint={tint}
                value="open"
              />
            </Specimen>
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

export const Wrapping: Story = {
  render: () => {
    const [language, setLanguage] = useState('zh')
    const languages = [
      { value: 'zh', label: 'Chinese', count: 168 },
      { value: 'ja', label: 'Japanese', count: 96 },
      { value: 'de', label: 'German', count: 64 },
      { value: 'fr', label: 'French', count: 48 },
      { value: 'es', label: 'Spanish', count: 32 },
      { value: 'pt', label: 'Portuguese', count: 24 },
      { value: 'it', label: 'Italian', count: 12 },
      { value: 'ko', label: 'Korean', count: 8 },
      { value: 'nl', label: 'Dutch', count: 4 },
    ]

    return (
      <Page
        title="Wrapping"
        description="Free-standing pills wrap, which is the reason to reach for them over a segmented control: a filter row grows with the data, and a track that had to hold nine segments would run off the pane."
      >
        <Section title="Nine of them, in a narrow column">
          <div style={{ maxWidth: '22rem' }}>
            <PillTabs
              aria-label="Target language"
              items={languages}
              value={language}
              onValueChange={setLanguage}
            />
          </div>
        </Section>
      </Page>
    )
  },
}

export const InAPaneHeader: Story = {
  render: () => {
    const [filter, setFilter] = useState('all')
    const [query, setQuery] = useState('')

    return (
      <Page
        title="In a pane header"
        description="Where these usually live: above a list, beside the search that narrows the same list. The pills say which subset, the field says which text — two axes over one collection."
      >
        <Section title="Above a list">
          <div style={{ maxWidth: '34rem', display: 'grid', gap: '0.75rem' }}>
            <ActionBar align="between">
              <PillTabs
                aria-label="History filter"
                items={FILTERS}
                value={filter}
                onValueChange={setFilter}
              />
              <div style={{ width: '12rem' }}>
                <SearchField
                  placeholder="Search"
                  size="small"
                  value={query}
                  onValueChange={setQuery}
                />
              </div>
            </ActionBar>
            <Card variant="sunken">
              <span style={{ color: 'var(--color-content-muted)' }}>
                Showing “{filter}”{query ? ` matching “${query}”` : ''}
              </span>
            </Card>
          </div>
        </Section>
      </Page>
    )
  },
}

export const Tokens: Story = {
  render: () => (
    <Page title="Tokens">
      <Section title="PillTabs">
        <TokenTable
          tokens={componentTokens('pill-tabs')}
          labelOf={token => token.name.slice('pill-tabs-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
