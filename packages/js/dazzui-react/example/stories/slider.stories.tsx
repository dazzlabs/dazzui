import { defineStory } from '../src/lib/scene'
import scenes from './slider.scene.json'
import { Slider } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../src/components/gallery'
import { Col } from '../src/components/stage'
import { componentTokens } from '../src/lib/tokens'

const meta: Meta<typeof Slider> = {
  title: 'Components/Slider',
  component: Slider,
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
      title="Slider"
      description="A value picked along a range. Base UI owns everything that makes one usable rather than merely draggable — the arrow keys and their large step, the press on the track that jumps the nearest thumb, the hidden inputs a form reads, and, on a range, which thumb the pointer actually grabbed when the two are on top of each other. Reach for it where the reader is feeling for a value rather than stating one; a number they know is a NumberField."
    >
      <Section
        title="Sizes"
        description="The row's height comes off the control profile, so a slider in a form sits on the same rhythm as the fields around it. The groove and the thumb are the component's own tokens, and both are even on every size for the reason the radio's ring and dot are: the thumb is centred on the groove, and an odd difference lands the centre on a half pixel."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <div style={{ width: '18rem' }}>
                <Slider defaultValue={40} size={size} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Range"
        description="A range is the same component: pass an array and it draws a thumb per value. There is no second component, because everything but the number of thumbs is identical — and the collision behaviour, which is the part a hand-rolled range never finishes, is Base UI's."
      >
        <Col width="20rem">
          <Slider defaultValue={30} showValue />
          <Slider defaultValue={[20, 70]} showValue />
        </Col>
      </Section>

      <Section
        title="Steps and the printed value"
        description="The printed value takes the mono face and tabular figures, so the row does not reflow as the digits change under the pointer. `format` gives it a unit; `formatValue` replaces it outright for a unit `Intl` has no name for."
      >
        <Col width="20rem">
          <Slider
            defaultValue={50}
            step={10}
            showValue
            format={{ style: 'percent' }}
          />
          <Slider
            defaultValue={13}
            min={8}
            max={24}
            showValue
            tint="info"
            formatValue={values => `${values[0]}px`}
          />
        </Col>
      </Section>

      <Section
        title="Disabled"
        description="It flattens to the groove like every other control — no lift, no accent, one dead look — rather than becoming a translucent ghost of the live one."
      >
        <div style={{ width: '18rem' }}>
          <Slider defaultValue={40} disabled showValue />
        </div>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('slider')}
          labelOf={token => token.name.slice('slider-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const Bare: Story = defineStory(scenes.Bare)

export const WithValue: Story = defineStory(scenes.WithValue)

export const Range: Story = defineStory(scenes.Range)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Tints: Story = defineStory(scenes.Tints)

export const Disabled: Story = defineStory(scenes.Disabled)

export const Interactive: Story = defineStory(scenes.Interactive)

export const InSettings: Story = defineStory(scenes.InSettings)
