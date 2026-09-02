import {
  Checkbox,
  PreferenceRow,
  PreferenceSection,
  TINTS,
  Switch,
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

const meta: Meta<typeof Switch> = {
  title: 'Components/Switch',
  component: Switch,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Switch"
      description="A setting that takes effect as you flip it, which is what separates it from a checkbox: a checkbox states an intention a form will act on later, a switch acts now. The rule is when it takes effect, not how it looks — a switch inside a form with a Save button is the usual way this goes wrong. The track carries the state and the thumb slides to the side that says which."
    >
      <Section
        title="Sizes"
        description="The thumb keeps one colour in both states; only the track changes. A thumb that darkened when off would read as the heavier of the two halves, and the eye would go to the wrong side of the switch. Track and thumb are separate tokens per size rather than one derived from the other, because the gap between them has to stay a constant hairline."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Switch aria-label={`On (${size})`} checked size={size} />
              <Switch
                aria-label={`Off (${size})`}
                checked={false}
                size={size}
              />
              <Switch
                aria-label={`Disabled (${size})`}
                checked
                disabled
                size={size}
              />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="The on-track is the filled recipe, so it moves with the ramp; the off-track stays neutral on every tint, because off is the absence of the setting rather than a state of its own."
      >
        <Stack>
          <Specimen label="on">
            {TINTS.map(tint => (
              <Switch key={tint} aria-label={tint} checked tint={tint} />
            ))}
          </Specimen>
          <Specimen label="off">
            {TINTS.map(tint => (
              <Switch
                key={tint}
                aria-label={tint}
                checked={false}
                tint={tint}
              />
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Where it lands"
        description="The trailing slot of a preference row, one per setting, each taking effect as it is flipped. The row carries the label, so the switch itself needs only an accessible name."
      >
        <div style={{ maxWidth: '30rem' }}>
          <PreferenceSection label="General">
            <PreferenceRow
              title="Launch at login"
              subtitle="Starts minimised in the menu bar"
            >
              <Switch aria-label="Launch at login" checked />
            </PreferenceRow>
            <PreferenceRow title="Play a sound when a job finishes">
              <Switch aria-label="Play a sound" checked={false} />
            </PreferenceRow>
          </PreferenceSection>
        </div>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('switch')}
          labelOf={token => token.name.slice('switch-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const On: Story = {
  render: () => <Switch aria-label="On" checked />,
}

export const Off: Story = {
  render: () => <Switch aria-label="Off" checked={false} />,
}

export const Disabled: Story = {
  render: () => (
    <Row>
      <Switch aria-label="On, disabled" checked disabled />
      <Switch aria-label="Off, disabled" checked={false} disabled />
    </Row>
  ),
}

export const Interactive: Story = {
  render: () => {
    const [on, setOn] = useState(true)
    return (
      <Row>
        <Switch
          aria-label="Launch at login"
          checked={on}
          onCheckedChange={setOn}
        />
        <span style={{ color: 'var(--color-content-muted)' }}>
          {on ? 'On' : 'Off'}
        </span>
      </Row>
    )
  },
}

export const Sizes: Story = {
  render: () => (
    <Col>
      {SIZES.map(size => (
        <Row key={size}>
          <Switch aria-label={`On (${size})`} checked size={size} />
          <Switch aria-label={`Off (${size})`} checked={false} size={size} />
        </Row>
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      <Row>
        {TINTS.map(tint => (
          <Switch key={tint} aria-label={tint} checked tint={tint} />
        ))}
      </Row>
      <Row>
        {TINTS.map(tint => (
          <Switch key={tint} aria-label={tint} checked={false} tint={tint} />
        ))}
      </Row>
    </Col>
  ),
}

export const AgainstACheckbox: Story = {
  render: () => {
    const [live, setLive] = useState(true)
    const [pending, setPending] = useState(true)

    return (
      <Col gap="var(--spacing-4)">
        <Row>
          <Switch
            aria-label="Dark mode"
            checked={live}
            onCheckedChange={setLive}
          />
          <span style={{ color: 'var(--color-content-muted)' }}>
            Applied immediately
          </span>
        </Row>
        <Row>
          <Checkbox checked={pending} onCheckedChange={setPending}>
            Include reviewer notes
          </Checkbox>
          <span style={{ color: 'var(--color-content-muted)' }}>
            Applied when the export runs
          </span>
        </Row>
      </Col>
    )
  },
}

export const InSettings: Story = {
  render: () => {
    const [settings, setSettings] = useState({
      launch: true,
      sounds: false,
      beta: false,
      telemetry: true,
    })
    const set = (key: keyof typeof settings) => (value: boolean) =>
      setSettings(current => ({ ...current, [key]: value }))

    return (
      <div style={{ width: '30rem' }}>
        <PreferenceSection label="General">
          <PreferenceRow
            title="Launch at login"
            subtitle="Starts minimised in the menu bar"
          >
            <Switch
              aria-label="Launch at login"
              checked={settings.launch}
              onCheckedChange={set('launch')}
            />
          </PreferenceRow>
          <PreferenceRow title="Play a sound when a job finishes">
            <Switch
              aria-label="Play a sound when a job finishes"
              checked={settings.sounds}
              onCheckedChange={set('sounds')}
            />
          </PreferenceRow>
          <PreferenceRow
            title="Join the beta channel"
            subtitle="Updates arrive earlier and break more often"
          >
            <Switch
              aria-label="Join the beta channel"
              checked={settings.beta}
              onCheckedChange={set('beta')}
            />
          </PreferenceRow>
          <PreferenceRow
            title="Send crash reports"
            subtitle="Cannot be changed on a managed device"
          >
            <Switch
              aria-label="Send crash reports"
              checked={settings.telemetry}
              disabled
            />
          </PreferenceRow>
        </PreferenceSection>
      </div>
    )
  },
}
