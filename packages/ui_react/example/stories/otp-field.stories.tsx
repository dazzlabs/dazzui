import { Button, Callout, FormField, OtpField } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof OtpField> = {
  title: 'Components/OtpField',
  component: OtpField,
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
      title="OtpField"
      description="The one-time code a reader is copying out of a message. It is a run of cells rather than a single field because that is what makes it fast: Base UI moves the caret between them, takes a paste of the whole code into all of them at once, hands the browser the autofill hint that pulls the code out of an SMS, and can submit the form the moment the last cell fills."
    >
      <Section
        title="Sizes"
        description="Each cell is the text controls' box at one character wide, so a code sits on the same rhythm as the fields around it — a code entered in a box of its own design would be the only control on the page that was. The mono face and tabular figures keep the digits on one width."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <OtpField length={6} defaultValue="204" size={size} />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Grouping"
        description="`groupAfter` breaks the run where a printed code breaks — three and three is how a six-digit code is read aloud. It is a step of air rather than a rule, because a rule would read as two fields."
      >
        <Stack>
          <Specimen label="ungrouped">
            <OtpField length={6} defaultValue="204815" />
          </Specimen>
          <Specimen label="groupAfter 3">
            <OtpField length={6} groupAfter={3} defaultValue="204815" />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Masked and disabled"
        description="`mask` is for a code that is a secret rather than a one-time token — a PIN. A filled cell lifts to the paper and keeps it, so the run says how far through the code the reader is without the caret having to be the only thing that does."
      >
        <Stack>
          <Specimen label="masked">
            <OtpField length={4} mask defaultValue="20" />
          </Specimen>
          <Specimen label="disabled">
            <OtpField length={6} disabled defaultValue="204815" />
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Bare: Story = {
  render: () => <OtpField length={6} />,
}

export const Filled: Story = {
  render: () => <OtpField length={6} defaultValue="204815" />,
}

export const Grouped: Story = {
  render: () => <OtpField length={6} groupAfter={3} defaultValue="204815" />,
}

export const Sizes: Story = {
  render: () => (
    <Col>
      {SIZES.map(size => (
        <OtpField key={size} length={6} size={size} defaultValue="204" />
      ))}
    </Col>
  ),
}

export const Masked: Story = {
  render: () => <OtpField length={4} mask defaultValue="2048" />,
}

export const Disabled: Story = {
  render: () => <OtpField length={6} disabled defaultValue="204815" />,
}

export const InAForm: Story = {
  render: () => {
    const [code, setCode] = useState('')
    const wrong = code.length === 6 && code !== '204815'

    return (
      <Col width="20rem" gap="var(--spacing-4)">
        <Callout tint="info">
          We sent a six-digit code to ada@example.com.
        </Callout>
        <FormField
          label="Verification code"
          hint={wrong ? 'That code did not match' : 'It expires in 10 minutes'}
          state={wrong ? 'error' : 'default'}
        >
          <OtpField
            length={6}
            groupAfter={3}
            value={code}
            onValueChange={setCode}
            state={wrong ? 'error' : 'default'}
          />
        </FormField>
        <Button variant="filled" disabled={code.length < 6}>
          Verify
        </Button>
      </Col>
    )
  },
}
