import {
  Add20Regular as Plus,
  Delete20Regular as Trash,
  Settings20Regular as Gear,
} from '@fluentui/react-icons'
import { type ButtonVariant, Icon, IconButton } from '@dazzlabs/dazzui'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const VARIANTS: ButtonVariant[] = [
  'filled',
  'tinted',
  'outlined',
  'normal',
  'recessed',
  'plain',
]

const story: Story = {
  id: 'icon-button',
  name: 'IconButton',
  summary: 'A button that is only its glyph, with the label kept for a reader.',
  Preview: () => (
    <Row>
      <IconButton label="Add">
        <Icon icon={Plus} />
      </IconButton>
      <IconButton label="Settings" variant="tinted">
        <Icon icon={Gear} />
      </IconButton>
      <IconButton label="Delete" variant="filled" tint="danger">
        <Icon icon={Trash} />
      </IconButton>
    </Row>
  ),
  examples: [
    {
      title: 'Variants',
      Demo: () => (
        <Row>
          {VARIANTS.map(variant => (
            <IconButton key={variant} label={variant} variant={variant}>
              <Icon icon={Gear} />
            </IconButton>
          ))}
        </Row>
      ),
    },
    {
      title: 'Sizes',
      Demo: () => (
        <Row>
          {(['tiny', 'small', 'medium', 'large'] as const).map(size => (
            <IconButton key={size} label={size} size={size}>
              <Icon icon={Plus} />
            </IconButton>
          ))}
        </Row>
      ),
    },
  ],
}

export default story
