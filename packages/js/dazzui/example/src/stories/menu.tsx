import {
  Copy20Regular,
  Delete20Regular,
  Edit20Regular,
} from '@fluentui/react-icons'
import {
  Button,
  Card,
  ContextMenu,
  Icon,
  KeyCap,
  Menu,
  Menubar,
  type MenuItem,
} from '@dazzlabs/dazzui'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const ACTIONS: MenuItem[] = [
  { label: 'Rename', icon: <Icon icon={Edit20Regular} /> },
  {
    label: 'Duplicate',
    icon: <Icon icon={Copy20Regular} />,
    shortcut: <KeyCap variant="hint">⌘D</KeyCap>,
  },
  { label: 'Delete', icon: <Icon icon={Delete20Regular} />, disabled: true },
]

const VIEW: MenuItem[] = [
  { label: 'Sidebar', checked: true },
  { label: 'Theme panel', checked: true },
  { label: 'Full screen' },
]

const story: Story = {
  id: 'menu',
  name: 'Menu',
  summary: 'Actions kept out of the way until they are asked for.',
  Preview: () => <Menu trigger={<Button>Actions</Button>} items={ACTIONS} />,
  examples: [
    {
      title: 'From a button',
      Demo: () => <Menu trigger={<Button>Actions</Button>} items={ACTIONS} />,
    },
    {
      title: 'From a right click',
      Demo: () => (
        <ContextMenu items={ACTIONS}>
          <Col width="16rem">
            <Card variant="outlined">Right-click this card</Card>
          </Col>
        </ContextMenu>
      ),
    },
    {
      title: 'A menubar',
      Demo: () => (
        <Menubar
          menus={[
            { label: 'File', items: ACTIONS },
            { label: 'View', items: VIEW },
            { label: 'Help', items: [{ label: 'Documentation' }] },
          ]}
        />
      ),
    },
  ],
}

export default story
