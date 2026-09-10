import { Document16Regular, Folder16Regular } from '@fluentui/react-icons'
import { Badge, TINTS, Tree } from '@dazzlabs/dazzui-react'
import type { TreeNode } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof Tree> = {
  title: 'Components/Tree',
  component: Tree,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const file = (id: string, label: string): TreeNode => ({
  id,
  label,
  icon: <Document16Regular />,
})

const folder = (id: string, label: string, children: TreeNode[]): TreeNode => ({
  id,
  label,
  icon: <Folder16Regular />,
  children,
})

const FILES: TreeNode[] = [
  folder('src', 'src', [
    folder('components', 'components', [
      file('badge.tsx', 'badge.tsx'),
      file('button.tsx', 'button.tsx'),
      file('tree.tsx', 'tree.tsx'),
    ]),
    folder('styles', 'styles', [
      file('base.css', 'base.css'),
      file('components.css', 'components.css'),
    ]),
    file('index.ts', 'index.ts'),
  ]),
  folder('empty', 'generated', []),
  file('package.json', 'package.json'),
  file('README.md', 'README.md'),
]

const CHANGED: TreeNode[] = [
  folder('src2', 'src', [
    { ...file('a', 'tree.tsx'), meta: <Badge size="small">M</Badge> },
    {
      ...file('b', 'tree.css'),
      meta: (
        <Badge size="small" tint="success">
          A
        </Badge>
      ),
    },
    { ...file('c', 'menu.tsx'), meta: '+12 −3' },
  ]),
  { ...file('d', 'AGENTS.md'), disabled: true, meta: 'locked' },
]

function FileList({ size }: { size?: 'small' | 'medium' }) {
  const [expanded, setExpanded] = useState<string[]>(['src', 'components'])
  const [selected, setSelected] = useState<string | null>('tree.tsx')
  return (
    <Tree
      aria-label="Files"
      expanded={expanded}
      items={FILES}
      selected={selected}
      size={size}
      onExpandedChange={setExpanded}
      onSelectedChange={setSelected}
    />
  )
}

export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Tree"
      description="A hierarchy the reader walks. This is the one component in the package with no Base UI part under it, because a tree's keyboard is not a list's: ArrowRight opens a shut branch and descends into an open one, ArrowLeft shuts an open branch and climbs out of a leaf, and the whole tree is a single tab stop so focus roves inside it rather than tabbing through a hundred files. None of that falls out of a Collapsible per node — a tree of disclosures is a hundred tab stops and no way across them."
    >
      <Section
        title="Branches and leaves"
        description="`children` present at all — an empty array included — is what makes a node a branch. That is how an empty folder still draws a twisty and still opens, which is the difference between 'nothing in here' and 'not a folder'. The twisty keeps its column on a leaf too, empty: files and folders at one level start at one x, and a leaf that reclaimed the space would put every filename at a different indent from its siblings."
      >
        <Col width="18rem">
          <FileList />
        </Col>
      </Section>

      <Section
        title="Two targets, two meanings"
        description="Clicking a folder opens it and selects it, because in a file tree those are one gesture. Clicking the twisty alone opens it and leaves the selection where it was — which is how a reader looks inside a folder without losing the file they were reading."
      >
        <Col width="18rem">
          <FileList />
        </Col>
      </Section>

      <Section
        title="Selection"
        description="Drawn the way every selected thing in this system is: the tinted wash, and the label at the label weight rather than the quiet one. On a column of forty rows the weight is doing most of the work — one grade of colour is hardest to find in exactly this shape."
      >
        <Col width="18rem">
          <Tree
            aria-label="Changes"
            defaultExpanded={['src2']}
            items={CHANGED}
            selected="a"
          />
        </Col>
      </Section>

      <Section
        title="Sizes"
        description="Two densities, both under control height: a tree is a column of names rather than a column of controls, and a row at 28px would fit six fewer files on the screen for nothing."
      >
        <Col gap="var(--spacing-4)" width="18rem">
          <FileList size="small" />
          <FileList size="medium" />
        </Col>
      </Section>
    </Page>
  ),
}

export const FileTree: Story = {
  render: () => (
    <Col width="18rem">
      <FileList />
    </Col>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col gap="var(--spacing-4)" width="18rem">
      <FileList size="small" />
      <FileList size="medium" />
    </Col>
  ),
}

export const WithMeta: Story = {
  render: () => (
    <Col width="18rem">
      <Tree
        aria-label="Changes"
        defaultExpanded={['src2']}
        items={CHANGED}
        selected="a"
      />
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col gap="var(--spacing-4)" width="18rem">
      {TINTS.map(tint => (
        <Tree
          key={tint}
          aria-label={`Files (${tint})`}
          defaultExpanded={['src']}
          items={FILES}
          selected="index.ts"
          tint={tint}
        />
      ))}
    </Col>
  ),
}

export const Collapsed: Story = {
  render: () => (
    <Col width="18rem">
      <Tree aria-label="Files" items={FILES} />
    </Col>
  ),
}

export const Disabled: Story = {
  render: () => (
    <Col width="18rem">
      <Tree aria-label="Changes" defaultExpanded={['src2']} items={CHANGED} />
    </Col>
  ),
}
