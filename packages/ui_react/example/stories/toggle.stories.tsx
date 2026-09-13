import {
  TextBold24Regular,
  TextItalic24Regular,
  TextUnderline24Regular,
} from '@fluentui/react-icons'
import { SegmentedControl, Switch, Toggle, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col, Row } from '../src/components/stage'

const meta: Meta<typeof Toggle> = {
  title: 'Components/Toggle',
  component: Toggle,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['tiny', 'small', 'medium', 'large'] as const
const VARIANTS = [
  'recessed',
  'normal',
  'filled',
  'tinted',
  'outlined',
  'plain',
] as const

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

export const Pressed: Story = {
  render: () => (
    <Row>
      <Toggle>Wrap</Toggle>
      <Toggle defaultPressed>Wrap</Toggle>
    </Row>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Row>
      {SIZES.map(size => (
        <Toggle key={size} defaultPressed size={size} variant="recessed">
          {size}
        </Toggle>
      ))}
    </Row>
  ),
}

export const Variants: Story = {
  render: () => (
    <Col>
      <Row>
        {VARIANTS.map(variant => (
          <Toggle key={variant} variant={variant}>
            {variant}
          </Toggle>
        ))}
      </Row>
      <Row>
        {VARIANTS.map(variant => (
          <Toggle key={variant} defaultPressed variant={variant}>
            {variant}
          </Toggle>
        ))}
      </Row>
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Row>
      {TINTS.map(tint => (
        <Toggle key={tint} defaultPressed tint={tint}>
          {tint}
        </Toggle>
      ))}
    </Row>
  ),
}

export const Disabled: Story = {
  render: () => (
    <Row>
      <Toggle disabled>Wrap</Toggle>
      <Toggle disabled defaultPressed>
        Wrap
      </Toggle>
    </Row>
  ),
}

export const AToolbar: Story = {
  render: () => (
    <Row gap="var(--spacing-1)">
      <Bold />
      <Toggle label="Italic">
        <TextItalic24Regular />
      </Toggle>
      <Toggle label="Underline">
        <TextUnderline24Regular />
      </Toggle>
    </Row>
  ),
}
