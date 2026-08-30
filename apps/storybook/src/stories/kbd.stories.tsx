import { Kbd } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Kbd> = {
  title: 'Components/Kbd',
  component: Kbd,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="Kbd"
      description="One key cap. Unlike a badge it carries its own surface, content, and border colors in the token graph, so it takes no tint."
    >
      <Section
        title="Sizes"
        description="A chord is several caps composed by the caller, which keeps the separator the caller's decision."
      >
        <Stack>
          <Specimen label="small">
            <Kbd size="small">⌘</Kbd>
            <Kbd size="small">K</Kbd>
          </Specimen>
          <Specimen label="medium">
            <Kbd size="medium">⌘</Kbd>
            <Kbd size="medium">K</Kbd>
          </Specimen>
          <Specimen label="large">
            <Kbd size="large">⌘</Kbd>
            <Kbd size="large">K</Kbd>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="In a row of text"
        description="The cap sits on the text baseline it is set in."
      >
        <Stack>
          <Specimen label="shortcut">
            <span>
              Open the command palette with <Kbd size="small">⌘</Kbd>{' '}
              <Kbd size="small">K</Kbd>
            </span>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Tokens"
        description="Colors are shared across sizes; type and metrics are per size."
      >
        <TokenTable
          tokens={componentTokens('kbd')}
          labelOf={token => token.name.slice('kbd-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
