import { ShortcutRecorder } from '@dazzlabs/dazzui-react'
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

const meta: Meta<typeof ShortcutRecorder> = {
  title: 'Components/ShortcutRecorder',
  component: ShortcutRecorder,
}

export default meta
type Story = StoryObj<typeof meta>

function Live() {
  const [value, setValue] = useState('⌥⇧2')
  return <ShortcutRecorder value={value} onValueChange={setValue} />
}

export const Overview: Story = {
  render: () => (
    <Page
      title="ShortcutRecorder"
      description="Click it and it listens; press a combination and it keeps it. Escape backs out, ⌫ clears, clicking away cancels. Held modifiers print live while it listens, so pressing ⌥⇧ and then wondering which key to add is not done blind."
    >
      <Section
        title="Recording"
        description="Try it: a bare letter is rejected with a flash, because a key on its own is typing rather than a shortcut. ⇧ alone does not qualify either; a function key does."
      >
        <Stack>
          <Specimen label="bound">
            <Live />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="States"
        description="It is drawn as a field, not a key cap: a Kbd says what the key is, a field says you can change it."
      >
        <Stack>
          <Specimen label="unbound">
            <ShortcutRecorder value="" onValueChange={() => {}} />
          </Specimen>
          <Specimen label="bound">
            <ShortcutRecorder value="⌥ Space" onValueChange={() => {}} />
          </Specimen>
          <Specimen label="error">
            <ShortcutRecorder
              value="⌘C"
              state="error"
              onValueChange={() => {}}
            />
          </Specimen>
          <Specimen label="disabled">
            <ShortcutRecorder value="⌥⇧2" disabled onValueChange={() => {}} />
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('shortcut-recorder')}
          labelOf={token => token.name.slice('shortcut-recorder-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
