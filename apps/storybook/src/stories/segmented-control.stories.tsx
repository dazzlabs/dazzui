import { SegmentedControl } from '@dazzlabs/dazzui-react'
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

const meta: Meta<typeof SegmentedControl> = {
  title: 'Components/SegmentedControl',
  component: SegmentedControl,
}

export default meta
type Story = StoryObj<typeof meta>

const THEMES = [
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
  { value: 'system', label: 'System' },
] as const

function Picker({
  activeStyle,
  stretch,
}: {
  activeStyle?: 'raised' | 'filled'
  stretch?: boolean
}) {
  const [theme, setTheme] = useState<string>('system')
  return (
    <SegmentedControl
      aria-label="Appearance"
      activeStyle={activeStyle}
      items={THEMES}
      stretch={stretch}
      value={theme}
      onValueChange={setTheme}
    />
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="SegmentedControl"
      description="A choice between a few modes, held in one recessed track. Use it where the options are alternatives to each other and all of them fit on screen; past four or five, a Select says the same thing without the width."
    >
      <Section
        title="Active style"
        description="`raised` lifts the active segment off the track, the way a desktop control draws it. `filled` paints it with the tint — louder, and worth reserving for a choice that has to shout."
      >
        <Stack>
          <Specimen label="raised">
            <Picker activeStyle="raised" />
          </Specimen>
          <Specimen label="filled">
            <Picker activeStyle="filled" />
          </Specimen>
        </Stack>
      </Section>

      <Section title="Sizes">
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Specimen key={size} label={size}>
              <SegmentedControl
                aria-label={`Appearance (${size})`}
                items={THEMES}
                size={size}
                value="system"
              />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Stretch"
        description="Fills the available width and splits it evenly, for a control that has to span a column."
      >
        <Picker stretch />
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('segmented-control')}
          labelOf={token => token.name.slice('segmented-control-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
