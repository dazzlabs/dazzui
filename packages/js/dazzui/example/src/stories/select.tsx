import { Select } from '@dazzlabs/dazzui'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const LANGUAGES = [
  { value: 'en', label: 'English' },
  { value: 'zh', label: 'Chinese' },
  { value: 'ja', label: 'Japanese' },
  { value: 'de', label: 'German' },
]

const story: Story = {
  id: 'select',
  name: 'Select',
  summary: 'One value out of a list that is too long to lay out.',
  Preview: () => (
    <Col width="15rem">
      <Select options={LANGUAGES} defaultValue="en" aria-label="Language" />
    </Col>
  ),
  examples: [
    {
      title: 'Sizes',
      Demo: () => (
        <Col width="20rem">
          {(['small', 'medium', 'large'] as const).map(size => (
            <Select
              key={size}
              size={size}
              options={LANGUAGES}
              defaultValue="zh"
              aria-label={size}
            />
          ))}
        </Col>
      ),
    },
    {
      title: 'Placeholder and disabled',
      Demo: () => (
        <Col width="20rem">
          <Select
            options={LANGUAGES}
            placeholder="Choose a language"
            aria-label="Language"
          />
          <Select
            options={LANGUAGES}
            defaultValue="ja"
            disabled
            aria-label="Disabled"
          />
        </Col>
      ),
    },
  ],
}

export default story
