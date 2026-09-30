import {
  Alert20Filled,
  Cloud20Filled,
  Key20Filled,
  Star24Regular,
} from '@fluentui/react-icons'
import {
  Badge,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  Switch,
} from '@dazzlabs/dazzui-mobile'

import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="List" large />}>
      <List>
        <ListSection header="Rows">
          <ListItem title="Plain" />
          <ListItem title="With a value" value="Detail" />
          <ListItem title="Pressable" onClick={() => {}} />
          <ListItem
            title="Two lines"
            subtitle="The line beneath, in the footnote"
            onClick={() => {}}
          />
        </ListSection>
        <ListSection
          header="With glyphs"
          footer="iOS paints a leading glyph on a coloured tile, as Settings does."
        >
          <ListItem
            icon={<Alert20Filled />}
            iconColor="var(--color-accent-danger)"
            title="Notifications"
            trailing={<Badge count={12} />}
            onClick={() => {}}
          />
          <ListItem
            icon={<Cloud20Filled />}
            iconColor="var(--color-accent-info)"
            title="iCloud"
            value="2 TB"
            onClick={() => {}}
          />
          <ListItem
            icon={<Key20Filled />}
            iconColor="var(--color-accent-neutral)"
            title="Passwords"
            trailing={<Switch defaultChecked aria-label="Passwords" />}
          />
          <ListItem
            icon={<Star24Regular />}
            title="Favourites"
            disabled
            onClick={() => {}}
          />
        </ListSection>
        <ListSection>
          <ListItem title="Sign out" tone="destructive" onClick={() => {}} />
        </ListSection>
      </List>
    </Screen>
  )
}

const story: Story = {
  id: 'list',
  name: 'List',
  summary:
    'Inset grouped sections at the big corner; Material’s flat list with dividers.',
  Demo,
}

export default story
