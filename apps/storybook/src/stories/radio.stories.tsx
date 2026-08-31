import {
  Card,
  Checkbox,
  Radio,
  RadioGroup,
  SectionLabel,
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

const meta: Meta<typeof Radio> = {
  title: 'Components/Radio',
  component: Radio,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

const QUALITY = [
  { value: 'fast', label: 'Fast — one pass, no review' },
  { value: 'balanced', label: 'Balanced — a second pass on low scores' },
  { value: 'careful', label: 'Careful — every segment reviewed' },
  { value: 'custom', label: 'Custom (needs a paid plan)', disabled: true },
]

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Radio"
      description="One choice out of a set. A ring that keeps its outline when selected and grows a dot inside it, where a checkbox fills solid — the shapes differ because the choices do: a checkbox is on or off on its own, a radio is one of several. Use a group even for two options where they are alternatives; two checkboxes let a reader pick both, or neither."
    >
      <Section
        title="A group"
        description="`RadioGroup` takes the options and owns the selection, so a caller never wires the rows to each other. The label reports the state alongside the ring: selected takes the full ink and label weight, unselected rests one step back."
      >
        <RadioGroup
          aria-label="Translation quality"
          options={QUALITY}
          value="balanced"
        />
      </Section>

      <Section
        title="Sizes"
        description="The ring, the dot and the row grow together. Both the ring and the dot are their own tokens: a dot derived from the ring by a percentage would go soft at the small end, where it is only a few pixels across."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Radio checked size={size}>
                Selected
              </Radio>
              <Radio checked={false} size={size}>
                Not selected
              </Radio>
              <Radio checked disabled size={size}>
                Disabled
              </Radio>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="The ring keeps its outline and the dot carries the ramp, so a tinted radio still reads as a ring with something in it rather than as a filled square."
      >
        <Stack>
          <Specimen label="selected">
            {TINTS.map(tint => (
              <Radio key={tint} checked tint={tint}>
                {tint}
              </Radio>
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('radio')}
          labelOf={token => token.name.slice('radio-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const Selected: Story = {
  render: () => <Radio checked>Balanced</Radio>,
}

export const Unselected: Story = {
  render: () => <Radio checked={false}>Balanced</Radio>,
}

export const Disabled: Story = {
  render: () => (
    <Col gap="var(--spacing-2)">
      <Radio checked disabled>
        Custom
      </Radio>
      <Radio checked={false} disabled>
        Custom
      </Radio>
    </Col>
  ),
}

export const AGroup: Story = {
  render: () => {
    const [quality, setQuality] = useState('balanced')
    return (
      <RadioGroup
        aria-label="Translation quality"
        options={QUALITY}
        value={quality}
        onValueChange={setQuality}
      />
    )
  },
}

export const Sizes: Story = {
  render: () => (
    <Col gap="var(--spacing-2)">
      {SIZES.map(size => (
        <Row key={size}>
          <Radio checked size={size}>
            Selected
          </Radio>
          <Radio checked={false} size={size}>
            Not selected
          </Radio>
        </Row>
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col gap="var(--spacing-2)">
      <Row>
        {TINTS.map(tint => (
          <Radio key={tint} checked tint={tint}>
            {tint}
          </Radio>
        ))}
      </Row>
      <Row>
        {TINTS.map(tint => (
          <Radio key={tint} checked={false} tint={tint}>
            {tint}
          </Radio>
        ))}
      </Row>
    </Col>
  ),
}

export const AgainstACheckbox: Story = {
  render: () => (
    <Row gap="var(--spacing-8)">
      <div>
        <SectionLabel>Quality</SectionLabel>
        <RadioGroup
          aria-label="Quality"
          options={[
            { value: 'fast', label: 'Fast' },
            { value: 'careful', label: 'Careful' },
          ]}
          value="fast"
        />
      </div>
      <div>
        <SectionLabel>Include</SectionLabel>
        <Col gap="var(--spacing-2)">
          <Checkbox checked>Reviewer notes</Checkbox>
          <Checkbox checked={false}>Embedded images</Checkbox>
        </Col>
      </div>
    </Row>
  ),
}

export const InAForm: Story = {
  render: () => {
    const [scope, setScope] = useState('project')
    return (
      <Card style={{ width: '24rem' }}>
        <SectionLabel>Apply this glossary to</SectionLabel>
        <RadioGroup
          aria-label="Scope"
          options={[
            { value: 'project', label: 'This project only' },
            { value: 'account', label: 'Every project on this account' },
            { value: 'org', label: 'The whole organisation', disabled: true },
          ]}
          value={scope}
          onValueChange={setScope}
        />
      </Card>
    )
  },
}
