import { Button, EmptyState } from '@dazzlabs/dazzui'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'empty-state',
  name: 'EmptyState',
  summary: 'What a list says when there is nothing in it.',
  Preview: () => <EmptyState title="No projects yet" />,
  examples: [
    {
      title: 'On its own',
      Demo: () => (
        <Col width="24rem">
          <EmptyState title="Nothing matches that search" />
        </Col>
      ),
    },
    {
      title: 'With a way out',
      Demo: () => (
        <Col width="24rem">
          <EmptyState
            title="No projects yet"
            action={<Button variant="filled">New project</Button>}
          />
        </Col>
      ),
    },
  ],
}

export default story
