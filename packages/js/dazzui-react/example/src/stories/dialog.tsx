import {
  Button,
  Dialog,
  DialogBody,
  DialogClose,
  DialogFooter,
  DialogHeader,
  TextField,
} from '@dazzlabs/dazzui-react'

import { Row } from '../components/stage'
import type { Story } from '../lib/story'

function Rename() {
  return (
    <Dialog width="24rem" trigger={<Button>Rename…</Button>}>
      <DialogHeader
        title="Rename project"
        subtitle="Links to it keep working."
      />
      <DialogBody>
        <TextField defaultValue="Quarterly report" aria-label="Name" />
      </DialogBody>
      <DialogFooter>
        <DialogClose render={<Button variant="outlined">Cancel</Button>} />
        <DialogClose render={<Button variant="filled">Rename</Button>} />
      </DialogFooter>
    </Dialog>
  )
}

function Remove() {
  return (
    <Dialog
      tone="danger"
      width="24rem"
      trigger={
        <Button tint="danger" variant="tinted">
          Delete…
        </Button>
      }
    >
      <DialogHeader
        title="Delete this project?"
        subtitle="Its files go with it. This cannot be undone."
      />
      <DialogFooter>
        <DialogClose render={<Button variant="outlined">Keep it</Button>} />
        <DialogClose
          render={
            <Button variant="filled" tint="danger">
              Delete
            </Button>
          }
        />
      </DialogFooter>
    </Dialog>
  )
}

const story: Story = {
  id: 'dialog',
  name: 'Dialog',
  summary: 'A question the rest of the window waits on.',
  Preview: () => (
    <Row>
      <Rename />
    </Row>
  ),
  examples: [
    { title: 'A form', Demo: Rename },
    {
      title: 'A confirmation',
      note: 'The danger tone is for what cannot be taken back.',
      Demo: Remove,
    },
  ],
}

export default story
