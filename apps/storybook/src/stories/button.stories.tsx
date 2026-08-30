import { Button, type ButtonVariant, Kbd, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
}

export default meta
type Story = StoryObj<typeof meta>

const VARIANTS: ButtonVariant[] = [
  'normal',
  'filled',
  'tinted',
  'outlined',
  'plain',
]

export const Overview: Story = {
  render: () => (
    <Page
      title="Button"
      description="The five control recipes, each on any ramp. Hover, press, and disabled are not written here at all — `data-control` puts the recipe's own states on the element, so a button's stylesheet is the box and nothing else."
    >
      <Section
        title="Variants"
        description="`filled` is the one action a view points at; `normal` and `outlined` are its neighbours; `tinted` is a quiet chip; `plain` looks like text at rest but keeps the box, because its recipe washes on hover."
      >
        <Stack>
          {VARIANTS.map(variant => (
            <Specimen key={variant} label={variant}>
              <Button variant={variant}>Save changes</Button>
              <Button variant={variant} disabled>
                Disabled
              </Button>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="Hover any of these: the hover and press states come from the same recipe, so a danger button darkens the way a primary one does."
      >
        <Stack>
          <Specimen label="filled">
            {TINTS.map(tint => (
              <Button key={tint} tint={tint} variant="filled">
                {tint}
              </Button>
            ))}
          </Specimen>
          <Specimen label="tinted">
            {TINTS.map(tint => (
              <Button key={tint} tint={tint} variant="tinted">
                {tint}
              </Button>
            ))}
          </Specimen>
          <Specimen label="outlined">
            {TINTS.map(tint => (
              <Button key={tint} tint={tint} variant="outlined">
                {tint}
              </Button>
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section title="Sizes">
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Specimen key={size} label={size}>
              <Button size={size} variant="filled">
                Export
              </Button>
              <Button size={size}>Cancel</Button>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="With a shortcut"
        description="A trailing hint naming the key that also fires the action."
      >
        <Stack>
          <Specimen label="confirming">
            <Button variant="filled" shortcut="⌘S">
              Translate
            </Button>
            <Button shortcut="⌘⇧S">Translate all</Button>
            <span>
              A Kbd is the other way to print a key: <Kbd size="small">⌘F</Kbd>
            </span>
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('button')}
          labelOf={token => token.name.slice('button-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
