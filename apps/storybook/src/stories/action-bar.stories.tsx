import {
  ActionBar,
  Badge,
  Button,
  Icon,
  IconButton,
  type IconGlyph,
  SearchField,
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

const Filter: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M3 5a1 1 0 0 1 1-1h12a1 1 0 0 1 .8 1.6l-4.3 5.6V16a1 1 0 0 1-1.4.9l-3-1.4a1 1 0 0 1-.6-.9v-3.4L3.2 5.6A1 1 0 0 1 3 5Z" />
  </svg>
)

const Sort: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M6 3a1 1 0 0 1 1 1v10.6l1.3-1.3a1 1 0 1 1 1.4 1.4l-3 3a1 1 0 0 1-1.4 0l-3-3a1 1 0 1 1 1.4-1.4L5 14.6V4a1 1 0 0 1 1-1Zm8 14a1 1 0 0 1-1-1V5.4l-1.3 1.3a1 1 0 1 1-1.4-1.4l3-3a1 1 0 0 1 1.4 0l3 3a1 1 0 1 1-1.4 1.4L15 5.4V16a1 1 0 0 1-1 1Z" />
  </svg>
)

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
    const [active, setActive] = useState(false)
    return (
      <ActionBar>
        <IconButton
          active={active}
          label="Filter"
          onClick={() => setActive(!active)}
        >
          <Icon icon={Filter} size={16} />
        </IconButton>
        <IconButton label="Sort">
          <Icon icon={Sort} size={16} />
        </IconButton>
        <Button variant="plain">Reset</Button>
      </ActionBar>
    )
  },
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
