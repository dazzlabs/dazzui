import { OptionCard } from '@dazzlabs/dazzui'
import { useState } from 'react'

import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const PLANS = [
  { value: 'free', title: 'Free', description: 'Three projects, one seat.' },
  { value: 'team', title: 'Team', description: 'Unlimited projects.' },
  { value: 'business', title: 'Business', description: 'SSO and audit log.' },
]

function Plans({ count = PLANS.length }: { count?: number }) {
  const [plan, setPlan] = useState('team')
  return (
    <Row align="stretch" width="100%">
      {PLANS.slice(0, count).map(({ value, title, description }) => (
        <OptionCard
          key={value}
          title={title}
          description={description}
          selected={plan === value}
          onClick={() => setPlan(value)}
        />
      ))}
    </Row>
  )
}

const story: Story = {
  id: 'option-card',
  name: 'OptionCard',
  summary: 'One of a few, where each needs a sentence to be chosen by.',
  Preview: () => <Plans count={2} />,
  examples: [
    { title: 'Choosing one', Demo: () => <Plans /> },
    {
      title: 'Disabled',
      Demo: () => (
        <Row align="stretch">
          <OptionCard title="Enterprise" description="Talk to us." disabled />
        </Row>
      ),
    },
  ],
}

export default story
