import {
  Accessibility20Filled,
  BrightnessHigh20Filled,
  Bug20Filled,
  Code20Filled,
  Settings20Filled,
} from '@fluentui/react-icons'
import {
  Disclosure,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  Switch,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import type { Story } from '../lib/story'

function Demo() {
  const [details, setDetails] = useState(false)
  return (
    <Screen bar={<NavigationBar title="Disclosure" />}>
      <List>
        <ListSection
          header="Between rows"
          footer="The chevron turns as the rows it holds open beneath it."
        >
          <ListItem
            icon={<Settings20Filled />}
            iconColor="var(--color-accent-neutral)"
            title="General"
            onClick={() => {}}
          />
          <Disclosure
            icon={<BrightnessHigh20Filled />}
            iconColor="var(--color-accent-info)"
            title="Display & Brightness"
            defaultOpen
          >
            <ListItem title="Appearance" value="Automatic" onClick={() => {}} />
            <ListItem title="Text Size" onClick={() => {}} />
            <ListItem
              title="Bold Text"
              trailing={<Switch aria-label="Bold Text" />}
            />
          </Disclosure>
          <ListItem
            icon={<Accessibility20Filled />}
            iconColor="var(--color-accent-info)"
            title="Accessibility"
            onClick={() => {}}
          />
        </ListSection>
        <ListSection header="Nested">
          <Disclosure title="Advanced" subtitle="For people who know">
            <ListItem title="Background Refresh" value="Wi-Fi" />
            <Disclosure title="Developer">
              <ListItem
                title="Show Layout Bounds"
                trailing={<Switch aria-label="Show Layout Bounds" />}
              />
              <ListItem
                title="Slow Animations"
                trailing={<Switch aria-label="Slow Animations" />}
              />
            </Disclosure>
          </Disclosure>
          <Disclosure title="Unavailable" disabled>
            <ListItem title="Never shown" />
          </Disclosure>
        </ListSection>
        <ListSection header="Controlled">
          <ListItem
            title="Show Diagnostics"
            trailing={
              <Switch
                checked={details}
                onCheckedChange={setDetails}
                aria-label="Show Diagnostics"
              />
            }
          />
          <Disclosure
            icon={<Bug20Filled />}
            iconColor="var(--color-accent-danger)"
            title="Diagnostics"
            open={details}
            onOpenChange={setDetails}
          >
            <ListItem
              icon={<Code20Filled />}
              iconColor="var(--color-accent-neutral)"
              title="Logs"
              value="2.4 MB"
            />
            <ListItem
              icon={<Code20Filled />}
              iconColor="var(--color-accent-neutral)"
              title="Crash Reports"
              value="None"
            />
          </Disclosure>
        </ListSection>
      </List>
    </Screen>
  )
}

const story: Story = {
  id: 'disclosure',
  name: 'Disclosure',
  summary:
    'SwiftUI’s DisclosureGroup in a list; Material’s expandable list item.',
  Demo,
}

export default story
