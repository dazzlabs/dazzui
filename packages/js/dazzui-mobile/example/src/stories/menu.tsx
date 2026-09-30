import {
  ArrowSort24Regular,
  Copy24Regular,
  Delete24Regular,
  MoreHorizontal24Regular,
  Rename24Regular,
  Share24Regular,
} from '@fluentui/react-icons'
import {
  Button,
  IconButton,
  Menu,
  NavigationBar,
  Screen,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  const [sort, setSort] = useState('date')
  return (
    <Screen
      bar={
        <NavigationBar
          title="Menu"
          trailing={
            <Menu
              trigger={
                <IconButton label="More">
                  <MoreHorizontal24Regular />
                </IconButton>
              }
              items={[
                { label: 'Rename', icon: <Rename24Regular /> },
                { label: 'Duplicate', icon: <Copy24Regular /> },
                { label: 'Share', icon: <Share24Regular /> },
                null,
                {
                  label: 'Delete',
                  icon: <Delete24Regular />,
                  role: 'destructive',
                },
              ]}
            />
          }
        />
      }
    >
      <Stage>
        <Group title="From a button">
          <Col>
            <Menu
              align="start"
              trigger={<Button variant="tonal">Actions</Button>}
              items={[
                { label: 'Rename', icon: <Rename24Regular /> },
                { label: 'Duplicate', icon: <Copy24Regular /> },
                { label: 'Share', icon: <Share24Regular />, disabled: true },
                null,
                {
                  label: 'Delete',
                  icon: <Delete24Regular />,
                  role: 'destructive',
                },
              ]}
            />
            <Menu
              align="start"
              trigger={
                <Button variant="glass" icon={<ArrowSort24Regular />}>
                  Sort by {sort}
                </Button>
              }
              items={['date', 'name', 'size'].map(value => ({
                label: value[0]!.toUpperCase() + value.slice(1),
                checked: sort === value,
                onSelect: () => setSort(value),
              }))}
            />
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'menu',
  name: 'Menu',
  summary:
    'The glass pull-down with trailing glyphs; Material’s menu with leading ones.',
  Demo,
}

export default story
