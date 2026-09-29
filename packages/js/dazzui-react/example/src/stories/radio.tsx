import { RadioGroup } from '@dazzlabs/dazzui-react'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const PLANS = [
  { value: 'free', label: 'Free' },
  { value: 'team', label: 'Team' },
  { value: 'business', label: 'Business' },
]

const story: Story = {
  id: 'radio',
  name: 'Radio',
  summary: 'One of a few, with every option in view.',
  Preview: () => (
    <RadioGroup options={PLANS} defaultValue="team" aria-label="Plan" />
  ),
  examples: [
    {
      title: 'A group',
      Demo: () => (
        <RadioGroup options={PLANS} defaultValue="free" aria-label="Plan" />
      ),
    },
    {
      title: 'Sizes',
      Demo: () => (
        <Row gap="8" align="start">
          {(['small', 'medium', 'large'] as const).map(size => (
            <RadioGroup
              key={size}
              size={size}
              options={PLANS}
              defaultValue="team"
              aria-label={size}
            />
          ))}
        </Row>
      ),
    },
  ],
}

export default story
