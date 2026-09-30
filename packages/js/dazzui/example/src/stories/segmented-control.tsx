import { SegmentedControl } from '@dazzlabs/dazzui'
import { useState } from 'react'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const VIEWS = [
  { value: 'list', label: 'List' },
  { value: 'board', label: 'Board' },
  { value: 'timeline', label: 'Timeline' },
]

function Segments({ size }: { size?: 'small' | 'medium' | 'large' }) {
  const [view, setView] = useState('list')
  return (
    <SegmentedControl
      items={VIEWS}
      value={view}
      onValueChange={setView}
      size={size}
      aria-label="View"
    />
  )
}

const story: Story = {
  id: 'segmented-control',
  name: 'SegmentedControl',
  summary: 'A few views of one thing, and the one being looked at.',
  Preview: () => <Segments />,
  examples: [
    {
      title: 'Sizes',
      Demo: () => (
        <Col align="start">
          <Segments size="small" />
          <Segments size="medium" />
          <Segments size="large" />
        </Col>
      ),
    },
  ],
}

export default story
