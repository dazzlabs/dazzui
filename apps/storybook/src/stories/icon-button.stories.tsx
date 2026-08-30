import { ActionBar, Icon, IconButton, type IconGlyph } from '@dazzlabs/dazzui'
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

const meta: Meta<typeof IconButton> = {
  title: 'Components/IconButton',
  component: IconButton,
}

export default meta
type Story = StoryObj<typeof meta>

const Pin: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M12.3 2.2a1 1 0 0 1 1.5 0l4 4a1 1 0 0 1-.6 1.7l-2 .2-3.4 3.4.2 2.3a1 1 0 0 1-1.7.8l-2.6-2.6-4 4a.7.7 0 0 1-1-1l4-4-2.6-2.6a1 1 0 0 1 .8-1.7l2.3.2 3.4-3.4.2-2a1 1 0 0 1 .5-.3Z" />
  </svg>
)

const Copy: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M7 2.5h6.5A2.5 2.5 0 0 1 16 5v8.5a1 1 0 1 1-2 0V5a.5.5 0 0 0-.5-.5H7a1 1 0 0 1 0-2Zm-2.5 3h6A2.5 2.5 0 0 1 13 8v7a2.5 2.5 0 0 1-2.5 2.5h-6A2.5 2.5 0 0 1 2 15V8a2.5 2.5 0 0 1 2.5-2.5Z" />
  </svg>
)

function Toolbar() {
  const [pinned, setPinned] = useState(false)
  return (
    <ActionBar>
      <IconButton
        active={pinned}
        label={pinned ? 'Unpin window' : 'Pin window'}
        onClick={() => setPinned(!pinned)}
      >
        <Icon icon={Pin} size={16} />
      </IconButton>
      <IconButton label="Copy translation">
        <Icon icon={Copy} size={16} />
      </IconButton>
      <IconButton active label="Starred" tint="primary">
        <Icon icon={Pin} size={16} />
      </IconButton>
    </ActionBar>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="IconButton"
      description="A square button with no label of its own, so `label` is required and doubles as the tooltip. It reads the same control recipes as Button, plus a pressed state for a toggle that stays down."
    >
      <Section
        title="A toolbar"
        description="Click the pin: an on-state is the tinted recipe whatever variant the button rests in, because “this is on” has to read the same wherever the button sits."
      >
        <Toolbar />
      </Section>

      <Section title="Variants">
        <Stack>
          {(['plain', 'normal', 'filled', 'tinted', 'outlined'] as const).map(
            variant => (
              <Specimen key={variant} label={variant}>
                <IconButton label={`Copy (${variant})`} variant={variant}>
                  <Icon icon={Copy} size={16} />
                </IconButton>
                <IconButton disabled label="Disabled" variant={variant}>
                  <Icon icon={Copy} size={16} />
                </IconButton>
              </Specimen>
            )
          )}
        </Stack>
      </Section>

      <Section title="Sizes">
        <Stack>
          <Specimen label="small / medium / large">
            <IconButton label="Copy (small)" size="small">
              <Icon icon={Copy} size={14} />
            </IconButton>
            <IconButton label="Copy (medium)" size="medium">
              <Icon icon={Copy} size={16} />
            </IconButton>
            <IconButton label="Copy (large)" size="large">
              <Icon icon={Copy} size={20} />
            </IconButton>
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('icon-button')}
          labelOf={token => token.name.slice('icon-button-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
