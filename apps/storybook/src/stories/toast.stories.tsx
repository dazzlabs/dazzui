import { Button, Spinner, Toast, ToastViewport } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Stack, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Toast> = {
  title: 'Components/Toast',
  component: Toast,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="Toast"
      description="A transient receipt for something that happened out of view. One line, no wrapping — anything needing a second sentence belongs in an AlertBanner in the flow instead. Lifetime is the host's job; the system stays stateless."
    >
      <Section
        title="Shapes"
        description="A toast sizes to its content and stops at the viewport, so the stack needs no fixed width and a long message truncates rather than pushing wide."
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
          <Toast onDismiss={() => {}}>
            A message long enough that it runs out of room and has to be cut
            short rather than wrap onto a second line
          </Toast>
        </Stack>
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

/** The viewport is fixed to the window edge, so it gets its own story. */
export const InAViewport: Story = {
  render: () => (
    <ToastViewport>
      <Toast tint="success" icon="✓">
        Translation copied
      </Toast>
      <Toast onDismiss={() => {}}>Saved to Downloads</Toast>
    </ToastViewport>
  ),
}
