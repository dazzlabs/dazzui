import { ArrowDownload20Regular as Download } from '@fluentui/react-icons'
import {
  ActionBar,
  Button,
  type ButtonVariant,
  Icon,
  Kbd,
  Spinner,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const VARIANTS: ButtonVariant[] = [
  'normal',
  'filled',
  'tinted',
  'outlined',
  'plain',
]

const SIZES = ['small', 'medium', 'large'] as const

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Button"
      description="The five control recipes, each on any ramp. Hover, press, and disabled are not written here at all — `data-control` puts the recipe's own states on the element, so a button's stylesheet is the box and nothing else."
    >
      <Section
        title="Variants"
        description="`filled` is the one action a view points at; `normal` and `outlined` are its neighbours; `tinted` is a quiet chip; `plain` looks like text at rest but keeps the box, because its recipe washes on hover."
      >
        <Stack>
          {VARIANTS.map(variant => (
            <Specimen key={variant} label={variant}>
              <Button variant={variant}>Save changes</Button>
              <Button variant={variant} disabled>
                Disabled
              </Button>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="Hover any of these: the hover and press states come from the same recipe, so a danger button darkens the way a primary one does. Nothing below sets a colour — each row is one variant read against six ramps."
      >
        <Stack>
          {VARIANTS.map(variant => (
            <Specimen key={variant} label={variant}>
              {TINTS.map(tint => (
                <Button key={tint} tint={tint} variant={variant}>
                  {tint}
                </Button>
              ))}
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Sizes"
        description="Button keeps no per-size block at all: the `[data-size]` profile carries the height, the padding, the radius and the type face, and the stylesheet reads them once."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Button size={size} variant="filled">
                Export
              </Button>
              <Button size={size}>Cancel</Button>
              <Button size={size} variant="plain">
                Learn more
              </Button>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Content"
        description="An icon is a child rather than a prop, so it costs the component no API and takes the button's own colour through `currentColor`. A shortcut is the one trailing slot, set in the button's colour at a lowered opacity — a key cap inside a button would read as a second control. There is no `loading` prop either: work in flight is a spinner in the label and `disabled` on the button."
      >
        <Stack>
          <Specimen label="icon">
            <Button variant="filled">
              <Icon icon={Download} size={14} />
              Export
            </Button>
          </Specimen>
          <Specimen label="shortcut">
            <Button variant="filled" shortcut="⌘S">
              Translate
            </Button>
            <Button shortcut="⌘⇧S">Translate all</Button>
            <span>
              A Kbd is the other way to print a key: <Kbd size="small">⌘F</Kbd>
            </span>
          </Specimen>
          <Specimen label="busy">
            <Button disabled variant="filled">
              <Spinner size="small" tint="primary" />
              Translating…
            </Button>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="In a row"
        description="Where more than one button sits together, the row is an ActionBar so the gap comes from the control scale. The confirming action goes last and is the only filled one: two filled buttons side by side ask the reader to choose which is the default."
      >
        <Stack>
          <Specimen label="confirming">
            <ActionBar align="end">
              <Button>Cancel</Button>
              <Button variant="filled">Export</Button>
            </ActionBar>
          </Specimen>
          <Specimen label="destructive">
            <ActionBar align="end">
              <Button>Keep</Button>
              <Button tint="danger" variant="filled">
                Delete glossary
              </Button>
            </ActionBar>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Normal: Story = {
  render: () => <Button>Save changes</Button>,
}

export const Filled: Story = {
  render: () => <Button variant="filled">Save changes</Button>,
}

export const Tinted: Story = {
  render: () => <Button variant="tinted">Save changes</Button>,
}

export const Outlined: Story = {
  render: () => <Button variant="outlined">Save changes</Button>,
}

export const Plain: Story = {
  render: () => <Button variant="plain">Save changes</Button>,
}

export const AllVariants: Story = {
  render: () => (
    <Row>
      {VARIANTS.map(variant => (
        <Button key={variant} variant={variant}>
          {variant}
        </Button>
      ))}
    </Row>
  ),
}

export const Disabled: Story = {
  render: () => (
    <Row>
      {VARIANTS.map(variant => (
        <Button key={variant} disabled variant={variant}>
          {variant}
        </Button>
      ))}
    </Row>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Row>
      {SIZES.map(size => (
        <Button key={size} size={size} variant="filled">
          Export
        </Button>
      ))}
    </Row>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      {(['filled', 'tinted', 'outlined', 'normal', 'plain'] as const).map(
        variant => (
          <Row key={variant}>
            {TINTS.map(tint => (
              <Button key={tint} tint={tint} variant={variant}>
                {tint}
              </Button>
            ))}
          </Row>
        )
      )}
    </Col>
  ),
}

export const WithAnIcon: Story = {
  render: () => (
    <Row>
      <Button variant="filled">
        <Icon icon={Download} size={14} />
        Export
      </Button>
      <Button>
        <Icon icon={Download} size={14} />
        Download
      </Button>
      <Button variant="plain">
        <Icon icon={Download} size={14} />
        Save a copy
      </Button>
    </Row>
  ),
}

export const WithAShortcut: Story = {
  render: () => (
    <Row>
      <Button variant="filled" shortcut="⌘S">
        Translate
      </Button>
      <Button shortcut="⌘⇧S">Translate all</Button>
    </Row>
  ),
}

export const Busy: Story = {
  render: () => (
    <Row>
      <Button disabled variant="filled">
        <Spinner size="small" tint="primary" />
        Translating…
      </Button>
      <Button disabled>
        <Spinner size="small" tint="neutral" />
        Checking the key…
      </Button>
    </Row>
  ),
}

export const FullWidth: Story = {
  render: () => (
    <Col width="18rem">
      <Button fullWidth variant="filled">
        Connect provider
      </Button>
      <Button fullWidth>Use without an account</Button>
      <Button fullWidth tint="danger" variant="plain">
        Remove this device
      </Button>
    </Col>
  ),
}

export const Destructive: Story = {
  render: () => (
    <Row>
      <Button tint="danger" variant="filled">
        Delete glossary
      </Button>
      <Button tint="danger" variant="outlined">
        Delete
      </Button>
      <Button tint="danger" variant="plain">
        Remove this device
      </Button>
    </Row>
  ),
}

export const InAnActionBar: Story = {
  render: () => (
    <Col width="26rem">
      <ActionBar align="end">
        <Button>Cancel</Button>
        <Button variant="filled">Export</Button>
      </ActionBar>
      <ActionBar align="between">
        <Button tint="danger" variant="outlined">
          Delete
        </Button>
        <Button variant="filled">Save</Button>
      </ActionBar>
    </Col>
  ),
}
