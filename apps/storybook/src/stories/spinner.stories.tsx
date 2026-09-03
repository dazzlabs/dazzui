import {
  Button,
  Callout,
  Card,
  SectionLabel,
  Spinner,
  TINTS,
  ToastProvider,
  ToastViewport,
  useToast,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

/**
 * A toast is made rather than drawn — the queue owns it — so the specimen is
 * the button that makes one, with a viewport beside it to catch it.
 */
function Raise() {
  const toast = useToast()
  return (
    <Button
      onClick={() =>
        toast.add({
          title: 'Uploading the glossary…',
          icon: <Spinner size="small" tint="neutral" />,
          timeout: 0,
        })
      }
    >
      Upload the glossary
    </Button>
  )
}

function InAToastStage() {
  return (
    <ToastProvider>
      <Raise />
      <ToastViewport />
    </ToastProvider>
  )
}

const meta: Meta<typeof Spinner> = {
  title: 'Components/Spinner',
  component: Spinner,
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
      title="Spinner"
      description="A ring with one segment picked out: the track sits low on the ramp, the head high. Under reduced motion it slows rather than stops — a still spinner reads as a frozen app, which is the opposite of what it is there to say."
    >
      <Section
        title="Sizes"
        description="The ring thickness is shared across sizes; only the diameter steps. A ring that thinned as it shrank would disappear at the small end, where it is most often used — inside a button, beside a line of text."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Spinner size={size} />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="Both the track and the head come off one ramp, two steps apart, so a spinner keeps its contrast on every tint without a second recipe. Match it to whatever it sits in — a spinner in a danger callout tinted primary would look like a stray part."
      >
        <Stack>
          <Specimen label="all">
            {TINTS.map(tint => (
              <Spinner key={tint} tint={tint} />
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Where it lands"
        description="A spinner is rarely alone: it goes in the slot of something that already says what is happening. On its own it reports that the app is busy without saying at what, which is only ever right for a first paint — and even then a line of copy under it helps."
      >
        <Stack>
          <Specimen label="button">
            <Button disabled variant="filled">
              <Spinner size="small" tint="primary" />
              Translating…
            </Button>
          </Specimen>
          <Specimen label="toast">
            <InAToastStage />
          </Specimen>
          <Specimen label="in prose">
            <span
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: 'var(--spacing-2)',
                color: 'var(--color-content-muted)',
              }}
            >
              <Spinner size="small" tint="neutral" />
              Testing the connection — 1.4s elapsed
            </span>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => <Spinner />,
}

export const Sizes: Story = {
  render: () => (
    <Row>
      {SIZES.map(size => (
        <Spinner key={size} size={size} />
      ))}
    </Row>
  ),
}

export const Tints: Story = {
  render: () => (
    <Row>
      {TINTS.map(tint => (
        <Spinner key={tint} tint={tint} />
      ))}
    </Row>
  ),
}

export const InAButton: Story = {
  render: () => (
    <Row>
      <Button disabled variant="filled">
        <Spinner size="small" tint="primary" />
        Translating…
      </Button>
      <Button disabled>
        <Spinner size="small" tint="neutral" />
        Checking the key…
      </Button>
    </Row>
  ),
}

export const InACallout: Story = {
  render: () => (
    <div style={{ width: '32rem' }}>
      <Callout
        icon={<Spinner size="small" tint="primary" />}
        tint="primary"
        title="Translation memory is syncing"
      >
        Segments you edit now are queued and upload when it finishes.
      </Callout>
    </div>
  ),
}

export const InAToast: Story = {
  render: () => <InAToastStage />,
}

export const InProse: Story = {
  render: () => (
    <span
      style={{
        display: 'inline-flex',
        alignItems: 'center',
        gap: 'var(--spacing-2)',
        color: 'var(--color-content-muted)',
      }}
    >
      <Spinner size="small" tint="neutral" />
      Testing the connection — 1.4s elapsed
    </span>
  ),
}

export const InAPanel: Story = {
  render: () => (
    <Card variant="sunken" style={{ width: '24rem' }}>
      <Col gap="var(--spacing-3)">
        <div
          style={{
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            gap: 'var(--spacing-3)',
            paddingBlock: 'var(--spacing-10)',
          }}
        >
          <Spinner size="large" />
          <SectionLabel>Parsing handbook.docx</SectionLabel>
        </div>
      </Col>
    </Card>
  ),
}
