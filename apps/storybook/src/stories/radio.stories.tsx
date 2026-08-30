import { Radio, RadioGroup, TINTS } from '@dazzlabs/dazzui-react'
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

const meta: Meta<typeof Radio> = {
  title: 'Components/Radio',
  component: Radio,
}

export default meta
type Story = StoryObj<typeof meta>

function QualityPicker() {
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

export const Overview: Story = {
  render: () => (
    <Page
      title="Radio"
      description="One choice out of a set. A ring that keeps its outline when selected and grows a dot inside it, where a checkbox fills solid — the shapes differ because the choices do: a checkbox is on or off on its own, a radio is one of several."
    >
      <Section
        title="A group"
        description="`RadioGroup` takes the options and owns the selection, so a caller never wires the rows to each other."
      >
        <QualityPicker />
      </Section>

      <Section title="Sizes">
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Specimen key={size} label={size}>
              <Radio checked size={size}>
                Selected
              </Radio>
              <Radio checked={false} size={size}>
                Not selected
              </Radio>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section title="Tints">
        <Stack>
          <Specimen label="tinted">
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
