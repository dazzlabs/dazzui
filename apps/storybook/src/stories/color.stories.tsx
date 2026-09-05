import type { Meta, StoryObj } from '@storybook/react-vite'

import { Grid, Page, Ramp, Section, Swatch } from '../components/gallery'
import { byPrefix, groupScales, isColorShade } from '../lib/tokens'

const meta: Meta = {
  title: 'Tokens/Color',
}

export default meta
type Story = StoryObj<typeof meta>

/** Ramps read brand-first; anything added later falls to the end. */
const RAMP_ORDER = [
  'primary',
  'neutral',
  'info',
  'success',
  'warning',
  'danger',
]

/** Single-value colors, ordered the way a surface is composed. */
const ROLE_ORDER = [
  'color-surface',
  'color-surface-muted',
  'color-surface-chrome',
  'color-content',
  'color-content-muted',
  'color-border',
  'color-border-muted',
]

function rankIn(order: string[], key: string): number {
  const index = order.indexOf(key)
  return index === -1 ? Number.MAX_SAFE_INTEGER : index
}

const ramps = groupScales('color-', isColorShade).sort(
  (a, b) => rankIn(RAMP_ORDER, a.key) - rankIn(RAMP_ORDER, b.key)
)

const roles = byPrefix('color-')
  .filter(token => !isColorShade(token.name.split('-').pop() ?? ''))
  .sort((a, b) => rankIn(ROLE_ORDER, a.name) - rankIn(ROLE_ORDER, b.name))

export const Palette: Story = {
  render: () => (
    <Page
      title="Color"
      description="Semantic color ramps. Every step points at a base primitive, so a theme can re-map an entire ramp by changing one reference."
    >
      <Section title="Ramps">
        {ramps.map(ramp => (
          <Ramp key={ramp.key} label={ramp.key} steps={ramp.steps} />
        ))}
      </Section>
    </Page>
  ),
}

export const Roles: Story = {
  render: () => (
    <Page
      title="Color roles"
      description="The single-value colors a component reaches for directly: page surfaces, text, and borders."
    >
      <Section title="Roles">
        <Grid min="18rem">
          {roles.map(token => (
            <Swatch key={token.name} token={token} />
          ))}
        </Grid>
      </Section>
    </Page>
  ),
}
