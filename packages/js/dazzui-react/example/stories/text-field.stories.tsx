import { defineStory } from '../src/lib/scene'
import scenes from './text-field.scene.json'
import { ActionBar, Button, FormField, TextField } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

/*
 * `multiline` discriminates the props, so `typeof TextField` is a union of two
 * element shapes and Storybook cannot derive one args type from it. Every
 * story here renders explicitly rather than through args, so the component is
 * still named — the docs page needs it — and the args type is left open.
 */
const meta: Meta = {
  title: 'Components/TextField',
  component: TextField,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj

const SIZES = ['small', 'medium', 'large'] as const

const WIDTH = '22rem'

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="TextField"
      description="A line of text, or a box of it. `multiline` is the axis rather than a second component, because the box, the states and the tinting are one design either way. On a single line it shares its height with Select and the buttons beside it, so a form row never steps. It stays a native `<input>` or `<textarea>`, so `type`, `rows` and validation behave the way the platform's do. Wrap it in a `FormField` to give it a label and a hint."
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
        title="Multiline"
        description="`multiline` swaps the element for a `<textarea>` and changes three things and no more: the box takes the container corner rather than the control corner — a theme that makes controls into pills would otherwise turn a growing box into a lozenge — the leading opens up, because this is the one text control whose text wraps, and the height becomes a floor rather than a fixed size. It resizes vertically only; letting it widen would break it out of the column it was laid out in."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            <Specimen label="filled">
              <TextField
                multiline
                defaultValue="Keep product names untranslated."
              />
            </Specimen>
            <Specimen label="empty">
              <TextField multiline placeholder="Notes for the reviewer" />
            </Specimen>
            <Specimen label="rows=6">
              <TextField multiline placeholder="Six rows" rows={6} />
            </Specimen>
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

export const Default: Story = defineStory(scenes.Default)

export const Placeholder: Story = defineStory(scenes.Placeholder)

export const ReadOnly: Story = defineStory(scenes.ReadOnly)

export const Disabled: Story = defineStory(scenes.Disabled)

export const ErrorState: Story = defineStory(scenes.ErrorState, {
  name: 'Error',
})

export const States: Story = defineStory(scenes.States)

export const Sizes: Story = defineStory(scenes.Sizes)

export const BesideAButton: Story = defineStory(scenes.BesideAButton)

export const Mono: Story = defineStory(scenes.Mono)

export const Tints: Story = defineStory(scenes.Tints)

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

export const Controlled: Story = defineStory(scenes.Controlled)

export const Multiline: Story = {
  render: () => (
    <Col width={WIDTH}>
      <TextField multiline defaultValue="Keep product names untranslated." />
      <TextField multiline placeholder="Notes for the reviewer" />
      <TextField multiline readOnly defaultValue="Read only" />
      <TextField multiline disabled defaultValue="Disabled" />
      <TextField multiline state="error" defaultValue="Rejected value" />
    </Col>
  ),
}

export const Rows: Story = {
  render: () => (
    <Col width={WIDTH}>
      <TextField multiline placeholder="Two rows" rows={2} />
      <TextField multiline placeholder="Six rows" rows={6} />
    </Col>
  ),
}

export const MultilineMono: Story = {
  render: () => (
    <Col width={WIDTH}>
      <TextField
        multiline
        mono
        rows={5}
        defaultValue={'{\n  "sourceLang": "en",\n  "formality": "more"\n}'}
      />
      <TextField
        multiline
        mono
        rows={3}
        placeholder="One glob per line — *.docx"
      />
    </Col>
  ),
}

export const MultilineInAForm: Story = {
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
          <TextField
            multiline
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
