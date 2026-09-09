import {
  Button,
  Checkbox,
  Fieldset,
  Form,
  FormField,
  RadioGroup,
  Select,
  TextField,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section } from '../components/gallery'
import { Row } from '../components/stage'

const meta: Meta<typeof Form> = {
  title: 'Components/Form',
  component: Form,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Form"
      description="The run of fields, and the one thing above them that knows about all of them: errors. That is what it is for. A form element already submits; what it cannot do is take what the server said back — { email: 'Already taken' } — and put each message under the field it belongs to, move the focus to the first one, and clear it again when that field is edited. Base UI's Form does, as long as the fields inside are FormFields with names."
    >
      <Section
        title="Server errors, distributed"
        description="Submit with the address below and the form puts the message on the field rather than at the top of the page. That is the difference between an error a reader can act on and a banner they have to translate into a field."
      >
        <div style={{ width: '22rem' }}>
          <ServerErrorForm />
        </div>
      </Section>

      <Section
        title="Fieldset"
        description="A run of fields that are one question between them. The legend is the group's accessible name, which is the whole point: a screen reader announces it before each field inside, so “Street” is heard as “Billing address, Street”. A heading in a paragraph above the fields looks the same and says none of that."
      >
        <div style={{ width: '24rem' }}>
          <Form>
            <Fieldset legend="Billing address">
              <FormField label="Street" name="street">
                <TextField />
              </FormField>
              <Row>
                <FormField label="City" name="city">
                  <TextField />
                </FormField>
                <FormField label="Postcode" name="postcode">
                  <TextField />
                </FormField>
              </Row>
            </Fieldset>
            <Fieldset legend="Delivery">
              <RadioGroup
                defaultValue="standard"
                options={[
                  { value: 'standard', label: 'Standard, 3–5 days' },
                  { value: 'express', label: 'Express, next day' },
                ]}
              />
            </Fieldset>
          </Form>
        </div>
      </Section>

      <Section
        title="What it is not"
        description="A fieldset claims the fields inside it belong together. Heading a run of unrelated fields is a SectionLabel's job, or a PreferenceSection's — reaching for a fieldset there tells a screen reader something untrue."
      >
        <div style={{ width: '22rem' }}>
          <Form>
            <FormField label="Project name" name="name">
              <TextField defaultValue="Acme Web" />
            </FormField>
            <FormField label="Visibility" name="visibility">
              <Select
                defaultValue="private"
                options={[
                  { value: 'private', label: 'Private' },
                  { value: 'public', label: 'Public' },
                ]}
              />
            </FormField>
            <Checkbox defaultChecked>Add a README</Checkbox>
          </Form>
        </div>
      </Section>
    </Page>
  ),
}

function ServerErrorForm() {
  const [errors, setErrors] = useState<Record<string, string>>({})

  return (
    <Form
      errors={errors}
      onSubmit={event => {
        event.preventDefault()
        const email = new FormData(event.currentTarget).get('email')
        setErrors(
          email === 'ada@example.com'
            ? { email: 'That address is already registered' }
            : {}
        )
      }}
    >
      <FormField label="Email" name="email" hint="Try ada@example.com">
        <TextField
          name="email"
          type="email"
          defaultValue="ada@example.com"
          placeholder="you@example.com"
        />
      </FormField>
      <Row>
        <Button type="submit" variant="filled">
          Sign up
        </Button>
      </Row>
    </Form>
  )
}

export const Bare: Story = {
  render: () => (
    <div style={{ width: '22rem' }}>
      <Form>
        <FormField label="Project name" name="name">
          <TextField defaultValue="Acme Web" />
        </FormField>
        <FormField label="Description" name="description">
          <TextField multiline rows={3} />
        </FormField>
        <Row>
          <Button type="submit" variant="filled">
            Create
          </Button>
          <Button type="reset">Reset</Button>
        </Row>
      </Form>
    </div>
  ),
}

export const WithFieldsets: Story = {
  render: () => (
    <div style={{ width: '24rem' }}>
      <Form>
        <Fieldset legend="Billing address">
          <FormField label="Street" name="street">
            <TextField />
          </FormField>
          <Row>
            <FormField label="City" name="city">
              <TextField />
            </FormField>
            <FormField label="Postcode" name="postcode">
              <TextField />
            </FormField>
          </Row>
        </Fieldset>
        <Fieldset legend="Delivery">
          <RadioGroup
            defaultValue="standard"
            options={[
              { value: 'standard', label: 'Standard, 3–5 days' },
              { value: 'express', label: 'Express, next day' },
            ]}
          />
        </Fieldset>
      </Form>
    </div>
  ),
}

export const ServerErrors: Story = {
  render: () => (
    <div style={{ width: '22rem' }}>
      <ServerErrorForm />
    </div>
  ),
}
