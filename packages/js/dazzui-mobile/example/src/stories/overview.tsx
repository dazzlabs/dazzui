import {
  Airplane20Filled,
  Bluetooth20Filled,
  Home24Regular,
  Person24Regular,
  PhoneUpdate20Filled,
  Search24Regular,
  Settings20Filled,
  Sparkle24Regular,
  Speaker220Filled,
  Wifi120Filled,
} from '@fluentui/react-icons'
import {
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  SearchField,
  Switch,
  TabBar,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import type { Story } from '../lib/story'

function Demo() {
  const [tab, setTab] = useState('home')
  return (
    <Screen
      bar={<NavigationBar title="Settings" large />}
      onRefresh={() => new Promise(resolve => setTimeout(resolve, 1500))}
      tabBar={
        <TabBar
          value={tab}
          onValueChange={setTab}
          onSearch={() => {}}
          items={[
            { value: 'home', label: 'Home', icon: <Home24Regular /> },
            {
              value: 'new',
              label: 'For You',
              icon: <Sparkle24Regular />,
              badge: 3,
            },
            { value: 'me', label: 'Account', icon: <Person24Regular /> },
          ]}
        />
      }
    >
      <div style={{ padding: '4px 16px 0' }}>
        <SearchField />
      </div>
      <List>
        <ListSection>
          <ListItem
            icon={<Airplane20Filled />}
            iconColor="var(--color-accent-warning)"
            title="Airplane Mode"
            trailing={<Switch aria-label="Airplane Mode" />}
          />
          <ListItem
            icon={<Wifi120Filled />}
            iconColor="var(--color-accent-info)"
            title="Wi-Fi"
            value="Dazzlabs"
            onClick={() => {}}
          />
          <ListItem
            icon={<Bluetooth20Filled />}
            iconColor="var(--color-accent-info)"
            title="Bluetooth"
            value="On"
            onClick={() => {}}
          />
        </ListSection>
        <ListSection
          header="General"
          footer="Sounds and haptics follow the ring switch."
        >
          <ListItem
            icon={<Settings20Filled />}
            iconColor="var(--color-accent-neutral)"
            title="General"
            onClick={() => {}}
          />
          <ListItem
            icon={<Speaker220Filled />}
            iconColor="var(--color-accent-danger)"
            title="Sounds & Haptics"
            subtitle="Ringer at 60%"
            onClick={() => {}}
          />
          <ListItem
            icon={<PhoneUpdate20Filled />}
            iconColor="var(--color-accent-success)"
            title="Software Update"
            value="27.1"
            onClick={() => {}}
          />
        </ListSection>
        <ListSection header="Privacy">
          <ListItem
            title="Location Services"
            trailing={<Switch defaultChecked aria-label="Location Services" />}
          />
          <ListItem
            title="Analytics"
            trailing={<Switch aria-label="Analytics" />}
          />
          <ListItem
            title="Reset All Settings"
            tone="destructive"
            onClick={() => {}}
          />
        </ListSection>
        <ListSection>
          <ListItem
            icon={<Search24Regular />}
            iconColor="var(--color-accent)"
            title="Search"
            subtitle="Suggestions, results and Siri"
            onClick={() => {}}
          />
        </ListSection>
      </List>
    </Screen>
  )
}

const story: Story = {
  id: 'overview',
  name: 'Overview',
  summary:
    'A Settings screen: a large title that scrolls into the bar, a search field, grouped rows, pull to refresh and a tab bar.',
  Demo,
}

export default story
