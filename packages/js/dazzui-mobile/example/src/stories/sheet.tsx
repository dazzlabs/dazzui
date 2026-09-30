import {
  Button,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  Sheet,
  SheetClose,
  Text,
} from '@dazzlabs/dazzui-mobile'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

const PLACES = [
  'Coffee Collective',
  'The Book Room',
  'Harbour Market',
  'North Park',
  'City Library',
  'Glasshouse Café',
  'Riverside Gym',
  'Old Town Bakery',
  'Museum of Light',
]

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Sheet" />}>
      <Stage>
        <Group title="Detents">
          <Col>
            <Sheet
              trigger={<Button variant="filled">Fit to content</Button>}
              title="Share"
            >
              <List>
                <ListSection>
                  <ListItem title="Copy link" onClick={() => {}} />
                  <ListItem title="Add to reading list" onClick={() => {}} />
                  <ListItem title="Print" onClick={() => {}} />
                </ListSection>
              </List>
            </Sheet>
            <Sheet
              trigger={<Button variant="tonal">Medium</Button>}
              title="Details"
              detent="medium"
            >
              <Stage>
                <Text>Half the screen, whatever is in it.</Text>
                <SheetClose
                  render={<Button variant="filled" size="large" fullWidth />}
                >
                  Done
                </SheetClose>
              </Stage>
            </Sheet>
            <Sheet
              trigger={<Button variant="glass">Large</Button>}
              title="New Event"
              detent="large"
              onConfirm={() => {}}
              confirmLabel="Add"
            >
              <List>
                <ListSection>
                  <ListItem title="Title" value="Design review" />
                  <ListItem title="Location" value="Studio" />
                </ListSection>
                <ListSection footer="The screen behind recedes on iOS.">
                  <ListItem title="Starts" value="Oct 1, 10:00" />
                  <ListItem title="Ends" value="Oct 1, 11:00" />
                </ListSection>
              </List>
            </Sheet>
          </Col>
        </Group>
        <Group title="Resizable">
          <Sheet
            trigger={<Button variant="outlined">Medium and large</Button>}
            title="Places"
            detents={['medium', 'large']}
          >
            <List>
              <ListSection header="Nearby">
                {PLACES.map(place => (
                  <ListItem
                    key={place}
                    title={place}
                    subtitle="Open until 22:00"
                    onClick={() => {}}
                  />
                ))}
              </ListSection>
            </List>
          </Sheet>
          <Text role="footnote" tone="secondary">
            Drag the grabber between the two detents, or fling the sheet down to
            dismiss it.
          </Text>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'sheet',
  name: 'Sheet',
  summary:
    'A sheet that drags between detents and flings away: floating glass on iOS, Material’s modal bottom sheet.',
  Demo,
}

export default story
