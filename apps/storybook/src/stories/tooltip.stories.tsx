import {
  Bookmark20Regular,
  Delete20Regular as Trash,
  Pin20Regular,
  Share20Regular,
} from '@fluentui/react-icons'
import {
  ActionBar,
  Button,
  Icon,
  IconButton,
  Tooltip,
  TooltipProvider,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Row } from '../components/stage'

const meta: Meta<typeof Tooltip> = {
  title: 'Components/Tooltip',
  component: Tooltip,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIDES = ['top', 'bottom', 'left', 'right'] as const

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Tooltip"
      description="The label a control surfaces on hover. It never carries anything a reader has to have: it is a reminder of what an icon means, and the accessible name lives on the control itself — which is why an IconButton has one whether or not you ask for it. It replaces the native `title`, which is late, unstyled, and absent from every keyboard."
    >
      <Section
        title="On an icon button"
        description="Every IconButton carries one, built from the `label` it already needs. Nothing is added at the call site: the label was always the accessible name, and now it is also what the pointer is told."
      >
        <ActionBar>
          <IconButton label="Pin this session">
            <Icon icon={Pin20Regular} size={16} />
          </IconButton>
          <IconButton label="Add to reading list">
            <Icon icon={Bookmark20Regular} size={16} />
          </IconButton>
          <IconButton label="Share a link">
            <Icon icon={Share20Regular} size={16} />
          </IconButton>
          <IconButton label="Delete" tint="danger">
            <Icon icon={Trash} size={16} />
          </IconButton>
        </ActionBar>
      </Section>

      <Section
        title="Sides"
        description="`top` is the default and the one to keep: a tooltip under the pointer is a tooltip the hand covers. All four give way to the viewport, so one against an edge flips on its own."
      >
        <Stack>
          {SIDES.map(side => (
            <Specimen key={side} label={side}>
              <Tooltip label={`Opens on the ${side}`} side={side}>
                <Button tint="neutral">{side}</Button>
              </Tooltip>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Delay"
        description="It waits before opening, so crossing a toolbar does not fire four of them. Wrap a group in a TooltipProvider and the wait is shared: once one has opened, moving to its neighbour opens that one at once, which is how a toolbar is actually read."
      >
        <TooltipProvider>
          <ActionBar>
            {['Undo', 'Redo', 'Cut', 'Copy', 'Paste'].map(label => (
              <Tooltip key={label} label={label}>
                <Button size="small" tint="neutral" variant="plain">
                  {label[0]}
                </Button>
              </Tooltip>
            ))}
          </ActionBar>
        </TooltipProvider>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => (
    <Tooltip label="Pin this session">
      <Button tint="neutral">Hover me</Button>
    </Tooltip>
  ),
}

export const Sides: Story = {
  render: () => (
    <Row>
      {SIDES.map(side => (
        <Tooltip key={side} label={`Opens on the ${side}`} side={side}>
          <Button tint="neutral">{side}</Button>
        </Tooltip>
      ))}
    </Row>
  ),
}

export const OnIconButtons: Story = {
  render: () => (
    <ActionBar>
      <IconButton label="Pin this session">
        <Icon icon={Pin20Regular} size={16} />
      </IconButton>
      <IconButton label="Add to reading list">
        <Icon icon={Bookmark20Regular} size={16} />
      </IconButton>
      <IconButton label="Share a link">
        <Icon icon={Share20Regular} size={16} />
      </IconButton>
    </ActionBar>
  ),
}

export const AToolbar: Story = {
  render: () => (
    <TooltipProvider>
      <ActionBar>
        {['Undo', 'Redo', 'Cut', 'Copy', 'Paste'].map(label => (
          <Tooltip key={label} label={label}>
            <Button size="small" tint="neutral" variant="plain">
              {label[0]}
            </Button>
          </Tooltip>
        ))}
      </ActionBar>
    </TooltipProvider>
  ),
}

export const ALongLabel: Story = {
  render: () => (
    <Tooltip label="Runs the whole suite, including the slow integration tests">
      <Button tint="neutral">Run everything</Button>
    </Tooltip>
  ),
}
