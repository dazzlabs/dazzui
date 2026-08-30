import { Kbd, SearchField } from '@dazzlabs/dazzui-react'
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

const meta: Meta<typeof SearchField> = {
  title: 'Components/SearchField',
  component: SearchField,
}

export default meta
type Story = StoryObj<typeof meta>

function Search({
  size,
  hint,
  initial = '',
}: {
  size?: 'small' | 'medium' | 'large'
  hint?: boolean
  initial?: string
}) {
  const [query, setQuery] = useState(initial)
  return (
    <div style={{ width: '18rem' }}>
      <SearchField
        hint={hint ? <Kbd size="small">⌘F</Kbd> : undefined}
        placeholder="Search terms"
        size={size}
        value={query}
        onValueChange={setQuery}
      />
    </div>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="SearchField"
      description="Inline search for a list or a table. The box belongs to the wrapper rather than the input, because the glyph and the trailing slot sit inside it — which is also why the focus ring comes from `:focus-within`."
    >
      <Section
        title="The trailing slot"
        description="A hint sits there while the field is empty and gives way to the clear button once there is something to clear, so the two never compete for the same corner."
      >
        <Stack>
          <Specimen label="empty">
            <Search hint />
          </Specimen>
          <Specimen label="with a query">
            <Search hint initial="placeholder" />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Escape"
        description="Type something and press Escape: it clears first, and only dismisses once the field is already empty, so a stray press never closes a search mid-way."
      >
        <Search />
      </Section>

      <Section title="Sizes">
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Specimen key={size} label={size}>
              <Search size={size} />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('search-field')}
          labelOf={token => token.name.slice('search-field-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
