import { Meter, Progress, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof Meter> = {
  title: 'Components/Meter',
  component: Meter,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Meter"
      description="A reading inside a known range — disk used of disk there, spend against a budget, an API quota. It is not a Progress, and the difference is not how they look: progress reports on something running, so it moves on its own, it can be indeterminate, and it ends. A meter measures something that already has a value, so it never animates and never has none — which is why it prints its number and progress does not."
    >
      <Section
        title="Sizes"
        description="The same groove a Progress bar is cut into — the sunken surface, the soft square corner — because they are the same hole in the page. What differs is everything above it."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <div style={{ width: '18rem' }}>
                <Meter value={62} size={size} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Label and value"
        description="The number takes the mono face and tabular figures, so a column of meters keeps its numbers on one edge. The label is the meter's accessible name as well as its heading."
      >
        <Col width="20rem">
          <Meter label="Storage" showValue value={62} />
          <Meter
            label="Bandwidth"
            showValue
            value={874}
            max={1000}
            format={{ style: 'unit', unit: 'gigabyte' }}
            tint="info"
          />
          <Meter label="Seats" showValue value={19} max={20} tint="warning" />
        </Col>
      </Section>

      <Section
        title="Against a Progress"
        description="Set side by side the rule is easy to keep: if the number would change on its own while the reader watched, it is progress. If it is a fact about right now, it is a meter."
      >
        <Col width="20rem">
          <Meter label="Disk used" showValue value={62} />
          <Progress value={62} />
        </Col>
      </Section>
    </Page>
  ),
}

export const Bare: Story = {
  render: () => (
    <div style={{ width: '18rem' }}>
      <Meter value={62} />
    </div>
  ),
}

export const WithValue: Story = {
  render: () => (
    <div style={{ width: '20rem' }}>
      <Meter label="Storage" showValue value={62} />
    </div>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width="20rem">
      {SIZES.map(size => (
        <Meter key={size} size={size} value={62} />
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col width="20rem">
      {TINTS.map(tint => (
        <Meter key={tint} tint={tint} value={62} />
      ))}
    </Col>
  ),
}

export const InAQuotaList: Story = {
  render: () => (
    <Col width="22rem" gap="var(--spacing-4)">
      <Meter label="Storage" showValue value={62} />
      <Meter label="Bandwidth" showValue value={41} tint="info" />
      <Meter label="Build minutes" showValue value={88} tint="warning" />
      <Meter label="Seats" showValue value={100} tint="danger" />
    </Col>
  ),
}
