import {
  Camera24Regular,
  Delete24Regular,
  Image24Regular,
  Link24Regular,
} from '@fluentui/react-icons'
import {
  ActionSheet,
  Button,
  NavigationBar,
  Screen,
} from '@dazzlabs/dazzui-mobile'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Action sheet" />}>
      <Stage>
        <Group title="Choices">
          <Col>
            <ActionSheet
              trigger={<Button variant="filled">Add a photo</Button>}
              title="Add a photo"
              message="Choose where it comes from."
              actions={[
                { label: 'Take Photo', icon: <Camera24Regular /> },
                { label: 'Choose from Library', icon: <Image24Regular /> },
                { label: 'Paste a Link', icon: <Link24Regular /> },
              ]}
            />
            <ActionSheet
              trigger={
                <Button variant="tonal" tone="destructive">
                  Delete photo
                </Button>
              }
              actions={[
                {
                  label: 'Delete Photo',
                  role: 'destructive',
                  icon: <Delete24Regular />,
                },
              ]}
            />
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'action-sheet',
  name: 'Action sheet',
  summary:
    'Choices in a glass group with cancel apart; Material’s list in a bottom sheet.',
  Demo,
}

export default story
