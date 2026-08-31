import {
  Button,
  Spinner,
  TINTS,
  Toast,
  ToastViewport,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { Col } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Toast> = {
  title: 'Components/Toast',
  component: Toast,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Toast"
      description="A transient receipt for something that happened out of view. One line, no wrapping — anything needing a second sentence belongs in an AlertBanner in the flow instead. Lifetime is the host's job; the system holds no queue, no timers, and no z-index of its own beyond the viewport. The contract worth keeping is: a few seconds for a plain notice, longer when it carries an action, and a failure stays until it is dismissed."
    >
      <Section
        title="Shapes"
        description="The leading slot takes a glyph or a Spinner for work still in flight; the trailing slot takes one control — undo, retry, open. `onDismiss` renders the close button and is called when it is pressed, so removing the toast stays the host's job."
      >
        <Stack>
          <Toast>Saved to Downloads</Toast>
          <Toast tint="success" icon="✓">
            Translation copied
          </Toast>
          <Toast tint="primary" icon={<Spinner size="small" tint="primary" />}>
            Uploading the glossary…
          </Toast>
          <Toast
            tint="warning"
            icon="!"
            action={
              <Button size="small" tint="warning" variant="plain">
                Undo
              </Button>
            }
          >
            Switched to the backup provider
          </Toast>
          <Toast tint="danger" icon="✕" onDismiss={() => {}}>
            The connection dropped
          </Toast>
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="Neutral by default, because most toasts report that something ordinary worked. Reach for a ramp where the outcome itself is the news — a failure, a fallback, a success that was in doubt."
      >
        <Stack>
          {TINTS.map(tint => (
            <Specimen key={tint} label={tint}>
              <Toast tint={tint}>Saved to Downloads</Toast>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Truncation"
        description="A toast sizes to its content and stops at its max width, so the stack needs no fixed width and a long message is cut rather than pushed onto a second line. If the message cannot survive that cut, it was never a toast."
      >
        <Toast onDismiss={() => {}}>
          A message long enough that it runs out of room and has to be cut short
          rather than wrap onto a second line
        </Toast>
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
}

export const Default: Story = {
  render: () => <Toast>Saved to Downloads</Toast>,
}

export const Tints: Story = {
  render: () => (
    <Col>
      {TINTS.map(tint => (
        <Toast key={tint} tint={tint}>
          Saved to Downloads
        </Toast>
      ))}
    </Col>
  ),
}

export const WithAnIcon: Story = {
  render: () => (
    <Col>
      <Toast tint="success" icon="✓">
        Translation copied
      </Toast>
      <Toast tint="warning" icon="!">
        Switched to the backup provider
      </Toast>
      <Toast tint="danger" icon="✕">
        The connection dropped
      </Toast>
      <Toast tint="primary" icon={<Spinner size="small" tint="primary" />}>
        Uploading the glossary…
      </Toast>
    </Col>
  ),
}

export const WithAnAction: Story = {
  render: () => (
    <Col>
      <Toast
        tint="warning"
        icon="!"
        action={
          <Button size="small" tint="warning" variant="plain">
            Undo
          </Button>
        }
      >
        Switched to the backup provider
      </Toast>
      <Toast
        tint="danger"
        icon="✕"
        action={
          <Button size="small" tint="danger" variant="plain">
            Retry
          </Button>
        }
        onDismiss={() => {}}
      >
        The connection dropped
      </Toast>
    </Col>
  ),
}

export const Dismissable: Story = {
  render: () => {
    const initial = [
      { id: 1, message: 'Saved to Downloads' },
      { id: 2, message: 'Translation copied' },
      { id: 3, message: 'Glossary updated' },
    ]
    const [toasts, setToasts] = useState(initial)

    return (
      <Col>
        {toasts.map(entry => (
          <Toast
            key={entry.id}
            onDismiss={() =>
              setToasts(rest => rest.filter(item => item.id !== entry.id))
            }
          >
            {entry.message}
          </Toast>
        ))}
        {toasts.length === 0 ? (
          <Button onClick={() => setToasts(initial)}>Bring them back</Button>
        ) : null}
      </Col>
    )
  },
}

export const Truncation: Story = {
  render: () => (
    <Col>
      <Toast onDismiss={() => {}}>
        A message long enough that it runs out of room and has to be cut short
        rather than wrap onto a second line
      </Toast>
      <Toast
        tint="danger"
        icon="✕"
        action={
          <Button size="small" tint="danger" variant="plain">
            Retry
          </Button>
        }
        onDismiss={() => {}}
      >
        The provider rejected the last request and the job has been paused until
        a working key is supplied
      </Toast>
    </Col>
  ),
}

/** The viewport is fixed to the window edge, so it gets its own story. */
export const InAViewport: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <ToastViewport>
      <Toast tint="success" icon="✓">
        Translation copied
      </Toast>
      <Toast onDismiss={() => {}}>Saved to Downloads</Toast>
    </ToastViewport>
  ),
}

/** Top placement, for a shell whose status area is at the head of the window. */
export const InATopViewport: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <ToastViewport placement="top">
      <Toast tint="primary" icon={<Spinner size="small" tint="primary" />}>
        Uploading the glossary…
      </Toast>
      <Toast onDismiss={() => {}}>Saved to Downloads</Toast>
    </ToastViewport>
  ),
}
