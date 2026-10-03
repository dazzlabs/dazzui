import {
  AddRegular,
  BranchRegular,
  CircleRegular,
  HistoryRegular,
  PlayRegular,
  TagRegular,
} from '@fluentui/react-icons'
import { Icon, IconButton, type TabItem, Tabs } from '@dazzlabs/dazzui'
import { type CSSProperties, useState } from 'react'
import type { Story } from '../../lib/story'

const SECTIONS: TabItem[] = [
  { value: 'pulls', label: 'Pull Requests', icon: BranchRegular, count: 2 },
  { value: 'issues', label: 'Issues', icon: CircleRegular, count: 4 },
  { value: 'actions', label: 'Actions', icon: PlayRegular },
  { value: 'releases', label: 'Releases', icon: TagRegular, count: 12 },
  { value: 'history', label: 'History', icon: HistoryRegular },
]

const DOCUMENTS: TabItem[] = [
  { value: 'a', label: 'Query 1', status: 'success', closable: true },
  {
    value: 'b',
    label: 'sessions',
    status: 'primary',
    busy: true,
    closable: true,
  },
  { value: 'c', label: 'events', status: 'danger', closable: true },
  {
    value: 'd',
    label: 'bench',
    detail: 'Mac Studio',
    status: 'warning',
    closable: true,
  },
]

/** A head like the ones the tabs sit in: one row high, its rule under it. */
const head = (width: number): CSSProperties => ({
  display: 'flex',
  width,
  maxWidth: '100%',
  height: 36,
  paddingInline: 'var(--spacing-3)',
  borderBlockEnd: 'var(--stroke-hairline) solid var(--color-border)',
  background: 'var(--color-surface-chrome)',
})

function Sections({ width }: { width: number }) {
  const [value, setValue] = useState('pulls')
  return (
    <div style={head(width)}>
      <Tabs
        aria-label="Sections"
        items={SECTIONS}
        value={value}
        onValueChange={setValue}
      />
    </div>
  )
}

function Documents() {
  const [items, setItems] = useState(DOCUMENTS)
  const [value, setValue] = useState('a')
  return (
    <div style={head(520)}>
      <Tabs
        aria-label="Open documents"
        items={items}
        value={value}
        onValueChange={setValue}
        onClose={closed =>
          setItems(current => current.filter(item => item.value !== closed))
        }
        trailing={
          <IconButton label="New" size="tiny" tint="neutral" variant="plain">
            <Icon icon={AddRegular} size={14} />
          </IconButton>
        }
      />
    </div>
  )
}

const story: Story = {
  id: 'tabs',
  name: 'Tabs',
  summary:
    'The navigation of a view or a toolbar: names in a row, the one showing over a rule.',
  Preview: () => <Sections width={520} />,
  examples: [
    { title: 'Sections', Demo: () => <Sections width={560} /> },
    {
      title: 'Compact',
      note: 'Not enough room for every name: the tabs not showing keep their icons, and the one showing stays whole.',
      Demo: () => <Sections width={340} />,
    },
    {
      title: 'Overflowing',
      note: 'Still not enough: the last ones go behind the more-button.',
      Demo: () => <Sections width={200} />,
    },
    {
      title: 'Documents',
      note: 'A status dot, a close on the one showing and under the pointer, and the control that opens another.',
      Demo: Documents,
    },
  ],
}

export default story
