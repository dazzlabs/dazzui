import {
  Add20Regular as Plus,
  Book20Regular as Book,
  ChevronDown20Regular as Chevron,
  Document20Regular as Doc,
  MoreHorizontal20Regular as More,
  Settings20Regular as Gear,
} from '@fluentui/react-icons'
import {
  Badge,
  Card,
  Icon,
  Progress,
  SearchField,
  SectionLabel,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupAction,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuAction,
  SidebarMenuBadge,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
  type SidebarProps,
  SidebarProvider,
  SidebarRail,
  SidebarSeparator,
  SidebarTrigger,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { type ReactNode, useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { Col, Frame, Row } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Sidebar> = {
  title: 'Components/Sidebar',
  component: Sidebar,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** A window: the frame, and the provider every sidebar has to stand in. */
function Window({
  height,
  width,
  children,
}: {
  height?: string
  width?: string
  children: ReactNode
}) {
  return (
    <Frame height={height} width={width}>
      <SidebarProvider>{children}</SidebarProvider>
    </Frame>
  )
}

/** The pane beside a column, with the trigger where a toolbar would put it. */
function Pane({ children }: { children?: ReactNode }) {
  return (
    <SidebarInset>
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: 'var(--spacing-2)',
          height: 'var(--frame-titlebar-size)',
          paddingInline: 'var(--spacing-3)',
          borderBlockEnd: 'var(--stroke-hairline) solid var(--color-border)',
        }}
      >
        <SidebarTrigger />
      </div>
      <div style={{ flex: 1, minWidth: 0, padding: 'var(--spacing-5)' }}>
        {children}
      </div>
    </SidebarInset>
  )
}

/** Rows are full-width, so on their own they need a column to be read fairly. */
function Column({
  children,
  width = '13rem',
}: {
  children: ReactNode
  width?: string
}) {
  return (
    <div
      style={{
        width,
        padding: 'var(--spacing-2)',
        /* The container corner, not the control corner: Bright makes the
           control steps pills, and a column of rows is not a control. */
        borderRadius: 'var(--radius-large)',
        background: 'var(--color-surface-muted)',
      }}
    >
      <SidebarMenu>{children}</SidebarMenu>
    </div>
  )
}

const PANES = [
  { label: 'Documents', icon: Doc },
  { label: 'Glossaries', icon: Book },
  { label: 'Settings', icon: Gear },
]

function Panes({
  pane,
  onPick,
  size,
}: {
  pane: string
  onPick: (pane: string) => void
  size?: 'small' | 'medium'
}) {
  return (
    <SidebarMenu>
      {PANES.map(entry => (
        <SidebarMenuItem key={entry.label}>
          <SidebarMenuButton
            active={pane === entry.label}
            icon={<Icon icon={entry.icon} size={14} />}
            size={size}
            tooltip={entry.label}
            onClick={() => onPick(entry.label)}
          >
            <span>{entry.label}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      ))}
    </SidebarMenu>
  )
}

/** The column most stories are: two groups, and a card at the foot. */
function TheSidebar({
  collapsible,
  variant,
}: Pick<SidebarProps, 'collapsible' | 'variant'>) {
  const [pane, setPane] = useState('Documents')

  return (
    <Sidebar collapsible={collapsible} variant={variant}>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <Panes pane={pane} onPick={setPane} />
          </SidebarGroupContent>
        </SidebarGroup>
        <SidebarGroup>
          <SidebarGroupLabel>Library</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  icon={<Icon icon={Book} size={14} />}
                  tooltip="Translation memory"
                >
                  <span>Translation memory</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  icon={<Icon icon={Doc} size={14} />}
                  tooltip="Archive"
                >
                  <span>Archive</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      {/* The foot holds a row rather than a card, so it can collapse to a
          glyph with the rest of the column. */}
      <SidebarFooter>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              icon={<Icon icon={Book} size={16} />}
              size="large"
              tooltip="Acme Ltd"
              variant="outline"
            >
              <span
                style={{
                  display: 'flex',
                  flexDirection: 'column',
                  gap: 'var(--spacing-0-5)',
                }}
              >
                <span>Acme Ltd</span>
                <span style={{ color: 'var(--color-content-muted)' }}>
                  12 members
                </span>
              </span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
      {collapsible === 'none' ? null : <SidebarRail />}
    </Sidebar>
  )
}

/** The second column: what is inside whatever the sidebar selected. */
function TheRail() {
  const [item, setItem] = useState('handbook.docx')
  return (
    <Sidebar collapsible="none">
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              {['handbook.docx', 'contract.pdf', 'release-notes.md'].map(
                entry => (
                  <SidebarMenuItem key={entry}>
                    <SidebarMenuButton
                      active={item === entry}
                      size="small"
                      onClick={() => setItem(entry)}
                    >
                      <span>{entry}</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                )
              )}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  )
}

function Field({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div>
      <SectionLabel>{label}</SectionLabel>
      <div style={{ marginTop: 'var(--spacing-2)' }}>{children}</div>
    </div>
  )
}

/** The information column: what the selection *is*, on the other edge. */
function TheAside() {
  return (
    <Sidebar collapsible="none" side="right">
      <SidebarContent
        style={{ gap: 'var(--spacing-5)', padding: 'var(--spacing-4)' }}
      >
        <Field label="Source">handbook.docx</Field>
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
      </SidebarContent>
    </Sidebar>
  )
}

/** Every kind of thing a group can hold. */
function GroupsColumn() {
  return (
    <Sidebar>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton active icon={<Icon icon={Doc} size={14} />}>
                  <span>Documents</span>
                </SidebarMenuButton>
                <SidebarMenuBadge>24</SidebarMenuBadge>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton icon={<Icon icon={Book} size={14} />}>
                  <span>Glossaries</span>
                </SidebarMenuButton>
                <SidebarMenuBadge>3</SidebarMenuBadge>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
        <SidebarGroup>
          <SidebarGroupLabel>Library</SidebarGroupLabel>
          <SidebarGroupAction label="Add to the library">
            <Icon icon={Plus} size={14} />
          </SidebarGroupAction>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton>
                  <span>Translation memory</span>
                </SidebarMenuButton>
                <SidebarMenuAction showOnHover label="More">
                  <Icon icon={More} size={14} />
                </SidebarMenuAction>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton>
                  <span>Archive</span>
                </SidebarMenuButton>
                <SidebarMenuAction showOnHover label="More">
                  <Icon icon={More} size={14} />
                </SidebarMenuAction>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
        <SidebarGroup>
          <SidebarGroupLabel>Shared</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton icon={<Icon icon={Chevron} size={14} />}>
                  <span>Team glossary</span>
                </SidebarMenuButton>
                <SidebarMenuSub>
                  <SidebarMenuSubItem>
                    <SidebarMenuSubButton active>
                      <span>Product terms</span>
                    </SidebarMenuSubButton>
                  </SidebarMenuSubItem>
                  <SidebarMenuSubItem>
                    <SidebarMenuSubButton>
                      <span>Legal</span>
                    </SidebarMenuSubButton>
                  </SidebarMenuSubItem>
                </SidebarMenuSub>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton>
                  <span>Style guide</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  )
}

function SizesRow() {
  return (
    <Row>
      <Column width="11rem">
        <SidebarMenuItem>
          <SidebarMenuButton
            active
            icon={<Icon icon={Doc} size={12} />}
            size="small"
          >
            <span>handbook.docx</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
        <SidebarMenuItem>
          <SidebarMenuButton icon={<Icon icon={Doc} size={12} />} size="small">
            <span>contract.pdf</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </Column>
      <Column width="11rem">
        <SidebarMenuItem>
          <SidebarMenuButton active icon={<Icon icon={Doc} size={14} />}>
            <span>handbook.docx</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
        <SidebarMenuItem>
          <SidebarMenuButton icon={<Icon icon={Doc} size={14} />}>
            <span>contract.pdf</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </Column>
      <Column width="11rem">
        <SidebarMenuItem>
          <SidebarMenuButton
            icon={<Icon icon={Book} size={16} />}
            size="large"
            variant="outline"
          >
            <span
              style={{
                display: 'flex',
                flexDirection: 'column',
                gap: 'var(--spacing-0-5)',
              }}
            >
              <span>Acme Ltd</span>
              <span style={{ color: 'var(--color-content-muted)' }}>
                12 members
              </span>
            </span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </Column>
    </Row>
  )
}

function VariantsColumn() {
  return (
    <Column>
      <SidebarMenuItem>
        <SidebarMenuButton active>
          <span>default, selected</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton>
          <span>default</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton variant="outline">
          <span>outline</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton variant="plain">
          <Icon icon={Plus} size={14} />
          <span>plain</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </Column>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Sidebar"
      description="The column a window is organised around, and every part it is built from. There is one of it: a second navigation column is a second Sidebar with a narrower width and no way to collapse, and the information column on the far edge is one with `side` set the other way. What differs between them is what is put inside, not the column."
    >
      <Section
        title="The whole shell"
        description="Sidebar, a second column, the content, an aside. The first is the one that opens and closes — ⌘B, the trigger in the toolbar, or the rail along its edge — and the other two stay, because a column that comes and goes moves the content under the pointer. Their widths are frame metrics rather than props: the columns of a window are one scale, and a column that sized itself would only agree with the others by luck."
      >
        <Window height="24rem" width="100%">
          <TheSidebar />
          <TheRail />
          <Pane />
          <TheAside />
        </Window>
      </Section>

      <Section
        title="Groups"
        description="Rows come in groups. The gap between groups does most of the work and the label only names it, and the first run usually goes unlabelled because the window already names it. A group can carry an action on its label; a row can carry one on its trailing edge, or a count, or a run of sub-rows set in under it."
      >
        <Window height="26rem" width="100%">
          <GroupsColumn />
          <Pane />
        </Window>
      </Section>

      <Section
        title="Collapsing"
        description="`offcanvas` slides the column out and leaves the pane the whole window; `icon` narrows it to its glyphs, with each row's label in a tooltip on the right. Either way the content inside slides rather than reflows, because the column is drawn over the space it takes rather than in it."
      >
        <Stack>
          <Specimen label="icon">
            <Window height="18rem" width="40rem">
              <TheSidebar collapsible="icon" />
              <Pane />
            </Window>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Variants"
        description="`sidebar` runs the window's full height with a hairline where it meets the content. `floating` is a card set in from the edge by a step. `inset` is the opposite: the window takes the sidebar's ground, and the content is the sheet raised on it."
      >
        <Stack>
          <Specimen label="floating">
            <Window height="18rem" width="40rem">
              <TheSidebar variant="floating" />
              <Pane />
            </Window>
          </Specimen>
          <Specimen label="inset">
            <Window height="18rem" width="40rem">
              <TheSidebar variant="inset" />
              <Pane />
            </Window>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Rows"
        description="Hover an idle row: the wash comes off the tinted ramp, which is the opposite of what a menu item or a table row gets. The difference is what selection looks like — a nav row selects with the filled recipe, so a tinted hover previews it from well below. Two sizes for a list, and a third for the two-line row an account takes. `plain` is the row that adds rather than selects, printed in the ramp instead of filled with it."
      >
        <Stack>
          <Specimen label="states">
            <Column width="11rem">
              <SidebarMenuItem>
                <SidebarMenuButton active icon={<Icon icon={Doc} size={14} />}>
                  <span>Current page</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton icon={<Icon icon={Doc} size={14} />}>
                  <span>Idle</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  disabled
                  icon={<Icon icon={Doc} size={14} />}
                >
                  <span>Disabled</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </Column>
          </Specimen>
          <Specimen label="sizes">
            <SizesRow />
          </Specimen>
          <Specimen label="variants">
            <VariantsColumn />
          </Specimen>
          <Specimen label="tints">
            <Row>
              {TINTS.map(tint => (
                <SidebarMenuButton
                  key={tint}
                  active
                  tint={tint}
                  style={{ width: 'auto' }}
                >
                  {tint}
                </SidebarMenuButton>
              ))}
            </Row>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Frame metrics"
        description="The shell's shared scale, which the columns and the window chrome both alias. The sidebar owns none of its own: a column that sized itself would agree with the titlebar only by luck."
      >
        <TokenTable
          tokens={componentTokens('frame')}
          labelOf={token => token.name.slice('frame-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const AColumn: Story = {
  render: () => (
    <Window width="44rem">
      <TheSidebar />
      <Pane />
    </Window>
  ),
}

export const Groups: Story = {
  render: () => (
    <Window height="26rem" width="44rem">
      <GroupsColumn />
      <Pane />
    </Window>
  ),
}

export const WithAHeader: Story = {
  render: () => {
    const [query, setQuery] = useState('')
    const [pane, setPane] = useState('Documents')
    return (
      <Window width="44rem">
        <Sidebar>
          <SidebarHeader>
            <SearchField
              placeholder="Search"
              size="small"
              value={query}
              onValueChange={setQuery}
            />
          </SidebarHeader>
          <SidebarContent>
            <SidebarGroup>
              <SidebarGroupContent>
                <Panes pane={pane} onPick={setPane} />
              </SidebarGroupContent>
            </SidebarGroup>
          </SidebarContent>
        </Sidebar>
        <Pane />
      </Window>
    )
  },
}

export const WithAFooter: Story = {
  render: () => (
    <Window height="24rem" width="44rem">
      <Sidebar>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupContent>
              <SidebarMenu>
                {Array.from({ length: 10 }, (_, index) => (
                  <SidebarMenuItem key={index}>
                    <SidebarMenuButton active={index === 0}>
                      <span>Document {index + 1}</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
        <SidebarSeparator />
        <SidebarFooter>
          <Card>
            <SectionLabel>Today</SectionLabel>
            <span style={{ fontSize: 'var(--body-small-font-size)' }}>
              148 of 392 segments
            </span>
            <Progress size="small" value={38} />
          </Card>
        </SidebarFooter>
      </Sidebar>
      <Pane />
    </Window>
  ),
}

export const WithAnAction: Story = {
  render: () => (
    <Window width="44rem">
      <Sidebar>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton active>
                    <span>Documents</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton>
                    <span>Glossaries</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupLabel>Workspaces</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton>
                    <span>Personal</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton>
                    <span>Acme Ltd</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton variant="plain">
                    <Icon icon={Plus} size={14} />
                    <span>New workspace</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
      <Pane />
    </Window>
  ),
}

export const CollapsingOffcanvas: Story = {
  render: () => (
    <Window width="44rem">
      <TheSidebar />
      <Pane>Press ⌘B, the trigger, or the edge of the column.</Pane>
    </Window>
  ),
}

export const CollapsingToIcons: Story = {
  render: () => (
    <Window width="44rem">
      <TheSidebar collapsible="icon" />
      <Pane />
    </Window>
  ),
}

export const Floating: Story = {
  render: () => (
    <Window width="44rem">
      <TheSidebar variant="floating" />
      <Pane />
    </Window>
  ),
}

export const Inset: Story = {
  render: () => (
    <Window width="44rem">
      <TheSidebar variant="inset" />
      <Pane />
    </Window>
  ),
}

export const OnTheRight: Story = {
  render: () => (
    <Window width="44rem">
      <Pane />
      <TheAside />
    </Window>
  ),
}

export const TwoColumns: Story = {
  render: () => (
    <Window width="52rem">
      <TheSidebar />
      <TheRail />
      <Pane />
    </Window>
  ),
}

export const TheWholeShell: Story = {
  render: () => (
    <Window height="26rem" width="60rem">
      <TheSidebar />
      <TheRail />
      <Pane />
      <TheAside />
    </Window>
  ),
}

/* Rows --------------------------------------------------------------------- */

export const Idle: Story = {
  render: () => (
    <Column>
      <SidebarMenuItem>
        <SidebarMenuButton>
          <span>Documents</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </Column>
  ),
}

export const Active: Story = {
  render: () => (
    <Column>
      <SidebarMenuItem>
        <SidebarMenuButton active>
          <span>Documents</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </Column>
  ),
}

export const Disabled: Story = {
  render: () => (
    <Column>
      <SidebarMenuItem>
        <SidebarMenuButton disabled>
          <span>Documents</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </Column>
  ),
}

export const States: Story = {
  render: () => (
    <Column>
      <SidebarMenuItem>
        <SidebarMenuButton active>
          <span>Current page</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton>
          <span>Idle</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton disabled>
          <span>Disabled</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </Column>
  ),
}

export const Sizes: Story = {
  render: () => <SizesRow />,
}

export const Variants: Story = {
  render: () => <VariantsColumn />,
}

export const WithACount: Story = {
  render: () => (
    <Column>
      {[
        { label: 'Needs review', count: 24, active: true },
        { label: 'In progress', count: 6, active: false },
        { label: 'Done', count: 168, active: false },
      ].map(entry => (
        <SidebarMenuItem key={entry.label}>
          <SidebarMenuButton active={entry.active}>
            <span>{entry.label}</span>
          </SidebarMenuButton>
          <SidebarMenuBadge>{entry.count}</SidebarMenuBadge>
        </SidebarMenuItem>
      ))}
    </Column>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      <Row>
        {TINTS.map(tint => (
          <SidebarMenuButton
            key={tint}
            active
            tint={tint}
            style={{ width: 'auto' }}
          >
            {tint}
          </SidebarMenuButton>
        ))}
      </Row>
      <Row>
        {TINTS.map(tint => (
          <SidebarMenuButton key={tint} tint={tint} style={{ width: 'auto' }}>
            {tint}
          </SidebarMenuButton>
        ))}
      </Row>
    </Col>
  ),
}

export const Selecting: Story = {
  render: () => {
    const [pane, setPane] = useState('Documents')
    return (
      <Column>
        {PANES.map(entry => (
          <SidebarMenuItem key={entry.label}>
            <SidebarMenuButton
              active={pane === entry.label}
              icon={<Icon icon={entry.icon} size={14} />}
              onClick={() => setPane(entry.label)}
            >
              <span>{entry.label}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        ))}
      </Column>
    )
  },
}
