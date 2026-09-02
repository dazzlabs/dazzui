import {
  ActionBar,
  Button,
  FormField,
  TextField,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof TextField> = {
  title: 'Components/TextField',
  component: TextField,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

const WIDTH = '22rem'

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="TextField"
      description="A single line of text. It shares its box and its height with Select, Textarea and the buttons beside it, so a form row never steps. It stays a native `<input>`, so `type` behaves the way the platform's does and validation stays where the form already is. Wrap it in a `FormField` to give it a label and a hint."
    >
      <Section
        title="States"
        description="Focus one: the border and the ring both come from the tint, which is why re-tinting is all an error state has to do. Disabled dims the whole box rather than only the text, because a box that still looks live invites a click that goes nowhere."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            <Specimen label="filled">
              <TextField defaultValue="Jianying Li" />
            </Specimen>
            <Specimen label="empty">
              <TextField placeholder="Placeholder" />
            </Specimen>
            <Specimen label="read-only">
              <TextField readOnly defaultValue="Read only" />
            </Specimen>
            <Specimen label="disabled">
              <TextField disabled defaultValue="Disabled" />
            </Specimen>
            <Specimen label="error">
              <TextField state="error" defaultValue="Rejected value" />
            </Specimen>
          </Stack>
        </div>
      </Section>

      <Section
        title="Sizes"
        description="Height, padding, radius and type come off the shared control profile, so an input at `small` and a button at `small` are the same height by construction rather than by agreement."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            {SIZES.map(size => (
              <Specimen key={size} label={size}>
                <ActionBar>
                  <TextField placeholder={size} size={size} />
                  <Button size={size} variant="filled">
                    Go
                  </Button>
                </ActionBar>
              </Specimen>
            ))}
          </Stack>
        </div>
      </Section>

      <Section
        title="Mono"
        description="For a value that is a key, an ID or a URL — something read character by character rather than as words. The face changes; nothing else does, so a mono field still lines up with the ones above and below it."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            <TextField mono defaultValue="https://api.deepl.com/v2" />
            <TextField mono defaultValue="sk-9f3c7a01b4224e8d" />
          </Stack>
        </div>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <TextField defaultValue="Jianying Li" />
    </div>
  ),
}

export const Placeholder: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <TextField placeholder="Search terms" />
    </div>
  ),
}

export const ReadOnly: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <TextField readOnly defaultValue="Read only" />
    </div>
  ),
}

export const Disabled: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <TextField disabled defaultValue="Disabled" />
    </div>
  ),
}

export const ErrorState: Story = {
  name: 'Error',
  render: () => (
    <div style={{ width: WIDTH }}>
      <TextField state="error" defaultValue="Rejected value" />
    </div>
  ),
}

export const States: Story = {
  render: () => (
    <Col width={WIDTH}>
      <TextField defaultValue="Jianying Li" />
      <TextField placeholder="Placeholder" />
      <TextField readOnly defaultValue="Read only" />
      <TextField disabled defaultValue="Disabled" />
      <TextField state="error" defaultValue="Rejected value" />
    </Col>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width={WIDTH}>
      {SIZES.map(size => (
        <TextField key={size} placeholder={size} size={size} />
      ))}
    </Col>
  ),
}

export const BesideAButton: Story = {
  render: () => (
    <Col width={WIDTH}>
      {SIZES.map(size => (
        <ActionBar key={size}>
          <TextField placeholder="Search terms" size={size} />
          <Button size={size} variant="filled">
            Go
          </Button>
        </ActionBar>
      ))}
    </Col>
  ),
}

export const Mono: Story = {
  render: () => (
    <Col width={WIDTH}>
      <TextField mono defaultValue="https://api.deepl.com/v2" />
      <TextField mono defaultValue="sk-9f3c7a01b4224e8d" />
      <TextField mono placeholder="~/Library/Application Support" />
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col width={WIDTH}>
      {TINTS.map(tint => (
        <TextField key={tint} placeholder={tint} tint={tint} />
      ))}
    </Col>
  ),
}

export const NativeTypes: Story = {
  render: () => (
    <Col width={WIDTH}>
      <TextField defaultValue="handbook.docx" />
      <TextField type="password" defaultValue="not-a-real-key" />
      <TextField type="number" defaultValue={500} />
      <TextField type="email" placeholder="you@example.com" />
      <TextField type="date" defaultValue="2026-09-14" />
    </Col>
  ),
}

export const Controlled: Story = {
  render: () => {
    const [name, setName] = useState('handbook')
    const invalid = name.trim() === ''

    return (
      <Col width={WIDTH}>
        <FormField
          label="File name"
          state={invalid ? 'error' : 'default'}
          hint={
            invalid ? 'A file needs a name.' : `Saved as ${name.trim()}.docx`
          }
        >
          <TextField
            state={invalid ? 'error' : 'default'}
            value={name}
            onChange={event => setName(event.target.value)}
          />
        </FormField>
        <Button disabled={invalid} variant="filled">
          Export
        </Button>
      </Col>
    )
  },
}
