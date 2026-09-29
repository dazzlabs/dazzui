import { ShortcutRecorder } from '@dazzlabs/dazzui-react'
import { useState } from 'react'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

function Recorder({ initial = '' }: { initial?: string }) {
  const [value, setValue] = useState(initial)
  return (
    <ShortcutRecorder
      value={value}
      onValueChange={setValue}
      onClear={() => setValue('')}
      aria-label="Shortcut"
    />
  )
}

const story: Story = {
  id: 'shortcut-recorder',
  name: 'ShortcutRecorder',
  summary: 'A field that takes the keys pressed in it as its value.',
  Preview: () => <Recorder initial="⌘K" />,
  examples: [
    {
      title: 'Empty and set',
      note: 'Click one, then press the keys.',
      Demo: () => (
        <Col align="start">
          <Recorder />
          <Recorder initial="⌥ Space" />
        </Col>
      ),
    },
    {
      title: 'In error and disabled',
      Demo: () => (
        <Col align="start">
          <ShortcutRecorder
            value="⌘C"
            state="error"
            onValueChange={() => {}}
            aria-label="Taken"
          />
          <ShortcutRecorder
            value="⌥⇧2"
            disabled
            onValueChange={() => {}}
            aria-label="Disabled"
          />
        </Col>
      ),
    },
  ],
}

export default story
