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
              title="Editor"
              detent="large"
            >
              <Stage>
                <Text>All the way up, a little short of the top edge.</Text>
              </Stage>
            </Sheet>
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'sheet',
  name: 'Sheet',
  summary:
    'A sheet from the bottom edge, with a grabber and the big corner; Material’s modal bottom sheet.',
  Demo,
}

export default story
