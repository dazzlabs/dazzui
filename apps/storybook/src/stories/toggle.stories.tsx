import { TINTS, Toggle } from '@dazzlabs/dazzui-react'
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

const meta: Meta<typeof Toggle> = {
  title: 'Components/Toggle',
  component: Toggle,
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

function Row({ size }: { size: (typeof SIZES)[number] }) {
  const [on, setOn] = useState(true)
  return (
    <Specimen label={size}>
      <Toggle
        aria-label={`Enabled (${size})`}
        checked={on}
        size={size}
        onCheckedChange={setOn}
      />
      <Toggle aria-label="Off" checked={false} size={size} />
      <Toggle aria-label="Disabled" checked disabled size={size} />
    </Specimen>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="Toggle"
      description="A setting that takes effect as you flip it, which is what separates it from a checkbox: a checkbox states an intention a form will act on later, a switch acts now. The track carries the state and the thumb slides to the side that says which."
    >
      <Section
        title="Sizes"
        description="The thumb keeps one colour in both states; only the track changes. A thumb that darkened when off would read as the heavier of the two halves."
      >
        <Stack>
          {SIZES.map(size => (
            <Row key={size} size={size} />
          ))}
        </Stack>
      </Section>

      <Section title="Tints">
        <Stack>
          <Specimen label="on">
            {TINTS.map(tint => (
              <Toggle key={tint} aria-label={tint} checked tint={tint} />
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('toggle')}
          labelOf={token => token.name.slice('toggle-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
