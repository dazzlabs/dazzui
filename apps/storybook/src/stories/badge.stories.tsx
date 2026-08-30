import { Badge, type BadgeVariant, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Badge> = {
  title: 'Components/Badge',
  component: Badge,
}

export default meta
type Story = StoryObj<typeof meta>

const VARIANTS: BadgeVariant[] = ['filled', 'tinted', 'outlined']

export const Overview: Story = {
  render: () => (
    <Page
      title="Badge"
      description="A short label that names a state or a count. It is not interactive, so it wears only the resting state of a control recipe: pick the recipe with `variant`, pick the ramp it resolves against with `tint`."
    >
      <Section
        title="Variant and tint"
        description="Every combination the two axes allow. The recipe decides where on the ramp the surface and the content sit; the tint decides which ramp that is."
      >
        <Stack>
          {VARIANTS.map(variant => (
            <Specimen key={variant} label={variant}>
              {TINTS.map(tint => (
                <Badge key={tint} tint={tint} variant={variant}>
                  {tint}
                </Badge>
              ))}
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Sizes"
        description="Each size borrows the padding of the control size one step below it, so a badge reads as a label rather than as a control."
      >
        <Stack>
          <Specimen label="small">
            <Badge size="small">Beta</Badge>
            <Badge size="small" tint="success" variant="filled">
              Live
            </Badge>
          </Specimen>
          <Specimen label="medium">
            <Badge size="medium">Beta</Badge>
            <Badge size="medium" tint="success" variant="filled">
              Live
            </Badge>
          </Specimen>
          <Specimen label="large">
            <Badge size="large">Beta</Badge>
            <Badge size="large" tint="success" variant="filled">
              Live
            </Badge>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Tokens"
        description="Everything above is drawn from these; the component itself sets no dimension of its own."
      >
        <TokenTable
          tokens={componentTokens('badge')}
          labelOf={token => token.name.slice('badge-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
