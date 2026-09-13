import { defineStory } from '../src/lib/scene'
import scenes from './spinner.scene.json'
import {
  Button,
  Card,
  SectionLabel,
  Spinner,
  TINTS,
  ToastProvider,
  ToastViewport,
  useToast,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

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

export const Default: Story = defineStory(scenes.Default)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Tints: Story = defineStory(scenes.Tints)

/**
 * On a filled control the ring goes white: a tinted spinner on its own
 * tint's fill is invisible.
 */
export const OnAnAccent: Story = defineStory(scenes.OnAnAccent)

export const InAButton: Story = defineStory(scenes.InAButton)

export const InACallout: Story = defineStory(scenes.InACallout)

export const InAToast: Story = {
  render: () => <InAToastStage />,
}

export const InProse: Story = defineStory(scenes.InProse)

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
