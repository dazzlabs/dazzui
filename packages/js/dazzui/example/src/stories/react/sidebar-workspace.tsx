import {
  Add20Regular,
  Chat20Regular,
  Document20Regular,
  Flash20Regular,
  Folder20Regular,
  Home20Regular,
  MoreHorizontal20Regular,
  Phone20Regular,
  QuestionCircle20Regular,
  Settings20Regular,
} from '@fluentui/react-icons'
import {
  Avatar,
  Badge,
  Button,
  Card,
  Divider,
  Icon,
  IconButton,
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
  SidebarProvider,
  SidebarTrigger,
  Tree,
  type TreeNode,
  useSidebar,
} from '@dazzlabs/dazzui'
import { useState } from 'react'

import { Col, Frame, Row, Text } from '../../components/stage'
import type { StoryExtension } from '../../lib/story'

const PAGES = [
  { id: 'new-chat', label: 'New Chat', icon: Home20Regular },
  { id: 'automation', label: 'Automation', icon: Flash20Regular },
  { id: 'library', label: 'Library', icon: Document20Regular },
]

const PINNED = [
  { id: 'atlas', label: 'Atlas', count: 4 },
  { id: 'website', label: 'Website', count: 1 },
]

const PROJECTS: TreeNode[] = [
  {
    id: 'ledger',
    label: 'Ledger',
    icon: <Icon icon={Folder20Regular} />,
    meta: (
      <Row gap="2">
        <Row gap="1">
          <Icon icon={Folder20Regular} />
          <Text>2</Text>
        </Row>
        <Text tint="warning">2 need you</Text>
      </Row>
    ),
    children: [
      {
        id: 'ledger-overview',
        label: 'Overview',
        icon: <Icon icon={Document20Regular} />,
      },
      {
        id: 'ledger-changes',
        label: 'Changes',
        icon: <Icon icon={Document20Regular} />,
        meta: '8',
      },
      {
        id: 'ledger-sessions',
        label: 'Sessions',
        icon: <Icon icon={Chat20Regular} />,
        meta: 4,
        children: [
          {
            id: 'ledger-session-1',
            label: 'Reconcile import',
            icon: <Text tint="success">●</Text>,
            meta: 'now',
          },
          {
            id: 'ledger-session-2',
            label: 'Ledger permissions',
            icon: <Text tint="warning">●</Text>,
            meta: '2h',
          },
          {
            id: 'ledger-session-3',
            label: 'Export to CSV',
            icon: <Text tint="primary">●</Text>,
            meta: '1d',
          },
        ],
      },
    ],
  },
  {
    id: 'docs',
    label: 'Docs',
    icon: <Icon icon={Folder20Regular} />,
    meta: <Text tone="secondary">1 session</Text>,
    children: [
      {
        id: 'docs-pages',
        label: 'Pages',
        icon: <Icon icon={Document20Regular} />,
        meta: '3',
      },
      {
        id: 'docs-sessions',
        label: 'Sessions',
        icon: <Icon icon={Chat20Regular} />,
        meta: 1,
        children: [
          {
            id: 'docs-session-1',
            label: 'Rewrite the guide',
            icon: <Text tint="primary">●</Text>,
            meta: '3d',
          },
        ],
      },
    ],
  },
]

const LABELS: Record<string, string> = {
  'new-chat': 'New Chat',
  automation: 'Automation',
  library: 'Library',
  atlas: 'Atlas',
  website: 'Website',
  ledger: 'Ledger',
  docs: 'Docs',
  'atlas-overview': 'Atlas · Overview',
  'atlas-changes': 'Atlas · Changes',
  'atlas-session-1': 'Deny approvals past their timeout',
  'ledger-overview': 'Ledger · Overview',
  'ledger-changes': 'Ledger · Changes',
  'ledger-session-1': 'Reconcile import',
}

function WorkspaceSidebar({
  selected,
  onSelect,
}: {
  selected: string
  onSelect: (id: string) => void
}) {
  const { open } = useSidebar()

  return (
    <Sidebar collapsible="icon" density="comfortable" width="19.5rem">
      <SidebarHeader>
        {open ? (
          <Row justify="between">
            <Row gap="2">
              <Avatar
                name="DazzUI"
                shape="square"
                size="small"
                tint="primary"
              />
              <Text strong>DazzUI</Text>
            </Row>
            <SidebarTrigger label="Collapse sidebar" size="small" />
          </Row>
        ) : (
          <Col align="center">
            <Avatar name="DazzUI" shape="square" size="small" tint="primary" />
          </Col>
        )}
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              {PAGES.map(page => (
                <SidebarMenuItem key={page.id}>
                  <SidebarMenuButton
                    active={selected === page.id}
                    icon={<Icon icon={page.icon} />}
                    tooltip={page.label}
                    onClick={() => onSelect(page.id)}
                  >
                    {page.label}
                  </SidebarMenuButton>
                  {page.id === 'library' ? (
                    <SidebarMenuBadge>12</SidebarMenuBadge>
                  ) : null}
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Pinned</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {PINNED.map(project => (
                <SidebarMenuItem key={project.id}>
                  <SidebarMenuButton
                    active={selected === project.id}
                    icon={<Icon icon={Folder20Regular} />}
                    tooltip={project.label}
                    onClick={() => onSelect(project.id)}
                  >
                    {project.label}
                  </SidebarMenuButton>
                  <SidebarMenuBadge>{project.count}</SidebarMenuBadge>
                  <SidebarMenuAction
                    label={`More actions for ${project.label}`}
                    showOnHover
                    onClick={() => onSelect(project.id)}
                  >
                    <Icon icon={MoreHorizontal20Regular} />
                  </SidebarMenuAction>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Projects</SidebarGroupLabel>
          <SidebarGroupAction label="Add project" size="tiny">
            <Icon icon={Add20Regular} />
          </SidebarGroupAction>
          <SidebarGroupContent>
            {open ? (
              <Tree
                aria-label="Projects"
                defaultExpanded={['ledger', 'ledger-sessions']}
                items={PROJECTS}
                selected={selected}
                variant="sidebar"
                onSelectedChange={onSelect}
              />
            ) : (
              <SidebarMenu>
                {[
                  { id: 'ledger', label: 'Ledger' },
                  { id: 'docs', label: 'Docs' },
                ].map(project => (
                  <SidebarMenuItem key={project.id}>
                    <SidebarMenuButton
                      active={selected === project.id}
                      icon={<Icon icon={Folder20Regular} />}
                      tooltip={project.label}
                      onClick={() => onSelect(project.id)}
                    />
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            )}
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter>
        {open ? (
          <Row gap="1">
            <IconButton label="Settings" size="small" variant="plain">
              <Icon icon={Settings20Regular} />
            </IconButton>
            <IconButton label="Help" size="small" variant="plain">
              <Icon icon={QuestionCircle20Regular} />
            </IconButton>
            <IconButton label="Connect phone" size="small" variant="plain">
              <Icon icon={Phone20Regular} />
            </IconButton>
          </Row>
        ) : (
          <Col gap="1" align="center">
            <IconButton label="Settings" size="small" variant="plain">
              <Icon icon={Settings20Regular} />
            </IconButton>
            <IconButton label="Help" size="small" variant="plain">
              <Icon icon={QuestionCircle20Regular} />
            </IconButton>
            <IconButton label="Connect phone" size="small" variant="plain">
              <Icon icon={Phone20Regular} />
            </IconButton>
          </Col>
        )}
      </SidebarFooter>
    </Sidebar>
  )
}

function Inset({ selected }: { selected: string }) {
  const label = LABELS[selected] ?? selected
  return (
    <SidebarInset>
      <SidebarHeader>
        <Row justify="between">
          <Row gap="2">
            <SidebarTrigger />
            <Text strong>{label}</Text>
            <Badge size="small" tint="success" variant="tinted">
              Open
            </Badge>
          </Row>
          <Row gap="1">
            <Button size="small" variant="plain">
              Pull
            </Button>
            <Button size="small" variant="filled">
              Push
            </Button>
          </Row>
        </Row>
      </SidebarHeader>

      <Col gap="4" width="100%">
        <Card size="medium" variant="outlined">
          <Row justify="between">
            <Col gap="1">
              <Text strong>Deny approvals past their timeout</Text>
              <Text tone="secondary">
                #12 · session/approval-timeout → main · 2 commits
              </Text>
            </Col>
            <Badge size="small" tint="success" variant="tinted">
              Open
            </Badge>
          </Row>
          <Text tone="secondary">
            Each approval records when it was requested, and approval_timeout in
            the daemon config denies it once the timeout passes. Covered by
            tests/approval_timeout.rs.
          </Text>
          <Divider />
          <Col gap="2">
            <CheckRow state="success" label="fmt" detail="12s" />
            <CheckRow
              state="success"
              label="test (macos-latest)"
              detail="3m 41s"
            />
            <CheckRow state="danger" label="clippy" detail="1m 07s" />
          </Col>
        </Card>
      </Col>
    </SidebarInset>
  )
}

function CheckRow({
  state,
  label,
  detail,
}: {
  state: 'success' | 'danger'
  label: string
  detail: string
}) {
  return (
    <Row justify="between">
      <Row gap="2">
        <Badge size="small" tint={state} variant="tinted">
          {state === 'success' ? '✓' : '×'}
        </Badge>
        <Text>{label}</Text>
      </Row>
      <Text tone="secondary">{detail}</Text>
    </Row>
  )
}

function CompleteWorkspace() {
  const [selected, setSelected] = useState('ledger-overview')

  return (
    <Frame width="100%" height="36rem">
      <SidebarProvider>
        <WorkspaceSidebar selected={selected} onSelect={setSelected} />
        <Inset selected={selected} />
      </SidebarProvider>
    </Frame>
  )
}

const extension: StoryExtension = {
  id: 'sidebar',
  examples: [
    {
      title: 'Complete workspace',
      note: 'Pages, pinned projects, project trees, sessions, hover actions, a project view and the icon rail.',
      Demo: CompleteWorkspace,
    },
  ],
}

export default extension
