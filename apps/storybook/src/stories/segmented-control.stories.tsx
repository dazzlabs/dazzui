import { FormField, SegmentedControl, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { Col } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof SegmentedControl> = {
  title: 'Components/SegmentedControl',
  component: SegmentedControl,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

const THEMES = [
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
  { value: 'system', label: 'System' },
] as const

function Picker({
  activeStyle,
  stretch,
  size,
  tint,
}: {
  activeStyle?: 'raised' | 'filled'
  stretch?: boolean
  size?: (typeof SIZES)[number]
  tint?: (typeof TINTS)[number]
}) {
  const [theme, setTheme] = useState<string>('system')
  return (
    <SegmentedControl
      aria-label="Appearance"
      activeStyle={activeStyle}
      items={THEMES}
      size={size}
      stretch={stretch}
      tint={tint}
      value={theme}
      onValueChange={setTheme}
    />
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="SegmentedControl"
      description="A choice between a few modes, held in one recessed track. Use it where the options are alternatives to each other and all of them fit on screen; past four or five, a Select says the same thing without the width."
    >
      <Section
        title="Active style"
        description="`raised` lifts the active segment off the track, the way a desktop control draws it. `filled` paints it with the tint — louder, and worth reserving for a choice that has to shout. With `raised` the tint reaches only the label, which is why the two are separate axes."
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

      <Section
        title="Sizes"
        description="The track's inset holds while the segments step, so the raised segment keeps the same hairline of track around it at every size."
      >
        <Stack>
          {SIZES.map(size => (
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
        description="Fills the available width and splits it evenly, for a control that has to span a column. Without it the track sizes to its labels, which is what a toolbar wants and a form does not. A mode that is not available yet stays in the track rather than being removed, so the control keeps its width."
      >
        <div style={{ maxWidth: '24rem' }}>
          <Stack>
            <Specimen label="natural">
              <Picker />
            </Specimen>
            <Specimen label="stretch">
              <div style={{ flex: 1 }}>
                <Picker stretch />
              </div>
            </Specimen>
          </Stack>
        </div>
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

export const Raised: Story = {
  render: () => <Picker activeStyle="raised" />,
}

export const Filled: Story = {
  render: () => <Picker activeStyle="filled" />,
}

export const Sizes: Story = {
  render: () => (
    <Col>
      {SIZES.map(size => (
        <SegmentedControl
          key={size}
          aria-label={`Appearance (${size})`}
          items={THEMES}
          size={size}
          value="system"
        />
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      {TINTS.map(tint => (
        <Picker key={tint} tint={tint} />
      ))}
    </Col>
  ),
}

export const FilledTints: Story = {
  render: () => (
    <Col>
      {TINTS.map(tint => (
        <Picker key={tint} activeStyle="filled" tint={tint} />
      ))}
    </Col>
  ),
}

export const Stretch: Story = {
  render: () => (
    <Col width="24rem">
      <Picker stretch />
    </Col>
  ),
}

export const Disabled: Story = {
  render: () => (
    <SegmentedControl
      aria-label="Quality"
      items={[
        { value: 'fast', label: 'Fast' },
        { value: 'balanced', label: 'Balanced' },
        { value: 'careful', label: 'Careful', disabled: true },
      ]}
      value="balanced"
    />
  ),
}

export const TwoSegments: Story = {
  render: () => {
    const [view, setView] = useState('source')
    return (
      <SegmentedControl
        aria-label="View"
        items={[
          { value: 'source', label: 'Source' },
          { value: 'target', label: 'Target' },
        ]}
        value={view}
        onValueChange={setView}
      />
    )
  },
}

export const InAForm: Story = {
  render: () => {
    const [quality, setQuality] = useState('balanced')
    const [formality, setFormality] = useState('default')

    return (
      <Col width="24rem" gap="var(--spacing-4)">
        <FormField label="Quality" hint="Applies to new jobs only.">
          <SegmentedControl
            aria-label="Quality"
            items={[
              { value: 'fast', label: 'Fast' },
              { value: 'balanced', label: 'Balanced' },
              { value: 'careful', label: 'Careful' },
            ]}
            stretch
            value={quality}
            onValueChange={setQuality}
          />
        </FormField>
        <FormField label="Formality">
          <SegmentedControl
            aria-label="Formality"
            items={[
              { value: 'less', label: 'Less' },
              { value: 'default', label: 'Default' },
              { value: 'more', label: 'More' },
            ]}
            stretch
            value={formality}
            onValueChange={setFormality}
          />
        </FormField>
      </Col>
    )
  },
}
