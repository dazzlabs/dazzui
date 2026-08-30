import { Checkbox, RadioGroup, TINTS, Toggle } from '@dazzlabs/dazzui'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta = {
  title: 'Components/Selection',
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

function CheckboxDemo() {
  const [parts, setParts] = useState({ text: true, notes: true, images: false })

  return (
    <Stack>
      <Checkbox
        checked={parts.text}
        note="392 segments"
        onCheckedChange={text => setParts({ ...parts, text })}
      >
        Translated text
      </Checkbox>
      <Checkbox
        checked={parts.notes}
        note="24 comments"
        onCheckedChange={notes => setParts({ ...parts, notes })}
      >
        Reviewer notes
      </Checkbox>
      <Checkbox
        checked={parts.images}
        note="+2.1 MB"
        onCheckedChange={images => setParts({ ...parts, images })}
      >
        Embedded images
      </Checkbox>
      <Checkbox checked={false} disabled>
        Revision history
      </Checkbox>
    </Stack>
  )
}

function RadioDemo() {
  const [quality, setQuality] = useState('balanced')

  return (
    <RadioGroup
      aria-label="Translation quality"
      value={quality}
      onValueChange={setQuality}
      options={[
        { value: 'fast', label: 'Fast — one pass, no review' },
        { value: 'balanced', label: 'Balanced — a second pass on low scores' },
        { value: 'careful', label: 'Careful — every segment reviewed' },
        {
          value: 'custom',
          label: 'Custom (needs a paid plan)',
          disabled: true,
        },
      ]}
    />
  )
}

function ToggleRow({ size }: { size: (typeof SIZES)[number] }) {
  const [on, setOn] = useState(true)
  return (
    <Specimen label={size}>
      <Toggle
        aria-label={`Enabled (${size})`}
        checked={on}
        size={size}
        onCheckedChange={setOn}
      />
      <Toggle aria-label="Off" checked={false} size={size} />
      <Toggle aria-label="Disabled" checked disabled size={size} />
    </Specimen>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="Checkbox, Radio & Toggle"
      description="The three ways of setting a choice. Each is a box plus a label on a row whose height is the hit target, which is why the square has its own token alongside the control height it sits in."
    >
      <Section
        title="Checkbox"
        description="A filled square when set, a neutral outline when not — an empty box is an outline, not a quiet accent."
      >
        <CheckboxDemo />
      </Section>

      <Section
        title="Radio"
        description="A ring that keeps its outline when selected and grows a dot inside it, where a checkbox fills solid."
      >
        <RadioDemo />
      </Section>

      <Section title="Toggle">
        <Stack>
          {SIZES.map(size => (
            <ToggleRow key={size} size={size} />
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="All three read the filled recipe for their set state, so a ramp change moves them together."
      >
        <Stack>
          <Specimen label="checkbox">
            {TINTS.map(tint => (
              <Checkbox key={tint} checked tint={tint}>
                {tint}
              </Checkbox>
            ))}
          </Specimen>
          <Specimen label="toggle">
            {TINTS.map(tint => (
              <Toggle key={tint} aria-label={tint} checked tint={tint} />
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section title="Checkbox tokens">
        <TokenTable
          tokens={componentTokens('checkbox')}
          labelOf={token => token.name.slice('checkbox-'.length)}
          nameHeader="Part"
        />
      </Section>

      <Section title="Toggle tokens">
        <TokenTable
          tokens={componentTokens('toggle')}
          labelOf={token => token.name.slice('toggle-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
