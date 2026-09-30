import { PillTabs } from '@dazzlabs/dazzui'
import type { Story } from '../lib/story'

const INBOX = [
  { value: 'all', label: 'All', count: 24 },
  { value: 'unread', label: 'Unread', count: 3 },
  { value: 'archived', label: 'Archived' },
]

const story: Story = {
  id: 'pill-tabs',
  name: 'PillTabs',
  summary: 'Tabs that carry a count, for a list cut several ways.',
  Preview: () => (
    <PillTabs items={INBOX} defaultValue="all" aria-label="Inbox" />
  ),
  examples: [
    {
      title: 'With counts',
      Demo: () => (
        <PillTabs items={INBOX} defaultValue="unread" aria-label="Inbox" />
      ),
    },
  ],
}

export default story
