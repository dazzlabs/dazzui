import {
  Button,
  Checkbox,
  Drawer,
  DrawerClose,
  type DrawerSide,
  FormField,
  TextField,
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
import { Col, Row } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Drawer> = {
  title: 'Components/Drawer',
  component: Drawer,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIDES: DrawerSide[] = ['right', 'left', 'top', 'bottom']

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Drawer"
      description="A sheet that comes in from an edge and can be pushed back out. It is the Dialog's sibling, and the choice between them is about where the reader's attention has to go rather than about the screen it is on: a dialog takes the middle of the view and asks a question; a drawer takes a side of it and holds a panel the reader dips into — filters, a detail pane — while what they were reading stays where it was."
    >
      <Section
        title="Sides"
        description="It keeps a corner only on the two sides still inside the view. A sheet flush to an edge with all four corners rounded reads as floating just off the screen rather than as attached to it. It arrives from its own edge and leaves the same way; Base UI drives the transform while a swipe is in progress."
      >
        <Stack>
          {SIDES.map(side => (
            <Specimen key={side} label={side}>
              <Drawer
                side={side}
                trigger={<Button size="small">{side}</Button>}
                title="Filters"
                description="Narrow what the list is showing."
              >
                <Checkbox defaultChecked>Open</Checkbox>
                <Checkbox>Merged</Checkbox>
                <Checkbox>Closed</Checkbox>
              </Drawer>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="The three rows"
        description="Header, body and footer are props here rather than the separate components a Dialog has, because those read a dialog's own context — a DialogHeader inside a drawer would find no title to register. Only the body scrolls, so a long panel keeps its heading and its actions in view."
      >
        <Drawer
          trigger={<Button variant="filled">Edit remote</Button>}
          title="Edit remote"
          description="origin · git@github.com:acme/web.git"
          footer={
            <Row>
              <Button variant="filled">Save</Button>
              <DrawerClose render={<Button>Cancel</Button>} />
            </Row>
          }
        >
          <FormField label="Name">
            <TextField defaultValue="origin" mono />
          </FormField>
          <FormField label="URL" hint="SSH or HTTPS">
            <TextField defaultValue="git@github.com:acme/web.git" mono />
          </FormField>
        </Drawer>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('drawer')}
          labelOf={token => token.name.slice('drawer-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const Bare: Story = {
  render: () => (
    <Drawer
      trigger={<Button>Open drawer</Button>}
      title="Filters"
      description="Narrow what the list is showing."
    >
      <Checkbox defaultChecked>Open</Checkbox>
      <Checkbox>Merged</Checkbox>
      <Checkbox>Closed</Checkbox>
    </Drawer>
  ),
}

export const Sides: Story = {
  render: () => (
    <Row>
      {SIDES.map(side => (
        <Drawer
          key={side}
          side={side}
          trigger={<Button size="small">{side}</Button>}
          title="Filters"
        >
          <Checkbox defaultChecked>Open</Checkbox>
          <Checkbox>Merged</Checkbox>
        </Drawer>
      ))}
    </Row>
  ),
}

export const WithFooter: Story = {
  render: () => (
    <Drawer
      trigger={<Button variant="filled">Edit remote</Button>}
      title="Edit remote"
      description="origin · git@github.com:acme/web.git"
      footer={
        <Row>
          <Button variant="filled">Save</Button>
          <DrawerClose render={<Button>Cancel</Button>} />
        </Row>
      }
    >
      <FormField label="Name">
        <TextField defaultValue="origin" mono />
      </FormField>
      <FormField label="URL" hint="SSH or HTTPS">
        <TextField defaultValue="git@github.com:acme/web.git" mono />
      </FormField>
    </Drawer>
  ),
}

export const Controlled: Story = {
  render: () => {
    const [open, setOpen] = useState(false)
    return (
      <Col>
        <Button onClick={() => setOpen(true)}>Open</Button>
        <Drawer
          open={open}
          onOpenChange={setOpen}
          title="Detail"
          description="Driven by the caller rather than by a trigger."
          footer={<Button onClick={() => setOpen(false)}>Done</Button>}
        >
          Nothing here opened it; the button beside it did.
        </Drawer>
      </Col>
    )
  },
}

export const Wide: Story = {
  render: () => (
    <Drawer
      trigger={<Button>Open a wide drawer</Button>}
      size="32rem"
      title="Review"
      description="A panel that needs more room than the token gives it."
    >
      The width is a token, and `size` is the override for a panel that
      genuinely needs more.
    </Drawer>
  ),
}
