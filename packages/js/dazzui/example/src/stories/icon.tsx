import {
  Add20Regular,
  Delete20Regular,
  Document20Regular,
  Folder20Regular,
  Home20Regular,
  Search20Regular,
  Settings20Regular,
} from '@fluentui/react-icons'
import { Icon } from '@dazzlabs/dazzui'

import { Row, Text } from '../components/stage'
import type { Story } from '../lib/story'

const GLYPHS = [
  Home20Regular,
  Search20Regular,
  Folder20Regular,
  Document20Regular,
  Add20Regular,
  Settings20Regular,
  Delete20Regular,
]

const story: Story = {
  id: 'icon',
  name: 'Icon',
  summary: 'A glyph, set at the size and in the colour of the text beside it.',
  Preview: () => (
    <Row gap="4">
      {GLYPHS.slice(0, 5).map((glyph, index) => (
        <Icon key={index} icon={glyph} />
      ))}
    </Row>
  ),
  examples: [
    {
      title: 'Glyphs',
      Demo: () => (
        <Row gap="4">
          {GLYPHS.map((glyph, index) => (
            <Icon key={index} icon={glyph} />
          ))}
        </Row>
      ),
    },
    {
      title: 'Sizes',
      Demo: () => (
        <Row gap="4">
          {[12, 16, 20, 24, 32].map(size => (
            <Icon key={size} icon={Settings20Regular} size={size} />
          ))}
        </Row>
      ),
    },
    {
      title: 'In the colour of its text',
      Demo: () => (
        <Row gap="4">
          {(['primary', 'success', 'warning', 'danger'] as const).map(tint => (
            <Text key={tint} tint={tint}>
              <Icon icon={Folder20Regular} />
            </Text>
          ))}
        </Row>
      ),
    },
  ],
}

export default story
