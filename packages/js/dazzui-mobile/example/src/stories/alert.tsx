import { Delete24Regular } from '@fluentui/react-icons'
import { Alert, Button, NavigationBar, Screen } from '@dazzlabs/dazzui-mobile'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Alert" />}>
      <Stage>
        <Group title="Two actions">
          <Col>
            <Alert
              trigger={<Button variant="filled">Sign out?</Button>}
              title="Sign out of this device?"
              message="Your drafts stay on the device until you sign in again."
              actions={[
                { label: 'Cancel', role: 'cancel' },
                { label: 'Sign Out' },
              ]}
            />
            <Alert
              trigger={
                <Button variant="tonal" tone="destructive">
                  Delete album
                </Button>
              }
              icon={<Delete24Regular />}
              title="Delete “Summer”?"
              message="The 48 photos in it stay in your library."
              actions={[
                { label: 'Cancel', role: 'cancel' },
                { label: 'Delete', role: 'destructive' },
              ]}
            />
          </Col>
        </Group>
        <Group title="Three actions">
          <Alert
            trigger={<Button variant="glass">Unsaved changes</Button>}
            title="Save changes?"
            message="You have unsaved changes to this note."
            actions={[
              { label: 'Save' },
              { label: 'Don’t Save', role: 'destructive' },
              { label: 'Cancel', role: 'cancel' },
            ]}
          />
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'alert',
  name: 'Alert',
  summary:
    'The centred glass alert with hairline actions; Material’s basic dialog.',
  Demo,
}

export default story
