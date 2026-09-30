import {
  ActionBar,
  Button,
  Checkbox,
  Fieldset,
  Form,
  FormField,
  TextField,
} from '@dazzlabs/dazzui'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'form',
  name: 'Form',
  summary: 'Fields that are sent together, and checked before they are.',
  Preview: () => (
    <Col width="16rem">
      <Form onSubmit={event => event.preventDefault()}>
        <FormField label="Email">
          <TextField type="email" placeholder="you@example.com" />
        </FormField>
        <Button type="submit" variant="filled" fullWidth>
          Subscribe
        </Button>
      </Form>
    </Col>
  ),
  examples: [
    {
      title: 'A sign-up',
      Demo: () => (
        <Col width="24rem">
          <Form onSubmit={event => event.preventDefault()}>
            <FormField label="Name">
              <TextField required placeholder="Ada Lovelace" />
            </FormField>
            <FormField label="Email" hint="Only used to sign in.">
              <TextField type="email" required placeholder="you@example.com" />
            </FormField>
            <ActionBar>
              <Button type="reset" variant="outlined">
                Clear
              </Button>
              <Button type="submit" variant="filled">
                Create account
              </Button>
            </ActionBar>
          </Form>
        </Col>
      ),
    },
    {
      title: 'A fieldset',
      Demo: () => (
        <Col width="24rem">
          <Fieldset legend="Notify me about">
            <Checkbox defaultChecked>Mentions</Checkbox>
            <Checkbox defaultChecked>Replies</Checkbox>
            <Checkbox>Everything else</Checkbox>
          </Fieldset>
        </Col>
      ),
    },
  ],
}

export default story
