import {
  ActionBar,
  AlertBanner,
  Badge,
  Button,
  Checkbox,
  Dialog,
  DialogBody,
  DialogFooter,
  DialogHeader,
  DialogScrim,
  FormField,
  Input,
  OptionCard,
  Select,
  Textarea,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Stack, TokenTable } from '../components/gallery'
import { Col } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Dialog> = {
  title: 'Components/Dialog',
  component: Dialog,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const FORMATS = [
  { id: 'docx', title: 'DOCX', description: 'Keeps styles and comments.' },
  { id: 'pdf', title: 'PDF', description: 'Fixed layout, no further editing.' },
  { id: 'md', title: 'Markdown', description: 'Plain text, images linked.' },
]

function ExportSheet() {
  const [format, setFormat] = useState('docx')

  return (
    <Dialog>
      <DialogHeader
        title="Export translation"
        subtitle="The source layout is preserved wherever the format allows it."
      />
      <DialogBody>
        <div role="radiogroup" style={{ display: 'flex', gap: '0.5rem' }}>
          {FORMATS.map(entry => (
            <OptionCard
              key={entry.id}
              title={entry.title}
              description={entry.description}
              selected={format === entry.id}
              onClick={() => setFormat(entry.id)}
            />
          ))}
        </div>
      </DialogBody>
      <DialogFooter>
        <Badge tint="neutral">About 3.6 MB</Badge>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <Button>Cancel</Button>
          <Button variant="filled">Export</Button>
        </ActionBar>
      </DialogFooter>
    </Dialog>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Dialog"
      description="The sheet a modal form or a confirmation is built in. Header and footer hold their height and the body scrolls, so a sheet never grows past its scrim however much it carries. All three parts are optional and are separate components rather than props precisely so a sheet can leave one out."
    >
      <Section
        title="A sheet"
        description="Shown without its scrim so the page stays readable. The footer keeps its buttons on one rhythm with an ActionBar, and the size on the left rides opposite them."
      >
        <ExportSheet />
      </Section>

      <Section
        title="Tone"
        description="`danger` is what a destructive confirmation wears. Only the border changes, because the copy and the action already carry the weight — a whole sheet in red would say the app is broken rather than that this step cannot be undone."
      >
        <Dialog tone="danger" width="22rem">
          <DialogHeader
            title="Delete this glossary?"
            subtitle="412 terms will be removed from every project using it."
          />
          <DialogFooter>
            <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
              <Button>Cancel</Button>
              <Button tint="danger" variant="filled">
                Delete
              </Button>
            </ActionBar>
          </DialogFooter>
        </Dialog>
      </Section>

      <Section
        title="Width"
        description="`dialog.width` is the default, and it is the one a sheet should keep. `width` overrides it for the two cases the token cannot cover: a confirmation that would look lost at full width, and a form with a column of its own to fit."
      >
        <Stack>
          <Dialog width="20rem">
            <DialogHeader title="Narrow — a confirmation" />
          </Dialog>
          <Dialog width="34rem">
            <DialogHeader title="Wide — a form with two columns" />
          </Dialog>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('dialog')}
          labelOf={token => token.name.slice('dialog-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const ASheet: Story = {
  render: () => <ExportSheet />,
}

export const AConfirmation: Story = {
  render: () => (
    <Dialog width="22rem">
      <DialogHeader
        title="Discard this draft?"
        subtitle="The 24 segments you edited since the last save will be lost."
      />
      <DialogFooter>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <Button>Keep editing</Button>
          <Button variant="filled">Discard</Button>
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}

export const Danger: Story = {
  render: () => (
    <Dialog tone="danger" width="22rem">
      <DialogHeader
        title="Delete this glossary?"
        subtitle="412 terms will be removed from every project using it."
      />
      <DialogFooter>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <Button>Cancel</Button>
          <Button tint="danger" variant="filled">
            Delete
          </Button>
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}

export const AForm: Story = {
  render: () => (
    <Dialog width="26rem">
      <DialogHeader
        title="Add a provider"
        subtitle="Keys are stored in the system keychain."
      />
      <DialogBody>
        <AlertBanner size="small" tint="info">
          A free key is rate limited to 500,000 characters a month.
        </AlertBanner>
        <FormField label="Name" hint="Shown in the provider list.">
          <Input defaultValue="DeepL — work" />
        </FormField>
        <FormField label="Endpoint">
          <Select defaultValue="pro">
            <option value="pro">api.deepl.com</option>
            <option value="free">api-free.deepl.com</option>
          </Select>
        </FormField>
        <FormField label="API key" hint="Never leaves this device.">
          <Input mono placeholder="sk-…" />
        </FormField>
        <Checkbox checked>Use as the primary provider</Checkbox>
      </DialogBody>
      <DialogFooter>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <Button>Cancel</Button>
          <Button variant="filled">Add provider</Button>
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}

export const AScrollingBody: Story = {
  render: () => (
    <div style={{ display: 'flex', height: '24rem' }}>
      <Dialog width="26rem">
        <DialogHeader
          title="Review the glossary"
          subtitle="412 terms · 24 conflicts"
        />
        <DialogBody>
          {Array.from({ length: 12 }, (_, index) => (
            <FormField
              key={index}
              label={`Term ${index + 1}`}
              hint="Applies to every project."
            >
              <Input defaultValue={`translation ${index + 1}`} />
            </FormField>
          ))}
        </DialogBody>
        <DialogFooter>
          <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
            <Button>Cancel</Button>
            <Button variant="filled">Save all</Button>
          </ActionBar>
        </DialogFooter>
      </Dialog>
    </div>
  ),
}

export const Widths: Story = {
  render: () => (
    <Col>
      <Dialog>
        <DialogHeader title="Default — the dialog.width token" />
      </Dialog>
      <Dialog width="20rem">
        <DialogHeader title="Narrow — a confirmation" />
      </Dialog>
      <Dialog width="34rem">
        <DialogHeader title="Wide — a form with two columns" />
      </Dialog>
    </Col>
  ),
}

export const BodyOnly: Story = {
  render: () => (
    <Dialog width="22rem">
      <DialogBody>
        <Textarea
          placeholder="What went wrong?"
          style={{ minHeight: '6rem' }}
        />
        <ActionBar align="end">
          <Button size="small">Cancel</Button>
          <Button size="small" variant="filled">
            Send
          </Button>
        </ActionBar>
      </DialogBody>
    </Dialog>
  ),
}

export const WithoutASubtitle: Story = {
  render: () => (
    <Dialog width="22rem">
      <DialogHeader title="Rename document" />
      <DialogBody>
        <Input defaultValue="handbook.docx" />
      </DialogBody>
      <DialogFooter>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <Button>Cancel</Button>
          <Button variant="filled">Rename</Button>
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}

/** The scrim covers the viewport, so it gets a story of its own. */
export const Modal: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <DialogScrim>
      <ExportSheet />
    </DialogScrim>
  ),
}
