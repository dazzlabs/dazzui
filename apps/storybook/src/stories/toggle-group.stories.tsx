import {
  TextBold24Regular,
  TextItalic24Regular,
  TextUnderline24Regular,
} from '@fluentui/react-icons'
import { SegmentedControl, ToggleGroup, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof ToggleGroup> = {
  title: 'Components/ToggleGroup',
  component: ToggleGroup,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const STYLES = [
  { value: 'bold', label: <TextBold24Regular />, hint: 'Bold' },
  { value: 'italic', label: <TextItalic24Regular />, hint: 'Italic' },
  { value: 'underline', label: <TextUnderline24Regular />, hint: 'Underline' },
]

const SHOW = [
  { value: 'whitespace', label: 'Whitespace' },
  { value: 'minimap', label: 'Minimap' },
  { value: 'blame', label: 'Blame' },
]

function Styles() {
  const [on, setOn] = useState<string[]>(['bold'])
  return (
    <ToggleGroup
      aria-label="Text style"
      items={STYLES}
      value={on}
      onValueChange={setOn}
    />
  )
}

export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="ToggleGroup"
      description="Several toggles that belong together, joined into one control. Joined rather than merely adjacent: the corners are rounded at the ends of the run only and the shared edges collapse to one, which is what says the group is one thing with several states rather than three buttons that happen to touch."
    >
      <Section
        title="Beside a SegmentedControl"
        description="This is where the two part company. A segmented control is a choice between its segments and always has exactly one; a toggle group can have several on at once, or none. Drawing a mandatory single choice with this one would let the reader clear a choice that cannot be cleared."
      >
        <Stack>
          <Specimen label="SegmentedControl — one, always">
            <SegmentedControl
              aria-label="View"
              items={[
                { value: 'source', label: 'Source' },
                { value: 'preview', label: 'Preview' },
              ]}
              value="source"
            />
          </Specimen>
          <Specimen label="ToggleGroup — any number, including none">
            <Styles />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="One tab stop"
        description="Base UI owns the part that is easy to get wrong: the group is a single tab stop and the arrow keys move inside it, which is what keeps a toolbar of twelve toggles from being twelve stops on the way to the document."
      >
        <Styles />
      </Section>

      <Section
        title="Words"
        description="Nothing about the group is glyph-only. A run of labelled toggles is what a filter bar wants, and the joined edges are what say the three answers are about one question."
      >
        <ToggleGroup
          aria-label="Show"
          items={SHOW}
          value={['whitespace']}
          onValueChange={() => {}}
        />
      </Section>
    </Page>
  ),
}

export const Glyphs: Story = {
  render: () => <Styles />,
}

export const Words: Story = {
  render: () => {
    const [on, setOn] = useState<string[]>(['whitespace'])
    return (
      <ToggleGroup
        aria-label="Show"
        items={SHOW}
        value={on}
        onValueChange={setOn}
      />
    )
  },
}

export const Single: Story = {
  render: () => {
    const [on, setOn] = useState<string[]>(['minimap'])
    return (
      <ToggleGroup
        aria-label="Panel"
        items={SHOW}
        multiple={false}
        value={on}
        onValueChange={setOn}
      />
    )
  },
}

export const Variants: Story = {
  render: () => (
    <Col>
      {(['recessed', 'normal', 'outlined', 'plain'] as const).map(variant => (
        <ToggleGroup
          key={variant}
          aria-label={`Show (${variant})`}
          items={SHOW}
          value={['whitespace']}
          variant={variant}
          onValueChange={() => {}}
        />
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      {TINTS.map(tint => (
        <ToggleGroup
          key={tint}
          aria-label={`Show (${tint})`}
          items={SHOW}
          tint={tint}
          value={['whitespace']}
          onValueChange={() => {}}
        />
      ))}
    </Col>
  ),
}

export const Disabled: Story = {
  render: () => (
    <Row>
      <ToggleGroup
        aria-label="Show"
        items={[...SHOW.slice(0, 2), { ...SHOW[2]!, disabled: true }]}
        value={['whitespace']}
        onValueChange={() => {}}
      />
    </Row>
  ),
}
