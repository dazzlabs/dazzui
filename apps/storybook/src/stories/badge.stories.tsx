import { Circle20Filled as Dot } from '@fluentui/react-icons'
import {
  Badge,
  type BadgeVariant,
  Icon,
  NavItem,
  PreferenceRow,
  PreferenceSection,
  Table,
  TableCell,
  TableHead,
  TableRow,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { label } from '../lib/label'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof Badge> = {
  title: 'Components/Badge',
  component: Badge,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The recipes a badge reads on any ramp; `raised` has no tint and is shown on its own. */
const VARIANTS: BadgeVariant[] = ['filled', 'tinted', 'outlined']

/* The grey card a `raised` badge is made for — off it, the paper chip is
   invisible on paper. */
const card = {
  display: 'inline-flex',
  gap: 'var(--spacing-2)',
  padding: 'var(--spacing-3)',
  background: 'var(--color-surface-muted)',
  borderRadius: 'var(--radius-large)',
}

const SIZES = ['small', 'medium', 'large'] as const

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Badge"
      description="A short label that names a state or a count. It is not interactive, so it wears only the resting state of a control recipe and deliberately skips the shared state machine — a label that changed colour under the pointer would read as something you can press. Pick the recipe with `variant`, pick the ramp it resolves against with `tint`."
    >
      <Section
        title="Variant and tint"
        description="Every combination the two axes allow — three of the six recipes, the ones that read as a label rather than as a control — plus the one variant the badge owns. `raised` is paper fill and quiet ink, no ramp: the chip that sits on a grey card, where a `tinted` neutral would sink into the card and a `filled` one would shout. There is no `normal` or `plain` badge, because both would be text with padding around it."
      >
        <Stack>
          {VARIANTS.map(variant => (
            <Specimen key={variant} label={variant}>
              {TINTS.map(tint => (
                <Badge key={tint} tint={tint} variant={variant}>
                  {label(tint)}
                </Badge>
              ))}
            </Specimen>
          ))}
          <Specimen label="raised">
            <div style={card}>
              <Badge variant="raised">Draft</Badge>
              <Badge variant="raised">3 items</Badge>
              <Badge variant="raised">Synced</Badge>
            </div>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Sizes"
        description="Each size borrows the padding of the control size one step below it, and sets its label in the caption face, so a badge reads as a label rather than as a control that stopped working."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Badge size={size}>Beta</Badge>
              <Badge size={size} tint="success" variant="filled">
                Live
              </Badge>
              <Badge size={size} tint="neutral" variant="outlined">
                Draft
              </Badge>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Where it lands"
        description="The trailing cell of a table, the trailing slot of a preference row, the count on a navigation row. In all three it annotates something the row already names, which is why it stays small and never carries the only copy that matters. A leading dot needs no prop: the badge is an inline flex row and the glyph inherits its content colour."
      >
        <Stack>
          <Specimen label="glyph">
            <Badge tint="success">
              <Icon icon={Dot} size={8} /> Connected
            </Badge>
            <Badge tint="warning">
              <Icon icon={Dot} size={8} /> Degraded
            </Badge>
            <Badge tint="danger">
              <Icon icon={Dot} size={8} /> Offline
            </Badge>
          </Specimen>
          <Specimen label="counts">
            <Badge tint="neutral">412 terms</Badge>
            <Badge tint="primary">168 / 392</Badge>
            <Badge tint="neutral" variant="outlined">
              v2.4.1
            </Badge>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Tinted: Story = {
  render: () => <Badge>Beta</Badge>,
}

export const Filled: Story = {
  render: () => (
    <Badge tint="success" variant="filled">
      Live
    </Badge>
  ),
}

export const Outlined: Story = {
  render: () => (
    <Badge tint="neutral" variant="outlined">
      Draft
    </Badge>
  ),
}

export const Raised: Story = {
  render: () => (
    <div style={card}>
      <Badge variant="raised">Draft</Badge>
      <Badge variant="raised">3 items</Badge>
    </div>
  ),
}

/* The three recipes on the paper, then the paper chip on the card it is
   made for — `raised` on the bare canvas would be a label with no box. */
export const AllVariants: Story = {
  render: () => (
    <Row>
      {VARIANTS.map(variant => (
        <Badge key={variant} variant={variant}>
          {label(variant)}
        </Badge>
      ))}
      <div style={card}>
        <Badge variant="raised">Raised</Badge>
      </div>
    </Row>
  ),
}

export const States: Story = {
  render: () => (
    <Row>
      <Badge tint="success">Connected</Badge>
      <Badge tint="warning">Rate limited</Badge>
      <Badge tint="danger" variant="filled">
        Failed
      </Badge>
      <Badge tint="neutral" variant="outlined">
        Draft
      </Badge>
    </Row>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      {VARIANTS.map(variant => (
        <Row key={variant}>
          {TINTS.map(tint => (
            <Badge key={tint} tint={tint} variant={variant}>
              {label(tint)}
            </Badge>
          ))}
        </Row>
      ))}
    </Col>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Row>
      {SIZES.map(size => (
        <Badge key={size} size={size} tint="success" variant="filled">
          Live
        </Badge>
      ))}
    </Row>
  ),
}

export const WithAGlyph: Story = {
  render: () => (
    <Row>
      <Badge tint="success">
        <Icon icon={Dot} size={8} /> Connected
      </Badge>
      <Badge tint="warning">
        <Icon icon={Dot} size={8} /> Degraded
      </Badge>
      <Badge tint="danger">
        <Icon icon={Dot} size={8} /> Offline
      </Badge>
    </Row>
  ),
}

export const Counts: Story = {
  render: () => (
    <Row>
      <Badge tint="neutral">412 terms</Badge>
      <Badge tint="primary">168 / 392</Badge>
      <Badge tint="neutral" variant="outlined">
        v2.4.1
      </Badge>
    </Row>
  ),
}

export const InATable: Story = {
  render: () => (
    <div style={{ width: '32rem' }}>
      <Table>
        <TableHead>
          <TableCell head>Provider</TableCell>
          <TableCell head align="end">
            Status
          </TableCell>
        </TableHead>
        <TableRow>
          <TableCell>DeepL</TableCell>
          <TableCell align="end">
            <Badge size="small" tint="success">
              Connected
            </Badge>
          </TableCell>
        </TableRow>
        <TableRow>
          <TableCell>Google Translate</TableCell>
          <TableCell align="end">
            <Badge size="small" tint="neutral">
              Idle
            </Badge>
          </TableCell>
        </TableRow>
        <TableRow>
          <TableCell>Custom endpoint</TableCell>
          <TableCell align="end">
            <Badge size="small" tint="danger">
              Unreachable
            </Badge>
          </TableCell>
        </TableRow>
      </Table>
    </div>
  ),
}

export const InAPreferenceRow: Story = {
  render: () => (
    <div style={{ width: '30rem' }}>
      <PreferenceSection label="Providers">
        <PreferenceRow title="Primary" subtitle="DeepL">
          <Badge tint="success">Connected</Badge>
        </PreferenceRow>
        <PreferenceRow title="Fallback" subtitle="Google Translate">
          <Badge tint="neutral">Idle</Badge>
        </PreferenceRow>
      </PreferenceSection>
    </div>
  ),
}

export const InANavRow: Story = {
  render: () => (
    <Col gap="var(--spacing-1)" width="13rem">
      <NavItem active>
        Needs review
        <Badge
          size="small"
          tint="neutral"
          style={{ marginInlineStart: 'auto' }}
        >
          24
        </Badge>
      </NavItem>
      <NavItem>
        Done
        <Badge
          size="small"
          tint="neutral"
          style={{ marginInlineStart: 'auto' }}
        >
          168
        </Badge>
      </NavItem>
    </Col>
  ),
}
