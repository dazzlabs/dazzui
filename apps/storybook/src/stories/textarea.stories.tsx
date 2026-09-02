import {
  ActionBar,
  Button,
  FormField,
  Textarea,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof Textarea> = {
  title: 'Components/Textarea',
  component: Textarea,
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
      title="Textarea"
      description="Text that runs to more than a line. It takes the container corner rather than the control corner: a theme that makes controls into pills would turn a box that grows into a lozenge, so the two radii are separate axes on purpose. It resizes vertically only — letting it widen would break it out of the column it was laid out in."
    >
      <Section
        title="States"
        description="The same set a TextField has, and drawn from the same tokens. `state='error'` is the danger ramp under another name."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            <Specimen label="filled">
              <Textarea defaultValue="Keep product names untranslated." />
            </Specimen>
            <Specimen label="empty">
              <Textarea placeholder="Notes for the reviewer" />
            </Specimen>
            <Specimen label="disabled">
              <Textarea disabled defaultValue="Disabled" />
            </Specimen>
            <Specimen label="error">
              <Textarea state="error" defaultValue="Rejected value" />
            </Specimen>
          </Stack>
        </div>
      </Section>

      <Section
        title="Sizes and rows"
        description="Padding and type step with the profile; the height does not, because a box that grows is sized by its `rows` or by the layout around it rather than by a control height."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            {SIZES.map(size => (
              <Specimen key={size} label={size}>
                <Textarea placeholder={size} size={size} />
              </Specimen>
            ))}
            <Specimen label="rows=6">
              <Textarea placeholder="Six rows" rows={6} />
            </Specimen>
          </Stack>
        </div>
      </Section>

      <Section
        title="Mono"
        description="For text read character by character rather than as words — a payload, a config block, a template. The face changes and nothing else does."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Textarea
            mono
            rows={5}
            defaultValue={'{\n  "sourceLang": "en",\n  "formality": "more"\n}'}
          />
        </div>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Textarea defaultValue="Keep product names untranslated." />
    </div>
  ),
}

export const Placeholder: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Textarea placeholder="Notes for the reviewer" />
    </div>
  ),
}

export const ReadOnly: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Textarea readOnly defaultValue="Read only" />
    </div>
  ),
}

export const Disabled: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Textarea disabled defaultValue="Disabled" />
    </div>
  ),
}

export const ErrorState: Story = {
  name: 'Error',
  render: () => (
    <div style={{ width: WIDTH }}>
      <Textarea state="error" defaultValue="Rejected value" />
    </div>
  ),
}

export const States: Story = {
  render: () => (
    <Col width={WIDTH}>
      <Textarea defaultValue="Keep product names untranslated." />
      <Textarea placeholder="Notes for the reviewer" />
      <Textarea readOnly defaultValue="Read only" />
      <Textarea disabled defaultValue="Disabled" />
      <Textarea state="error" defaultValue="Rejected value" />
    </Col>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width={WIDTH}>
      {SIZES.map(size => (
        <Textarea key={size} placeholder={size} size={size} />
      ))}
    </Col>
  ),
}

export const Rows: Story = {
  render: () => (
    <Col width={WIDTH}>
      <Textarea placeholder="Two rows" rows={2} />
      <Textarea placeholder="Six rows" rows={6} />
    </Col>
  ),
}

export const Mono: Story = {
  render: () => (
    <Col width={WIDTH}>
      <Textarea
        mono
        rows={5}
        defaultValue={'{\n  "sourceLang": "en",\n  "formality": "more"\n}'}
      />
      <Textarea mono rows={3} placeholder="One glob per line — *.docx" />
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col width={WIDTH}>
      {TINTS.map(tint => (
        <Textarea key={tint} placeholder={tint} rows={2} tint={tint} />
      ))}
    </Col>
  ),
}

export const InAForm: Story = {
  render: () => {
    const [notes, setNotes] = useState('Keep product names untranslated.')
    const tooLong = notes.length > 120

    return (
      <Col width={WIDTH}>
        <FormField
          label="Instructions for the provider"
          state={tooLong ? 'error' : 'default'}
          hint={
            tooLong
              ? `${notes.length} / 120 — too long`
              : `${notes.length} / 120 characters`
          }
        >
          <Textarea
            rows={4}
            state={tooLong ? 'error' : 'default'}
            value={notes}
            onChange={event => setNotes(event.target.value)}
          />
        </FormField>
        <ActionBar align="end">
          <Button onClick={() => setNotes('')}>Clear</Button>
          <Button disabled={tooLong} variant="filled">
            Save
          </Button>
        </ActionBar>
      </Col>
    )
  },
}
