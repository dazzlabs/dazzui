import { Spinner, TINTS } from '@dazzlabs/dazzui'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Spinner> = {
  title: 'Components/Spinner',
  component: Spinner,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="Spinner"
      description="A ring with one segment picked out: the track sits low on the ramp, the head high. Under reduced motion it slows rather than stops — a still spinner reads as a frozen app, which is the opposite of what it is there to say."
    >
      <Section
        title="Sizes"
        description="The ring thickness is shared across sizes; only the diameter steps."
      >
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Specimen key={size} label={size}>
              <Spinner size={size} />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section title="Tints">
        <Stack>
          <Specimen label="tinted">
            {TINTS.map(tint => (
              <Spinner key={tint} tint={tint} />
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('spinner')}
          labelOf={token => token.name.slice('spinner-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
