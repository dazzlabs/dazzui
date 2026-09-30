import { SearchField } from '@dazzlabs/dazzui'
import { useState } from 'react'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

function Search() {
  const [value, setValue] = useState('')
  return (
    <SearchField
      value={value}
      onValueChange={setValue}
      placeholder="Search components"
      aria-label="Search"
    />
  )
}

const story: Story = {
  id: 'search-field',
  name: 'SearchField',
  summary: 'A field that filters what is already on screen.',
  Preview: () => (
    <Col width="15rem">
      <Search />
    </Col>
  ),
  examples: [
    {
      title: 'Default',
      Demo: () => (
        <Col width="20rem">
          <Search />
        </Col>
      ),
    },
  ],
}

export default story
