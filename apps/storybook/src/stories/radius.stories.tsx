import type { Meta, StoryObj } from '@storybook/react-vite'

import { Grid, Page, Section, Tile } from '../components/gallery'
import { byPrefix } from '../lib/tokens'

const meta: Meta = {
  title: 'Tokens/Radius',
}

export default meta
type Story = StoryObj<typeof meta>

const radii = byPrefix('radius-', { sorted: true })

export const Scale: Story = {
  render: () => (
    <Page
      title="Radius"
      description="Corner rounding, from square to fully pill-shaped."
    >
      <Section title="Scale">
        <Grid min="14rem">
          {radii.map(token => (
            <Tile key={token.name} token={token}>
              <div
                style={{
                  width: '100%',
                  height: 'var(--base-spacing-16)',
                  borderRadius: `var(${token.property})`,
                  background: 'var(--color-primary-500)',
                }}
              />
            </Tile>
          ))}
        </Grid>
      </Section>
    </Page>
  ),
}
