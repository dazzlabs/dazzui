import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Grid,
  Page,
  Ramp,
  Section,
  Stack,
  Swatch,
  TokenTable,
  TYPE_SAMPLE,
  useResolvedValue,
} from '../components/gallery'
import { byPrefix, groupScales, isColorShade, type Token } from '../lib/tokens'

const meta: Meta = {
  title: 'Tokens/Primitives',
}

export default meta
type Story = StoryObj<typeof meta>

function labelOf(prefix: string) {
  return (token: Token) => token.name.slice(prefix.length)
}

const colorRamps = groupScales('base-color-', isColorShade)
const standaloneColors = byPrefix('base-color-', { leafOnly: true })
const corners = byPrefix('base-corner-', { sorted: true })
const typeSteps = byPrefix('base-type-', { sorted: true })
const leading = byPrefix('base-leading-', { sorted: true })
const controlHeights = byPrefix('base-control-height-', { sorted: true })
const families = byPrefix('base-font-', { leafOnly: true, sorted: true })
const weights = byPrefix('base-font-weight-', { sorted: true })

function FamilySample({ token }: { token: Token }) {
  const resolved = useResolvedValue(token.property)
  return (
    <div>
      <div className="dt-name">{token.property}</div>
      <p
        style={{
          margin: 'var(--spacing-2) 0',
          fontFamily: `var(${token.property})`,
          fontSize: 'var(--title-medium-font-size)',
        }}
      >
        {TYPE_SAMPLE}
      </p>
      <div className="dt-value">{resolved}</div>
    </div>
  )
}

function WeightSample({ token }: { token: Token }) {
  const resolved = useResolvedValue(token.property)
  return (
    <div>
      <span
        style={{
          fontSize: 'var(--title-medium-font-size)',
          fontWeight: `var(${token.property})`,
        }}
      >
        {token.name.slice('base-font-weight-'.length)}
      </span>{' '}
      <span className="dt-value">{resolved}</span>
    </div>
  )
}

export const Colors: Story = {
  render: () => (
    <Page
      title="Primitive colors"
      description="The whole primitive palette, hand-authored: the three brand ramps the design specifies, and the five semantic hues frozen from Tailwind at the values it shipped. Semantic ramps alias these; components never reference them directly."
    >
      <Section title="Standalone">
        <Grid min="18rem">
          {standaloneColors.map(token => (
            <Swatch key={token.name} token={token} />
          ))}
        </Grid>
      </Section>
      <Section title="Ramps">
        {colorRamps.map(ramp => (
          <Ramp key={ramp.key} label={ramp.key} steps={ramp.steps} />
        ))}
      </Section>
    </Page>
  ),
}

export const Metrics: Story = {
  render: () => (
    <Page
      title="Primitive metrics"
      description="The AppKit scales the design is drawn to: corners, the type scale with its paired leading, and the control heights. `radius`, the typography faces, and the control profiles alias these."
    >
      <Section title="Corners">
        <TokenTable tokens={corners} labelOf={labelOf('base-corner-')} />
      </Section>

      <Section title="Type scale">
        <TokenTable tokens={typeSteps} labelOf={labelOf('base-type-')} />
      </Section>

      <Section title="Leading">
        <TokenTable tokens={leading} labelOf={labelOf('base-leading-')} />
      </Section>

      <Section title="Control heights">
        <TokenTable
          tokens={controlHeights}
          labelOf={labelOf('base-control-height-')}
        />
      </Section>
    </Page>
  ),
}

export const Typography: Story = {
  render: () => (
    <Page
      title="Primitive typography"
      description="The typeface stacks and weights the composed type styles are assembled from."
    >
      <Section title="Families">
        <Stack>
          {families.map(token => (
            <FamilySample key={token.name} token={token} />
          ))}
        </Stack>
      </Section>

      <Section title="Weights">
        <Stack>
          {weights.map(token => (
            <WeightSample key={token.name} token={token} />
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}
