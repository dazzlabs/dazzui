import type { Meta, StoryObj } from '../src/lib/story'

import {
  Page,
  Section,
  Stack,
  TYPE_SAMPLE,
  useResolvedValue,
} from '../src/components/gallery'
import { getToken } from '../src/lib/tokens'

const meta: Meta = {
  title: 'Tokens/Typography',
}

export default meta
type Story = StoryObj<typeof meta>

const ROLES = ['headline', 'title', 'body', 'label', 'caption']
const SIZES = ['large', 'medium', 'small']

const SAMPLE = TYPE_SAMPLE

function TypeSample({ role, size }: { role: string; size: string }) {
  const name = `${role}-${size}`
  const fontSize = useResolvedValue(`--${name}-font-size`)
  const lineHeight = useResolvedValue(`--${name}-line-height`)
  const fontWeight = useResolvedValue(`--${name}-font-weight`)

  return (
    <div>
      <div className="dt-name">--{name}-*</div>
      <p
        style={{
          margin: 'var(--spacing-2) 0',
          fontFamily: `var(--${name}-font-family)`,
          fontSize: `var(--${name}-font-size)`,
          fontWeight: `var(--${name}-font-weight)`,
          lineHeight: `var(--${name}-line-height)`,
        }}
      >
        {SAMPLE}
      </p>
      <div className="dt-value">
        {fontSize} / {lineHeight} · weight {fontWeight}
      </div>
    </div>
  )
}

/** Sizes that actually exist for a role, largest first. */
function sizesFor(role: string): string[] {
  return SIZES.filter(size => getToken(`${role}-${size}-font-size`))
}

export const Scale: Story = {
  render: () => (
    <Page
      title="Typography"
      description="Composed type styles. Each one bundles a family, size, weight, and line height so a component only has to name the role it needs."
    >
      {ROLES.map(role => (
        <Section key={role} title={role}>
          <Stack>
            {sizesFor(role).map(size => (
              <TypeSample key={size} role={role} size={size} />
            ))}
          </Stack>
        </Section>
      ))}
    </Page>
  ),
}
