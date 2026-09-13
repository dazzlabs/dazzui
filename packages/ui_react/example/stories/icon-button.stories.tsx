import {
  Copy20Regular as Copy,
  Pin20Regular as Pin,
  Add20Regular as Plus,
  Delete20Regular as Trash,
} from '@fluentui/react-icons'
import {
  ActionBar,
  type ButtonVariant,
  Divider,
  Icon,
  IconButton,
  SearchField,
  TINTS,
  Toggle,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col, Row } from '../src/components/stage'

const meta: Meta<typeof IconButton> = {
  title: 'Components/IconButton',
  component: IconButton,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const VARIANTS: ButtonVariant[] = [
  'plain',
  'normal',
  'filled',
  'tinted',
  'outlined',
]

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="IconButton"
      description="A square button with no label of its own, so `label` is required and doubles as the native tooltip. It reads the same control recipes as Button, plus a pressed state for a toggle that stays down. It defaults to `plain` on the neutral ramp: a toolbar of these should read as chrome, not as a row of accented actions."
    >
      <Section
        title="Variants"
        description="The same five recipes a Button wears. `plain` is the default because most icon buttons are chrome; `filled` is for the one that leads — a compose, an add."
      >
        <Stack>
          {VARIANTS.map(variant => (
            <Specimen key={variant} label={variant}>
              <IconButton label={`Copy (${variant})`} variant={variant}>
                <Icon icon={Copy} size={16} />
              </IconButton>
              <IconButton
                disabled
                label={`Disabled (${variant})`}
                variant={variant}
              >
                <Icon icon={Copy} size={16} />
              </IconButton>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="It does not latch"
        description="A glyph button that stays down is a `Toggle`, which is square at this same control height when its label is a lone glyph — so a latching glyph and a plain one sit in one toolbar without a seam. Keeping an on-state here as well would be a second drawing for one state, and the system draws a selection one way: a tinted wash and a heavier label."
      >
        <Stack>
          <Specimen label="IconButton — acts and returns">
            <IconButton label="Copy">
              <Icon icon={Copy} size={16} />
            </IconButton>
          </Specimen>
          <Specimen label="Toggle — stays down">
            <Toggle label="Pin window">
              <Icon icon={Pin} size={16} />
            </Toggle>
            <Toggle defaultPressed label="Pinned">
              <Icon icon={Pin} size={16} />
            </Toggle>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Sizes"
        description="The button is square at the shared control height, so it lines up with the labelled buttons and the fields beside it. Size the glyph a couple of steps under the box — a glyph filling its button reads as a full stop."
      >
        <Stack>
          <Specimen label="small">
            <IconButton label="Copy (small)" size="small">
              <Icon icon={Copy} size={14} />
            </IconButton>
          </Specimen>
          <Specimen label="medium">
            <IconButton label="Copy (medium)" size="medium">
              <Icon icon={Copy} size={16} />
            </IconButton>
          </Specimen>
          <Specimen label="large">
            <IconButton label="Copy (large)" size="large">
              <Icon icon={Copy} size={20} />
            </IconButton>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="Neutral by default, which is what a toolbar wants. Tint the one that carries a consequence — the delete — and leave the rest alone, or the row goes back to saying nothing."
      >
        <Stack>
          <Specimen label="plain">
            {TINTS.map(tint => (
              <IconButton key={tint} label={tint} tint={tint}>
                <Icon icon={Copy} size={16} />
              </IconButton>
            ))}
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Plain: Story = {
  render: () => (
    <IconButton label="Copy translation">
      <Icon icon={Copy} size={16} />
    </IconButton>
  ),
}

export const Filled: Story = {
  render: () => (
    <IconButton label="New term" tint="primary" variant="filled">
      <Icon icon={Plus} size={16} />
    </IconButton>
  ),
}

export const Variants: Story = {
  render: () => (
    <Row>
      {VARIANTS.map(variant => (
        <IconButton key={variant} label={variant} variant={variant}>
          <Icon icon={Copy} size={16} />
        </IconButton>
      ))}
    </Row>
  ),
}

export const Disabled: Story = {
  render: () => (
    <Row>
      {VARIANTS.map(variant => (
        <IconButton key={variant} disabled label={variant} variant={variant}>
          <Icon icon={Copy} size={16} />
        </IconButton>
      ))}
    </Row>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Row>
      <IconButton label="Copy (small)" size="small">
        <Icon icon={Copy} size={14} />
      </IconButton>
      <IconButton label="Copy (medium)" size="medium">
        <Icon icon={Copy} size={16} />
      </IconButton>
      <IconButton label="Copy (large)" size="large">
        <Icon icon={Copy} size={20} />
      </IconButton>
    </Row>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      {(['plain', 'tinted', 'filled'] as const).map(variant => (
        <Row key={variant}>
          {TINTS.map(tint => (
            <IconButton
              key={tint}
              label={`${variant} ${tint}`}
              tint={tint}
              variant={variant}
            >
              <Icon icon={Copy} size={16} />
            </IconButton>
          ))}
        </Row>
      ))}
    </Col>
  ),
}

export const AToolbar: Story = {
  render: () => {
    const [pinned, setPinned] = useState(false)
    return (
      <ActionBar>
        <IconButton label="Copy translation">
          <Icon icon={Copy} size={16} />
        </IconButton>
        <Toggle label="Pin window" pressed={pinned} onPressedChange={setPinned}>
          <Icon icon={Pin} size={16} />
        </Toggle>
        <Divider orientation="vertical" />
        <IconButton label="Delete" tint="danger">
          <Icon icon={Trash} size={16} />
        </IconButton>
      </ActionBar>
    )
  },
}

export const InAListHeader: Story = {
  render: () => {
    const [query, setQuery] = useState('')
    return (
      <div style={{ width: '28rem' }}>
        <ActionBar>
          <div style={{ flex: 1 }}>
            <SearchField
              placeholder="Search terms"
              value={query}
              onValueChange={setQuery}
            />
          </div>
          <IconButton label="New term" tint="primary" variant="filled">
            <Icon icon={Plus} size={16} />
          </IconButton>
          <IconButton label="Delete selected" tint="danger">
            <Icon icon={Trash} size={16} />
          </IconButton>
        </ActionBar>
      </div>
    )
  },
}
