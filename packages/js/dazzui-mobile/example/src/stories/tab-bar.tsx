import {
  Chat24Regular,
  Home24Regular,
  Map24Regular,
  Person24Regular,
} from '@fluentui/react-icons'
import { NavigationBar, Screen, TabBar, Text } from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import { Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  const [tab, setTab] = useState('home')
  return (
    <Screen
      bar={<NavigationBar title="Tab bar" />}
      tabBar={
        <TabBar
          aria-label="Sections"
          value={tab}
          onValueChange={setTab}
          onSearch={() => {}}
          items={[
            { value: 'home', label: 'Home', icon: <Home24Regular /> },
            { value: 'map', label: 'Map', icon: <Map24Regular /> },
            { value: 'chat', label: 'Chat', icon: <Chat24Regular />, badge: 5 },
            {
              value: 'me',
              label: 'You',
              icon: <Person24Regular />,
              badge: true,
            },
          ]}
        />
      }
    >
      <Stage>
        <Text role="title2">
          {tab[0]?.toUpperCase()}
          {tab.slice(1)}
        </Text>
        {Array.from({ length: 24 }, (_, i) => (
          <Text key={i} tone="secondary">
            Content scrolls under the floating bar.
          </Text>
        ))}
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'tab-bar',
  name: 'Tab bar',
  summary:
    'The floating glass capsule with a search circle beside it; Material’s navigation bar.',
  Demo,
}

export default story
