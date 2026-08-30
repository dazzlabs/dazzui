import { Kbd, PillTabs, SearchField, SegmentedControl } from '@dazzlabs/dazzui'
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

const meta: Meta = {
  title: 'Components/Pickers',
}

export default meta
type Story = StoryObj<typeof meta>

const THEMES = [
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
  { value: 'system', label: 'System' },
] as const

function Segmented({ activeStyle }: { activeStyle: 'raised' | 'filled' }) {
  const [theme, setTheme] = useState<string>('system')
  return (
    <SegmentedControl
      aria-label="Appearance"
      activeStyle={activeStyle}
      items={THEMES}
      value={theme}
      onValueChange={setTheme}
    />
  )
}

function Pills() {
  const [filter, setFilter] = useState('all')
  return (
    <PillTabs
      aria-label="History filter"
      value={filter}
      onValueChange={setFilter}
      items={[
        { value: 'all', label: 'All history' },
        { value: 'starred', label: 'Starred', count: 64 },
        { value: 'edited', label: 'Edited by me', count: 18 },
        { value: 'shared', label: 'Shared', disabled: true },
      ]}
    />
  )
}

function Search() {
  const [query, setQuery] = useState('')
  return (
    <div style={{ width: '18rem' }}>
      <SearchField
        value={query}
        onValueChange={setQuery}
        placeholder="Search terms"
        hint={<Kbd size="small">⌘F</Kbd>}
      />
    </div>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="SegmentedControl, PillTabs & SearchField"
      description="Three ways of narrowing what a view shows: a recessed track for a choice between modes, free-standing pills for a filter, and a search box for everything else."
    >
      <Section
        title="SegmentedControl"
        description="`raised` lifts the active segment off the track, the way a desktop control draws it. `filled` paints it with the tint — louder, and worth reserving for a choice that has to shout."
      >
        <Stack>
          <Specimen label="raised">
            <Segmented activeStyle="raised" />
          </Specimen>
          <Specimen label="filled">
            <Segmented activeStyle="filled" />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="PillTabs"
        description="No track, so an unselected pill needs its own fill to be a target at all. A count rides inside the pill, dimmed against the label."
      >
        <Pills />
      </Section>

      <Section
        title="SearchField"
        description="The hint sits in the trailing slot while the field is empty and gives way to the clear button once there is something to clear. Escape clears first and dismisses only once it is already empty."
      >
        <Search />
      </Section>

      <Section title="SegmentedControl tokens">
        <TokenTable
          tokens={componentTokens('segmented-control')}
          labelOf={token => token.name.slice('segmented-control-'.length)}
          nameHeader="Part"
        />
      </Section>

      <Section title="PillTabs tokens">
        <TokenTable
          tokens={componentTokens('pill-tabs')}
          labelOf={token => token.name.slice('pill-tabs-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
