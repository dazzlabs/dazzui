import { FormField, Select, TextField } from '@dazzlabs/dazzui'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'form-field',
  name: 'FormField',
  summary: 'A control with the label that names it and the hint under it.',
  Preview: () => (
    <Col width="15rem">
      <FormField label="Project name" hint="Shown in the sidebar.">
        <TextField defaultValue="Quarterly report" />
      </FormField>
    </Col>
  ),
  examples: [
    {
      title: 'With a hint',
      Demo: () => (
        <Col width="22rem">
          <FormField label="Email" hint="Only used to sign in.">
            <TextField type="email" placeholder="you@example.com" />
          </FormField>
        </Col>
      ),
    },
    {
      title: 'In error',
      Demo: () => (
        <Col width="22rem">
          <FormField
            label="Email"
            state="error"
            hint="That does not look like an address."
          >
            <TextField state="error" defaultValue="ada@" />
          </FormField>
        </Col>
      ),
    },
    {
      title: 'Over any control',
      Demo: () => (
        <Col width="22rem">
          <FormField label="Language">
            <Select
              defaultValue="en"
              options={[
                { value: 'en', label: 'English' },
                { value: 'zh', label: 'Chinese' },
              ]}
            />
          </FormField>
        </Col>
      ),
    },
  ],
}

export default story
