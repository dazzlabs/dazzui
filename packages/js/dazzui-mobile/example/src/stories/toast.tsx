import { Checkmark20Filled, CloudSync20Regular } from '@fluentui/react-icons'
import {
  Button,
  NavigationBar,
  Screen,
  useToast,
} from '@dazzlabs/dazzui-mobile'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  const toast = useToast()
  return (
    <Screen bar={<NavigationBar title="Toast" />}>
      <Stage>
        <Group title="Notices">
          <Col>
            <Button
              variant="filled"
              onClick={() =>
                toast.add({ title: 'Copied', icon: <Checkmark20Filled /> })
              }
            >
              Copy
            </Button>
            <Button
              variant="tonal"
              onClick={() =>
                toast.add({
                  title: 'Note moved to Archive',
                  icon: <CloudSync20Regular />,
                  action: { label: 'Undo', onClick: () => {} },
                })
              }
            >
              Archive with undo
            </Button>
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'toast',
  name: 'Toast',
  summary: 'The glass capsule that drops from the top; Material’s snackbar.',
  Demo,
}

export default story
