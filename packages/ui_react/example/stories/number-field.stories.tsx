import {
  FormField,
  NumberField,
  PreferenceRow,
  PreferenceSection,
  TextField,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col, Row } from '../src/components/stage'

const meta: Meta<typeof NumberField> = {
  title: 'Components/NumberField',
  component: NumberField,
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
      title="NumberField"
      description="A number, with the two things a plain type=&quot;number&quot; never gets right: a stepper you can hold down, and a value that stays a number. Base UI owns the parsing and the formatting — the locale's own decimal separator, percent and currency, the clamp to min and max, the arrow keys and their large step, and the scrub. What is drawn here is the TextField's own box, so a form's rows do not step, and the stepper at its trailing edge."
    >
      <Section
        title="Sizes"
        description="The box is the text controls' — text-field.css names this group alongside `.dz-text-field`, so the two cannot drift. The stepper takes its room out of the trailing padding rather than out of the value."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <div style={{ width: '10rem' }}>
                <NumberField defaultValue={12} size={size} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Bounds, steps and units"
        description="`min` and `max` clamp; `step` is what one press of the stepper moves and `largeStep` what Page Up moves. A unit inside the box is a hint rather than part of the value — it is the faint ink, and it never enters what the form submits."
      >
        <Col width="12rem">
          <NumberField defaultValue={13} min={8} max={24} unit="px" />
          <NumberField defaultValue={50} step={5} unit="%" />
          <NumberField
            defaultValue={19.99}
            step={0.01}
            format={{ style: 'currency', currency: 'USD' }}
          />
        </Col>
      </Section>

      <Section
        title="States"
        description="Erroring re-tints the control to danger rather than bolting on a second palette, exactly as a TextField does — the border, the wash and the value's ink all move together."
      >
        <Stack>
          <Specimen label="default">
            <div style={{ width: '10rem' }}>
              <NumberField defaultValue={12} />
            </div>
          </Specimen>
          <Specimen label="error">
            <div style={{ width: '10rem' }}>
              <NumberField defaultValue={0} state="error" />
            </div>
          </Specimen>
          <Specimen label="disabled">
            <div style={{ width: '10rem' }}>
              <NumberField defaultValue={12} disabled />
            </div>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Against a TextField"
        description="A number the reader knows is this; a number they are feeling for is a Slider; anything that is only digits — a port, a version, an ID — is a TextField, because a stepper on it would be offering an operation that means nothing."
      >
        <Row>
          <div style={{ width: '10rem' }}>
            <NumberField defaultValue={12} />
          </div>
          <div style={{ width: '10rem' }}>
            <TextField defaultValue="5173" mono />
          </div>
        </Row>
      </Section>
    </Page>
  ),
}

export const Bare: Story = {
  render: () => (
    <div style={{ width: '10rem' }}>
      <NumberField defaultValue={12} />
    </div>
  ),
}

export const WithUnit: Story = {
  render: () => (
    <div style={{ width: '10rem' }}>
      <NumberField defaultValue={13} min={8} max={24} unit="px" />
    </div>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width="10rem">
      {SIZES.map(size => (
        <NumberField key={size} size={size} defaultValue={12} />
      ))}
    </Col>
  ),
}

export const States: Story = {
  render: () => (
    <Col width="10rem">
      <NumberField defaultValue={12} />
      <NumberField defaultValue={0} state="error" />
      <NumberField defaultValue={12} disabled />
    </Col>
  ),
}

export const InAForm: Story = {
  render: () => {
    const [port, setPort] = useState<number | null>(5173)
    return (
      <div style={{ width: '18rem' }}>
        <FormField
          label="Dev server port"
          hint={port === null ? 'Required' : 'Restart to apply'}
          state={port === null ? 'error' : 'default'}
        >
          <NumberField
            value={port}
            onValueChange={setPort}
            min={1024}
            max={65535}
            state={port === null ? 'error' : 'default'}
          />
        </FormField>
      </div>
    )
  },
}

export const InSettings: Story = {
  render: () => (
    <div style={{ width: '30rem' }}>
      <PreferenceSection label="Editor">
        <PreferenceRow title="Font size" subtitle="Applies to every project">
          <div style={{ width: '7rem' }}>
            <NumberField
              defaultValue={13}
              min={8}
              max={24}
              size="small"
              unit="px"
            />
          </div>
        </PreferenceRow>
        <PreferenceRow title="Tab width">
          <div style={{ width: '7rem' }}>
            <NumberField defaultValue={2} min={1} max={8} size="small" />
          </div>
        </PreferenceRow>
      </PreferenceSection>
    </div>
  ),
}
