import {
  TextBold20Regular,
  TextItalic20Regular,
  TextUnderline20Regular,
} from '@fluentui/react-icons'
import { Icon, Toggle, ToggleGroup } from '@dazzlabs/dazzui'
import { useState } from 'react'

import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const MARKS = [
  { value: 'bold', label: <Icon icon={TextBold20Regular} />, hint: 'Bold' },
  {
    value: 'italic',
    label: <Icon icon={TextItalic20Regular} />,
    hint: 'Italic',
  },
  {
    value: 'underline',
    label: <Icon icon={TextUnderline20Regular} />,
    hint: 'Underline',
  },
]

function Marks({ multiple = true }: { multiple?: boolean }) {
  const [marks, setMarks] = useState<readonly string[]>(['bold'])
  return (
    <ToggleGroup
      items={MARKS}
      value={marks}
      onValueChange={setMarks}
      multiple={multiple}
      size="small"
      aria-label="Text style"
    />
  )
}

const story: Story = {
  id: 'toggle',
  name: 'Toggle',
  summary: 'A button that stays pressed.',
  Preview: () => <Marks />,
  examples: [
    {
      title: 'On its own',
      Demo: () => (
        <Row>
          <Toggle label="Bold" size="small" defaultPressed>
            <Icon icon={TextBold20Regular} />
          </Toggle>
          <Toggle label="Italic" size="small">
            <Icon icon={TextItalic20Regular} />
          </Toggle>
          <Toggle size="small" variant="outlined">
            Wrap lines
          </Toggle>
        </Row>
      ),
    },
    { title: 'A group, several at once', Demo: () => <Marks /> },
    {
      title: 'A group, one at a time',
      Demo: () => <Marks multiple={false} />,
    },
  ],
}

export default story
