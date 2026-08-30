import {
  ActionBar,
  Button,
  type ButtonVariant,
  Icon,
  type IconGlyph,
  Kbd,
  Spinner,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
}

export default meta
type Story = StoryObj<typeof meta>

const VARIANTS: ButtonVariant[] = [
  'recessed',
  'normal',
  'filled',
  'tinted',
  'outlined',
  'plain',
]

const SIZES = ['small', 'medium', 'large'] as const

const Download: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M10 2.5a1 1 0 0 1 1 1v7.1l2.3-2.3a1 1 0 1 1 1.4 1.4l-4 4a1 1 0 0 1-1.4 0l-4-4a1 1 0 1 1 1.4-1.4L9 10.6V3.5a1 1 0 0 1 1-1ZM3.5 15a1 1 0 0 1 1 1v.5h11V16a1 1 0 1 1 2 0v.5a1.5 1.5 0 0 1-1.5 1.5h-12A1.5 1.5 0 0 1 2.5 16.5V16a1 1 0 0 1 1-1Z" />
  </svg>
)

export const Overview: Story = {
  render: () => (
    <Page
      title="Button"
      description="The five control recipes, each on any ramp. Hover, press, and disabled are not written here at all — `data-control` puts the recipe's own states on the element, so a button's stylesheet is the box and nothing else."
    >
      <Section
        title="The set"
        description="`filled` is the one action a view points at; `normal` and `outlined` are its neighbours; `tinted` is a quiet chip; `plain` looks like text at rest but keeps the box, because its recipe washes on hover."
      >
        <Stack>
          <Specimen label="a footer">
            <Button variant="plain">Learn more</Button>
            <Button>Cancel</Button>
            <Button variant="filled">Save changes</Button>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Variants: Story = {
  render: () => (
    <Page
      title="Variants"
      description="Every recipe, at rest and disabled. Hover and press are on all of them; the difference between the five is only where on the ramp each state lands."
    >
      <Section title="Enabled and disabled">
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
    </Page>
  ),
}

export const Tints: Story = {
  render: () => (
    <Page
      title="Tints"
      description="Hover any of these: the hover and press states come from the same recipe, so a danger button darkens the way a primary one does. Nothing below sets a colour — each row is one variant read against six ramps."
    >
      <Section title="Every ramp, every variant">
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
    </Page>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Page
      title="Sizes"
      description="Button keeps no per-size block at all: the `[data-size]` profile carries the height, the padding, the radius and the type face, and the stylesheet reads them once."
    >
      <Section title="The three profiles">
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
    </Page>
  ),
}

export const WithAnIcon: Story = {
  render: () => (
    <Page
      title="With an icon"
      description="An icon is a child rather than a prop, so it costs the component no API and takes the button's own colour through `currentColor`. Size it a step under the label's line so it sits on the text rather than over it."
    >
      <Section title="Leading glyph">
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Button size={size} variant="filled">
                <Icon icon={Download} size={size === 'large' ? 18 : 14} />
                Export
              </Button>
              <Button size={size}>
                <Icon icon={Download} size={size === 'large' ? 18 : 14} />
                Download
              </Button>
            </Specimen>
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

export const WithAShortcut: Story = {
  render: () => (
    <Page
      title="With a shortcut"
      description="A trailing hint naming the key that also fires the action. It is set in the button's own colour at a lowered opacity rather than in a key cap: a cap inside a button would read as a second control."
    >
      <Section title="On the action">
        <Stack>
          <Specimen label="confirming">
            <Button variant="filled" shortcut="⌘S">
              Translate
            </Button>
            <Button shortcut="⌘⇧S">Translate all</Button>
          </Specimen>
          <Specimen label="in prose">
            <span>
              A Kbd is the other way to print a key: <Kbd size="small">⌘F</Kbd>
            </span>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Busy: Story = {
  render: () => (
    <Page
      title="Busy"
      description="There is no `loading` prop. Work in flight is a spinner in the label and `disabled` on the button, which is two things the system already has — and it leaves the caller to decide whether the label changes with it."
    >
      <Section title="While the work runs">
        <Stack>
          <Specimen label="filled">
            <Button disabled variant="filled">
              <Spinner size="small" tint="primary" />
              Translating…
            </Button>
          </Specimen>
          <Specimen label="normal">
            <Button disabled>
              <Spinner size="small" tint="neutral" />
              Checking the key…
            </Button>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const FullWidth: Story = {
  render: () => (
    <Page
      title="Full width"
      description="Fills its container instead of sizing to its label, for a button that ends a column — a form, a sheet, a sidebar footer."
    >
      <Section title="In a column">
        <div style={{ maxWidth: '18rem' }}>
          <Stack>
            <Button fullWidth variant="filled">
              Connect provider
            </Button>
            <Button fullWidth>Use without an account</Button>
            <Button fullWidth tint="danger" variant="plain">
              Remove this device
            </Button>
          </Stack>
        </div>
      </Section>
    </Page>
  ),
}

export const InAnActionBar: Story = {
  render: () => (
    <Page
      title="In an action bar"
      description="Where more than one button sits together, the row is an ActionBar so the gap comes from the control scale rather than from whatever the surrounding layout happened to set."
    >
      <Section
        title="Confirming"
        description="The confirming action goes last, and it is the only filled one: two filled buttons side by side ask the reader to choose which is the default."
      >
        <ActionBar align="end">
          <Button>Cancel</Button>
          <Button variant="filled">Export</Button>
        </ActionBar>
      </Section>

      <Section
        title="Destructive"
        description="A destructive action is filled on the danger ramp, and the way back out stays neutral beside it."
      >
        <ActionBar align="end">
          <Button>Keep</Button>
          <Button tint="danger" variant="filled">
            Delete glossary
          </Button>
        </ActionBar>
      </Section>

      <Section
        title="Split"
        description="`between` pushes a secondary action away from the pair, which is what a footer holding a “Delete” and a “Save” needs."
      >
        <ActionBar align="between">
          <Button tint="danger" variant="outlined">
            Delete
          </Button>
          <Button variant="filled">Save</Button>
        </ActionBar>
      </Section>
    </Page>
  ),
}

export const Tokens: Story = {
  render: () => (
    <Page
      title="Tokens"
      description="What the component owns. Everything else — height, padding, radius, type — is the shared control profile, documented on Tokens/Control."
    >
      <Section title="Button">
        <TokenTable
          tokens={componentTokens('button')}
          labelOf={token => token.name.slice('button-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
