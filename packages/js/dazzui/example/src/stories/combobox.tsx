import { Autocomplete, Combobox } from '@dazzlabs/dazzui'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const COUNTRIES = [
  { value: 'cn', label: 'China' },
  { value: 'de', label: 'Germany' },
  { value: 'jp', label: 'Japan' },
  { value: 'nz', label: 'New Zealand' },
  { value: 'us', label: 'United States' },
]

const GROUPED = [
  { label: 'Asia', options: COUNTRIES.slice(0, 1).concat(COUNTRIES[2]!) },
  { label: 'Europe', options: COUNTRIES.slice(1, 2) },
]

const FONTS = ['Inter', 'IBM Plex Sans', 'JetBrains Mono', 'Source Serif']

const story: Story = {
  id: 'combobox',
  name: 'Combobox',
  summary: 'One value out of a list long enough to be searched.',
  Preview: () => (
    <Col width="15rem">
      <Combobox options={COUNTRIES} defaultValue="jp" />
    </Col>
  ),
  examples: [
    {
      title: 'Searching a list',
      Demo: () => (
        <Col width="20rem">
          <Combobox
            options={COUNTRIES}
            placeholder="Choose a country"
            empty="No country by that name"
          />
        </Col>
      ),
    },
    {
      title: 'Grouped, and clearable',
      Demo: () => (
        <Col width="20rem">
          <Combobox options={GROUPED} defaultValue="cn" clearable />
        </Col>
      ),
    },
    {
      title: 'Autocomplete',
      note: 'The same list, offered to a field that also takes what was typed.',
      Demo: () => (
        <Col width="20rem">
          <Autocomplete items={FONTS} placeholder="Font family" />
        </Col>
      ),
    },
  ],
}

export default story
