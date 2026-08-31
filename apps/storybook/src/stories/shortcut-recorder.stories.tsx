import {
  Button,
  Kbd,
  PreferenceRow,
  PreferenceSection,
  ShortcutRecorder,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { Col, Row } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof ShortcutRecorder> = {
  title: 'Components/ShortcutRecorder',
  component: ShortcutRecorder,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

function Live({ initial = '⌥⇧2' }: { initial?: string }) {
  const [value, setValue] = useState(initial)
  return <ShortcutRecorder value={value} onValueChange={setValue} />
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="ShortcutRecorder"
      description="Click it and it listens; press a combination and it keeps it. Escape backs out, ⌫ clears, clicking away cancels. Held modifiers print live while it listens, so pressing ⌥⇧ and then wondering which key to add is not done blind."
    >
      <Section
        title="Recording"
        description="Try it: a bare letter is rejected with a flash, because a key on its own is typing rather than a shortcut. It has to carry ⌃, ⌥ or ⌘ — ⇧ alone is still typing — unless it is a function key, which is a shortcut by itself. Word keys get a space after the modifiers, so `⌥ Space` reads as two things and `⌥⇧2` as one."
      >
        <Stack>
          <Specimen label="bound">
            <Live />
          </Specimen>
          <Specimen label="unbound">
            <Live initial="" />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="States"
        description="It is drawn as a field, not a key cap: a Kbd says what the key is, a field says you can change it. The clear button appears only once there is something bound and the field is not listening. `error` is for a binding the page cannot accept — the recorder does not know what else is bound, so that comparison is the page's business."
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
          <Specimen label="versus a cap">
            <span style={{ display: 'inline-flex', gap: 'var(--spacing-1)' }}>
              <Kbd size="small">⌘</Kbd>
              <Kbd size="small">K</Kbd>
            </span>
            <ShortcutRecorder value="⌘K" onValueChange={() => {}} />
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

export const Bound: Story = {
  render: () => <Live />,
}

export const Unbound: Story = {
  render: () => <Live initial="" />,
}

export const ErrorState: Story = {
  name: 'Error',
  render: () => (
    <ShortcutRecorder value="⌘C" state="error" onValueChange={() => {}} />
  ),
}

export const Disabled: Story = {
  render: () => (
    <ShortcutRecorder value="⌥⇧2" disabled onValueChange={() => {}} />
  ),
}

export const States: Story = {
  render: () => (
    <Col>
      <ShortcutRecorder value="" onValueChange={() => {}} />
      <ShortcutRecorder value="⌥ Space" onValueChange={() => {}} />
      <ShortcutRecorder value="⌘C" state="error" onValueChange={() => {}} />
      <ShortcutRecorder value="⌥⇧2" disabled onValueChange={() => {}} />
    </Col>
  ),
}

export const Modified: Story = {
  render: () => (
    <Row>
      <ShortcutRecorder value="⌥⇧2" onValueChange={() => {}} />
      <ShortcutRecorder value="⌘⇧K" onValueChange={() => {}} />
      <ShortcutRecorder value="⌃⌥T" onValueChange={() => {}} />
    </Row>
  ),
}

export const WordKeys: Story = {
  render: () => (
    <Row>
      <ShortcutRecorder value="⌥ Space" onValueChange={() => {}} />
      <ShortcutRecorder value="⌘ ↩" onValueChange={() => {}} />
      <ShortcutRecorder value="⌥ ⌫" onValueChange={() => {}} />
    </Row>
  ),
}

export const FunctionKeys: Story = {
  render: () => (
    <Row>
      <ShortcutRecorder value="F5" onValueChange={() => {}} />
      <ShortcutRecorder value="⇧F12" onValueChange={() => {}} />
    </Row>
  ),
}

export const AgainstAKbd: Story = {
  render: () => (
    <Row>
      <span style={{ display: 'inline-flex', gap: 'var(--spacing-1)' }}>
        <Kbd size="small">⌘</Kbd>
        <Kbd size="small">K</Kbd>
      </span>
      <ShortcutRecorder value="⌘K" onValueChange={() => {}} />
    </Row>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      <Row>
        {TINTS.map(tint => (
          <ShortcutRecorder
            key={tint}
            tint={tint}
            value="⌥⇧2"
            onValueChange={() => {}}
          />
        ))}
      </Row>
      <Row>
        {TINTS.map(tint => (
          <ShortcutRecorder
            key={tint}
            tint={tint}
            value=""
            onValueChange={() => {}}
          />
        ))}
      </Row>
    </Col>
  ),
}

export const Conflicts: Story = {
  render: () => {
    const [translate, setTranslate] = useState('⌥⇧2')
    const [capture, setCapture] = useState('⌥⇧3')
    const clash = translate !== '' && translate === capture

    return (
      <div style={{ width: '30rem' }}>
        <PreferenceSection
          label="Shortcuts"
          footer={
            clash
              ? 'Two actions cannot share one shortcut.'
              : 'Give both rows the same combination to see the error state.'
          }
        >
          <PreferenceRow title="Translate selection">
            <ShortcutRecorder value={translate} onValueChange={setTranslate} />
          </PreferenceRow>
          <PreferenceRow title="Capture and translate">
            <ShortcutRecorder
              state={clash ? 'error' : 'default'}
              value={capture}
              onValueChange={setCapture}
            />
          </PreferenceRow>
        </PreferenceSection>
      </div>
    )
  },
}

export const InSettings: Story = {
  render: () => {
    const [translate, setTranslate] = useState('⌥⇧2')
    const [capture, setCapture] = useState('')
    const [palette, setPalette] = useState('⌘K')

    return (
      <div style={{ width: '30rem' }}>
        <PreferenceSection
          label="Shortcuts"
          action={
            <Button
              size="small"
              onClick={() => {
                setTranslate('⌥⇧2')
                setCapture('')
                setPalette('⌘K')
              }}
            >
              Reset all
            </Button>
          }
          footer="A shortcut already taken by another app will not fire here."
        >
          <PreferenceRow
            title="Translate selection"
            subtitle="Works in any app"
          >
            <ShortcutRecorder value={translate} onValueChange={setTranslate} />
          </PreferenceRow>
          <PreferenceRow title="Capture and translate">
            <ShortcutRecorder value={capture} onValueChange={setCapture} />
          </PreferenceRow>
          <PreferenceRow title="Command palette">
            <ShortcutRecorder value={palette} onValueChange={setPalette} />
          </PreferenceRow>
        </PreferenceSection>
      </div>
    )
  },
}
