import {
  Board20Regular as Board,
  TextAlignCenter20Regular as AlignCenter,
  TextAlignJustify20Regular as AlignJustify,
  TextAlignLeft20Regular as AlignLeft,
  TextAlignRight20Regular as AlignRight,
  TextBulletList20Regular as List,
  Timeline20Regular as Timeline,
} from '@fluentui/react-icons'
import { SegmentedControl, type SegmentedItem } from '@dazzlabs/dazzui'
import { useState } from 'react'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const VIEWS = [
  { value: 'list', label: 'List' },
  { value: 'board', label: 'Board' },
  { value: 'timeline', label: 'Timeline' },
]

const VIEWS_WITH_ICONS = [
  { value: 'list', label: 'List', icon: List },
  { value: 'board', label: 'Board', icon: Board },
  { value: 'timeline', label: 'Timeline', icon: Timeline },
]

const ALIGNMENTS = [
  { value: 'left', label: 'Align left', icon: AlignLeft },
  { value: 'center', label: 'Align center', icon: AlignCenter },
  { value: 'right', label: 'Align right', icon: AlignRight },
  { value: 'justify', label: 'Justify', icon: AlignJustify },
]

function Segments({
  items = VIEWS,
  initial = 'list',
  size,
  iconOnly,
  activeStyle,
  label = 'View',
}: {
  items?: SegmentedItem[]
  initial?: string
  size?: 'small' | 'medium' | 'large'
  iconOnly?: boolean
  activeStyle?: 'raised' | 'filled'
  label?: string
}) {
  const [view, setView] = useState(initial)
  return (
    <SegmentedControl
      items={items}
      value={view}
      onValueChange={setView}
      size={size}
      iconOnly={iconOnly}
      activeStyle={activeStyle}
      aria-label={label}
    />
  )
}

const story: Story = {
  id: 'segmented-control',
  name: 'SegmentedControl',
  summary: 'A few views of one thing, and the one being looked at.',
  Preview: () => <Segments />,
  examples: [
    {
      title: 'Sizes',
      Demo: () => (
        <Col align="start">
          <Segments size="small" />
          <Segments size="medium" />
          <Segments size="large" />
        </Col>
      ),
    },
    {
      title: 'With icons',
      note: 'A glyph before each label, drawn a half-step under the label’s height.',
      Demo: () => (
        <Col align="start">
          <Segments items={VIEWS_WITH_ICONS} size="small" />
          <Segments items={VIEWS_WITH_ICONS} size="medium" />
          <Segments items={VIEWS_WITH_ICONS} size="large" />
        </Col>
      ),
    },
    {
      title: 'Icons only',
      note: 'The labels stay the segments’ names for a screen reader.',
      Demo: () => (
        <Col align="start">
          <Segments
            items={ALIGNMENTS}
            initial="left"
            iconOnly
            size="small"
            label="Alignment"
          />
          <Segments
            items={ALIGNMENTS}
            initial="left"
            iconOnly
            size="medium"
            label="Alignment"
          />
          <Segments
            items={ALIGNMENTS}
            initial="left"
            iconOnly
            size="large"
            label="Alignment"
          />
        </Col>
      ),
    },
    {
      title: 'Filled, with icons',
      Demo: () => (
        <Col align="start">
          <Segments items={VIEWS_WITH_ICONS} activeStyle="filled" />
          <Segments
            items={ALIGNMENTS}
            initial="left"
            iconOnly
            activeStyle="filled"
            label="Alignment"
          />
        </Col>
      ),
    },
  ],
}

export default story
