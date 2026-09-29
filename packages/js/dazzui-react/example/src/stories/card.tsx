import { Card, type CardVariant } from '@dazzlabs/dazzui-react'
import { Col, Row, Text } from '../components/stage'
import type { Story } from '../lib/story'

const CARDS: CardVariant[] = ['raised', 'outlined', 'sunken', 'tinted']

const story: Story = {
  id: 'card',
  name: 'Card',
  summary: 'A surface that holds one thing apart from what is around it.',
  Preview: () => (
    <Col width="15rem">
      <Card>
        <Col gap="1">
          <Text strong>Weekly digest</Text>
          <Text tone="secondary">Seven updates across three projects.</Text>
        </Col>
      </Card>
    </Col>
  ),
  examples: [
    {
      title: 'Variants',
      Demo: () => (
        <Row align="stretch">
          {CARDS.map(variant => (
            <Col key={variant} width="10rem">
              <Card variant={variant}>{variant}</Card>
            </Col>
          ))}
        </Row>
      ),
    },
    {
      title: 'Sizes',
      Demo: () => (
        <Row align="start">
          {(['small', 'medium', 'large'] as const).map(size => (
            <Col key={size} width="10rem">
              <Card size={size}>{size}</Card>
            </Col>
          ))}
        </Row>
      ),
    },
  ],
}

export default story
