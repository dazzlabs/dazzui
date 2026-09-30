import {
  Add24Regular,
  Delete24Regular,
  Heart24Regular,
  MoreHorizontal24Regular,
  Share24Regular,
} from '@fluentui/react-icons'
import { IconButton, NavigationBar, Screen } from '@dazzlabs/dazzui-mobile'

import { Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen
      bar={
        <NavigationBar
          title="Icon button"
          back={{ label: 'Back', onClick: () => {} }}
          trailing={
            <>
              <IconButton label="Share">
                <Share24Regular />
              </IconButton>
              <IconButton label="More">
                <MoreHorizontal24Regular />
              </IconButton>
            </>
          }
        />
      }
    >
      <Stage>
        <Group title="Styles">
          <Row>
            <IconButton label="Glass" variant="glass">
              <Heart24Regular />
            </IconButton>
            <IconButton label="Plain" variant="plain">
              <Heart24Regular />
            </IconButton>
            <IconButton label="Filled" variant="filled">
              <Add24Regular />
            </IconButton>
            <IconButton label="Tonal" variant="tonal">
              <Add24Regular />
            </IconButton>
            <IconButton label="Outlined" variant="outlined">
              <Add24Regular />
            </IconButton>
          </Row>
        </Group>
        <Group title="Sizes">
          <Row>
            <IconButton label="Small" size="small" variant="filled">
              <Add24Regular />
            </IconButton>
            <IconButton label="Medium" size="medium" variant="filled">
              <Add24Regular />
            </IconButton>
            <IconButton label="Large" size="large" variant="filled">
              <Add24Regular />
            </IconButton>
          </Row>
        </Group>
        <Group title="Destructive and disabled">
          <Row>
            <IconButton label="Delete" tone="destructive">
              <Delete24Regular />
            </IconButton>
            <IconButton label="Delete" variant="tonal" tone="destructive">
              <Delete24Regular />
            </IconButton>
            <IconButton label="Disabled" variant="filled" disabled>
              <Add24Regular />
            </IconButton>
          </Row>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'icon-button',
  name: 'Icon button',
  summary:
    'A glyph in a glass circle, or Material’s standard, filled, tonal and outlined icon buttons.',
  Demo,
}

export default story
