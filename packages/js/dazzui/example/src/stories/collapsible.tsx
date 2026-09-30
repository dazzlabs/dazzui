import { Badge, Collapsible } from '@dazzlabs/dazzui'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'collapsible',
  name: 'Collapsible',
  summary: 'One section that can be put away.',
  Preview: () => (
    <Col width="18rem">
      <Collapsible title="Advanced" defaultOpen>
        Options most people never change.
      </Collapsible>
    </Col>
  ),
  examples: [
    {
      title: 'Open and closed',
      Demo: () => (
        <Col width="28rem">
          <Collapsible title="Build log" defaultOpen>
            Compiled 214 modules in 1.8 seconds.
          </Collapsible>
          <Collapsible title="Test log">All 37 tests passed.</Collapsible>
        </Col>
      ),
    },
    {
      title: 'With something beside the title',
      Demo: () => (
        <Col width="28rem">
          <Collapsible
            title="Warnings"
            meta={<Badge tint="warning">3</Badge>}
            defaultOpen
          >
            Three imports are unused.
          </Collapsible>
        </Col>
      ),
    },
  ],
}

export default story
