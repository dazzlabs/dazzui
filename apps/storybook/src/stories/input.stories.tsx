import { Input } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Input> = {
  title: 'Components/Input',
  component: Input,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="Input"
      description="A single line of text. It shares its box and its height with Select, Textarea and the buttons beside it, so a form row never steps. Wrap it in a `FormField` to give it a label and a hint."
    >
      <Section
        title="States"
        description="Focus one: the border and the ring both come from the tint, which is why re-tinting is all an error state has to do."
      >
        <div style={{ maxWidth: '22rem' }}>
          <Stack>
            <Input defaultValue="Jianying Li" />
            <Input placeholder="Placeholder" />
            <Input disabled defaultValue="Disabled" />
            <Input state="error" defaultValue="Rejected value" />
          </Stack>
        </div>
      </Section>

      <Section
        title="Mono"
        description="For a value that is a key, an ID or a URL — something read character by character rather than as words."
      >
        <div style={{ maxWidth: '22rem' }}>
          <Input mono defaultValue="https://api.deepl.com/v2" />
        </div>
      </Section>

      <Section title="Sizes">
        <div style={{ maxWidth: '22rem' }}>
          <Stack>
            {(['small', 'medium', 'large'] as const).map(size => (
              <Specimen key={size} label={size}>
                <Input placeholder={size} size={size} />
              </Specimen>
            ))}
          </Stack>
        </div>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('input')}
          labelOf={token => token.name.slice('input-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
