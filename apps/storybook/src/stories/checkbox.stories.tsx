import { Checkbox, TINTS } from '@dazzlabs/dazzui-react'
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

const meta: Meta<typeof Checkbox> = {
  title: 'Components/Checkbox',
  component: Checkbox,
}

export default meta
type Story = StoryObj<typeof meta>

function Checklist() {
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

export const Overview: Story = {
  render: () => (
    <Page
      title="Checkbox"
      description="A box and a label on one row. The row's height is the hit target — the shared control height — while the box is its own much smaller square, which is why `box` sits alongside `size` in the tokens."
    >
      <Section
        title="A checklist"
        description="A filled square when set, a neutral outline when not: an empty box is an outline, not a quiet accent."
      >
        <Checklist />
      </Section>

      <Section
        title="Sizes"
        description="The square and the row grow together, so a list of them keeps one rhythm whatever size it is set at."
      >
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Specimen key={size} label={size}>
              <Checkbox checked size={size}>
                Set
              </Checkbox>
              <Checkbox checked={false} size={size}>
                Unset
              </Checkbox>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="The set state is the filled recipe, so it moves with the ramp."
      >
        <Stack>
          <Specimen label="tinted">
            {TINTS.map(tint => (
              <Checkbox key={tint} checked tint={tint}>
                {tint}
              </Checkbox>
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('checkbox')}
          labelOf={token => token.name.slice('checkbox-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
