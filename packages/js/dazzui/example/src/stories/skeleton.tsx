import { Card, Skeleton, SkeletonText } from '@dazzlabs/dazzui'

import { Col, Row } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'skeleton',
  name: 'Skeleton',
  summary: 'The shape of what is coming, while it is on its way.',
  Preview: () => (
    <Row width="15rem">
      <Skeleton shape="circle" width={36} height={36} />
      <Col width="10rem" gap="2">
        <Skeleton width="80%" />
        <Skeleton width="50%" />
      </Col>
    </Row>
  ),
  examples: [
    {
      title: 'Shapes',
      Demo: () => (
        <Row gap="5">
          <Skeleton shape="text" width={120} />
          <Skeleton shape="circle" width={40} height={40} />
          <Skeleton shape="rect" width={120} height={64} />
        </Row>
      ),
    },
    {
      title: 'A paragraph',
      Demo: () => (
        <Col width="22rem">
          <SkeletonText lines={4} />
        </Col>
      ),
    },
    {
      title: 'In a card',
      Demo: () => (
        <Col width="20rem">
          <Card>
            <Row>
              <Skeleton shape="circle" width={36} height={36} />
              <Col width="12rem" gap="2">
                <Skeleton width="70%" />
                <Skeleton width="40%" />
              </Col>
            </Row>
          </Card>
        </Col>
      ),
    },
  ],
}

export default story
