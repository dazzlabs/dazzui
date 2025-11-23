import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  TokenTable,
  useResolvedValue,
} from '../components/gallery'
import { byPrefix, getToken, type Token } from '../lib/tokens'

const meta: Meta = {
  title: 'Tokens/Control',
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['tiny', 'small', 'medium', 'large', 'big']
const METRICS = ['size', 'gap', 'padding-block', 'padding-inline']

const VARIANTS = ['normal', 'filled', 'tinted', 'outlined', 'plain']
const PARTS = ['surface', 'content', 'border']
const STATES = ['normal', 'hovered', 'pressed', 'disabled']
const RECIPE_PROPS = ['color', 'shade', 'alpha'] as const

type RecipeProp = (typeof RECIPE_PROPS)[number]
type Recipe = Partial<Record<RecipeProp, Token>>

/** Known keys first, then anything the tokens added since. */
function ordered(found: Set<string>, preferred: string[]): string[] {
  const extras = [...found].filter(key => !preferred.includes(key)).sort()
  return [...preferred.filter(key => found.has(key)), ...extras]
}

function isRecipeProp(value: string): value is RecipeProp {
  return (RECIPE_PROPS as readonly string[]).includes(value)
}

const colorRecipes = new Map<string, Recipe>()
const foundVariants = new Set<string>()

for (const token of byPrefix('control-color-')) {
  const parts = token.name.slice('control-color-'.length).split('-')
  if (parts.length !== 4) continue

  const [variant, part, state, prop] = parts
  if (!isRecipeProp(prop)) continue

  foundVariants.add(variant)
  const key = `${variant}/${part}/${state}`
  const recipe = colorRecipes.get(key) ?? {}
  recipe[prop] = token
  colorRecipes.set(key, recipe)
}

const variants = ordered(foundVariants, VARIANTS)

function MetricCell({ size, metric }: { size: string; metric: string }) {
  const name = `control-${size}-${metric}`
  const value = useResolvedValue(`--${name}`)
  const token = getToken(name)
  return <td className="dt-value">{token ? value : '—'}</td>
}

function RecipeValue({ token, prop }: { token: Token; prop: RecipeProp }) {
  const value = useResolvedValue(token.property)
  return (
    <div className="dt-value">
      {prop}: {value}
    </div>
  )
}

function RecipeCell({ recipe }: { recipe: Recipe | undefined }) {
  if (!recipe) return <td className="dt-value">—</td>
  return (
    <td>
      {RECIPE_PROPS.map(prop => {
        const token = recipe[prop]
        return token ? (
          <RecipeValue key={prop} prop={prop} token={token} />
        ) : null
      })}
    </td>
  )
}

export const Sizing: Story = {
  render: () => (
    <Page
      title="Control sizing"
      description="The shared sizing scale every interactive control is built from. Button, checkbox, and badge tokens all alias these values rather than defining their own."
    >
      <Section title="Metrics">
        <div className="dt-table__scroll">
          <table className="dt-table">
            <thead>
              <tr>
                <th>Size</th>
                {METRICS.map(metric => (
                  <th key={metric}>{metric}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {SIZES.map(size => (
                <tr key={size}>
                  <td className="dt-name">{size}</td>
                  {METRICS.map(metric => (
                    <MetricCell key={metric} metric={metric} size={size} />
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Section>

      <Section
        title="References"
        description="The same tokens with the alias each one points at."
      >
        <TokenTable
          tokens={SIZES.flatMap(size =>
            METRICS.map(metric => getToken(`control-${size}-${metric}`)).filter(
              (token): token is Token => Boolean(token)
            )
          )}
          labelOf={token => token.name.slice('control-'.length)}
        />
      </Section>
    </Page>
  ),
}

export const Colors: Story = {
  render: () => (
    <Page
      title="Control colors"
      description="Interactive colors are stored as recipes rather than finished colors: a shade to pick off the ramp and an alpha to apply. A platform resolves them against whichever ramp the control is tinted with."
    >
      {variants.map(variant => (
        <Section key={variant} title={variant}>
          <div className="dt-table__scroll">
            <table className="dt-table">
              <thead>
                <tr>
                  <th>Part</th>
                  {STATES.map(state => (
                    <th key={state}>{state}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {PARTS.map(part => (
                  <tr key={part}>
                    <td className="dt-name">{part}</td>
                    {STATES.map(state => (
                      <RecipeCell
                        key={state}
                        recipe={colorRecipes.get(`${variant}/${part}/${state}`)}
                      />
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </Section>
      ))}
    </Page>
  ),
}
