import {
  ActionBar,
  Badge,
  Button,
  Dialog,
  DialogBody,
  DialogFooter,
  DialogHeader,
  DialogScrim,
  OptionCard,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Dialog> = {
  title: 'Components/Dialog',
  component: Dialog,
}

export default meta
type Story = StoryObj<typeof meta>

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
          <OptionCard
            title="DOCX"
            description="Keeps styles and comments."
            selected={format === 'docx'}
            onClick={() => setFormat('docx')}
          />
          <OptionCard
            title="PDF"
            description="Fixed layout, no further editing."
            selected={format === 'pdf'}
            onClick={() => setFormat('pdf')}
          />
          <OptionCard
            title="Markdown"
            description="Plain text, images linked."
            selected={format === 'markdown'}
            onClick={() => setFormat('markdown')}
          />
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

export const Overview: Story = {
  render: () => (
    <Page
      title="Dialog"
      description="The sheet a modal form or a confirmation is built in. Header and footer hold their height and the body scrolls, so a sheet never grows past its scrim however much it carries."
    >
      <Section
        title="A sheet"
        description="Shown without its scrim so the page stays readable. The footer keeps its buttons on one rhythm with an ActionBar, and the size on the left rides opposite them."
      >
        <ExportSheet />
      </Section>

      <Section
        title="Danger"
        description="The tone a destructive confirmation wears; only the border changes, because the copy and the action already carry the weight."
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

/** The scrim covers the viewport, so it gets a story of its own. */
export const Modal: Story = {
  render: () => (
    <DialogScrim>
      <ExportSheet />
    </DialogScrim>
  ),
}
