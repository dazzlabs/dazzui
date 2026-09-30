import {
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  Select,
} from '@dazzlabs/dazzui-mobile'

import { Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

const LANGUAGES = [
  { value: 'en', label: 'English' },
  { value: 'pt', label: 'Português' },
  { value: 'zh', label: '中文' },
  { value: 'ja', label: '日本語' },
]

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Select" />}>
      <List>
        <ListSection header="In a row">
          <ListItem
            title={
              <Select label="Language" options={LANGUAGES} defaultValue="en" />
            }
          />
        </ListSection>
      </List>
      <Stage>
        <Group title="Alone">
          <Select aria-label="Language" options={LANGUAGES} defaultValue="pt" />
        </Group>
        <Group title="Empty and disabled">
          <Select
            aria-label="Region"
            options={LANGUAGES}
            placeholder="Choose"
          />
          <Select
            aria-label="Disabled"
            options={LANGUAGES}
            defaultValue="ja"
            disabled
          />
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'select',
  name: 'Select',
  summary: 'The menu picker; Material’s exposed dropdown.',
  Demo,
}

export default story
