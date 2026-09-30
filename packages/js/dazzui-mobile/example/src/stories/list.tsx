import {
  Alert20Filled,
  Archive24Regular,
  Cloud20Filled,
  Delete24Regular,
  Flag24Regular,
  Key20Filled,
  MailRead24Regular,
  Star24Regular,
} from '@fluentui/react-icons'
import {
  Badge,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  Switch,
} from '@dazzlabs/dazzui-mobile'

import { useState } from 'react'

import type { Story } from '../lib/story'

const MESSAGES = [
  { from: 'Grace Hopper', subject: 'Compiler notes' },
  { from: 'Alan Turing', subject: 'Re: the imitation game' },
  { from: 'Ada Lovelace', subject: 'Engine diagrams, v3' },
  { from: 'Barbara Liskov', subject: 'Substitution, again' },
]

function Demo() {
  const [inbox, setInbox] = useState(MESSAGES)
  const remove = (from: string) =>
    setInbox(list => list.filter(message => message.from !== from))
  return (
    <Screen bar={<NavigationBar title="List" large />}>
      <List>
        <ListSection header="Rows">
          <ListItem title="Plain" />
          <ListItem title="With a value" value="Detail" />
          <ListItem title="Pressable" onClick={() => {}} />
          <ListItem
            title="Two lines"
            subtitle="The line beneath, in the footnote"
            onClick={() => {}}
          />
        </ListSection>
        <ListSection
          header="With glyphs"
          footer="iOS paints a leading glyph on a coloured tile, as Settings does."
        >
          <ListItem
            icon={<Alert20Filled />}
            iconColor="var(--color-accent-danger)"
            title="Notifications"
            trailing={<Badge count={12} />}
            onClick={() => {}}
          />
          <ListItem
            icon={<Cloud20Filled />}
            iconColor="var(--color-accent-info)"
            title="iCloud"
            value="2 TB"
            onClick={() => {}}
          />
          <ListItem
            icon={<Key20Filled />}
            iconColor="var(--color-accent-neutral)"
            title="Passwords"
            trailing={<Switch defaultChecked aria-label="Passwords" />}
          />
          <ListItem
            icon={<Star24Regular />}
            title="Favourites"
            disabled
            onClick={() => {}}
          />
        </ListSection>
        <ListSection
          header="Swipe actions"
          footer="Swipe a row either way. iOS uncovers its buttons and a long swipe performs the outermost; Android colours the background once letting go would act."
        >
          {inbox.map(message => (
            <ListItem
              key={message.from}
              title={message.from}
              subtitle={message.subject}
              onClick={() => {}}
              leadingActions={[
                {
                  label: 'Read',
                  icon: <MailRead24Regular />,
                  tone: 'accent',
                  onAction: () => {},
                },
              ]}
              trailingActions={[
                {
                  label: 'Delete',
                  icon: <Delete24Regular />,
                  tone: 'destructive',
                  onAction: () => remove(message.from),
                },
                {
                  label: 'Flag',
                  icon: <Flag24Regular />,
                  color: 'var(--color-accent-warning)',
                  onAction: () => {},
                },
                {
                  label: 'Archive',
                  icon: <Archive24Regular />,
                  onAction: () => remove(message.from),
                },
              ]}
            />
          ))}
          {inbox.length === 0 ? (
            <ListItem
              title="Restore the messages"
              onClick={() => setInbox(MESSAGES)}
            />
          ) : null}
        </ListSection>
        <ListSection>
          <ListItem title="Sign out" tone="destructive" onClick={() => {}} />
        </ListSection>
      </List>
    </Screen>
  )
}

const story: Story = {
  id: 'list',
  name: 'List',
  summary:
    'Inset grouped sections at the big corner, with swipe actions; Material’s flat list with dividers and swipe-to-dismiss.',
  Demo,
}

export default story
