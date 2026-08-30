import {
  Badge,
  Button,
  Divider,
  PreferenceGroup,
  PreferenceRow,
  PreferenceSection,
  ShortcutRecorder,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof PreferenceSection> = {
  title: 'Components/Preference',
  component: PreferenceSection,
}

export default meta
type Story = StoryObj<typeof meta>

function Shortcuts() {
  const [translate, setTranslate] = useState('⌥⇧2')
  const [capture, setCapture] = useState('')

  return (
    <PreferenceSection
      label="Shortcuts"
      footer="A shortcut already taken by another app will not fire here."
    >
      <PreferenceRow title="Translate selection">
        <ShortcutRecorder value={translate} onValueChange={setTranslate} />
      </PreferenceRow>
      <PreferenceRow title="Capture and translate">
        <ShortcutRecorder value={capture} onValueChange={setCapture} />
      </PreferenceRow>
    </PreferenceSection>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="Preference"
      description="The settings column. The spacing is the whole design: rows sit close together, a heading sits further from the section above it than from its own rows, and a group title stands further still — so a heading always reads as belonging to what follows it."
    >
      <Section
        title="A section"
        description="Rows carry whatever control they are for. A row with `onOpen` becomes a button and grows the hover wash that says so; the rest stay inert."
      >
        <div style={{ maxWidth: '30rem' }}>
          <PreferenceSection label="General">
            <PreferenceRow
              title="Launch at login"
              subtitle="Starts minimised in the menu bar"
            >
              <Badge tint="success">On</Badge>
            </PreferenceRow>
            <PreferenceRow title="Check for updates">
              <Badge tint="neutral" variant="outlined">
                Weekly
              </Badge>
            </PreferenceRow>
            <PreferenceRow
              title="Storage location"
              subtitle="~/Library/Application Support"
              onOpen={() => {}}
            />
          </PreferenceSection>
        </div>
      </Section>

      <Section
        title="A group"
        description="Sections stay one size everywhere; the level above them is a title that outranks them, not a section shrunk to fit underneath one."
      >
        <div style={{ maxWidth: '30rem' }}>
          <PreferenceGroup
            title="Translation"
            description="How segments are sent, and what comes back."
            action={<Button size="small">Reset</Button>}
          >
            <PreferenceSection label="Providers">
              <PreferenceRow title="Primary" subtitle="DeepL">
                <Badge tint="success">Connected</Badge>
              </PreferenceRow>
              <PreferenceRow title="Fallback" subtitle="Google Translate">
                <Badge tint="neutral">Idle</Badge>
              </PreferenceRow>
            </PreferenceSection>
            <Divider />
            <Shortcuts />
          </PreferenceGroup>
        </div>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('preference')}
          labelOf={token => token.name.slice('preference-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
