import { Textarea } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Stack, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Textarea> = {
  title: 'Components/Textarea',
  component: Textarea,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="Textarea"
      description="Text that runs to more than a line. It takes the container corner rather than the control corner: a theme that makes controls into pills would turn a box that grows into a lozenge, so the two radii are separate axes on purpose."
    >
      <Section
        title="States"
        description="It resizes vertically only. Letting it widen would break it out of the column it was laid out in."
      >
        <div style={{ maxWidth: '22rem' }}>
          <Stack>
            <Textarea defaultValue="Keep product names untranslated." />
            <Textarea placeholder="Notes for the reviewer" />
            <Textarea disabled defaultValue="Disabled" />
            <Textarea state="error" defaultValue="Rejected value" />
          </Stack>
        </div>
      </Section>

      <Section title="Mono">
        <div style={{ maxWidth: '22rem' }}>
          <Textarea
            mono
            defaultValue={'{\n  "sourceLang": "en",\n  "formality": "more"\n}'}
          />
        </div>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('textarea')}
          labelOf={token => token.name.slice('textarea-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
