import { TextField } from '@dazzlabs/dazzui'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'text-field',
  name: 'TextField',
  summary: 'A line of text, or several, that a person writes.',
  Preview: () => (
    <Col width="15rem">
      <TextField placeholder="you@example.com" aria-label="Email" />
    </Col>
  ),
  examples: [
    {
      title: 'Sizes',
      Demo: () => (
        <Col width="20rem">
          <TextField size="small" placeholder="Small" aria-label="Small" />
          <TextField size="medium" placeholder="Medium" aria-label="Medium" />
          <TextField size="large" placeholder="Large" aria-label="Large" />
        </Col>
      ),
    },
    {
      title: 'States',
      Demo: () => (
        <Col width="20rem">
          <TextField defaultValue="Filled in" aria-label="Filled" />
          <TextField
            state="error"
            defaultValue="not-an-email"
            aria-label="Error"
          />
          <TextField disabled defaultValue="Disabled" aria-label="Disabled" />
          <TextField mono defaultValue="sk-4f9a…c21e" aria-label="Key" />
        </Col>
      ),
    },
    {
      title: 'Multiline',
      Demo: () => (
        <Col width="20rem">
          <TextField
            multiline
            rows={3}
            placeholder="Anything the reviewer should know"
            aria-label="Notes"
          />
        </Col>
      ),
    },
  ],
}

export default story
