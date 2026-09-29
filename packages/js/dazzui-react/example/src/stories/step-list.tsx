import { Step, StepList } from '@dazzlabs/dazzui-react'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'step-list',
  name: 'StepList',
  summary: 'A job in stages, and the stage it has reached.',
  Preview: () => (
    <StepList>
      <Step status="done" label="Build" meta="42s" />
      <Step status="active" label="Test" />
      <Step status="idle" label="Deploy" />
    </StepList>
  ),
  examples: [
    {
      title: 'Advancing',
      Demo: () => (
        <Col width="18rem">
          <StepList>
            <Step status="done" label="Install dependencies" meta="12s" />
            <Step status="done" label="Build" meta="42s" />
            <Step status="active" label="Run tests" />
            <Step status="idle" label="Deploy" />
          </StepList>
        </Col>
      ),
    },
  ],
}

export default story
