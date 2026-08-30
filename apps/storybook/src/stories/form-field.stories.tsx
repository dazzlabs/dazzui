import { FieldValue, FormField, Input, Select } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Stack, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof FormField> = {
  title: 'Components/FormField',
  component: FormField,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="FormField"
      description="The label, the hint, and whatever control sits between them. It draws no box of its own — the control it wraps does that — so it works the same over an Input, a Select or a Textarea. It is `FormField` rather than `Field` because in this library the `*Field` suffix already belongs to controls, `SearchField` among them."
    >
      <Section title="A form">
        <div style={{ maxWidth: '24rem' }}>
          <Stack>
            <FormField label="Display name" hint="Shown on shared documents.">
              <Input defaultValue="Jianying Li" />
            </FormField>
            <FormField label="Target language">
              <Select defaultValue="zh">
                <option value="zh">Chinese (Simplified)</option>
                <option value="ja">Japanese</option>
              </Select>
            </FormField>
            <FormField label="Plan">
              <FieldValue>Pro — renews 2026-09-14</FieldValue>
            </FormField>
          </Stack>
        </div>
      </Section>

      <Section
        title="Error"
        description="One prop moves the label, the border, the wash and the hint together, because all four are reading the same ramp — the control is re-tinted rather than given a second palette."
      >
        <div style={{ maxWidth: '24rem' }}>
          <FormField
            label="API key"
            state="error"
            hint="The provider rejected this key."
          >
            <Input mono state="error" defaultValue="sk-0000000000" />
          </FormField>
        </div>
      </Section>

      <Section
        title="FieldValue"
        description="A value in the shape of an input, for a row that reports rather than edits. It is not a disabled input: nothing here was ever editable, so nothing is greyed out to say it stopped being."
      >
        <div style={{ maxWidth: '24rem' }}>
          <Stack>
            <FieldValue>Pro — renews 2026-09-14</FieldValue>
            <FieldValue mono>~/Library/Application Support</FieldValue>
          </Stack>
        </div>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('form-field')}
          labelOf={token => token.name.slice('form-field-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
