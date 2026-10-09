import {
  Add20Regular as Add,
  ArrowDownload20Regular as Download,
  ArrowLeft20Regular as Back,
  ArrowSync20Regular as Sync,
  ChevronDown20Regular as ChevronDown,
  ChevronRight20Regular as ChevronRight,
  Copy20Regular as Copy,
  Delete20Regular as Delete,
  Open20Regular as Open,
  Send20Regular as Send,
  Settings20Regular as Settings,
  Share20Regular as Share,
} from '@fluentui/react-icons'
import {
  Button,
  type ButtonVariant,
  Icon,
  KeyCap,
  TINTS,
} from '@dazzlabs/dazzui'
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
    {
      title: 'Icons across the variants',
      note: 'The glyph takes the label’s colour, so it follows every recipe.',
      Demo: () => (
        <Row>
          <Button variant="filled">
            <Icon icon={Add} />
            New file
          </Button>
          <Button variant="tinted">
            <Icon icon={Share} />
            Share
          </Button>
          <Button variant="outlined">
            <Icon icon={Copy} />
            Duplicate
          </Button>
          <Button variant="normal">
            <Icon icon={Sync} />
            Sync
          </Button>
          <Button variant="recessed">
            <Icon icon={Settings} />
            Settings
          </Button>
          <Button variant="plain">
            <Icon icon={Back} />
            Back
          </Button>
        </Row>
      ),
    },
    {
      title: 'Icons at every size',
      note: 'The glyph steps with the size profile, as the label does.',
      Demo: () => (
        <Row>
          <Button size="tiny">
            <Icon icon={Add} />
            Tiny
          </Button>
          <Button size="small">
            <Icon icon={Add} />
            Small
          </Button>
          <Button size="medium">
            <Icon icon={Add} />
            Medium
          </Button>
          <Button size="large">
            <Icon icon={Add} />
            Large
          </Button>
        </Row>
      ),
    },
    {
      title: 'Trailing icons',
      note: 'A glyph after the label says where the action goes next.',
      Demo: () => (
        <Row>
          <Button variant="filled">
            Continue
            <Icon icon={ChevronRight} />
          </Button>
          <Button variant="outlined">
            Open in browser
            <Icon icon={Open} />
          </Button>
          <Button>
            More
            <Icon icon={ChevronDown} />
          </Button>
        </Row>
      ),
    },
    {
      title: 'Tinted actions',
      Demo: () => (
        <Row>
          <Button variant="filled" tint="success">
            <Icon icon={Send} />
            Publish
          </Button>
          <Button variant="tinted" tint="danger">
            <Icon icon={Delete} />
            Delete
          </Button>
          <Button variant="filled" disabled>
            <Icon icon={Download} />
            Export
          </Button>
        </Row>
      ),
    },
  ],
}

export default story
