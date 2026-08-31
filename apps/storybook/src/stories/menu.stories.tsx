import {
  Button,
  Icon,
  IconButton,
  type IconGlyph,
  Kbd,
  Menu,
  type MenuProps,
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

const More: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M10 4.5a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3Zm0 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3Zm0 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3Z" />
  </svg>
)

const Doc: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M5.5 2h5.1a2 2 0 0 1 1.4.6l3.4 3.4a2 2 0 0 1 .6 1.4v8.1A2.5 2.5 0 0 1 13.5 18h-8A2.5 2.5 0 0 1 3 15.5v-11A2.5 2.5 0 0 1 5.5 2Zm5.5 1.9V6a1 1 0 0 0 1 1h2.1L11 3.9Z" />
  </svg>
)

const Trash: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M8 2h4a1 1 0 0 1 1 1v1h3.5a1 1 0 1 1 0 2H3.5a1 1 0 0 1 0-2H7V3a1 1 0 0 1 1-1Zm-3 6h10l-.7 8.1A2 2 0 0 1 12.3 18H7.7a2 2 0 0 1-2-1.9L5 8Z" />
  </svg>
)

const Trigger: MenuProps['trigger'] = ({ open, onClick }) => (
  <Button
    aria-expanded={open}
    aria-haspopup="menu"
    tint="neutral"
    variant={open ? 'filled' : 'normal'}
    onClick={onClick}
  >
    Options
  </Button>
)

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Menu"
      description="A popover menu. It closes on an outside pointer press and on Escape, so it stays operable from the keyboard as well as the mouse. The trigger is a render prop — the menu positions and toggles, it does not decide what opens it."
    >
      <Section
        title="Items"
        description="Hover and press come from the `plain` recipe, so the wash is the same one every quiet control uses. Shortcuts and icons are slots rather than strings, so a caller passes a Kbd, a bare glyph run, or whatever its platform prints. Give every item an icon or give none of them one: a column half-filled with icons leaves the labels on two different edges."
      >
        <Stack>
          <Specimen label="shortcuts">
            <Menu
              align="start"
              trigger={Trigger}
              items={[
                { label: 'Duplicate', shortcut: <Kbd size="small">⌘D</Kbd> },
                { label: 'Rename', shortcut: <Kbd size="small">↩</Kbd> },
                { label: 'Export…', shortcut: <Kbd size="small">⌘E</Kbd> },
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
        description="Set `checked` on every item of a group, including the unchecked ones, so the leading column keeps its width and the labels stay on one edge. The items then announce as `menuitemradio` rather than as plain items."
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
        title="Alignment and tint"
        description="`end` is the default, for a trigger sitting at the trailing edge of a toolbar; `start` is for one at the leading edge, where an end-aligned panel would run off the wrong side. The tint is the ramp the hover wash and the check glyph resolve against, and it defaults to neutral — a tinted wash on a row of ordinary commands reads as though every one of them were the accent."
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
        { label: 'Duplicate', shortcut: <Kbd size="small">⌘D</Kbd> },
        { label: 'Rename', shortcut: <Kbd size="small">↩</Kbd> },
        { label: 'Export…', shortcut: <Kbd size="small">⌘E</Kbd> },
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

export const FromAnIconButton: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={({ open, onClick }) => (
        <IconButton
          active={open}
          aria-expanded={open}
          aria-haspopup="menu"
          label="More"
          onClick={onClick}
        >
          <Icon icon={More} size={16} />
        </IconButton>
      )}
      items={[
        { label: 'Duplicate', shortcut: <Kbd size="small">⌘D</Kbd> },
        { label: 'Delete', icon: <Icon icon={Trash} size={14} /> },
      ]}
    />
  ),
}

export const FromAPlainButton: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={({ open, onClick }) => (
        <Button
          aria-expanded={open}
          aria-haspopup="menu"
          size="small"
          variant="plain"
          onClick={onClick}
        >
          Sort by ▾
        </Button>
      )}
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
          trigger={({ open, onClick }) => (
            <Button
              aria-expanded={open}
              aria-haspopup="menu"
              tint={tint}
              variant={open ? 'filled' : 'normal'}
              onClick={onClick}
            >
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
