import {
  Document20Regular as Doc,
  MoreHorizontal20Regular as More,
  Delete20Regular as Trash,
} from '@fluentui/react-icons'
import {
  Button,
  Icon,
  IconButton,
  KeyCap,
  Menu,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { Row } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Menu> = {
  title: 'Components/Menu',
  component: Menu,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const DENSITIES = [
  { value: 'comfortable', label: 'Comfortable' },
  { value: 'compact', label: 'Compact' },
  { value: 'dense', label: 'Dense' },
]

const Trigger = (
  <Button tint="neutral" variant="normal">
    Options
  </Button>
)

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Menu"
      description="A popover menu. It is positioned against its trigger and flips when it would run off an edge, it roves the arrow keys and types ahead, and it closes on an outside press and on Escape — none of which a menu should have to be told. The trigger is whatever element you pass; the menu wires the press and the `aria-*` onto it."
    >
      <Section
        title="Items"
        description="The row's wash comes from the `normal` recipe, and it follows the *highlight* rather than the pointer: the row under the mouse and the row the arrow keys reached are the same state, so the keyboard sees what the mouse sees. Shortcuts and icons are slots rather than strings, so a caller passes a KeyCap, a bare glyph run, or whatever its platform prints. Give every item an icon or give none of them one: a column half-filled with icons leaves the labels on two different edges."
      >
        <Stack>
          <Specimen label="shortcuts">
            <Menu
              align="start"
              trigger={Trigger}
              items={[
                {
                  label: 'Duplicate',
                  shortcut: <KeyCap size="small">⌘D</KeyCap>,
                },
                { label: 'Rename', shortcut: <KeyCap size="small">↩</KeyCap> },
                {
                  label: 'Export…',
                  shortcut: <KeyCap size="small">⌘E</KeyCap>,
                },
                { label: 'Delete', disabled: true },
              ]}
            />
          </Specimen>
          <Specimen label="icons">
            <Menu
              align="start"
              trigger={Trigger}
              items={[
                { label: 'Open', icon: <Icon icon={Doc} size={14} /> },
                { label: 'Duplicate', icon: <Icon icon={Doc} size={14} /> },
                { label: 'Delete', icon: <Icon icon={Trash} size={14} /> },
              ]}
            />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Checked"
        description="Set `checked` on every item of a group, including the unchecked ones, so the leading column keeps its width and the labels stay on one edge. The items then form a radio group and announce as one."
      >
        <Stack>
          <Specimen label="radio group">
            <Menu
              align="start"
              trigger={Trigger}
              items={DENSITIES.map(entry => ({
                label: entry.label,
                checked: entry.value === 'comfortable',
              }))}
            />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Placement and tint"
        description="`side` is which way it drops and `align` is which edge it lines up with; both give way to the viewport, so a menu at the foot of a window flips up on its own rather than being told to. The tint is the ramp the check glyph resolves against, and it defaults to neutral — a tinted wash on a row of ordinary commands reads as though every one of them were the accent."
      >
        <Stack>
          <Specimen label="start / end">
            <Menu
              align="start"
              trigger={Trigger}
              items={[{ label: 'Aligned to the start edge' }]}
            />
            <Menu
              align="end"
              trigger={Trigger}
              items={[{ label: 'Aligned to the end edge' }]}
            />
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('menu')}
          labelOf={token => token.name.slice('menu-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={Trigger}
      items={[
        { label: 'Duplicate' },
        { label: 'Rename' },
        { label: 'Export…' },
      ]}
    />
  ),
}

export const WithShortcuts: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={Trigger}
      items={[
        { label: 'Duplicate', shortcut: <KeyCap size="small">⌘D</KeyCap> },
        { label: 'Rename', shortcut: <KeyCap size="small">↩</KeyCap> },
        { label: 'Export…', shortcut: <KeyCap size="small">⌘E</KeyCap> },
      ]}
    />
  ),
}

export const WithBareShortcuts: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={Trigger}
      items={[
        { label: 'Duplicate', shortcut: '⌘D' },
        { label: 'Rename', shortcut: '↩' },
        { label: 'Export…', shortcut: '⌘E' },
      ]}
    />
  ),
}

export const WithIcons: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={Trigger}
      items={[
        { label: 'Open', icon: <Icon icon={Doc} size={14} /> },
        { label: 'Duplicate', icon: <Icon icon={Doc} size={14} /> },
        { label: 'Delete', icon: <Icon icon={Trash} size={14} /> },
      ]}
    />
  ),
}

export const Checked: Story = {
  render: () => {
    const [density, setDensity] = useState('comfortable')
    return (
      <Menu
        align="start"
        trigger={Trigger}
        items={DENSITIES.map(entry => ({
          label: entry.label,
          checked: density === entry.value,
          onSelect: () => setDensity(entry.value),
        }))}
      />
    )
  },
}

export const Disabled: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={Trigger}
      items={[
        { label: 'Duplicate' },
        { label: 'Rename' },
        { label: 'Export…', disabled: true },
        { label: 'Delete', disabled: true },
      ]}
    />
  ),
}

export const Alignment: Story = {
  render: () => (
    <div
      style={{
        display: 'flex',
        justifyContent: 'space-between',
        width: '30rem',
        padding: 'var(--spacing-4)',
        borderRadius: 'var(--radius-medium)',
        background: 'var(--color-surface-muted)',
      }}
    >
      <Menu
        align="start"
        trigger={Trigger}
        items={[{ label: 'Aligned to the start edge' }, { label: 'Rename' }]}
      />
      <Menu
        align="end"
        trigger={Trigger}
        items={[{ label: 'Aligned to the end edge' }, { label: 'Rename' }]}
      />
    </div>
  ),
}

/** Dropping upwards, for a trigger sitting in a strip along the window's foot. */
export const DroppingUp: Story = {
  render: () => (
    <Menu
      align="start"
      side="top"
      trigger={Trigger}
      items={[{ label: 'Zoom to fit', shortcut: '⇧1' }, { label: '100%' }]}
    />
  ),
}

export const FromAnIconButton: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={({ open }) => (
        <IconButton active={open} label="More">
          <Icon icon={More} size={16} />
        </IconButton>
      )}
      items={[
        { label: 'Duplicate', shortcut: <KeyCap size="small">⌘D</KeyCap> },
        { label: 'Delete', icon: <Icon icon={Trash} size={14} /> },
      ]}
    />
  ),
}

export const FromAPlainButton: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={
        <Button size="small" variant="plain">
          Sort by ▾
        </Button>
      }
      items={[
        { label: 'Name', checked: true },
        { label: 'Last edited', checked: false },
        { label: 'Size', checked: false },
      ]}
    />
  ),
}

export const Tints: Story = {
  render: () => (
    <Row>
      {TINTS.map(tint => (
        <Menu
          key={tint}
          align="start"
          tint={tint}
          trigger={({ open }) => (
            <Button tint={tint} variant={open ? 'filled' : 'normal'}>
              {tint}
            </Button>
          )}
          items={[
            { label: 'Checked', checked: true },
            { label: 'Unchecked', checked: false },
            { label: 'Hover me' },
          ]}
        />
      ))}
    </Row>
  ),
}
