import { Avatar, Badge, Button, Link, PreviewCard } from '@dazzlabs/dazzui'

import { Col, Row, Text } from '../components/stage'
import type { Story } from '../lib/story'

function Person() {
  return (
    <Row>
      <Avatar name="Ada Lovelace" />
      <Col gap="0-5">
        <Text strong>Ada Lovelace</Text>
        <Text tone="secondary">Wrote the first program</Text>
      </Col>
    </Row>
  )
}

const story: Story = {
  id: 'preview-card',
  name: 'PreviewCard',
  summary: 'What a link leads to, shown before it is followed.',
  Preview: () => (
    <Text>
      Reviewed by{' '}
      <PreviewCard trigger={<Link href="#/preview-card">@ada</Link>}>
        <Person />
      </PreviewCard>
    </Text>
  ),
  examples: [
    {
      title: 'Over a link',
      Demo: () => (
        <Text>
          Reviewed by{' '}
          <PreviewCard trigger={<Link href="#/preview-card">@ada</Link>}>
            <Person />
          </PreviewCard>{' '}
          three days ago.
        </Text>
      ),
    },
    {
      title: 'Over a button',
      Demo: () => (
        <PreviewCard
          width="18rem"
          trigger={<Button size="small">dazzlabs/dazzui</Button>}
        >
          <Col gap="2">
            <Row>
              <Text strong>dazzlabs/dazzui</Text>
              <Badge size="small" tint="neutral">
                public
              </Badge>
            </Row>
            <Text tone="secondary">
              One design, drawn once for every toolkit.
            </Text>
          </Col>
        </PreviewCard>
      ),
    },
  ],
}

export default story
