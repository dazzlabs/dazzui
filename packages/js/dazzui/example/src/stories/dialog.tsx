import {
  Button,
  Checkbox,
  Dialog,
  DialogBody,
  DialogClose,
  DialogFooter,
  DialogHeader,
  FormField,
  RadioGroup,
  Select,
  TextField,
} from '@dazzlabs/dazzui'

import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const ROLES = [
  { value: 'viewer', label: 'Viewer' },
  { value: 'editor', label: 'Editor' },
  { value: 'admin', label: 'Admin' },
]

const VISIBILITY = [
  { value: 'private', label: 'Only me' },
  { value: 'team', label: 'My team' },
  { value: 'public', label: 'Anyone with the link' },
]

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

function Invite() {
  return (
    <Dialog width="26rem" trigger={<Button>Invite…</Button>}>
      <DialogHeader
        title="Invite a member"
        subtitle="They will get an email with a link to join."
      />
      <DialogBody>
        <FormField label="Email">
          <TextField type="email" required placeholder="name@example.com" />
        </FormField>
        <FormField label="Role" hint="Admins can invite others.">
          <Select options={ROLES} defaultValue="editor" />
        </FormField>
        <Checkbox defaultChecked>Add them to every project</Checkbox>
      </DialogBody>
      <DialogFooter>
        <DialogClose render={<Button variant="outlined">Cancel</Button>} />
        <DialogClose render={<Button variant="filled">Send invite</Button>} />
      </DialogFooter>
    </Dialog>
  )
}

function NewProject() {
  return (
    <Dialog width="28rem" trigger={<Button>New project…</Button>}>
      <DialogHeader title="New project" />
      <DialogBody>
        <FormField label="Name">
          <TextField required placeholder="Quarterly report" />
        </FormField>
        <FormField label="Description" hint="Optional.">
          <TextField
            multiline
            rows={3}
            placeholder="What is this project for?"
          />
        </FormField>
        <FormField label="Who can see it">
          <RadioGroup options={VISIBILITY} defaultValue="team" />
        </FormField>
      </DialogBody>
      <DialogFooter>
        <DialogClose render={<Button variant="outlined">Cancel</Button>} />
        <DialogClose render={<Button variant="filled">Create</Button>} />
      </DialogFooter>
    </Dialog>
  )
}

function ChangeEmail() {
  return (
    <Dialog width="24rem" trigger={<Button>Change email…</Button>}>
      <DialogHeader
        title="Change email"
        subtitle="We will send a code to the new address."
      />
      <DialogBody>
        <FormField
          label="New email"
          state="error"
          hint="That does not look like an address."
        >
          <TextField type="email" state="error" defaultValue="ada@" />
        </FormField>
        <FormField label="Password" hint="To confirm it is you.">
          <TextField type="password" required />
        </FormField>
      </DialogBody>
      <DialogFooter>
        <DialogClose render={<Button variant="outlined">Cancel</Button>} />
        <DialogClose render={<Button variant="filled">Send code</Button>} />
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
      title: 'An invitation',
      note: 'A field, a choice and an option, each with its label above it.',
      Demo: Invite,
    },
    {
      title: 'A longer form',
      note: 'A description that wraps, and a choice laid out in full.',
      Demo: NewProject,
    },
    {
      title: 'A field in error',
      note: 'The hint turns into the reason, under the field it is about.',
      Demo: ChangeEmail,
    },
    {
      title: 'A confirmation',
      note: 'The danger tone is for what cannot be taken back.',
      Demo: Remove,
    },
  ],
}

export default story
