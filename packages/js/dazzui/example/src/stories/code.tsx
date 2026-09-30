import { Code, CodeBlock } from '@dazzlabs/dazzui'

import { Col, Text } from '../components/stage'
import type { Story } from '../lib/story'

const SNIPPET = `import { Button } from '@dazzlabs/dazzui'

export function Save() {
  return <Button variant="filled">Save changes</Button>
}`

const story: Story = {
  id: 'code',
  name: 'Code',
  summary: 'Something to be typed exactly, in a line or as a block.',
  Preview: () => (
    <Text>
      Run <Code>pnpm install</Code> first.
    </Text>
  ),
  examples: [
    {
      title: 'In a line',
      Demo: () => (
        <Text>
          Set <Code>data-theme</Code> on the root, or remove it for the default.
        </Text>
      ),
    },
    {
      title: 'A block',
      Demo: () => (
        <Col width="32rem">
          <CodeBlock code={SNIPPET} title="save.tsx" language="tsx" />
        </Col>
      ),
    },
    {
      title: 'With line numbers',
      Demo: () => (
        <Col width="32rem">
          <CodeBlock code={SNIPPET} lineNumbers copyable />
        </Col>
      ),
    },
  ],
}

export default story
