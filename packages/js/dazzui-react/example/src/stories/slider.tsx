import { Slider } from '@dazzlabs/dazzui-react'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'slider',
  name: 'Slider',
  summary:
    'A value along a range, where the feel matters more than the figure.',
  Preview: () => (
    <Col width="15rem">
      <Slider defaultValue={60} aria-label="Volume" />
    </Col>
  ),
  examples: [
    {
      title: 'Sizes',
      Demo: () => (
        <Col width="20rem" gap="5">
          {(['small', 'medium', 'large'] as const).map(size => (
            <Slider
              key={size}
              size={size}
              defaultValue={40}
              aria-label={size}
            />
          ))}
        </Col>
      ),
    },
    {
      title: 'With its value, and a range',
      Demo: () => (
        <Col width="20rem" gap="5">
          <Slider defaultValue={72} showValue aria-label="Quality" />
          <Slider defaultValue={[20, 80]} showValue aria-label="Range" />
        </Col>
      ),
    },
  ],
}

export default story
