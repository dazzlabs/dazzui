import {
  ActionBar,
  Card,
  PillTabs,
  SearchField,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof PillTabs> = {
  title: 'Components/PillTabs',
  component: PillTabs,
  parameters: { layout: 'centered' },
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

const LANGUAGES = [
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

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="PillTabs"
      description="Free-standing filter pills. Unlike a segmented control there is no track holding them together, so each pill needs its own fill to be a target at all — which also means they can wrap, and a filter row usually has to. That is the reason to reach for them over a segmented control: a filter row grows with the data."
    >
      <Section
        title="Filtering"
        description="A count rides inside the pill, dimmed against the label it follows rather than set beside it as a badge — a badge would be a second thing to read, and the number here is an attribute of the filter rather than a state of its own. A filter with nothing behind it stays in the row rather than disappearing, so the set does not reflow every time a permission changes."
      >
        <PillTabs aria-label="History filter" items={FILTERS} value="all" />
      </Section>

      <Section
        title="Tints"
        description="The selected pill is the filled recipe, so the ramp is what picks it out. A filter row keeps one tint: the colour means “this one is on”, and varying it per pill would make it mean something about the pill."
      >
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

      <Section
        title="Wrapping"
        description="Nine of them in a narrow column. A track that had to hold this many segments would run off the pane."
      >
        <div style={{ maxWidth: '22rem' }}>
          <PillTabs aria-label="Target language" items={LANGUAGES} value="zh" />
        </div>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => {
    const [filter, setFilter] = useState('all')
    return (
      <PillTabs
        aria-label="History filter"
        items={PLAIN}
        value={filter}
        onValueChange={setFilter}
      />
    )
  },
}

export const WithCounts: Story = {
  render: () => {
    const [filter, setFilter] = useState('needs-review')
    return (
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
    )
  },
}

export const Disabled: Story = {
  render: () => (
    <PillTabs aria-label="History filter" items={FILTERS} value="all" />
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      {TINTS.map(tint => (
        <PillTabs
          key={tint}
          aria-label={tint}
          items={PLAIN}
          tint={tint}
          value="open"
        />
      ))}
    </Col>
  ),
}

export const Wrapping: Story = {
  render: () => {
    const [language, setLanguage] = useState('zh')
    return (
      <div style={{ width: '22rem' }}>
        <PillTabs
          aria-label="Target language"
          items={LANGUAGES}
          value={language}
          onValueChange={setLanguage}
        />
      </div>
    )
  },
}

export const InAPaneHeader: Story = {
  render: () => {
    const [filter, setFilter] = useState('all')
    const [query, setQuery] = useState('')

    return (
      <Col width="38rem">
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
      </Col>
    )
  },
}
