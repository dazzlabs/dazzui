import {
  Button,
  TINTS,
  ToastProvider,
  ToastViewport,
  useToast,
} from '@dazzlabs/dazzui'
import type { ReactNode } from 'react'

import { Row } from '../components/stage'
import type { Story } from '../lib/story'

/** The provider a toast is raised through, and the corner it lands in. */
function Toasts({
  placement,
  children,
}: {
  placement?: 'top' | 'bottom'
  children: ReactNode
}) {
  return (
    <ToastProvider>
      {children}
      <ToastViewport placement={placement} />
    </ToastProvider>
  )
}

function Save() {
  const toast = useToast()
  return (
    <Button onClick={() => toast.add({ title: 'Saved to Downloads' })}>
      Save
    </Button>
  )
}

function Tints() {
  const toast = useToast()
  return (
    <Row>
      {TINTS.map(tint => (
        <Button
          key={tint}
          tint={tint}
          variant="tinted"
          onClick={() => toast.add({ title: `A ${tint} toast`, tint })}
        >
          {tint}
        </Button>
      ))}
    </Row>
  )
}

function Undo() {
  const toast = useToast()
  return (
    <Button
      onClick={() =>
        toast.add({
          title: 'Project archived',
          action: { label: 'Undo', onClick: () => {} },
        })
      }
    >
      Archive
    </Button>
  )
}

const story: Story = {
  id: 'toast',
  name: 'Toast',
  summary: 'What just happened, said once and then gone.',
  Preview: () => (
    <Toasts>
      <Save />
    </Toasts>
  ),
  examples: [
    {
      title: 'Tints',
      Demo: () => (
        <Toasts>
          <Tints />
        </Toasts>
      ),
    },
    {
      title: 'With an action',
      Demo: () => (
        <Toasts>
          <Undo />
        </Toasts>
      ),
    },
    {
      title: 'From the top',
      Demo: () => (
        <Toasts placement="top">
          <Save />
        </Toasts>
      ),
    },
  ],
}

export default story
