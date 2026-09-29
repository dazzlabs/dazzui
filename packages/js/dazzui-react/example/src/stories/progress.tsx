import { Progress } from '@dazzlabs/dazzui-react'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'progress',
  name: 'Progress',
  summary: 'How far along something is, or that it is under way at all.',
  Preview: () => (
    <Col width="15rem">
      <Progress value={64} aria-label="Uploading" />
    </Col>
  ),
  examples: [
    {
      title: 'Sizes',
      Demo: () => (
        <Col width="20rem" gap="4">
          {(['small', 'medium', 'large'] as const).map(size => (
            <Progress key={size} size={size} value={48} aria-label={size} />
          ))}
        </Col>
      ),
    },
    {
      title: 'Tints and gradient',
      Demo: () => (
        <Col width="20rem" gap="4">
          <Progress value={80} tint="success" aria-label="Success" />
          <Progress value={55} tint="warning" aria-label="Warning" />
          <Progress value={30} tint="danger" aria-label="Danger" />
          <Progress value={70} gradient aria-label="Gradient" />
        </Col>
      ),
    },
  ],
}

export default story
