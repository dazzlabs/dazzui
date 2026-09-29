import { ArrowDownload20Regular as Download } from '@fluentui/react-icons'
import {
  Button,
  type ButtonVariant,
  Icon,
  KeyCap,
  TINTS,
} from '@dazzlabs/dazzui-react'
import { Col, Row } from '../components/stage'
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
  id: 'button',
  name: 'Button',
  summary: 'The thing a person presses to make something happen.',
  Preview: () => (
    <Row>
      <Button variant="filled">Save changes</Button>
      <Button variant="outlined">Cancel</Button>
    </Row>
  ),
  examples: [
    {
      title: 'Variants',
      note: 'Six weights, from the one action a view is for to the one it barely mentions.',
      Demo: () => (
        <Row>
          {VARIANTS.map(variant => (
            <Button key={variant} variant={variant}>
              {variant}
            </Button>
          ))}
        </Row>
      ),
    },
    {
      title: 'Tints',
      Demo: () => (
        <Col>
          <Row>
            {TINTS.map(tint => (
              <Button key={tint} variant="filled" tint={tint}>
                {tint}
              </Button>
            ))}
          </Row>
          <Row>
            {TINTS.map(tint => (
              <Button key={tint} variant="tinted" tint={tint}>
                {tint}
              </Button>
            ))}
          </Row>
        </Col>
      ),
    },
    {
      title: 'Sizes',
      Demo: () => (
        <Row>
          <Button size="tiny">Tiny</Button>
          <Button size="small">Small</Button>
          <Button size="medium">Medium</Button>
          <Button size="large">Large</Button>
        </Row>
      ),
    },
    {
      title: 'With an icon or a shortcut',
      Demo: () => (
        <Row>
          <Button variant="filled">
            <Icon icon={Download} />
            Export
          </Button>
          <Button shortcut={<KeyCap>⌘S</KeyCap>}>Save</Button>
          <Button disabled>Disabled</Button>
        </Row>
      ),
    },
  ],
}

export default story
