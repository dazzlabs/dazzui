import type { Meta, StoryObj } from '@storybook/react-vite'

import { Bar, Page, Section, Stack } from '../components/gallery'
import { byPrefix } from '../lib/tokens'

const meta: Meta = {
  title: 'Tokens/Spacing',
}

export default meta
type Story = StoryObj<typeof meta>

const spacing = byPrefix('spacing-', { sorted: true })

export const Scale: Story = {
  render: () => (
    <Page
      title="Spacing"
      description="The spacing steps exposed to components, drawn to scale. Each one aliases a step of the base scale."
    >
      <Section title="Scale">
        <Stack>
          {spacing.map(token => (
            <Bar
              key={token.name}
              token={token}
              label={token.name.slice('spacing-'.length)}
            />
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}
