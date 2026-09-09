import {
  Button,
  Checkbox,
  FormField,
  Icon,
  IconButton,
  Popover,
  PopoverClose,
  SegmentedControl,
  TextField,
} from '@dazzlabs/dazzui-react'
import { Filter16Regular, Info16Regular } from '@fluentui/react-icons'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof Popover> = {
  title: 'Components/Popover',
  component: Popover,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Popover"
      description="A panel of anything, anchored to what opened it. It is the sibling of Menu rather than of Dialog: it wears the same floating sheet and is positioned against its trigger, but it holds content rather than rows — a form, a filter, a short explanation — and so it is the one popup that does not roll its focus through items. Reach for a Dialog instead when the reader has to finish before doing anything else; a popover closes on an outside press, which makes it the wrong home for something half-typed."
    >
      <Section
        title="A heading and a line"
        description="The title is the panel's accessible name and the description its accessible description, so they are elements Base UI knows about rather than a heading a reader can see and a screen reader cannot."
      >
        <Popover
          trigger={<Button>About this build</Button>}
          title="Build 4.2.1"
          description="Published 3 days ago from main."
        >
          <Row>
            <PopoverClose render={<Button size="tiny">Release notes</Button>} />
          </Row>
        </Popover>
      </Section>

      <Section
        title="Sides"
        description="Base UI positions it and flips it when it would collide. The gap between the panel and its trigger is padding on the positioner rather than an offset passed in code, so the gap is a token and the collision logic counts it when the panel flips."
      >
        <Stack>
          {(['top', 'bottom', 'left', 'right'] as const).map(side => (
            <Specimen key={side} label={side}>
              <Popover
                side={side}
                trigger={<Button size="small">{side}</Button>}
                title="Anchored"
                description={`Opens on the ${side}.`}
              />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="A panel with controls in it"
        description="This is what separates it from a Tooltip: a popover can be reached and typed into. A tooltip must carry nothing a reader needs, because a pointer never gets there."
      >
        <Popover
          trigger={
            <IconButton label="Filter">
              <Icon icon={Filter16Regular} size={16} />
            </IconButton>
          }
          title="Filter"
          align="start"
          width="16rem"
        >
          <Col gap="var(--spacing-3)">
            <Checkbox defaultChecked>Open</Checkbox>
            <Checkbox>Merged</Checkbox>
            <Checkbox>Closed</Checkbox>
          </Col>
        </Popover>
      </Section>
    </Page>
  ),
}

export const Bare: Story = {
  render: () => (
    <Popover
      trigger={<Button>About this build</Button>}
      title="Build 4.2.1"
      description="Published 3 days ago from main."
    />
  ),
}

export const Sides: Story = {
  render: () => (
    <Row>
      {(['top', 'bottom', 'left', 'right'] as const).map(side => (
        <Popover
          key={side}
          side={side}
          trigger={<Button size="small">{side}</Button>}
          title="Anchored"
          description={`Opens on the ${side}.`}
        />
      ))}
    </Row>
  ),
}

export const WithControls: Story = {
  render: () => (
    <Popover
      trigger={
        <IconButton label="Filter">
          <Icon icon={Filter16Regular} size={16} />
        </IconButton>
      }
      title="Filter"
      align="start"
      width="16rem"
    >
      <Col gap="var(--spacing-3)">
        <Checkbox defaultChecked>Open</Checkbox>
        <Checkbox>Merged</Checkbox>
        <Checkbox>Closed</Checkbox>
      </Col>
    </Popover>
  ),
}

export const AForm: Story = {
  render: () => {
    const [open, setOpen] = useState(false)
    const [name, setName] = useState('release/4.2')

    return (
      <Popover
        open={open}
        onOpenChange={setOpen}
        trigger={<Button variant="filled">Rename branch</Button>}
        title="Rename branch"
        description="The remote keeps its own name until you push."
        width="20rem"
      >
        <Col gap="var(--spacing-3)">
          <FormField label="Branch name">
            <TextField
              mono
              value={name}
              onChange={event => setName(event.target.value)}
            />
          </FormField>
          <Row>
            <Button variant="filled" onClick={() => setOpen(false)}>
              Rename
            </Button>
            <PopoverClose render={<Button>Cancel</Button>} />
          </Row>
        </Col>
      </Popover>
    )
  },
}

export const InAToolbar: Story = {
  render: () => {
    const [scope, setScope] = useState('all')

    return (
      <Row>
        <SegmentedControl
          aria-label="Scope"
          items={[
            { value: 'all', label: 'All' },
            { value: 'mine', label: 'Mine' },
          ]}
          value={scope}
          onValueChange={setScope}
        />
        <Popover
          trigger={
            <IconButton label="Filter">
              <Icon icon={Filter16Regular} size={16} />
            </IconButton>
          }
          title="Filter"
          align="end"
          width="15rem"
        >
          <Col gap="var(--spacing-3)">
            <Checkbox defaultChecked>Open</Checkbox>
            <Checkbox>Draft</Checkbox>
          </Col>
        </Popover>
        <Popover
          trigger={
            <IconButton label="About">
              <Icon icon={Info16Regular} size={16} />
            </IconButton>
          }
          title="About"
          description="42 pull requests, 8 of them yours."
          align="end"
        />
      </Row>
    )
  },
}
