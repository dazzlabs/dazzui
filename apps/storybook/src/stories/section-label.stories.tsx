import {
  Aside,
  Card,
  Divider,
  NavItem,
  SectionLabel,
  Sidebar,
  SidebarCard,
  SidebarGroup,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Frame, Row } from '../components/stage'

const meta: Meta<typeof SectionLabel> = {
  title: 'Components/SectionLabel',
  component: SectionLabel,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="SectionLabel"
      description="The micro-heading that organises a pane. Sentence case, no uppercasing and no added tracking — one size covers sidebars, rails, and pane headings, so tone is the only axis worth varying. `SidebarGroup`, `RailGroup`, `SidebarCard` and `PreferenceSection` all render one for you; this is what those are made of."
    >
      <Section
        title="Tints"
        description="Untinted it takes the muted content colour, which is what a heading almost always wants. A tint moves it onto a ramp, for a heading that has to carry a state with it — a section reporting a failure, a group flagged for review."
      >
        <Stack>
          <Specimen label="default">
            <SectionLabel>Quality signals</SectionLabel>
          </Specimen>
          <Specimen label="tinted">
            {TINTS.map(tint => (
              <SectionLabel key={tint} tint={tint}>
                {tint}
              </SectionLabel>
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Over a panel"
        description="It names what the numbers below it are. The card already sets the gap, so the label needs no margin — which is the reason it is a span rather than a heading element with a browser margin to undo."
      >
        <Row>
          <Card style={{ width: '13rem' }}>
            <SectionLabel>Segments</SectionLabel>
            <span style={{ fontSize: 'var(--title-large-font-size)' }}>
              392
            </span>
          </Card>
          <Card tint="warning" variant="tinted" style={{ width: '13rem' }}>
            <SectionLabel tint="warning">Low scores</SectionLabel>
            <span style={{ fontSize: 'var(--title-large-font-size)' }}>24</span>
          </Card>
        </Row>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => <SectionLabel>Quality signals</SectionLabel>,
}

/** The quieter tone, for a column that already has a heading above it. */
export const Faint: Story = {
  render: () => (
    <Col>
      <SectionLabel>Quality signals</SectionLabel>
      <SectionLabel faint>Quality signals</SectionLabel>
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Row>
      {TINTS.map(tint => (
        <SectionLabel key={tint} tint={tint}>
          {tint}
        </SectionLabel>
      ))}
    </Row>
  ),
}

export const InAPanel: Story = {
  render: () => (
    <Row>
      <Card style={{ width: '13rem' }}>
        <SectionLabel>Segments</SectionLabel>
        <span style={{ fontSize: 'var(--title-large-font-size)' }}>392</span>
      </Card>
      <Card tint="warning" variant="tinted" style={{ width: '13rem' }}>
        <SectionLabel tint="warning">Low scores</SectionLabel>
        <span style={{ fontSize: 'var(--title-large-font-size)' }}>24</span>
      </Card>
    </Row>
  ),
}

export const InAColumn: Story = {
  render: () => (
    <Frame height="20rem" width="44rem">
      <Sidebar>
        <SidebarGroup>
          <NavItem active>Documents</NavItem>
          <NavItem>Glossaries</NavItem>
        </SidebarGroup>
        <SidebarGroup label="Library">
          <NavItem>Translation memory</NavItem>
          <NavItem>Archive</NavItem>
        </SidebarGroup>
        <SidebarCard label="Today">
          <span style={{ fontSize: 'var(--body-small-font-size)' }}>
            148 segments translated
          </span>
        </SidebarCard>
      </Sidebar>
      <div style={{ flex: 1, minWidth: 0 }} />
      <Aside>
        <div>
          <SectionLabel>Source</SectionLabel>
          <p style={{ margin: 'var(--spacing-2) 0 0' }}>handbook.docx</p>
        </div>
        <Divider />
        <div>
          <SectionLabel>Progress</SectionLabel>
          <p style={{ margin: 'var(--spacing-2) 0 0' }}>168 of 392 segments</p>
        </div>
      </Aside>
    </Frame>
  ),
}
