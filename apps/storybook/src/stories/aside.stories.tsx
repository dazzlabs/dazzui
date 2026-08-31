import {
  Aside,
  Badge,
  Button,
  Divider,
  NavItem,
  Progress,
  Rail,
  RailGroup,
  SectionLabel,
  Sidebar,
  SidebarGroup,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import type { ReactNode } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { Col, Frame, Pane } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Aside> = {
  title: 'Components/Aside',
  component: Aside,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

function Field({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div>
      <SectionLabel>{label}</SectionLabel>
      <div style={{ marginTop: 'var(--spacing-2)' }}>{children}</div>
    </div>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Aside"
      description="The trailing information column. It carries what the current selection *is*, where the sidebar and rail carry what you can navigate to — which is why it hangs off the opposite edge and takes no rows. It shares the sidebar's surface, so a window reads as content between two quieter edges rather than as three different greys."
    >
      <Section
        title="Fields, not rows"
        description="A label and the value under it. A divider marks a change of subject, and anything measurable gets the component that measures it rather than a number in prose. An action here operates on the selection, not on the pane, so it stays at the foot below the fields it follows from."
      >
        <Frame>
          <Pane />
          <Aside>
            <Field label="Source">handbook.docx</Field>
            <Divider />
            <Field label="Progress">
              <Col gap="var(--spacing-2)">
                <Progress tint="primary" value={43} />
                <span style={{ color: 'var(--color-content-muted)' }}>
                  168 of 392 segments
                </span>
              </Col>
            </Field>
            <Field label="Provider">
              <Badge size="small" tint="success">
                Connected
              </Badge>
            </Field>
            <div style={{ marginBlockStart: 'auto' }}>
              <Button fullWidth size="small">
                Edit source
              </Button>
            </div>
          </Aside>
        </Frame>
      </Section>

      <Section
        title="The whole shell"
        description="Sidebar, rail, content, aside — the shape a document workspace settles into. The two navigation columns lighten as they approach the content; the aside comes back to the sidebar's surface because it is an edge again. Its width is a frame metric rather than an Aside token: the columns of a window are one scale, and a column that sized itself would only agree with the others by luck."
      >
        <Frame width="100%">
          <Sidebar>
            <SidebarGroup>
              <NavItem active>Documents</NavItem>
              <NavItem>Glossaries</NavItem>
              <NavItem>Settings</NavItem>
            </SidebarGroup>
          </Sidebar>
          <Rail>
            <RailGroup>
              <NavItem active size="small">
                handbook.docx
              </NavItem>
              <NavItem size="small">contract.pdf</NavItem>
              <NavItem size="small">release-notes.md</NavItem>
            </RailGroup>
          </Rail>
          <Pane />
          <Aside>
            <Field label="Source">handbook.docx</Field>
            <Field label="Progress">168 of 392 segments</Field>
            <Field label="Provider">
              <Badge size="small" tint="success">
                Connected
              </Badge>
            </Field>
          </Aside>
        </Frame>
      </Section>

      <Section title="Frame metrics">
        <TokenTable
          tokens={componentTokens('frame')}
          labelOf={token => token.name.slice('frame-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const BesideAPane: Story = {
  render: () => (
    <Frame>
      <Pane />
      <Aside>
        <Field label="Source">handbook.docx</Field>
        <Divider />
        <Field label="Progress">168 of 392 segments</Field>
        <Field label="Provider">
          <Badge size="small" tint="success">
            Connected
          </Badge>
        </Field>
      </Aside>
    </Frame>
  ),
}

export const AnInspector: Story = {
  render: () => (
    <Frame height="26rem">
      <Pane />
      <Aside>
        <Field label="Document">handbook.docx</Field>
        <Field label="Pages">15</Field>
        <Divider />
        <Field label="Progress">
          <Col gap="var(--spacing-2)">
            <Progress tint="primary" value={43} />
            <span style={{ color: 'var(--color-content-muted)' }}>
              168 of 392 segments
            </span>
          </Col>
        </Field>
        <Divider />
        <Field label="Quality">
          <Badge size="small" tint="warning">
            24 low scores
          </Badge>
        </Field>
        <Field label="Glossary">Product terms</Field>
      </Aside>
    </Frame>
  ),
}

export const WithAnAction: Story = {
  render: () => (
    <Frame>
      <Pane />
      <Aside>
        <Field label="Term">segment</Field>
        <Field label="Translation">段</Field>
        <Field label="Scope">
          <Badge size="small" tint="neutral">
            Global
          </Badge>
        </Field>
        <div style={{ marginBlockStart: 'auto' }}>
          <Col gap="var(--spacing-2)">
            <Button fullWidth size="small">
              Edit term
            </Button>
            <Button fullWidth size="small" tint="danger" variant="plain">
              Remove
            </Button>
          </Col>
        </div>
      </Aside>
    </Frame>
  ),
}

export const InAThreeColumnWindow: Story = {
  render: () => (
    <Frame height="26rem" width="58rem">
      <Sidebar>
        <SidebarGroup>
          <NavItem active>Documents</NavItem>
          <NavItem>Glossaries</NavItem>
          <NavItem>Settings</NavItem>
        </SidebarGroup>
      </Sidebar>
      <Rail>
        <RailGroup>
          <NavItem active size="small">
            handbook.docx
          </NavItem>
          <NavItem size="small">contract.pdf</NavItem>
          <NavItem size="small">release-notes.md</NavItem>
        </RailGroup>
      </Rail>
      <Pane />
      <Aside>
        <Field label="Source">handbook.docx</Field>
        <Field label="Progress">168 of 392 segments</Field>
        <Field label="Provider">
          <Badge size="small" tint="success">
            Connected
          </Badge>
        </Field>
      </Aside>
    </Frame>
  ),
}
