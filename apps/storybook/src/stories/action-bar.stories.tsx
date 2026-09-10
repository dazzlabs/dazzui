import {
  Filter20Regular as Filter,
  ArrowSort20Regular as Sort,
} from '@fluentui/react-icons'
import {
  ActionBar,
  Badge,
  Button,
  Icon,
  IconButton,
  KeyCap,
  SearchField,
  Toggle,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof ActionBar> = {
  title: 'Components/ActionBar',
  component: ActionBar,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="ActionBar"
      description="A row of controls kept on one rhythm. It owns no colour and no box — only the gap — so it takes the shared control gap rather than a token of its own, and a footer, a toolbar, and a form's buttons all end up spaced identically without anyone deciding so twice."
    >
      <Section
        title="Alignment"
        description="`end` is a dialog footer's usual shape; `between` splits a secondary action away from the confirming pair, which is what a footer holding both a “Delete” and a “Save” needs."
      >
        <Stack>
          <Specimen label="start">
            <div style={{ flex: 1 }}>
              <ActionBar>
                <Button variant="filled">Save</Button>
                <Button>Cancel</Button>
              </ActionBar>
            </div>
          </Specimen>
          <Specimen label="end">
            <div style={{ flex: 1 }}>
              <ActionBar align="end">
                <Button>Cancel</Button>
                <Button variant="filled">Save</Button>
              </ActionBar>
            </div>
          </Specimen>
          <Specimen label="between">
            <div style={{ flex: 1 }}>
              <ActionBar align="between">
                <Button tint="danger" variant="outlined">
                  Delete
                </Button>
                <Button variant="filled">Save</Button>
              </ActionBar>
            </div>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="What it holds"
        description="Anything sharing the control height lines up in one: icon buttons read as chrome beside labelled ones, and a search field or a badge sits on the same line without either being told the other's size. The bar has no size of its own — set the size on the children and the row follows."
      >
        <Stack>
          <Specimen label="toolbar">
            <ActionBar>
              <IconButton label="Filter">
                <Icon icon={Filter} size={16} />
              </IconButton>
              <IconButton label="Sort">
                <Icon icon={Sort} size={16} />
              </IconButton>
              <Button variant="plain">Reset</Button>
            </ActionBar>
          </Specimen>
          <Specimen label="mixed">
            <ActionBar>
              <Badge tint="neutral">412 terms</Badge>
              <Button>Import</Button>
              <Button variant="filled">New term</Button>
            </ActionBar>
          </Specimen>
          <Specimen label="sizes">
            <ActionBar>
              <Button size="small" variant="filled">
                Save
              </Button>
              <Button size="small">Cancel</Button>
            </ActionBar>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Start: Story = {
  render: () => (
    <ActionBar>
      <Button variant="filled">Save</Button>
      <Button>Cancel</Button>
    </ActionBar>
  ),
}

export const End: Story = {
  render: () => (
    <div style={{ width: '26rem' }}>
      <ActionBar align="end">
        <Button>Cancel</Button>
        <Button variant="filled">Save</Button>
      </ActionBar>
    </div>
  ),
}

export const Between: Story = {
  render: () => (
    <div style={{ width: '26rem' }}>
      <ActionBar align="between">
        <Button tint="danger" variant="outlined">
          Delete
        </Button>
        <Button variant="filled">Save</Button>
      </ActionBar>
    </div>
  ),
}

export const AsAToolbar: Story = {
  render: () => {
    const [filtering, setFiltering] = useState(false)
    return (
      <ActionBar>
        <Toggle
          label="Filter"
          pressed={filtering}
          onPressedChange={setFiltering}
        >
          <Icon icon={Filter} size={16} />
        </Toggle>
        <IconButton label="Sort">
          <Icon icon={Sort} size={16} />
        </IconButton>
        <Button variant="plain">Reset</Button>
      </ActionBar>
    )
  },
}

/*
 * The row a mini window or a popover closes with: a bar on the chrome fill,
 * cornered like a control, holding quiet chips with the confirming action
 * pushed to the far end. The box is the story's, not the component's — the
 * bar owns only the rhythm — so it is drawn here the way a host would draw it.
 */
const row = {
  width: '22.5rem',
  padding: 'var(--spacing-2)',
  background: 'var(--color-surface-chrome)',
  borderRadius: 'var(--radius-medium)',
}

export const AnActionRow: Story = {
  render: () => (
    <ActionBar align="between" style={row}>
      <ActionBar>
        <Button>Preview</Button>
        <Button>Copy</Button>
      </ActionBar>
      <Button variant="filled" shortcut="⏎">
        Save
      </Button>
    </ActionBar>
  ),
}

export const WithAKeyHint: Story = {
  render: () => (
    <ActionBar align="between" style={row}>
      <ActionBar>
        <Button>Favourite</Button>
        <Button>Add tag</Button>
      </ActionBar>
      <KeyCap>⌘⇧E</KeyCap>
    </ActionBar>
  ),
}

export const MixedControls: Story = {
  render: () => {
    const [query, setQuery] = useState('')
    return (
      <div style={{ width: '38rem' }}>
        <ActionBar align="between">
          <div style={{ width: '16rem' }}>
            <SearchField
              placeholder="Search terms"
              value={query}
              onValueChange={setQuery}
            />
          </div>
          <ActionBar>
            <Badge tint="neutral">412 terms</Badge>
            <Button>Import</Button>
            <Button variant="filled">New term</Button>
          </ActionBar>
        </ActionBar>
      </div>
    )
  },
}

export const Sizes: Story = {
  render: () => (
    <Col>
      {(['small', 'medium', 'large'] as const).map(size => (
        <ActionBar key={size}>
          <Button size={size} variant="filled">
            Save
          </Button>
          <Button size={size}>Cancel</Button>
        </ActionBar>
      ))}
    </Col>
  ),
}
