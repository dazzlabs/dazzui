import type { Meta, StoryObj } from '@storybook/react-vite'
import type { CSSProperties } from 'react'

import { Grid, Page, Section, Tile, TokenTable } from '../components/gallery'
import { byPrefix, type Token } from '../lib/tokens'

const meta: Meta = {
  title: 'Tokens/Effect',
}

export default meta
type Story = StoryObj<typeof meta>

const shadows = byPrefix('shadow-', { sorted: true })
const insetShadows = byPrefix('inset-shadow-', { sorted: true })
const dropShadows = byPrefix('drop-shadow-', { sorted: true })
const textShadows = byPrefix('text-shadow-', { sorted: true })

function labelOf(prefix: string) {
  return (token: Token) => token.name.slice(prefix.length)
}

function Box({ style }: { style: CSSProperties }) {
  return (
    <div
      style={{
        width: '100%',
        height: 'var(--base-spacing-14)',
        borderRadius: 'var(--radius-medium)',
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
      description="Elevation shadows, applied as `box-shadow`."
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

export const InsetShadow: Story = {
  render: () => (
    <Page
      title="Inset shadow"
      description="Inner shadows. The resolved value below shows whether the `inset` keyword survives the alias chain."
    >
      <Section title="Inset shadow">
        <Grid min="14rem">
          {insetShadows.map(token => (
            <Tile key={token.name} token={token}>
              <Box
                style={{
                  boxShadow: `var(${token.property})`,
                  background: 'var(--color-surface-muted)',
                }}
              />
            </Tile>
          ))}
        </Grid>
      </Section>
    </Page>
  ),
}

export const DropShadow: Story = {
  render: () => (
    <Page
      title="Drop shadow"
      description="Shadows that follow an element's alpha channel, applied as `filter: drop-shadow()`. That function takes at most three lengths, so any token carrying a spread renders without a shadow."
    >
      <Section title="Drop shadow">
        <Grid min="14rem">
          {dropShadows.map(token => (
            <Tile key={token.name} token={token}>
              <Box style={{ filter: `drop-shadow(var(${token.property}))` }} />
            </Tile>
          ))}
        </Grid>
      </Section>
      <Section title="Values">
        <TokenTable tokens={dropShadows} labelOf={labelOf('drop-shadow-')} />
      </Section>
    </Page>
  ),
}

export const TextShadow: Story = {
  render: () => (
    <Page
      title="Text shadow"
      description="Shadows behind text. `text-shadow` takes at most three lengths, so any token carrying a spread renders without a shadow."
    >
      <Section title="Text shadow">
        <Grid min="14rem">
          {textShadows.map(token => (
            <Tile key={token.name} token={token}>
              <span
                style={{
                  fontFamily: 'var(--headline-small-font-family)',
                  fontSize: 'var(--headline-small-font-size)',
                  fontWeight: 'var(--headline-small-font-weight)',
                  textShadow: `var(${token.property})`,
                }}
              >
                Aa
              </span>
            </Tile>
          ))}
        </Grid>
      </Section>
      <Section title="Values">
        <TokenTable tokens={textShadows} labelOf={labelOf('text-shadow-')} />
      </Section>
    </Page>
  ),
}
