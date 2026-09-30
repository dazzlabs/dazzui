import { Person20Regular } from '@fluentui/react-icons'
import {
  IconButton,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  SearchField,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import type { Story } from '../lib/story'

const CITIES = [
  'Lisbon',
  'Porto',
  'Coimbra',
  'Braga',
  'Faro',
  'Aveiro',
  'Évora',
  'Funchal',
]

function Demo() {
  const [query, setQuery] = useState('')
  const matches = CITIES.filter(city =>
    city.toLowerCase().includes(query.toLowerCase())
  )
  return (
    <Screen bar={<NavigationBar title="Search" large />}>
      <div style={{ padding: '4px 16px 8px' }}>
        <SearchField
          value={query}
          onValueChange={setQuery}
          placeholder="Cities"
          trailing={
            <IconButton label="Account" variant="plain" size="small">
              <Person20Regular />
            </IconButton>
          }
        />
      </div>
      <List>
        <ListSection
          header={query ? `${matches.length} results` : 'All cities'}
        >
          {matches.map(city => (
            <ListItem key={city} title={city} onClick={() => {}} />
          ))}
        </ListSection>
      </List>
    </Screen>
  )
}

const story: Story = {
  id: 'search-field',
  name: 'Search field',
  summary: 'The glass search capsule; Material’s search bar.',
  Demo,
}

export default story
