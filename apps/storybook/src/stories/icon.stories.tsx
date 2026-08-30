import { ActionBar, Badge, Icon, type IconGlyph } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'

const meta: Meta<typeof Icon> = {
  title: 'Components/Icon',
  component: Icon,
}

export default meta
type Story = StoryObj<typeof meta>

/** Stand-ins, so the gallery does not pull in an icon set of its own. */
const Check: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M16.7 5.3a1 1 0 0 1 0 1.4l-7.5 7.5a1 1 0 0 1-1.4 0l-3.5-3.5a1 1 0 1 1 1.4-1.4l2.8 2.8 6.8-6.8a1 1 0 0 1 1.4 0Z" />
  </svg>
)

const Bolt: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M11.6 1.6a.6.6 0 0 1 1 .6l-1.5 5.3h4a.7.7 0 0 1 .5 1.2l-8 9.7a.6.6 0 0 1-1-.6l1.5-5.3h-4a.7.7 0 0 1-.5-1.2l8-9.7Z" />
  </svg>
)

export const Overview: Story = {
  render: () => (
    <Page
      title="Icon"
      description="Draws a glyph at a given size. The system depends on no icon set: an icon set is the one thing a product replaces wholesale, and hard-coding one would make that a fork rather than a prop. Pass any component that renders an svg."
    >
      <Section
        title="Sizes"
        description="A fixed-size glyph is scaled to fit, so a 20px set works at any size the layout needs."
      >
        <Stack>
          <Specimen label="12 / 16 / 20 / 24">
            <Icon icon={Bolt} size={12} />
            <Icon icon={Bolt} size={16} />
            <Icon icon={Bolt} size={20} />
            <Icon icon={Bolt} size={24} />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Colour"
        description="Fill rides on `currentColor`, so an icon takes the tone of whatever it sits in and no component has to pass a colour down to it."
      >
        <Stack>
          <Specimen label="inherited">
            <ActionBar>
              <Badge tint="success">
                <Icon icon={Check} size={12} /> Connected
              </Badge>
              <Badge tint="danger" variant="filled">
                <Icon icon={Bolt} size={12} /> Rate limited
              </Badge>
              <span style={{ color: 'var(--color-content-muted)' }}>
                <Icon icon={Check} size={16} />
              </span>
            </ActionBar>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}
