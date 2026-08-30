import { Divider } from '@dazzlabs/dazzui'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'

const meta: Meta<typeof Divider> = {
  title: 'Components/Divider',
  component: Divider,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="Divider"
      description="A one-pixel rule. It carries no component tokens of its own: a separator is entirely described by a border color and the hairline step of the spacing scale, and inventing `divider-*` aliases for those would only add a layer to read through."
    >
      <Section
        title="Tones"
        description="The two border roles. `muted` is the heavier of the pair, for a rule that has to hold its own against a filled surface."
      >
        <Stack>
          <Specimen label="default">
            <div style={{ flex: 1 }}>
              <Divider />
            </div>
          </Specimen>
          <Specimen label="muted">
            <div style={{ flex: 1 }}>
              <Divider tone="muted" />
            </div>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Vertical"
        description="Stretches to its flex parent, so a toolbar row sizes it without a height being passed in."
      >
        <Stack>
          <Specimen label="vertical">
            <span>Source</span>
            <Divider orientation="vertical" />
            <span>Target</span>
            <Divider orientation="vertical" />
            <span>Glossary</span>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}
