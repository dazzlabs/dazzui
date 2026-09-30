import { Document20Regular, Folder20Regular } from '@fluentui/react-icons'
import { Badge, Icon, Tree, type TreeNode } from '@dazzlabs/dazzui'
import { useState } from 'react'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const file = (id: string, label = id): TreeNode => ({
  id,
  label,
  icon: <Icon icon={Document20Regular} />,
})

const folder = (id: string, children: TreeNode[], label = id): TreeNode => ({
  id,
  label,
  icon: <Icon icon={Folder20Regular} />,
  children,
})

const FILES: TreeNode[] = [
  folder('src', [
    folder('components', [file('button.tsx'), file('tree.tsx')]),
    folder('styles', [file('base.css')]),
    file('index.ts'),
  ]),
  file('package.json'),
  file('README.md'),
]

const CHANGED: TreeNode[] = [
  folder(
    'changed',
    [
      { ...file('a', 'tree.tsx'), meta: <Badge size="small">M</Badge> },
      {
        ...file('b', 'tree.css'),
        meta: (
          <Badge size="small" tint="success">
            A
          </Badge>
        ),
      },
    ],
    'src'
  ),
  { ...file('c', 'AGENTS.md'), disabled: true, meta: 'locked' },
]

function Files({
  items = FILES,
  size,
}: {
  items?: TreeNode[]
  size?: 'small' | 'medium'
}) {
  const [selected, setSelected] = useState<string | null>('tree.tsx')
  return (
    <Tree
      items={items}
      size={size}
      defaultExpanded={['src', 'components', 'changed']}
      selected={selected}
      onSelectedChange={setSelected}
      aria-label="Files"
    />
  )
}

const story: Story = {
  id: 'tree',
  name: 'Tree',
  summary: 'Things inside things, opened a level at a time.',
  Preview: () => (
    <Col width="15rem">
      <Files size="small" />
    </Col>
  ),
  examples: [
    {
      title: 'Files',
      note: 'The arrow keys walk it; typing a name goes to it.',
      Demo: () => (
        <Col width="20rem">
          <Files />
        </Col>
      ),
    },
    {
      title: 'With something beside each row',
      Demo: () => (
        <Col width="20rem">
          <Files items={CHANGED} />
        </Col>
      ),
    },
  ],
}

export default story
