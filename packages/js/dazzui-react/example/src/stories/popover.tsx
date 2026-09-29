import {
  Button,
  Popover,
  PopoverClose,
  TextField,
} from '@dazzlabs/dazzui-react'

import { Col, Row } from '../components/stage'
import type { Story } from '../lib/story'

function Share() {
  return (
    <Popover
      trigger={<Button>Share</Button>}
      title="Share this project"
      description="Anyone with the link can read it."
      width="20rem"
    >
      <Col>
        <TextField
          readOnly
          mono
          defaultValue="https://dazz.example/p/4f9a"
          aria-label="Link"
        />
        <PopoverClose render={<Button variant="filled">Done</Button>} />
      </Col>
    </Popover>
  )
}

const story: Story = {
  id: 'popover',
  name: 'Popover',
  summary: 'A small panel that stays with the thing it was opened from.',
  Preview: Share,
  examples: [
    { title: 'With a title and content', Demo: Share },
    {
      title: 'Sides',
      Demo: () => (
        <Row>
          {(['top', 'bottom', 'left', 'right'] as const).map(side => (
            <Popover
              key={side}
              side={side}
              trigger={<Button>{side}</Button>}
              title={`Opens to the ${side}`}
            />
          ))}
        </Row>
      ),
    },
  ],
}

export default story
