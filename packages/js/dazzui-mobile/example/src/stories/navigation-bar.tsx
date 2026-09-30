import {
  Add24Regular,
  Filter24Regular,
  MoreHorizontal24Regular,
} from '@fluentui/react-icons'
import {
  IconButton,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  SegmentedControl,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import type { Story } from '../lib/story'

const SENDERS = [
  'Ada Lovelace',
  'Grace Hopper',
  'Alan Turing',
  'Margaret Hamilton',
  'Edsger Dijkstra',
  'Barbara Liskov',
  'Donald Knuth',
  'Frances Allen',
  'Ken Thompson',
  'Radia Perlman',
  'John McCarthy',
  'Adele Goldberg',
]

function Demo() {
  const [size, setSize] = useState('large')
  const large = size === 'large'
  return (
    <Screen
      bar={
        <NavigationBar
          title="Inbox"
          subtitle="Updated just now"
          large={large}
          back={{ label: 'Mailboxes', onClick: () => {} }}
          trailing={
            <>
              <IconButton label="Filter">
                <Filter24Regular />
              </IconButton>
              <IconButton label="More">
                <MoreHorizontal24Regular />
              </IconButton>
              <IconButton label="Compose" variant="filled">
                <Add24Regular />
              </IconButton>
            </>
          }
        />
      }
    >
      <div style={{ padding: '4px 16px 8px' }}>
        <SegmentedControl
          value={size}
          onValueChange={setSize}
          segments={[
            { value: 'large', label: 'Large title' },
            { value: 'inline', label: 'Inline title' },
          ]}
        />
      </div>
      <List>
        <ListSection
          header="Today"
          footer="Scroll: the large title slides under the bar and the bar takes it; the scroll edge comes up beneath."
        >
          {SENDERS.map((name, i) => (
            <ListItem
              key={name}
              title={name}
              subtitle={i % 2 ? 'Re: the design review' : 'Notes from Friday'}
              value={`${9 + (i % 4)}:${(i * 7) % 60 < 10 ? '0' : ''}${(i * 7) % 60}`}
              onClick={() => {}}
            />
          ))}
        </ListSection>
      </List>
    </Screen>
  )
}

const story: Story = {
  id: 'navigation-bar',
  name: 'Navigation bar',
  summary:
    'A clear bar whose title and scroll edge follow the scroll; Material’s top app bar, small and large.',
  Demo,
}

export default story
