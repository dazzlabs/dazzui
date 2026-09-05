import type { Meta, StoryObj } from '@storybook/react-vite'
import type { CSSProperties } from 'react'

import { Grid, Page, Section, Tile } from '../components/gallery'
import { byPrefix } from '../lib/tokens'

const meta: Meta = {
  title: 'Tokens/Effect',
}

export default meta
type Story = StoryObj<typeof meta>

const shadows = byPrefix('shadow-', { sorted: true })

function Box({ style }: { style: CSSProperties }) {
  return (
    <div
      style={{
        width: '100%',
        height: 'var(--spacing-14)',
        borderRadius: 'var(--radius-large)',
        background: 'var(--color-surface)',
        ...style,
      }}
    />
  )
}

export const Shadow: Story = {
  render: () => (
    <Page
      title="Shadow"
      description="Elevation, applied as `box-shadow`. Each step is one of the design's roles — `2xs` the hairline lift under a filled button, `sm` a lifted chip, `md` a floating panel, `lg` a popover, `xl` a window — and the dark themes carry their own heavier set, since a shadow over a near-black canvas has to work harder to separate anything."
    >
      <Section title="Shadow">
        <Grid min="14rem">
          {shadows.map(token => (
            <Tile key={token.name} token={token}>
              <Box style={{ boxShadow: `var(${token.property})` }} />
            </Tile>
          ))}
        </Grid>
      </Section>
    </Page>
  ),
}
