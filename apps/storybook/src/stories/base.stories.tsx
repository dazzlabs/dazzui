import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Bar,
  Grid,
  Page,
  Ramp,
  Section,
  Stack,
  Swatch,
  Tile,
  TokenTable,
  useResolvedValue,
} from '../components/gallery'
import {
  byPrefix,
  compareKeys,
  groupScales,
  isColorShade,
  type Token,
} from '../lib/tokens'

const meta: Meta = {
  title: 'Base/Primitives',
}

export default meta
type Story = StoryObj<typeof meta>

function labelOf(prefix: string) {
  return (token: Token) => token.name.slice(prefix.length)
}

const colorRamps = groupScales('base-color-', isColorShade)
const standaloneColors = byPrefix('base-color-', { leafOnly: true })
const spacing = byPrefix('base-spacing-', { sorted: true })
const containers = byPrefix('base-container-', { sorted: true })
const families = byPrefix('base-font-', { leafOnly: true, sorted: true })
const weights = byPrefix('base-font-weight-', { sorted: true })
const leading = byPrefix('base-leading-', { sorted: true })
const tracking = byPrefix('base-tracking-', { sorted: true })
const blur = byPrefix('base-blur-', { sorted: true })
const shadows = byPrefix('base-shadow-', { sorted: true })

/** `base-text-lg-size` and `base-text-lg-line-height` belong to one step. */
const textSteps = byPrefix('base-text-')
  .filter(token => token.name.endsWith('-size'))
  .map(token => token.name.slice('base-text-'.length, -'-size'.length))
  .sort(compareKeys)

function TextStep({ step }: { step: string }) {
  const size = useResolvedValue(`--base-text-${step}-size`)
  const lineHeight = useResolvedValue(`--base-text-${step}-line-height`)
  return (
    <tr>
      <td className="dt-name">{step}</td>
      <td className="dt-value">{size}</td>
      <td className="dt-value">{lineHeight}</td>
    </tr>
  )
}

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
        Dazz, considered interfaces
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
      title="Base colors"
      description="The full primitive palette, generated from Tailwind's theme. Semantic ramps alias these; components never reference them directly."
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

export const Spacing: Story = {
  render: () => (
    <Page
      title="Base spacing"
      description="The full spacing scale, derived from a single base step."
    >
      <Section title="Scale">
        <Stack>
          {spacing.map(token => (
            <Bar
              key={token.name}
              token={token}
              label={token.name.slice('base-spacing-'.length)}
            />
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

export const Typography: Story = {
  render: () => (
    <Page
      title="Base typography"
      description="Families, sizes, weights, and spacing primitives that the composed type styles are assembled from."
    >
      <Section title="Families">
        <Stack>
          {families.map(token => (
            <FamilySample key={token.name} token={token} />
          ))}
        </Stack>
      </Section>

      <Section title="Sizes">
        <div className="dt-table__scroll">
          <table className="dt-table">
            <thead>
              <tr>
                <th>Step</th>
                <th>Size</th>
                <th>Line height</th>
              </tr>
            </thead>
            <tbody>
              {textSteps.map(step => (
                <TextStep key={step} step={step} />
              ))}
            </tbody>
          </table>
        </div>
      </Section>

      <Section title="Weights">
        <Stack>
          {weights.map(token => (
            <WeightSample key={token.name} token={token} />
          ))}
        </Stack>
      </Section>

      <Section title="Leading">
        <TokenTable tokens={leading} labelOf={labelOf('base-leading-')} />
      </Section>

      <Section title="Tracking">
        <TokenTable tokens={tracking} labelOf={labelOf('base-tracking-')} />
      </Section>
    </Page>
  ),
}

export const Sizing: Story = {
  render: () => (
    <Page
      title="Base sizing"
      description="Container max-widths for laying out reading columns and panels."
    >
      <Section title="Containers">
        <TokenTable tokens={containers} labelOf={labelOf('base-container-')} />
      </Section>
    </Page>
  ),
}

export const Effects: Story = {
  render: () => (
    <Page
      title="Base effects"
      description="Primitive shadows and blurs. The semantic effect tokens alias these."
    >
      <Section title="Shadows">
        <Grid min="14rem">
          {shadows.map(token => (
            <Tile key={token.name} token={token}>
              <div
                style={{
                  width: '100%',
                  height: 'var(--base-spacing-14)',
                  borderRadius: 'var(--radius-medium)',
                  background: 'var(--color-surface)',
                  boxShadow: `var(${token.property})`,
                }}
              />
            </Tile>
          ))}
        </Grid>
      </Section>

      <Section title="Blur">
        <TokenTable tokens={blur} labelOf={labelOf('base-blur-')} />
      </Section>
    </Page>
  ),
}
