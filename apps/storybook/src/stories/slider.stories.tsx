import {
  FormField,
  PreferenceRow,
  PreferenceSection,
  Slider,
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
import { Col } from '../components/stage'
import { componentTokens } from '../lib/tokens'

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

export const Bare: Story = {
  render: () => (
    <div style={{ width: '18rem' }}>
      <Slider defaultValue={40} />
    </div>
  ),
}

export const WithValue: Story = {
  render: () => (
    <div style={{ width: '20rem' }}>
      <Slider defaultValue={40} showValue />
    </div>
  ),
}

export const Range: Story = {
  render: () => (
    <div style={{ width: '20rem' }}>
      <Slider defaultValue={[20, 70]} showValue />
    </div>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width="20rem">
      {SIZES.map(size => (
        <Slider key={size} size={size} defaultValue={40} />
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col width="20rem">
      {TINTS.map(tint => (
        <Slider key={tint} tint={tint} defaultValue={60} />
      ))}
    </Col>
  ),
}

export const Disabled: Story = {
  render: () => (
    <div style={{ width: '20rem' }}>
      <Slider defaultValue={40} disabled showValue />
    </div>
  ),
}

export const Interactive: Story = {
  render: () => {
    const [size, setSize] = useState(13)
    return (
      <div style={{ width: '22rem' }}>
        <FormField label="Editor font size" hint={`${size}px`}>
          <Slider
            min={8}
            max={24}
            value={size}
            onValueChange={next => setSize(next as number)}
            showValue
            formatValue={values => `${values[0]}px`}
          />
        </FormField>
      </div>
    )
  },
}

export const InSettings: Story = {
  render: () => (
    <div style={{ width: '30rem' }}>
      <PreferenceSection label="Appearance">
        <PreferenceRow
          title="Editor font size"
          subtitle="Applies to every open project"
        >
          <div style={{ width: '10rem' }}>
            <Slider defaultValue={13} min={8} max={24} size="small" />
          </div>
        </PreferenceRow>
        <PreferenceRow title="Sidebar width">
          <div style={{ width: '10rem' }}>
            <Slider defaultValue={240} min={180} max={360} size="small" />
          </div>
        </PreferenceRow>
      </PreferenceSection>
    </div>
  ),
}
