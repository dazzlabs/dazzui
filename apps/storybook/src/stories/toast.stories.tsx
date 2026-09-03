import {
  ActionBar,
  Button,
  Spinner,
  TINTS,
  type Tint,
  ToastProvider,
  ToastViewport,
  useToast,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import type { ReactNode } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof ToastViewport> = {
  title: 'Components/Toast',
  component: ToastViewport,
  parameters: { layout: 'fullscreen' },
}

export default meta
type Story = StoryObj<typeof meta>

/**
 * A toast has no resting state to draw: it exists because something happened,
 * and it goes on its own. So every story is a button that makes one, and the
 * viewport that catches it — which is also how a host uses it.
 */
function Stage({
  children,
  placement,
}: {
  children: ReactNode
  placement?: 'top' | 'bottom'
}) {
  return (
    <ToastProvider>
      {children}
      <ToastViewport placement={placement} />
    </ToastProvider>
  )
}

function Buttons({
  entries,
}: {
  entries: { label: string; make: () => void }[]
}) {
  return (
    <ActionBar>
      {entries.map(entry => (
        <Button key={entry.label} onClick={entry.make}>
          {entry.label}
        </Button>
      ))}
    </ActionBar>
  )
}

function ShapeButtons() {
  const toast = useToast()
  return (
    <Buttons
      entries={[
        {
          label: 'Plain',
          make: () => toast.add({ title: 'Saved to Downloads' }),
        },
        {
          label: 'Success',
          make: () =>
            toast.add({
              title: 'Translation copied',
              tint: 'success',
              icon: '✓',
            }),
        },
        {
          label: 'In flight',
          make: () =>
            toast.add({
              title: 'Uploading the glossary…',
              tint: 'primary',
              icon: <Spinner size="small" tint="primary" />,
              timeout: 0,
            }),
        },
        {
          label: 'With an action',
          make: () =>
            toast.add({
              title: 'Switched to the backup provider',
              tint: 'warning',
              icon: '!',
              action: { label: 'Undo', onClick: () => {} },
            }),
        },
        {
          label: 'Failure',
          make: () =>
            toast.add({
              title: 'The connection dropped',
              tint: 'danger',
              icon: '✕',
              action: { label: 'Retry', onClick: () => {} },
            }),
        },
      ]}
    />
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  render: () => (
    <Page
      title="Toast"
      description="A transient receipt for something that happened out of view. One line, no wrapping — anything needing a second sentence belongs in a Callout in the flow instead. The queue is the component: `useToast().add()` from anywhere under a ToastProvider, and the viewport draws what is in it, keeps the timers, pauses them while the pointer is over the stack, and announces the message. The contract it holds to is a few seconds for a plain notice, longer when it carries an action, and a failure that stays until it is dismissed — which is what the default timeout works out per tint, so a caller rarely sets one."
    >
      <Section
        title="Shapes"
        description="The leading slot takes a glyph or a Spinner for work still in flight; the trailing slot takes one action, and pressing it closes the toast as well as doing the thing. A `danger` toast has no timeout and interrupts the screen reader, because a failure nobody saw is a failure that did not get reported."
      >
        <ShapeButtons />
      </Section>

      <Section
        title="Tints"
        description="Neutral by default, because most toasts report that something ordinary worked. Reach for a ramp where the outcome itself is the news — a failure, a fallback, a success that was in doubt."
      >
        <OneOfEach tints={TINTS} />
      </Section>

      <Section
        title="Truncation"
        description="A toast sizes to its content and stops at its max width, so the stack needs no fixed width and a long message is cut rather than pushed onto a second line. If the message cannot survive that cut, it was never a toast."
      >
        <Long />
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('toast')}
          labelOf={token => token.name.slice('toast-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
  decorators: [
    Story => (
      <Stage>
        <Story />
      </Stage>
    ),
  ],
}

function OneOfEach({ tints }: { tints: readonly Tint[] }) {
  const toast = useToast()
  return (
    <Buttons
      entries={tints.map(tint => ({
        label: tint,
        make: () =>
          toast.add({ title: 'Saved to Downloads', tint, timeout: 0 }),
      }))}
    />
  )
}

export const Default: Story = {
  render: () => (
    <Stage>
      <OneOfEach tints={['neutral']} />
    </Stage>
  ),
}

export const Tints: Story = {
  render: () => (
    <Stage>
      <OneOfEach tints={TINTS} />
    </Stage>
  ),
}

export const Shapes: Story = {
  render: () => (
    <Stage>
      <ShapeButtons />
    </Stage>
  ),
}

function Long() {
  const toast = useToast()
  return (
    <Buttons
      entries={[
        {
          label: 'A long message',
          make: () =>
            toast.add({
              title:
                'A message long enough that it runs out of room and has to be cut short rather than wrap onto a second line',
              timeout: 0,
            }),
        },
        {
          label: 'Long, with an action',
          make: () =>
            toast.add({
              title:
                'The provider rejected the last request and the job has been paused until a working key is supplied',
              tint: 'danger',
              icon: '✕',
              action: { label: 'Retry', onClick: () => {} },
            }),
        },
      ]}
    />
  )
}

export const Truncation: Story = {
  render: () => (
    <Stage>
      <Long />
    </Stage>
  ),
}

function Stacked() {
  const toast = useToast()
  return (
    <Buttons
      entries={[
        {
          label: 'Add one',
          make: () =>
            toast.add({
              title: `Saved at ${new Date().toLocaleTimeString()}`,
              timeout: 0,
            }),
        },
      ]}
    />
  )
}

/** The stack, which is what the limit and the hover pause are for. */
export const AStack: Story = {
  render: () => (
    <Stage>
      <Stacked />
    </Stage>
  ),
}

/** Top placement, for a shell whose status area is at the head of the window. */
export const InATopViewport: Story = {
  render: () => (
    <Stage placement="top">
      <Stacked />
    </Stage>
  ),
}
