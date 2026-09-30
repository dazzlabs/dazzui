import { Settings20Regular } from '@fluentui/react-icons'
import { Button, Icon, IconButton, Tooltip } from '@dazzlabs/dazzui'

import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'tooltip',
  name: 'Tooltip',
  summary: 'The name of a thing that has no room to say it.',
  Preview: () => (
    <Tooltip label="Settings">
      <IconButton label="Settings">
        <Icon icon={Settings20Regular} />
      </IconButton>
    </Tooltip>
  ),
  examples: [
    {
      title: 'Over an icon button',
      Demo: () => (
        <Tooltip label="Settings">
          <IconButton label="Settings">
            <Icon icon={Settings20Regular} />
          </IconButton>
        </Tooltip>
      ),
    },
    {
      title: 'Sides',
      Demo: () => (
        <Row>
          {(['top', 'bottom', 'left', 'right'] as const).map(side => (
            <Tooltip key={side} side={side} label={`On the ${side}`}>
              <Button>{side}</Button>
            </Tooltip>
          ))}
        </Row>
      ),
    },
  ],
}

export default story
