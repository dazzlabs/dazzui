import { defineStory } from '../src/lib/scene'
import scenes from './dialog.scene.json'
import {
  ActionBar,
  Badge,
  Button,
  Callout,
  Checkbox,
  Dialog,
  DialogBody,
  DialogClose,
  DialogFooter,
  DialogHeader,
  FormField,
  TextField,
  OptionCard,
  Select,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Stack, TokenTable } from '../src/components/gallery'
import { componentTokens } from '../src/lib/tokens'

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

/**
 * A sheet is a modal now, so every specimen is opened from a trigger rather
 * than drawn flat on the canvas. `defaultOpen` is what lets a story land with
 * its sheet already up.
 */
function ExportSheet(props: { defaultOpen?: boolean }) {
  const [format, setFormat] = useState('docx')

  return (
    <Dialog trigger={<Button variant="filled">Export…</Button>} {...props}>
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
          <DialogClose render={<Button>Cancel</Button>} />
          <DialogClose render={<Button variant="filled">Export</Button>} />
        </ActionBar>
      </DialogFooter>
    </Dialog>
  )
}

/*
 * The one sentence a confirmation carries — what happens and what it costs.
 * It is body copy, so it goes in the body: a header that carried it as the
 * subtitle would sit directly on the footer, and the two rules between them
 * would read as one thick one.
 */
const message = {
  margin: 0,
  color: 'var(--color-content-secondary)',
  fontFamily: 'var(--body-medium-font-family)',
  fontSize: 'var(--body-medium-font-size)',
  lineHeight: 'var(--body-medium-line-height)',
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Dialog"
      description="The sheet a modal form or a confirmation is built in. Header and footer hold their height and the body scrolls, so a sheet never grows past its scrim however much it carries. All three parts are optional and are separate components rather than props precisely so a sheet can leave one out. The scrim, the focus trap, the scroll lock and the Escape come with it; the title and the subtitle are what a screen reader is told the sheet is."
    >
      <Section
        title="A sheet"
        description="Press the trigger to open it. The footer keeps its buttons on one rhythm with an ActionBar, and the size on the left rides opposite them; wrapping a button in `DialogClose` is what makes it close the sheet."
      >
        <ExportSheet />
      </Section>

      <Section
        title="Tone"
        description="`danger` is what a destructive confirmation wears. Only the border changes — the copy and the action already carry the weight, and a whole sheet in red would say the app is broken rather than that this step cannot be undone — but the sheet also announces as an alert rather than as an ordinary dialog, which is the part a reader with a screen reader gets instead of the border."
      >
        <Dialog
          tone="danger"
          width="22rem"
          trigger={
            <Button tint="danger" variant="filled">
              Delete glossary…
            </Button>
          }
        >
          <DialogHeader title="Delete this glossary?" />
          <DialogBody>
            <p style={message}>
              412 terms will be removed from every project using it.
            </p>
          </DialogBody>
          <DialogFooter>
            <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
              <DialogClose render={<Button>Cancel</Button>} />
              <DialogClose
                render={
                  <Button tint="danger" variant="filled">
                    Delete
                  </Button>
                }
              />
            </ActionBar>
          </DialogFooter>
        </Dialog>
      </Section>

      <Section
        title="Width"
        description="`dialog.width` is the default, and it is the one a sheet should keep. `width` overrides it for the two cases the token cannot cover: a confirmation that would look lost at full width, and a form with a column of its own to fit."
      >
        <Stack>
          <Dialog width="20rem" trigger={<Button>Narrow</Button>}>
            <DialogHeader title="Narrow — a confirmation" />
          </Dialog>
          <Dialog width="34rem" trigger={<Button>Wide</Button>}>
            <DialogHeader title="Wide — a form with two columns" />
          </Dialog>
        </Stack>
      </Section>

      <Section
        title="Dismissible"
        description="`dismissible={false}` makes it an alert dialog: Escape and a press outside stop closing it, and the only way out is a control inside. It is an axis rather than a second component because everything a reader sees is identical — the scrim, the sheet, the three rows — and Base UI's alert dialog is literally the dialog's own parts under a different root. Reach for it where dismissing would itself be an answer."
      >
        <Stack>
          <Dialog width="22rem" trigger={<Button>Dismissible</Button>}>
            <DialogHeader title="Escape closes this one" />
            <DialogFooter>
              <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
                <DialogClose render={<Button>Close</Button>} />
              </ActionBar>
            </DialogFooter>
          </Dialog>
          <Dialog
            dismissible={false}
            tone="danger"
            width="22rem"
            trigger={<Button tint="danger">Answer required</Button>}
          >
            <DialogHeader title="Delete this glossary?" />
            <DialogBody>
              <p style={message}>
                412 terms will be removed from every project using it. Escape
                will not close this — the question has to be answered.
              </p>
            </DialogBody>
            <DialogFooter>
              <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
                <DialogClose render={<Button>Keep it</Button>} />
                <DialogClose
                  render={
                    <Button tint="danger" variant="filled">
                      Delete
                    </Button>
                  }
                />
              </ActionBar>
            </DialogFooter>
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
  parameters: { layout: 'fullscreen' },
  render: () => <ExportSheet defaultOpen />,
}

export const AConfirmation: Story = defineStory(scenes.AConfirmation, {
  parameters: { layout: 'fullscreen' },
})

export const Danger: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Dialog defaultOpen tone="danger" width="22rem">
      <DialogHeader title="Delete this glossary?" />
      <DialogBody>
        <p style={message}>
          412 terms will be removed from every project using it.
        </p>
      </DialogBody>
      <DialogFooter>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <DialogClose render={<Button>Cancel</Button>} />
          <DialogClose
            render={
              <Button tint="danger" variant="filled">
                Delete
              </Button>
            }
          />
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}

export const AForm: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Dialog defaultOpen width="26rem">
      <DialogHeader
        title="Add a provider"
        subtitle="Keys are stored in the system keychain."
      />
      <DialogBody>
        <Callout size="small" tint="info">
          A free key is rate limited to 500,000 characters a month.
        </Callout>
        <FormField label="Name" hint="Shown in the provider list.">
          <TextField defaultValue="DeepL — work" />
        </FormField>
        <FormField label="Endpoint">
          <Select
            defaultValue="pro"
            options={[
              { value: 'pro', label: 'api.deepl.com' },
              { value: 'free', label: 'api-free.deepl.com' },
            ]}
          />
        </FormField>
        <FormField label="API key" hint="Never leaves this device.">
          <TextField mono placeholder="sk-…" />
        </FormField>
        <Checkbox defaultChecked>Use as the primary provider</Checkbox>
      </DialogBody>
      <DialogFooter>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <DialogClose render={<Button>Cancel</Button>} />
          <DialogClose
            render={<Button variant="filled">Add provider</Button>}
          />
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}

export const AScrollingBody: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Dialog defaultOpen width="26rem">
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
            <TextField defaultValue={`translation ${index + 1}`} />
          </FormField>
        ))}
      </DialogBody>
      <DialogFooter>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <DialogClose render={<Button>Cancel</Button>} />
          <DialogClose render={<Button variant="filled">Save all</Button>} />
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}

export const Widths: Story = {
  render: () => (
    <ActionBar>
      <Dialog trigger={<Button>Default</Button>}>
        <DialogHeader title="Default — the dialog.width token" />
      </Dialog>
      <Dialog width="20rem" trigger={<Button>Narrow</Button>}>
        <DialogHeader title="Narrow — a confirmation" />
      </Dialog>
      <Dialog width="34rem" trigger={<Button>Wide</Button>}>
        <DialogHeader title="Wide — a form with two columns" />
      </Dialog>
    </ActionBar>
  ),
}

export const BodyOnly: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Dialog defaultOpen width="22rem">
      <DialogBody>
        <TextField
          multiline
          placeholder="What went wrong?"
          style={{ minHeight: '6rem' }}
        />
        <ActionBar align="end">
          <DialogClose render={<Button size="small">Cancel</Button>} />
          <DialogClose
            render={
              <Button size="small" variant="filled">
                Send
              </Button>
            }
          />
        </ActionBar>
      </DialogBody>
    </Dialog>
  ),
}

export const WithoutASubtitle: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Dialog defaultOpen width="22rem">
      <DialogHeader title="Rename document" />
      <DialogBody>
        <TextField defaultValue="handbook.docx" />
      </DialogBody>
      <DialogFooter>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <DialogClose render={<Button>Cancel</Button>} />
          <DialogClose render={<Button variant="filled">Rename</Button>} />
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}

/** A sheet opened from another, which is what `nested` is drawn for. */
export const Nested: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Dialog defaultOpen width="26rem">
      <DialogHeader
        title="Glossary settings"
        subtitle="Applies to every project on this account."
      />
      <DialogBody>
        <FormField label="Name">
          <TextField defaultValue="Product terms" />
        </FormField>
      </DialogBody>
      <DialogFooter>
        <Dialog
          tone="danger"
          width="22rem"
          trigger={
            <Button tint="danger" variant="plain">
              Delete…
            </Button>
          }
        >
          <DialogHeader title="Delete this glossary?" />
          <DialogBody>
            <p style={message}>
              412 terms will be removed from every project using it.
            </p>
          </DialogBody>
          <DialogFooter>
            <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
              <DialogClose render={<Button>Cancel</Button>} />
              <DialogClose
                render={
                  <Button tint="danger" variant="filled">
                    Delete
                  </Button>
                }
              />
            </ActionBar>
          </DialogFooter>
        </Dialog>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <DialogClose render={<Button>Cancel</Button>} />
          <DialogClose render={<Button variant="filled">Save</Button>} />
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}

export const AnAlert: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Dialog defaultOpen dismissible={false} tone="danger" width="22rem">
      <DialogHeader title="Delete this glossary?" />
      <DialogBody>
        <p style={message}>
          412 terms will be removed from every project using it. Escape will not
          close this — the question has to be answered.
        </p>
      </DialogBody>
      <DialogFooter>
        <ActionBar align="end" style={{ marginInlineStart: 'auto' }}>
          <DialogClose render={<Button>Keep it</Button>} />
          <DialogClose
            render={
              <Button tint="danger" variant="filled">
                Delete
              </Button>
            }
          />
        </ActionBar>
      </DialogFooter>
    </Dialog>
  ),
}
