import { SectionLabel, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof SectionLabel> = {
  title: 'Components/SectionLabel',
  component: SectionLabel,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="SectionLabel"
      description="The micro-heading that organises a pane. Sentence case, no uppercasing and no added tracking — one size covers sidebars, rails, and pane headings, so tone is the only axis worth varying."
    >
      <Section
        title="Default"
        description="Untinted, it takes the muted content color."
      >
        <Stack>
          <Specimen label="default">
            <SectionLabel>Quality signals</SectionLabel>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="A tint moves the label onto a ramp, for a heading that has to carry a state with it."
      >
        <Stack>
          <Specimen label="tinted">
            {TINTS.map(tint => (
              <SectionLabel key={tint} tint={tint}>
                {tint}
              </SectionLabel>
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('section-label')}
          labelOf={token => token.name.slice('section-label-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
