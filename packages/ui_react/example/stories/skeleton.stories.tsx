import { Card, Skeleton, SkeletonText } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col, Row } from '../src/components/stage'

const meta: Meta<typeof Skeleton> = {
  title: 'Components/Skeleton',
  component: Skeleton,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

function Placeholder() {
  return (
    <Row gap="var(--spacing-3)">
      <Skeleton shape="circle" width="var(--spacing-10)" />
      <Col gap="var(--spacing-2)" width="var(--spacing-20)">
        <Skeleton width="70%" />
        <Skeleton width="45%" />
      </Col>
    </Row>
  )
}

export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Skeleton"
      description="The shape of something that has not arrived. A Spinner says the app is busy; a skeleton says what is coming and where it will be — which is why it belongs on the first load of a view whose layout is already known, and a spinner belongs on an action whose result is not. Drawing one for content that then lands somewhere else is worse than drawing nothing."
    >
      <Section
        title="Shapes"
        description="`text` is a bar on the line box of the type it stands in for, so it needs no height. `circle` is an avatar or a glyph and squares itself from its width alone. `rect` is everything with a box of its own — a thumbnail, a chart, a card."
      >
        <Stack>
          <Specimen label="text">
            <Skeleton width="var(--spacing-20)" />
          </Specimen>
          <Specimen label="circle">
            <Skeleton shape="circle" width="var(--spacing-10)" />
          </Specimen>
          <Specimen label="rect">
            <Skeleton
              shape="rect"
              width="var(--spacing-20)"
              height="var(--spacing-12)"
            />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Lines"
        description="A paragraph's worth of bars, with the last one short. The ragged edge is most of what makes a block of bars legible as text at all — a full-width final line reads as a table."
      >
        <div style={{ width: '20rem' }}>
          <SkeletonText />
        </div>
      </Section>

      <Section
        title="Materials"
        description="The fill is surface-sunken — the same dead step a disabled control flattens to, which is exactly what a skeleton is. The sweep over it is surface-subtle, the ink wash the whole system already hovers with, so the highlight moves by the same amount on paper, on a card and on a dark canvas without a per-theme value. Unlike the spinner it takes no reduced-motion exception: a still spinner reads as a hung app, where a still skeleton still says everything it has to say."
      >
        <Card>
          <Col gap="var(--spacing-3)">
            <Placeholder />
            <SkeletonText lines={2} />
          </Col>
        </Card>
      </Section>
    </Page>
  ),
}

export const Shapes: Story = {
  render: () => (
    <Row>
      <Skeleton shape="circle" width="var(--spacing-10)" />
      <Skeleton
        shape="rect"
        width="var(--spacing-20)"
        height="var(--spacing-12)"
      />
    </Row>
  ),
}

export const Text: Story = {
  render: () => (
    <Col width="20rem">
      <SkeletonText />
    </Col>
  ),
}

export const Lines: Story = {
  render: () => (
    <Col width="20rem">
      <SkeletonText lines={5} />
    </Col>
  ),
}

export const AListRow: Story = {
  render: () => (
    <Col width="20rem">
      <Placeholder />
    </Col>
  ),
}

export const InACard: Story = {
  render: () => (
    <Col width="22rem">
      <Card>
        <Col gap="var(--spacing-3)">
          <Placeholder />
          <SkeletonText lines={2} />
        </Col>
      </Card>
    </Col>
  ),
}
