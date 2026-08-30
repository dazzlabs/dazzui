import { Field, FieldValue, Input, Select, Textarea } from '@dazzlabs/dazzui'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Stack, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Field> = {
  title: 'Components/Field',
  component: Field,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="Field"
      description="Label, control, hint. The text controls share one box and one height with the buttons beside them, so a form row never steps."
    >
      <Section
        title="Controls"
        description="Focus any of them: the border and the ring come from the tint, so re-tinting the control is all an error state has to do."
      >
        <div style={{ maxWidth: '26rem' }}>
          <Stack>
            <Field label="Display name" hint="Shown on shared documents.">
              <Input defaultValue="Jianying Li" />
            </Field>
            <Field label="Endpoint">
              <Input mono defaultValue="https://api.deepl.com/v2" />
            </Field>
            <Field label="Target language">
              <Select defaultValue="zh">
                <option value="zh">Chinese (Simplified)</option>
                <option value="ja">Japanese</option>
                <option value="de">German</option>
              </Select>
            </Field>
            <Field
              label="Glossary notes"
              hint="Applied to every project using this glossary."
            >
              <Textarea defaultValue="Keep product names untranslated." />
            </Field>
            <Field label="Plan">
              <FieldValue>Pro — renews 2026-09-14</FieldValue>
            </Field>
          </Stack>
        </div>
      </Section>

      <Section
        title="Error"
        description="One prop moves the label, the border, the wash, and the hint together, because they are all reading the same ramp."
      >
        <div style={{ maxWidth: '26rem' }}>
          <Field
            label="API key"
            state="error"
            hint="The provider rejected this key."
          >
            <Input mono state="error" defaultValue="sk-0000000000" />
          </Field>
        </div>
      </Section>

      <Section title="Sizes">
        <div style={{ maxWidth: '26rem' }}>
          <Stack>
            <Input placeholder="small" size="small" />
            <Input placeholder="medium" size="medium" />
            <Input placeholder="large" size="large" />
          </Stack>
        </div>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('field')}
          labelOf={token => token.name.slice('field-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
