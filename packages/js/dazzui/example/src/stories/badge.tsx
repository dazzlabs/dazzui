import { Badge, type BadgeVariant, TINTS } from '@dazzlabs/dazzui'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const VARIANTS: BadgeVariant[] = ['filled', 'tinted', 'outlined', 'raised']

const story: Story = {
  id: 'badge',
  name: 'Badge',
  summary: 'A status or a count, read at a glance and never pressed.',
  Preview: () => (
    <Row gap="2">
      <Badge tint="success">Live</Badge>
      <Badge tint="warning">Draft</Badge>
      <Badge variant="filled">12</Badge>
    </Row>
  ),
  examples: [
    {
      title: 'Variants',
      Demo: () => (
        <Row gap="2">
          {VARIANTS.map(variant => (
            <Badge key={variant} variant={variant}>
              {variant}
            </Badge>
          ))}
        </Row>
      ),
    },
    {
      title: 'Tints',
      Demo: () => (
        <Row gap="2">
          {TINTS.map(tint => (
            <Badge key={tint} tint={tint}>
              {tint}
            </Badge>
          ))}
        </Row>
      ),
    },
  ],
}

export default story
