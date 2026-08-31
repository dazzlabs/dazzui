import {
  ActionBar,
  AlertBanner,
  Badge,
  Button,
  Icon,
  IconButton,
  type IconGlyph,
  NavItem,
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
const Check: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M16.7 5.3a1 1 0 0 1 0 1.4l-7.5 7.5a1 1 0 0 1-1.4 0l-3.5-3.5a1 1 0 1 1 1.4-1.4l2.8 2.8 6.8-6.8a1 1 0 0 1 1.4 0Z" />
  </svg>
)

const Bolt: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M11.6 1.6a.6.6 0 0 1 1 .6l-1.5 5.3h4a.7.7 0 0 1 .5 1.2l-8 9.7a.6.6 0 0 1-1-.6l1.5-5.3h-4a.7.7 0 0 1-.5-1.2l8-9.7Z" />
  </svg>
)

const Doc: IconGlyph = props => (
  <svg viewBox="0 0 20 20" {...props}>
    <path d="M5.5 2h5.1a2 2 0 0 1 1.4.6l3.4 3.4a2 2 0 0 1 .6 1.4v8.1A2.5 2.5 0 0 1 13.5 18h-8A2.5 2.5 0 0 1 3 15.5v-11A2.5 2.5 0 0 1 5.5 2Zm5.5 1.9V6a1 1 0 0 0 1 1h2.1L11 3.9Z" />
  </svg>
)

/** A 24-viewBox glyph, to show that the box is scaled rather than assumed. */
const Globe: IconGlyph = props => (
  <svg viewBox="0 0 24 24" {...props}>
    <path
      d="M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18Zm0 0c2.5 2.2 3.8 5 3.8 9s-1.3 6.8-3.8 9c-2.5-2.2-3.8-5-3.8-9s1.3-6.8 3.8-9ZM3.5 9h17M3.5 15h17"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.6"
    />
  </svg>
)

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Icon"
      description="Draws a glyph at a given size. The system depends on no icon set: an icon set is the one thing a product replaces wholesale, and hard-coding one would make that a fork rather than a prop. Pass any component that renders an svg."
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
              <NavItem active icon={<Icon icon={Doc} size={14} />}>
                handbook.docx
              </NavItem>
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
      <Icon icon={Doc} size={24} />
      <Icon icon={Globe} size={24} />
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
      <NavItem active icon={<Icon icon={Doc} size={14} />}>
        Documents
      </NavItem>
      <NavItem icon={<Icon icon={Globe} size={14} />}>Glossaries</NavItem>
      <NavItem icon={<Icon icon={Bolt} size={14} />}>Shortcuts</NavItem>
    </Col>
  ),
}

export const InABanner: Story = {
  render: () => (
    <div style={{ width: '32rem' }}>
      <AlertBanner
        icon={<Icon icon={Check} size={16} />}
        tint="success"
        title="All 15 pages translated"
      >
        Layout and formulas were preserved.
      </AlertBanner>
    </div>
  ),
}
