import { Add24Regular, MoreHorizontal24Regular } from '@fluentui/react-icons'
import {
  Button,
  IconButton,
  NavigationBar,
  Screen,
  Text,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import { Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  const [large, setLarge] = useState(true)
  return (
    <Screen
      bar={
        <NavigationBar
          title="Inbox"
          large={large}
          back={{ label: 'Mail', onClick: () => {} }}
          trailing={
            <>
              <IconButton label="Compose">
                <Add24Regular />
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
        <Button variant="tonal" onClick={() => setLarge(value => !value)}>
          {large ? 'Collapse the title' : 'Expand the title'}
        </Button>
        {Array.from({ length: 14 }, (_, i) => (
          <Text key={i} tone={i % 3 ? 'secondary' : 'primary'}>
            {i % 3 ? 'Content scrolls under the bar.' : 'A row of content.'}
          </Text>
        ))}
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'navigation-bar',
  name: 'Navigation bar',
  summary:
    'A large title over glass controls; Material’s top app bar, small and large.',
  Demo,
}

export default story
