import {
  ActionBar,
  Badge,
  Button,
  Card,
  Divider,
  SectionLabel,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof Divider> = {
  title: 'Components/Divider',
  component: Divider,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

function Line({ label, value }: { label: string; value: string }) {
  return (
    <div
      style={{
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
      }}
    >
      <span>{label}</span>
      <Badge size="small" tint="neutral">
        {value}
      </Badge>
    </div>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Divider"
      description="A one-pixel rule. It carries no component tokens of its own: a separator is entirely described by a border colour and the hairline step of the spacing scale, and inventing `divider-*` aliases for those would only add a layer to read through."
    >
      <Section
        title="Tones"
        description="The two border roles. `muted` is the lighter of the pair, for a rule inside a filled surface where the default would cut too hard; `default` is what separates one region from another."
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
        description="Stretches to its flex parent, so a toolbar row sizes it without a height being passed in. That is the whole reason it is a div rather than an `<hr>`: a rule that has to be told how tall it is will be told wrong somewhere."
      >
        <Stack>
          <Specimen label="labels">
            <span>Source</span>
            <Divider orientation="vertical" />
            <span>Target</span>
            <Divider orientation="vertical" />
            <span>Glossary</span>
          </Specimen>
          <Specimen label="toolbar">
            <ActionBar>
              <Button size="small" variant="plain">
                Undo
              </Button>
              <Button size="small" variant="plain">
                Redo
              </Button>
              <Divider orientation="vertical" />
              <Button size="small" variant="plain">
                Copy
              </Button>
              <Button size="small" variant="plain">
                Paste
              </Button>
            </ActionBar>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="In a container"
        description="A card already sets the gap between its children, so a divider adds only the line — it does not need margins, and giving it any would double the space the card had decided on."
      >
        <Card style={{ width: '22rem' }}>
          <SectionLabel>handbook.docx</SectionLabel>
          <span>392 segments · 15 pages</span>
          <Divider />
          <span style={{ color: 'var(--color-content-muted)' }}>
            Last translated 12 minutes ago
          </span>
        </Card>
      </Section>
    </Page>
  ),
}

export const Horizontal: Story = {
  render: () => (
    <div style={{ width: '20rem' }}>
      <Divider />
    </div>
  ),
}

export const Muted: Story = {
  render: () => (
    <div style={{ width: '20rem' }}>
      <Divider tone="muted" />
    </div>
  ),
}

export const Tones: Story = {
  render: () => (
    <Col width="20rem">
      <Divider />
      <Divider tone="muted" />
    </Col>
  ),
}

export const Vertical: Story = {
  render: () => (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        gap: 'var(--spacing-3)',
        height: '1.5rem',
      }}
    >
      <span>Source</span>
      <Divider orientation="vertical" />
      <span>Target</span>
      <Divider orientation="vertical" />
      <span>Glossary</span>
    </div>
  ),
}

export const InAToolbar: Story = {
  render: () => (
    <ActionBar>
      <Button size="small" variant="plain">
        Undo
      </Button>
      <Button size="small" variant="plain">
        Redo
      </Button>
      <Divider orientation="vertical" />
      <Button size="small" variant="plain">
        Copy
      </Button>
      <Button size="small" variant="plain">
        Paste
      </Button>
    </ActionBar>
  ),
}

export const InACard: Story = {
  render: () => (
    <Card style={{ width: '22rem' }}>
      <SectionLabel>handbook.docx</SectionLabel>
      <span>392 segments · 15 pages</span>
      <Divider />
      <span style={{ color: 'var(--color-content-muted)' }}>
        Last translated 12 minutes ago
      </span>
      <Divider />
      <ActionBar align="end">
        <Button size="small">Open</Button>
      </ActionBar>
    </Card>
  ),
}

export const BetweenRows: Story = {
  render: () => (
    <Col width="22rem">
      <Line label="DeepL" value="Connected" />
      <Divider tone="muted" />
      <Line label="Google Translate" value="Idle" />
      <Divider tone="muted" />
      <Line label="Custom endpoint" value="Unreachable" />
    </Col>
  ),
}
