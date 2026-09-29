import { Card, type CardVariant } from '@dazzlabs/dazzui-react'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const CARDS: CardVariant[] = ['raised', 'outlined', 'sunken', 'tinted']

const story: Story = {
  id: 'card',
  name: 'Card',
  summary: 'A surface that holds one thing apart from what is around it.',
  Preview: () => (
    <Card style={{ width: '15rem' }}>
      <strong>Weekly digest</strong>
      <div style={{ color: 'var(--color-content-secondary)' }}>
        Seven updates across three projects.
      </div>
    </Card>
  ),
  examples: [
    {
      title: 'Variants',
      Demo: () => (
        <Row align="stretch">
          {CARDS.map(variant => (
            <Card key={variant} variant={variant} style={{ width: '10rem' }}>
              {variant}
            </Card>
          ))}
        </Row>
      ),
    },
    {
      title: 'Sizes',
      Demo: () => (
        <Row align="start">
          {(['small', 'medium', 'large'] as const).map(size => (
            <Card key={size} size={size} style={{ width: '10rem' }}>
              {size}
            </Card>
          ))}
        </Row>
      ),
    },
  ],
}

export default story
