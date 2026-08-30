import {
  Icon,
  type IconGlyph,
  NavAction,
  NavItem,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof NavItem> = {
  title: 'Components/NavItem',
  component: NavItem,
}

export default meta
type Story = StoryObj<typeof meta>

const Doc: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M5.5 2h5.1a2 2 0 0 1 1.4.6l3.4 3.4a2 2 0 0 1 .6 1.4v8.1A2.5 2.5 0 0 1 13.5 18h-8A2.5 2.5 0 0 1 3 15.5v-11A2.5 2.5 0 0 1 5.5 2Zm5.5 1.9V6a1 1 0 0 0 1 1h2.1L11 3.9Z" />
  </svg>
)

/** Rows are full-width, so they need a column to sit in to be read fairly. */
function Column({ children }: { children: React.ReactNode }) {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: 'var(--spacing-1)',
        width: '11rem',
        padding: 'var(--spacing-2)',
        borderRadius: 'var(--radius-medium)',
        background: 'var(--color-surface-muted)',
      }}
    >
      {children}
    </div>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="NavItem"
      description="One row of a navigation column. The same component fills a sidebar and a rail — they differ by a size, not by a component, because a rail row is a sidebar row at a tighter scale."
    >
      <Section
        title="States"
        description="Hover an idle row: the wash comes off the tinted ramp, which is the opposite of what a menu item or a table row gets. The difference is what selection looks like — a nav row selects with the filled recipe, so a tinted hover previews it from well below. A table row selects at a much lighter shade, and a tinted hover would outweigh it."
      >
        <Stack>
          <Specimen label="medium">
            <Column>
              <NavItem active icon={<Icon icon={Doc} size={14} />}>
                Current page
              </NavItem>
              <NavItem icon={<Icon icon={Doc} size={14} />}>Idle</NavItem>
              <NavItem disabled icon={<Icon icon={Doc} size={14} />}>
                Disabled
              </NavItem>
            </Column>
          </Specimen>

          <Specimen label="small">
            <Column>
              <NavItem active size="small">
                Current page
              </NavItem>
              <NavItem size="small">Idle</NavItem>
              <NavItem disabled size="small">
                Disabled
              </NavItem>
            </Column>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="NavAction"
        description="The action pinned to the foot of a column. Shaped like a row so the column still reads as one list, but printed in the ramp rather than filled with it, because it adds rather than selects."
      >
        <Column>
          <NavItem size="small">Product terms</NavItem>
          <NavItem size="small">Legal</NavItem>
          <NavAction size="small">＋ New glossary</NavAction>
        </Column>
      </Section>

      <Section title="Tints">
        <Stack>
          <Specimen label="selected">
            {TINTS.map(tint => (
              <NavItem key={tint} active tint={tint} style={{ width: 'auto' }}>
                {tint}
              </NavItem>
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('nav-item')}
          labelOf={token => token.name.slice('nav-item-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
