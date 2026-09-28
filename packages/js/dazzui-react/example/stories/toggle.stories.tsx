import { defineStory } from '../src/lib/scene'
import scenes from './toggle.scene.json'
import {
  TextBold24Regular,
  TextItalic24Regular,
  TextUnderline24Regular,
} from '@fluentui/react-icons'
import { SegmentedControl, Switch, Toggle } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Row } from '../src/components/stage'

const meta: Meta<typeof Toggle> = {
  title: 'Components/Toggle',
  component: Toggle,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

function Bold() {
  const [on, setOn] = useState(true)
  return (
    <Toggle label="Bold" pressed={on} onPressedChange={setOn}>
      <TextBold24Regular />
    </Toggle>
  )
}

export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Toggle"
      description="A button that stays down. Three components in this package are one press away from each other: a Switch commits immediately and reads as a setting; a SegmentedControl is a choice between its segments and always has one; a Toggle is neither — a single property that is on or off, in a row where none of them may be on."
    >
      <Section
        title="Beside its neighbours"
        description="Reach for a Switch where the thing is a setting in a list and the change takes effect on the spot. Reach for a SegmentedControl where the reader is choosing one of several alternatives and cannot choose none. Reach for this where the answer is yes or no and the control lives in a toolbar rather than a form."
      >
        <Stack>
          <Specimen label="Switch">
            <Switch defaultChecked />
          </Specimen>
          <Specimen label="SegmentedControl">
            <SegmentedControl
              aria-label="Align"
              items={[
                { value: 'left', label: 'Left' },
                { value: 'center', label: 'Center' },
              ]}
              value="left"
            />
          </Specimen>
          <Specimen label="Toggle">
            <Bold />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Pressed"
        description="Drawn the way the system draws every selection: the tinted wash, and the label a weight heavier. Colour alone would be one grade in a toolbar of glyphs, which is exactly where it is hardest to see — and the weight is the half that survives for a reader who cannot tell the two grades apart."
      >
        <Stack>
          <Specimen label="up">
            <Toggle>Wrap</Toggle>
          </Specimen>
          <Specimen label="down">
            <Toggle defaultPressed>Wrap</Toggle>
          </Specimen>
          <Specimen label="disabled, down">
            <Toggle disabled defaultPressed>
              Wrap
            </Toggle>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Glyphs"
        description="Most toggles live in toolbars and most toolbar labels are glyphs, so `label` sets the accessible name and raises the tooltip in one move — the same thing IconButton does, and for the same reason: the native title attribute is late, unstyled and absent from every keyboard."
      >
        <Row gap="var(--spacing-1)">
          <Bold />
          <Toggle label="Italic">
            <TextItalic24Regular />
          </Toggle>
          <Toggle label="Underline">
            <TextUnderline24Regular />
          </Toggle>
        </Row>
      </Section>
    </Page>
  ),
}

export const Pressed: Story = defineStory(scenes.Pressed)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Variants: Story = defineStory(scenes.Variants)

export const Tints: Story = defineStory(scenes.Tints)

export const Disabled: Story = defineStory(scenes.Disabled)

export const AToolbar: Story = defineStory(scenes.AToolbar)
