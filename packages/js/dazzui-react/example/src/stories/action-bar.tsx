import { ActionBar, Button, KeyCap } from '@dazzlabs/dazzui-react'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'action-bar',
  name: 'ActionBar',
  summary: 'The row of actions a view ends on.',
  Preview: () => (
    <Col width="16rem">
      <ActionBar>
        <Button variant="outlined">Cancel</Button>
        <Button variant="filled">Save</Button>
      </ActionBar>
    </Col>
  ),
  examples: [
    {
      title: 'Alignment',
      Demo: () => (
        <Col width="28rem" gap="5">
          {(['start', 'end', 'between'] as const).map(align => (
            <ActionBar key={align} align={align}>
              <Button variant="plain">{align}</Button>
              <Button variant="filled">Save</Button>
            </ActionBar>
          ))}
        </Col>
      ),
    },
    {
      title: 'With a key hint',
      Demo: () => (
        <Col width="28rem">
          <ActionBar align="between">
            <KeyCap variant="hint">⌘↵ to send</KeyCap>
            <Button variant="filled">Send</Button>
          </ActionBar>
        </Col>
      ),
    },
  ],
}

export default story
