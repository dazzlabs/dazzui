import { KeyCap, SearchField } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useMemo, useState } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { type Token, tokens } from '../lib/tokens'

const meta: Meta = {
  title: 'Tokens/Variables',
}

export default meta
type Story = StoryObj<typeof meta>

/**
 * The semantic layer, listed rather than derived — it is the short, stable
 * half of the graph. Anything under `base-` is a primitive and everything
 * else is a component, so a new component token needs no change here; only a
 * new *global* does, and the counts in the headings make a miss obvious.
 */
const SEMANTIC = [
  'color-',
  'control-',
  'spacing-',
  'radius-',
  'shadow-',
  'stroke-',
  'wash-',
  'focus-',
  'frame-',
  'motion-',
  'body-',
  'label-',
  'title-',
  'headline-',
  'caption-',
]

type Layer = 'semantic' | 'component' | 'primitive'

function layerOf(token: Token): Layer {
  if (token.name.startsWith('base-')) return 'primitive'
  return SEMANTIC.some(prefix => token.name.startsWith(prefix))
    ? 'semantic'
    : 'component'
}

const LAYERS: { key: Layer; title: string; description: string }[] = [
  {
    key: 'semantic',
    title: 'Semantic',
    description:
      'The roles a component asks for by name. These are what a stylesheet should read; the layer below is what they resolve to.',
  },
  {
    key: 'component',
    title: 'Component',
    description:
      'Values that are genuinely one component’s own — a part no shared role covers, or a wash it applies itself. Most components own none.',
  },
  {
    key: 'primitive',
    title: 'Primitive',
    description:
      'The raw values the semantic roles alias. A component must never read one of these directly.',
  },
]

const BY_LAYER = Object.fromEntries(
  LAYERS.map(layer => [
    layer.key,
    tokens.filter(token => layerOf(token) === layer.key),
  ])
) as Record<Layer, Token[]>

function Variables() {
  const [query, setQuery] = useState('')

  const matched = useMemo(() => {
    const needle = query.trim().toLowerCase()
    if (!needle) return BY_LAYER

    const hit = (token: Token) =>
      token.name.toLowerCase().includes(needle) ||
      token.value.toLowerCase().includes(needle)

    return Object.fromEntries(
      LAYERS.map(layer => [layer.key, BY_LAYER[layer.key].filter(hit)])
    ) as Record<Layer, Token[]>
  }, [query])

  const total = LAYERS.reduce((sum, l) => sum + matched[l.key].length, 0)

  return (
    <Page
      title="Variables"
      description={`Every custom property the token build emits — ${tokens.length} of them — with what it was declared as and what it resolves to. The resolved column follows the theme toolbar, so this is also how to read a theme: switch it and the right-hand column moves.`}
    >
      <Section
        title="Find one"
        description="Matches the name and the declared value, so searching a token name finds the roles that alias it as well as the token itself."
      >
        <div style={{ maxWidth: '22rem' }}>
          <SearchField
            hint={<KeyCap size="small">{tokens.length}</KeyCap>}
            placeholder="radius, primary-600, spacing…"
            value={query}
            onValueChange={setQuery}
          />
        </div>
      </Section>

      {total === 0 ? (
        <Section title="No matches">
          <p className="dt-empty">Nothing is named or valued “{query}”.</p>
        </Section>
      ) : (
        LAYERS.map(layer =>
          matched[layer.key].length === 0 ? null : (
            <Section
              key={layer.key}
              title={`${layer.title} · ${matched[layer.key].length}`}
              description={layer.description}
            >
              <TokenTable tokens={matched[layer.key]} />
            </Section>
          )
        )
      )}
    </Page>
  )
}

export const Overview: Story = {
  render: () => <Variables />,
}
