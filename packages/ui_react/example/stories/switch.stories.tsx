import { defineStory } from '../src/lib/scene'
import scenes from './switch.scene.json'
import {
  PreferenceRow,
  PreferenceSection,
  TINTS,
  Switch,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../src/components/gallery'
import { componentTokens } from '../src/lib/tokens'

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

export const On: Story = defineStory(scenes.On)

export const Off: Story = defineStory(scenes.Off)

export const Disabled: Story = defineStory(scenes.Disabled)

export const Interactive: Story = defineStory(scenes.Interactive)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Tints: Story = defineStory(scenes.Tints)

export const AgainstACheckbox: Story = defineStory(scenes.AgainstACheckbox)

export const InSettings: Story = defineStory(scenes.InSettings)
