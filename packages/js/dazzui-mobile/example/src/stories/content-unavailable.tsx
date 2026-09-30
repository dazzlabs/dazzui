import {
  CloudOff48Regular,
  MailInbox48Regular,
  Search48Regular,
  Star48Regular,
} from '@fluentui/react-icons'
import {
  Button,
  Card,
  ContentUnavailable,
  NavigationBar,
  Screen,
} from '@dazzlabs/dazzui-mobile'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Empty States" />}>
      <Stage>
        <Group title="With an action">
          <Card flush>
            <ContentUnavailable
              icon={<CloudOff48Regular />}
              title="You’re Offline"
              description="Connect to the internet to see your latest messages."
              actions={
                <Button variant="filled" size="small">
                  Try Again
                </Button>
              }
            />
          </Card>
        </Group>
        <Group title="Two actions">
          <Card flush>
            <ContentUnavailable
              icon={<MailInbox48Regular />}
              title="No Mail"
              description="Messages you receive will appear here."
            >
              <Button variant="tonal" size="small">
                Compose
              </Button>
              <Button variant="plain" size="small">
                Check Other Mailboxes
              </Button>
            </ContentUnavailable>
          </Card>
        </Group>
        <Group title="Glyph, title and description">
          <Card flush>
            <ContentUnavailable
              icon={<Search48Regular />}
              title="No Results for “Lisbon”"
              description="Check the spelling or try a new search."
            />
          </Card>
        </Group>
        <Group title="Title alone">
          <Card flush>
            <ContentUnavailable title="No Recent Searches" />
          </Card>
        </Group>
        <Group title="Filling its container">
          <Col>
            <Card flush style={{ height: 280 }}>
              <ContentUnavailable
                fill
                icon={<Star48Regular />}
                title="No Favourites"
                description="Tap the star on a place to keep it here."
              />
            </Card>
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'content-unavailable',
  name: 'Content Unavailable',
  summary:
    'SwiftUI’s ContentUnavailableView; the empty state Google’s own apps draw.',
  Demo,
}

export default story
