import {
  ActionBar,
  Badge,
  Button,
  Card,
  type CardVariant,
  Divider,
  EmptyState,
  Progress,
  SectionLabel,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Grid, Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof Card> = {
  title: 'Components/Card',
  component: Card,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const VARIANTS: CardVariant[] = ['raised', 'sunken', 'outlined', 'tinted']

const SIZES = ['small', 'medium', 'large'] as const

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Card"
      description="The filled or bordered box the panels in a layout are built from. It is a container rather than a control, so its padding is uniform and comes off the spacing scale instead of the control scale, and it stays out of the shared state machine — a panel that washed under the pointer would read as something you can press."
    >
      <Section
        title="Variants"
        description="Only `tinted` reads the tint; the rest resolve against the neutral surface and border roles. `raised` is a panel on the canvas, `sunken` a well inside one, `outlined` a boundary with no fill of its own."
      >
        <Grid min="14rem">
          {VARIANTS.map(variant => (
            <Card key={variant} variant={variant}>
              <SectionLabel>{variant}</SectionLabel>
              <span>Padding, gap, and radius all come from the size.</span>
            </Card>
          ))}
        </Grid>
      </Section>

      <Section
        title="Tints"
        description="The tinted variant across every ramp: a 50 fill under a 200 border. The other three ignore `tint` entirely, so setting it on them is harmless and does nothing."
      >
        <Grid min="10rem">
          {TINTS.map(tint => (
            <Card key={tint} size="small" tint={tint} variant="tinted">
              <SectionLabel tint={tint}>{tint}</SectionLabel>
            </Card>
          ))}
        </Grid>
      </Section>

      <Section
        title="Sizes"
        description="Padding, the gap between children, and the radius step together. Each step is a spacing token and a radius token, not a control profile — a card holds content rather than sitting in a row of controls."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Card size={size} style={{ width: '16rem' }}>
                <SectionLabel>Quality</SectionLabel>
                <span>Nine of twelve checks passed.</span>
              </Card>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="What goes in one"
        description="A card is a flex column with the gap already set, so a heading row, a body, and a footer need no spacing of their own — only the row that has to spread its children apart. `sunken` is the recessed variant, for a region that holds something rather than being something."
      >
        <Grid min="20rem">
          <Card>
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
              }}
            >
              <SectionLabel>handbook.docx</SectionLabel>
              <Badge size="small" tint="primary">
                In progress
              </Badge>
            </div>
            <Progress value={43} />
            <span style={{ color: 'var(--color-content-muted)' }}>
              168 of 392 segments
            </span>
            <Divider />
            <ActionBar align="end">
              <Button size="small" variant="plain">
                Pause
              </Button>
              <Button size="small" variant="filled">
                Open
              </Button>
            </ActionBar>
          </Card>
          <Card variant="sunken">
            <EmptyState
              title="No documents yet"
              action={<Button>Import a file</Button>}
            />
          </Card>
        </Grid>
      </Section>
    </Page>
  ),
}

export const Raised: Story = {
  render: () => (
    <Card style={{ width: '18rem' }}>
      <SectionLabel>Quality</SectionLabel>
      <span>Nine of twelve checks passed.</span>
    </Card>
  ),
}

export const Sunken: Story = {
  render: () => (
    <Card variant="sunken" style={{ width: '18rem' }}>
      <SectionLabel>Quality</SectionLabel>
      <span>Nine of twelve checks passed.</span>
    </Card>
  ),
}

export const Outlined: Story = {
  render: () => (
    <Card variant="outlined" style={{ width: '18rem' }}>
      <SectionLabel>Quality</SectionLabel>
      <span>Nine of twelve checks passed.</span>
    </Card>
  ),
}

export const Tinted: Story = {
  render: () => (
    <Card tint="warning" variant="tinted" style={{ width: '18rem' }}>
      <SectionLabel tint="warning">Low scores</SectionLabel>
      <span>Twenty-four segments need a second pass.</span>
    </Card>
  ),
}

export const Variants: Story = {
  render: () => (
    <Row>
      {VARIANTS.map(variant => (
        <Card key={variant} variant={variant} style={{ width: '12rem' }}>
          <SectionLabel>{variant}</SectionLabel>
          <span>Padding, gap, and radius come from the size.</span>
        </Card>
      ))}
    </Row>
  ),
}

export const Tints: Story = {
  render: () => (
    <Row>
      {TINTS.map(tint => (
        <Card
          key={tint}
          size="small"
          tint={tint}
          variant="tinted"
          style={{ width: '8rem' }}
        >
          <SectionLabel tint={tint}>{tint}</SectionLabel>
        </Card>
      ))}
    </Row>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col>
      {SIZES.map(size => (
        <Card key={size} size={size} style={{ width: '16rem' }}>
          <SectionLabel>{size}</SectionLabel>
          <span>Nine of twelve checks passed.</span>
        </Card>
      ))}
    </Col>
  ),
}

export const WithAnAction: Story = {
  render: () => (
    <Card style={{ width: '24rem' }}>
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
        }}
      >
        <SectionLabel>handbook.docx</SectionLabel>
        <Badge size="small" tint="primary">
          In progress
        </Badge>
      </div>
      <Progress value={43} />
      <span style={{ color: 'var(--color-content-muted)' }}>
        168 of 392 segments
      </span>
      <Divider />
      <ActionBar align="end">
        <Button size="small" variant="plain">
          Pause
        </Button>
        <Button size="small" variant="filled">
          Open
        </Button>
      </ActionBar>
    </Card>
  ),
}

export const AsAGrid: Story = {
  render: () => (
    <div style={{ width: '44rem' }}>
      <Grid min="13rem">
        <Card>
          <SectionLabel>Segments</SectionLabel>
          <span style={{ fontSize: 'var(--title-large-font-size)' }}>392</span>
        </Card>
        <Card>
          <SectionLabel>Translated</SectionLabel>
          <span style={{ fontSize: 'var(--title-large-font-size)' }}>168</span>
        </Card>
        <Card tint="warning" variant="tinted">
          <SectionLabel tint="warning">Low scores</SectionLabel>
          <span style={{ fontSize: 'var(--title-large-font-size)' }}>24</span>
        </Card>
        <Card variant="outlined">
          <SectionLabel>Glossary hits</SectionLabel>
          <span style={{ fontSize: 'var(--title-large-font-size)' }}>96</span>
        </Card>
      </Grid>
    </div>
  ),
}

export const AsAWell: Story = {
  render: () => (
    <Card variant="sunken" style={{ width: '26rem' }}>
      <EmptyState
        title="No documents yet"
        action={<Button>Import a file</Button>}
      />
    </Card>
  ),
}

export const Nested: Story = {
  render: () => (
    <Card style={{ width: '24rem' }}>
      <SectionLabel>Request</SectionLabel>
      <Card size="small" variant="sunken">
        <span style={{ fontFamily: 'var(--base-font-code)' }}>
          POST /v2/translate · 204 ms
        </span>
      </Card>
    </Card>
  ),
}
