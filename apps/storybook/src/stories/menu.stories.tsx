import { Button, Kbd, Menu } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Menu> = {
  title: 'Components/Menu',
  component: Menu,
}

export default meta
type Story = StoryObj<typeof meta>

function Trigger({ open, onClick }: { open: boolean; onClick: () => void }) {
  return (
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
}

function ViewMenu() {
  const [density, setDensity] = useState('comfortable')

  return (
    <Menu
      align="start"
      trigger={Trigger}
      items={[
        {
          label: 'Comfortable',
          checked: density === 'comfortable',
          onSelect: () => setDensity('comfortable'),
        },
        {
          label: 'Compact',
          checked: density === 'compact',
          onSelect: () => setDensity('compact'),
        },
        {
          label: 'Dense',
          checked: density === 'dense',
          onSelect: () => setDensity('dense'),
        },
      ]}
    />
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="Menu"
      description="A popover menu. It closes on an outside pointer press and on Escape, so it stays operable from the keyboard as well as the mouse. The trigger is a render prop — the menu positions and toggles, it does not decide what opens it."
    >
      <Section
        title="Items"
        description="Hover and press come from the `plain` recipe, so the wash is the same one every quiet control uses. Open one and hover a row."
      >
        <Stack>
          <Specimen label="with shortcuts">
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
        </Stack>
      </Section>

      <Section
        title="Checked"
        description="Set `checked` on every item of a group, including the unchecked ones, so the leading column keeps its width and the labels stay on one edge."
      >
        <Stack>
          <Specimen label="radio group">
            <ViewMenu />
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
