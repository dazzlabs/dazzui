import { KeyCap } from '@dazzlabs/dazzui'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'key-cap',
  name: 'KeyCap',
  summary: 'A key, drawn the way the keyboard shows it.',
  Preview: () => (
    <Row gap="1">
      <KeyCap variant="key">⌘</KeyCap>
      <KeyCap variant="key">⇧</KeyCap>
      <KeyCap variant="key">P</KeyCap>
    </Row>
  ),
  examples: [
    {
      title: 'Variants',
      Demo: () => (
        <Row>
          <KeyCap variant="hint">⌘K</KeyCap>
          <KeyCap variant="strong">⌘K</KeyCap>
          <KeyCap variant="key">⌘K</KeyCap>
        </Row>
      ),
    },
    {
      title: 'Sizes',
      Demo: () => (
        <Row>
          <KeyCap size="small">Esc</KeyCap>
          <KeyCap size="medium">Esc</KeyCap>
          <KeyCap size="large">Esc</KeyCap>
        </Row>
      ),
    },
  ],
}

export default story
