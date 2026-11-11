import {
  Copy20Regular,
  Delete20Regular,
  Edit20Regular,
  Folder20Regular,
} from '@fluentui/react-icons'
import {
  Button,
  Card,
  ContextMenu,
  Icon,
  KeyCap,
  Menu,
  type MenuItem,
} from '@dazzlabs/dazzui'

import { Col } from '../../components/stage'
import type { StoryExtension } from '../../lib/story'

// The examples the native ports read once they draw a branch row: a row that
// opens a list, and a row in that list that opens another. They join the
// Menu page on React, outside shared conformance, and compile for the
// platforms `platforms` names — the shared story stays flat until the rest
// map the shape.

const PROJECTS: MenuItem[] = [
  { label: 'DazzUI', icon: <Icon icon={Folder20Regular} /> },
  { label: 'Native shell', icon: <Icon icon={Folder20Regular} /> },
]

const MOVE_TO: MenuItem[] = [
  { label: 'Projects', items: PROJECTS },
  { label: 'Archive', icon: <Icon icon={Folder20Regular} /> },
]

const TREE: MenuItem[] = [
  { label: 'Rename', icon: <Icon icon={Edit20Regular} /> },
  {
    label: 'Duplicate',
    icon: <Icon icon={Copy20Regular} />,
    shortcut: <KeyCap variant="hint">⌘D</KeyCap>,
  },
  { label: 'Move to', items: MOVE_TO },
  { label: 'Delete', icon: <Icon icon={Delete20Regular} />, disabled: true },
]

const story: StoryExtension = {
  id: 'menu',
  platforms: ['swiftui', 'gpui'],
  examples: [
    {
      title: 'Through two levels',
      note: 'Hover opens a branch, and an arrow opens it from the keyboard; Escape closes one level, a press outside the whole menu.',
      Demo: () => <Menu trigger={<Button>Actions</Button>} items={TREE} />,
    },
    {
      title: 'A nested right click',
      Demo: () => (
        <ContextMenu items={TREE}>
          <Col width="16rem">
            <Card variant="outlined">Right-click this card</Card>
          </Col>
        </ContextMenu>
      ),
    },
  ],
}

export default story
