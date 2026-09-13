import { defineStory } from '../src/lib/scene'
import scenes from './form-field.scene.json'
import {
  ActionBar,
  Button,
  FieldValue,
  FormField,
  TextField,
  Select,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import { Page, Section, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof FormField> = {
  title: 'Components/FormField',
  component: FormField,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const WIDTH = '24rem'

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="FormField"
      description="The label, the hint, and whatever control sits between them. It draws no box of its own — the control it wraps does that — so it works the same over a TextField single-line or multiline, a Select, and controls that are not text fields at all. It is `FormField` rather than `Field` because in this library the `*Field` suffix already belongs to controls, `SearchField` among them."
    >
      <Section
        title="A form"
        description="Fields stacked in one column, an action row at the foot. The gap between fields is the only spacing decision, because every field brings its own internal rhythm with it. A hint is optional and should stay that way: a hint on every field is a form nobody reads."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            <FormField label="Display name" hint="Shown on shared documents.">
              <TextField defaultValue="Jianying Li" />
            </FormField>
            <FormField label="Target language">
              <Select
                defaultValue="zh"
                options={[
                  { value: 'zh', label: 'Chinese (Simplified)' },
                  { value: 'ja', label: 'Japanese' },
                ]}
              />
            </FormField>
            <FormField label="Plan">
              <FieldValue>Pro — renews 2026-09-14</FieldValue>
            </FormField>
            <ActionBar align="end">
              <Button>Cancel</Button>
              <Button variant="filled">Save</Button>
            </ActionBar>
          </Stack>
        </div>
      </Section>

      <Section
        title="Error"
        description="One prop moves the label, the border, the wash and the hint together, because all four are reading the same ramp — the control is re-tinted rather than given a second palette. Set `state` on both the field and the control it wraps. `error` overrides the tint onto danger, because an erroring field on the success ramp would be a contradiction the reader has to resolve."
      >
        <div style={{ maxWidth: WIDTH }}>
          <FormField
            label="API key"
            state="error"
            hint="The provider rejected this key."
          >
            <TextField mono state="error" defaultValue="sk-0000000000" />
          </FormField>
        </div>
      </Section>

      <Section
        title="FieldValue"
        description="A value in the shape of an input, for a row that reports rather than edits. It is not a disabled input: nothing here was ever editable, so nothing is greyed out to say it stopped being. Side by side the difference is the point — the greyed one says “not now”, the plain one says “not ever, by design”."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            <FormField label="Disabled input">
              <TextField disabled defaultValue="Pro — renews 2026-09-14" />
            </FormField>
            <FormField label="FieldValue">
              <FieldValue>Pro — renews 2026-09-14</FieldValue>
            </FormField>
            <FormField label="Storage location">
              <FieldValue mono>~/Library/Application Support</FieldValue>
            </FormField>
          </Stack>
        </div>
      </Section>
    </Page>
  ),
}

export const WithAHint: Story = defineStory(scenes.WithAHint)

export const WithoutAHint: Story = defineStory(scenes.WithoutAHint)

export const ErrorState: Story = defineStory(scenes.ErrorState, {
  name: 'Error',
})

export const Tints: Story = {
  render: () => (
    <Col width={WIDTH}>
      {TINTS.map(tint => (
        <FormField key={tint} label={tint} tint={tint} hint="Focus it.">
          <TextField placeholder={tint} tint={tint} />
        </FormField>
      ))}
    </Col>
  ),
}

export const OverASelect: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <FormField
        label="Glossary"
        hint="Terms here override the provider's own suggestions."
      >
        <Select
          defaultValue="product"
          options={[
            { value: 'product', label: 'Product terms' },
            { value: 'legal', label: 'Legal' },
          ]}
        />
      </FormField>
    </div>
  ),
}

export const OverAMultilineField: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <FormField
        label="Notes"
        hint="Sent with every request, so keep it short."
      >
        <TextField multiline defaultValue="Keep product names untranslated." />
      </FormField>
    </div>
  ),
}

export const OverAnyControl: Story = defineStory(scenes.OverAnyControl)

export const WithAFieldValue: Story = {
  render: () => (
    <Col width={WIDTH}>
      <FormField label="Plan">
        <FieldValue>Pro — renews 2026-09-14</FieldValue>
      </FormField>
      <FormField label="Storage location">
        <FieldValue mono>~/Library/Application Support</FieldValue>
      </FormField>
      <FormField label="Device ID" hint="Quote this in a bug report.">
        <FieldValue mono>7f3c-9a01-4b22</FieldValue>
      </FormField>
    </Col>
  ),
}

export const AFullForm: Story = {
  render: () => (
    <Col width={WIDTH}>
      <FormField label="Name" hint="Shown in the provider list.">
        <TextField defaultValue="DeepL — work" />
      </FormField>
      <FormField label="Endpoint">
        <Select
          defaultValue="pro"
          options={[
            { value: 'pro', label: 'api.deepl.com' },
            { value: 'free', label: 'api-free.deepl.com' },
          ]}
        />
      </FormField>
      <FormField
        label="API key"
        state="error"
        hint="The provider rejected this key."
      >
        <TextField mono state="error" defaultValue="sk-0000000000" />
      </FormField>
      <FormField label="Notes">
        <TextField multiline placeholder="Optional" />
      </FormField>
      <ActionBar align="end">
        <Button>Cancel</Button>
        <Button variant="filled">Add provider</Button>
      </ActionBar>
    </Col>
  ),
}
