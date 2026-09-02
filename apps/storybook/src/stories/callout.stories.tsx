import { Button, Callout, Card, Spinner, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof Callout> = {
  title: 'Components/Callout',
  component: Callout,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const WIDTH = '34rem'

const COPY: Record<string, { title: string; description: string }> = {
  primary: {
    title: 'Translation memory is syncing',
    description:
      'Segments you edit now are queued and upload when it finishes.',
  },
  neutral: {
    title: 'Export is about 3.6 MB',
    description: 'It will be saved to your Downloads folder.',
  },
  info: {
    title: 'Two glossaries apply to this document',
    description: 'Terms defined in both resolve to whichever ranks higher.',
  },
  success: {
    title: 'All 15 pages translated',
    description: 'Layout and formulas were preserved.',
  },
  warning: {
    title: 'Tables keep their original layout',
    description: 'Text inside them is translated, but column widths are not.',
  },
  danger: {
    title: 'The provider rejected the last request',
    description: 'Check the API key in Settings, then run the job again.',
  },
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Callout"
      description="A tinted notice that sits in the flow of a pane, above the thing it is about. That is the line between this and a Toast: a toast reports something that already happened somewhere else, a callout qualifies what you are looking at. The token graph gives it two type roles — a title and a description — so it is built for both, and drops the title when a one-line notice is all that is needed."
    >
      <Section
        title="Tints"
        description="A callout always wears the tinted treatment; anything louder would be a dialog, not a callout. So the ramp is the only colour axis, and it is the whole message: `danger` is a failure, `warning` a caveat, `info` a fact worth knowing."
      >
        <Stack>
          {TINTS.map(tint => (
            <Callout key={tint} tint={tint} title={COPY[tint]?.title}>
              {COPY[tint]?.description}
            </Callout>
          ))}
        </Stack>
      </Section>

      <Section
        title="Slots"
        description="The leading slot takes a glyph or a spinner and holds its own column, so a run of callouts keeps its titles on one edge whether or not each has one. The trailing slot is pushed to the far edge and holds the way out — keep it to one control, because a callout offering two choices is a dialog that forgot to open."
      >
        <Stack>
          <Callout
            icon={<Spinner size="small" tint="primary" />}
            tint="primary"
            title="Translation memory is syncing"
          >
            Segments you edit now are queued and upload when it finishes.
          </Callout>
          <Callout
            action={
              <Button size="small" tint="danger" variant="outlined">
                Open settings
              </Button>
            }
            icon="✕"
            tint="danger"
            title="The provider rejected the last request"
          >
            Check the API key in Settings, then run the job again.
          </Callout>
          <Callout tint="neutral">
            Testing the connection — 1.4s elapsed.
          </Callout>
        </Stack>
      </Section>

      <Section
        title="Sizes"
        description="Padding, radius, gap, and both type roles step together, so the title stays a step above its description at every size."
      >
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Callout key={size} size={size} tint="info" title={`Size ${size}`}>
              Padding, radius, gap, and both type roles step together.
            </Callout>
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

export const Info: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Callout tint="info" title={COPY.info?.title}>
        {COPY.info?.description}
      </Callout>
    </div>
  ),
}

export const Success: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Callout icon="✓" tint="success" title={COPY.success?.title}>
        {COPY.success?.description}
      </Callout>
    </div>
  ),
}

export const Warning: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Callout icon="!" tint="warning" title={COPY.warning?.title}>
        {COPY.warning?.description}
      </Callout>
    </div>
  ),
}

export const Danger: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Callout icon="✕" tint="danger" title={COPY.danger?.title}>
        {COPY.danger?.description}
      </Callout>
    </div>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col width={WIDTH}>
      {TINTS.map(tint => (
        <Callout key={tint} tint={tint} title={COPY[tint]?.title}>
          {COPY[tint]?.description}
        </Callout>
      ))}
    </Col>
  ),
}

export const WithoutATitle: Story = {
  render: () => (
    <Col width={WIDTH}>
      <Callout tint="neutral">Testing the connection — 1.4s elapsed.</Callout>
      <Callout tint="info">Changes are saved as you type.</Callout>
      <Callout tint="danger">No provider is configured for German.</Callout>
    </Col>
  ),
}

export const WithAnIcon: Story = {
  render: () => (
    <Col width={WIDTH}>
      <Callout
        icon={<Spinner size="small" tint="primary" />}
        tint="primary"
        title="Translation memory is syncing"
      >
        Segments you edit now are queued and upload when it finishes.
      </Callout>
      <Callout icon="✓" tint="success" title="All 15 pages translated">
        Layout and formulas were preserved.
      </Callout>
      <Callout icon="!" tint="warning">
        Two terms in the glossary conflict.
      </Callout>
    </Col>
  ),
}

export const WithAnAction: Story = {
  render: () => (
    <Col width={WIDTH}>
      <Callout
        action={
          <Button size="small" tint="danger" variant="outlined">
            Open settings
          </Button>
        }
        tint="danger"
        title="The provider rejected the last request"
      >
        Check the API key in Settings, then run the job again.
      </Callout>
      <Callout
        action={
          <Button size="small" tint="info" variant="plain">
            Review
          </Button>
        }
        tint="info"
        title="Two glossaries apply to this document"
      >
        Terms defined in both resolve to whichever ranks higher.
      </Callout>
    </Col>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width={WIDTH}>
      {(['small', 'medium', 'large'] as const).map(size => (
        <Callout key={size} size={size} tint="info" title={`Size ${size}`}>
          Padding, radius, gap, and both type roles step together.
        </Callout>
      ))}
    </Col>
  ),
}

export const InAPane: Story = {
  render: () => (
    <Col width={WIDTH}>
      <Callout
        action={
          <Button size="small" tint="warning" variant="plain">
            Switch
          </Button>
        }
        size="small"
        tint="warning"
        title="Running on the backup provider"
      >
        Quality scores may be lower than usual.
      </Callout>
      <Card variant="sunken">
        <span>392 segments · 168 translated</span>
      </Card>
    </Col>
  ),
}
