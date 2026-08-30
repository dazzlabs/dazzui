import { PillTabs } from '@dazzlabs/dazzui-react'
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

function Filters({ size }: { size?: 'small' | 'medium' | 'large' }) {
  const [filter, setFilter] = useState('all')
  return (
    <PillTabs
      aria-label="History filter"
      items={FILTERS}
      size={size}
      value={filter}
      onValueChange={setFilter}
    />
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="PillTabs"
      description="Free-standing filter pills. Unlike a segmented control there is no track holding them together, so each pill needs its own fill to be a target at all — which also means they can wrap, and a filter row usually has to."
    >
      <Section
        title="Filtering"
        description="A count rides inside the pill, dimmed against the label it follows rather than set beside it as a badge."
      >
        <Filters />
      </Section>

      <Section title="Sizes">
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Specimen key={size} label={size}>
              <Filters size={size} />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('pill-tabs')}
          labelOf={token => token.name.slice('pill-tabs-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
