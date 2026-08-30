import { Card, type CardVariant, SectionLabel, TINTS } from '@dazzlabs/dazzui'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Grid,
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Card> = {
  title: 'Components/Card',
  component: Card,
}

export default meta
type Story = StoryObj<typeof meta>

const VARIANTS: CardVariant[] = ['raised', 'sunken', 'outlined', 'tinted']

export const Overview: Story = {
  render: () => (
    <Page
      title="Card"
      description="The filled or bordered box the panels in a layout are built from. It is a container rather than a control, so its padding is uniform and comes off the spacing scale instead of the control scale."
    >
      <Section
        title="Variants"
        description="Only `tinted` reads the tint; the rest resolve against the neutral surface and border roles."
      >
        <Grid min="14rem">
          {VARIANTS.map(variant => (
            <Card key={variant} variant={variant}>
              <SectionLabel>{variant}</SectionLabel>
              <span>Padding, gap, and radius all come from the size.</span>
            </Card>
          ))}
        </Grid>
      </Section>

      <Section
        title="Tints"
        description="The tinted variant across every ramp: a 50 fill under a 200 border."
      >
        <Grid min="10rem">
          {TINTS.map(tint => (
            <Card key={tint} size="small" tint={tint} variant="tinted">
              <SectionLabel tint={tint}>{tint}</SectionLabel>
            </Card>
          ))}
        </Grid>
      </Section>

      <Section title="Sizes">
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Specimen key={size} label={size}>
              <Card size={size} style={{ width: '16rem' }}>
                <SectionLabel>Quality</SectionLabel>
                <span>Nine of twelve checks passed.</span>
              </Card>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('card')}
          labelOf={token => token.name.slice('card-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
