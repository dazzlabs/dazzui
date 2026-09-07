import {
  Flash20Regular as Bolt,
  Checkmark20Regular as Check,
  Document16Regular as Doc16,
  Document20Regular as Doc,
  Document24Regular as Doc24,
  Globe20Regular as Globe,
} from '@fluentui/react-icons'
import {
  ActionBar,
  Badge,
  Button,
  Callout,
  Icon,
  IconButton,
  SidebarMenuButton,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof Icon> = {
  title: 'Components/Icon',
  component: Icon,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** Stand-ins, so the gallery does not pull in an icon set of its own. */
/** A 24-viewBox glyph, to show that the box is scaled rather than assumed. */
/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Icon"
      description="Draws a glyph at a given size. The system draws with Fluent, which `ui_react` depends on, but `Icon` is not bound to it: the prop takes any component that renders an svg, so a product can pass its own set without forking the components that use one."
    >
      <Section
        title="Sizes"
        description="`size` sets the rendered edge length, in whatever unit CSS takes. There is no size scale here: an icon is sized by the thing it sits in — a small button's label, a rail row, a badge — so the number belongs at the call site. A fixed-size glyph is scaled to fit, so a 24-unit set and a 20-unit set line up without either being redrawn."
      >
        <Stack>
          <Specimen label="12 → 32">
            <Icon icon={Bolt} size={12} />
            <Icon icon={Bolt} size={14} />
            <Icon icon={Bolt} size={16} />
            <Icon icon={Bolt} size={20} />
            <Icon icon={Bolt} size={24} />
            <Icon icon={Bolt} size={32} />
          </Specimen>
          <Specimen label="20 vs 24">
            <Icon icon={Doc} size={24} />
            <Icon icon={Globe} size={24} />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Colour"
        description="Fill rides on `currentColor`, so an icon takes the tone of whatever it sits in and no component has to pass a colour down to it. That is what lets one glyph go inside a filled button, a tinted badge, and a muted caption without three variants of it."
      >
        <Stack>
          <Specimen label="in badges">
            {TINTS.map(tint => (
              <Badge key={tint} tint={tint}>
                <Icon icon={Check} size={12} /> {tint}
              </Badge>
            ))}
          </Specimen>
          <Specimen label="content roles">
            <span>
              <Icon icon={Doc} size={16} />
            </span>
            <span style={{ color: 'var(--color-content-muted)' }}>
              <Icon icon={Doc} size={16} />
            </span>
            <span style={{ color: 'var(--color-content-subtle)' }}>
              <Icon icon={Doc} size={16} />
            </span>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Where it lands"
        description="In each case the glyph is a child, not a prop on the component — which is why the system needs no icon registry and a product can drop its own set in."
      >
        <Stack>
          <Specimen label="button">
            <Button variant="filled">
              <Icon icon={Bolt} size={14} />
              Translate
            </Button>
            <IconButton label="Open">
              <Icon icon={Doc} size={16} />
            </IconButton>
          </Specimen>
          <Specimen label="nav row">
            <div style={{ width: '11rem' }}>
              <SidebarMenuButton active icon={<Icon icon={Doc} size={14} />}>
                <span>handbook.docx</span>
              </SidebarMenuButton>
            </div>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => <Icon icon={Bolt} />,
}

export const Sizes: Story = {
  render: () => (
    <Row>
      {[12, 14, 16, 20, 24, 32].map(size => (
        <Icon key={size} icon={Bolt} size={size} />
      ))}
    </Row>
  ),
}

export const SizedByAToken: Story = {
  render: () => (
    <Row>
      <Icon icon={Bolt} size="var(--spacing-4)" />
      <Icon icon={Bolt} size="var(--spacing-6)" />
      <Icon icon={Bolt} size="var(--spacing-8)" />
    </Row>
  ),
}

export const AnyViewBox: Story = {
  render: () => (
    <Row>
      <Icon icon={Doc16} size={24} />
      <Icon icon={Doc} size={24} />
      <Icon icon={Doc24} size={24} />
    </Row>
  ),
}

export const InheritedColour: Story = {
  render: () => (
    <Col>
      <Row>
        {TINTS.map(tint => (
          <Badge key={tint} tint={tint}>
            <Icon icon={Check} size={12} /> {tint}
          </Badge>
        ))}
      </Row>
      <Row>
        <span>
          <Icon icon={Doc} size={20} />
        </span>
        <span style={{ color: 'var(--color-content-muted)' }}>
          <Icon icon={Doc} size={20} />
        </span>
        <span style={{ color: 'var(--color-content-subtle)' }}>
          <Icon icon={Doc} size={20} />
        </span>
      </Row>
    </Col>
  ),
}

export const InAButton: Story = {
  render: () => (
    <Row>
      <Button variant="filled">
        <Icon icon={Bolt} size={14} />
        Translate
      </Button>
      <Button>
        <Icon icon={Doc} size={14} />
        Open document
      </Button>
    </Row>
  ),
}

export const InAToolbar: Story = {
  render: () => (
    <ActionBar>
      <IconButton label="Translate">
        <Icon icon={Bolt} size={16} />
      </IconButton>
      <IconButton label="Open">
        <Icon icon={Doc} size={16} />
      </IconButton>
      <IconButton label="Done">
        <Icon icon={Check} size={16} />
      </IconButton>
    </ActionBar>
  ),
}

export const InANavRow: Story = {
  render: () => (
    <Col gap="var(--spacing-1)" width="13rem">
      <SidebarMenuButton active icon={<Icon icon={Doc} size={14} />}>
        <span>Documents</span>
      </SidebarMenuButton>
      <SidebarMenuButton icon={<Icon icon={Globe} size={14} />}>
        <span>Glossaries</span>
      </SidebarMenuButton>
      <SidebarMenuButton icon={<Icon icon={Bolt} size={14} />}>
        <span>Shortcuts</span>
      </SidebarMenuButton>
    </Col>
  ),
}

export const InACallout: Story = {
  render: () => (
    <div style={{ width: '32rem' }}>
      <Callout
        icon={<Icon icon={Check} size={16} />}
        tint="success"
        title="All 15 pages translated"
      >
        Layout and formulas were preserved.
      </Callout>
    </div>
  ),
}
