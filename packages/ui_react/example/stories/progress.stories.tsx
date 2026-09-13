import { defineStory } from '../src/lib/scene'
import scenes from './progress.scene.json'
import { Progress, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../src/components/gallery'
import { componentTokens } from '../src/lib/tokens'

const meta: Meta<typeof Progress> = {
  title: 'Components/Progress',
  component: Progress,
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
      title="Progress"
      description="A neutral track with a tinted fill. The track is a token because it stays neutral whatever the fill is tinted with; the fill's width is data, so it is the one thing set inline. Width is also the only thing that animates, and it animates because the value changed — the component holds no timer of its own."
    >
      <Section
        title="Values"
        description="0 to 100, clamped. A value outside the range is a bug in the caller rather than a state to design for, so it is pulled back to the end of the track instead of overflowing it. Omit `value` when the work has no measurable end: the bar sweeps instead of filling, and reports no value to assistive tech."
      >
        <Stack>
          {[0, 12, 48, 76, 100].map(value => (
            <Specimen key={value} label={`${value}%`}>
              <div style={{ flex: 1 }}>
                <Progress value={value} />
              </div>
            </Specimen>
          ))}
          <Specimen label="no value">
            <div style={{ flex: 1 }}>
              <Progress />
            </div>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Sizes"
        description="Only the track's thickness and its radius change. `small` is the default, because a progress bar is usually reporting on something else on the page rather than being the page."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <div style={{ flex: 1 }}>
                <Progress size={size} value={64} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="The fill moves with the ramp; the track does not, because it is the same empty channel whatever is filling it. `danger` on a bar reads as a job in trouble rather than as a job about to end."
      >
        <Stack>
          {TINTS.map(tint => (
            <Specimen key={tint} label={tint}>
              <div style={{ flex: 1 }}>
                <Progress tint={tint} value={64} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('progress')}
          labelOf={token => token.name.slice('progress-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const Default: Story = defineStory(scenes.Default)

export const Values: Story = defineStory(scenes.Values)

export const Indeterminate: Story = defineStory(scenes.Indeterminate)

/** The document-progress fill: the theme's own two-stop accent ramp. */
export const Gradient: Story = defineStory(scenes.Gradient)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Tints: Story = defineStory(scenes.Tints)

export const Live: Story = defineStory(scenes.Live)

export const WithACount: Story = defineStory(scenes.WithACount)

export const BesideStages: Story = defineStory(scenes.BesideStages)
