import { Button, Spinner } from '@dazzlabs/dazzui-react'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'spinner',
  name: 'Spinner',
  summary: 'Work in progress that has no length to report.',
  Preview: () => (
    <Row gap="5">
      <Spinner size="small" />
      <Spinner size="medium" />
      <Spinner size="large" />
    </Row>
  ),
  examples: [
    {
      title: 'Sizes',
      Demo: () => (
        <Row gap="5">
          <Spinner size="small" />
          <Spinner size="medium" />
          <Spinner size="large" />
        </Row>
      ),
    },
    {
      title: 'In a button',
      Demo: () => (
        <Button variant="filled" disabled>
          <Spinner size="small" onAccent />
          Saving
        </Button>
      ),
    },
  ],
}

export default story
