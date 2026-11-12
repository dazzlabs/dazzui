import {
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
  Icon,
  IconButton,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
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
  { id: 'home', label: 'Home', icon: Home20Regular },
  { id: 'automations', label: 'Automations', icon: Flash20Regular },
  { id: 'library', label: 'Library', icon: Document20Regular },
]

const PROJECTS = [
  { id: 'atlas', label: 'Atlas', note: '4 sessions' },
  { id: 'website', label: 'Website', note: '1 session' },
]

const TREE: TreeNode[] = [
  {
    id: 'atlas',
    label: 'Atlas',
    icon: <Icon icon={Folder20Regular} />,
    children: [
      {
        id: 'overview',
        label: 'Overview',
        icon: <Icon icon={Document20Regular} />,
      },
      {
        id: 'changes',
        label: 'Changes',
        icon: <Icon icon={Document20Regular} />,
        meta: '4',
      },
      {
        id: 'sessions',
        label: 'Sessions',
        icon: <Icon icon={Chat20Regular} />,
        meta: 3,
        children: [
          {
            id: 'session-tokens',
            label: 'Theme tokens',
            icon: <Icon icon={Chat20Regular} />,
            meta: '2h',
          },
          {
            id: 'session-layout',
            label: 'Sidebar layout',
            icon: <Icon icon={Chat20Regular} />,
            meta: '1d',
          },
        ],
      },
    ],
  },
  {
    id: 'website',
    label: 'Website',
    icon: <Icon icon={Folder20Regular} />,
    children: [
      {
        id: 'website-pages',
        label: 'Pages',
        icon: <Icon icon={Document20Regular} />,
        meta: '1',
      },
    ],
  },
]

/** The complete column: pages, pinned projects, a tree and a footer. */
function WorkspaceSidebar({
  selected,
  onSelect,
}: {
  selected: string
  onSelect: (id: string) => void
}) {
  const { open } = useSidebar()

  return (
    <Sidebar collapsible="icon" density="comfortable">
      <SidebarHeader>
        <Row justify="between">
          <Text strong>Workspace</Text>
          <SidebarTrigger label="Collapse sidebar" size="small" />
        </Row>
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Workspace</SidebarGroupLabel>
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
              {PROJECTS.map(project => (
                <SidebarMenuItem key={project.id}>
                  <SidebarMenuButton
                    active={selected === project.id}
                    icon={<Icon icon={Folder20Regular} />}
                    tooltip={project.label}
                    onClick={() => onSelect(project.id)}
                  >
                    {project.label}
                  </SidebarMenuButton>
                  <SidebarMenuAction
                    label={`More actions for ${project.label}`}
                    showOnHover
                    onClick={() => onSelect(`${project.id}-more`)}
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
          <SidebarGroupContent>
            {open ? (
              <Tree
                aria-label="Projects"
                defaultExpanded={['atlas', 'sessions']}
                items={TREE}
                selected={selected}
                variant="sidebar"
                onSelectedChange={onSelect}
              />
            ) : (
              <SidebarMenu>
                {PROJECTS.map(project => (
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
      </SidebarFooter>
    </Sidebar>
  )
}

function CompleteWorkspace() {
  const [selected, setSelected] = useState('home')
  const title = selected.endsWith('-more')
    ? `More actions for ${selected.slice(0, -5)}`
    : (PAGES.find(page => page.id === selected)?.label ??
      PROJECTS.find(project => project.id === selected)?.label ??
      selected)

  return (
    <Frame width="100%" height="32rem">
      <SidebarProvider>
        <WorkspaceSidebar selected={selected} onSelect={setSelected} />
        <SidebarInset>
          <SidebarHeader>
            <Row gap="2">
              <SidebarTrigger />
              <Text strong>{title}</Text>
            </Row>
          </SidebarHeader>
          <Col gap="3" width="100%">
            <Text tone="secondary">
              The sidebar keeps its pages, pinned projects and project tree in
              one comfortable 32px column. Fold it to icons; the tree becomes a
              project rail and every action is still one click away.
            </Text>
          </Col>
        </SidebarInset>
      </SidebarProvider>
    </Frame>
  )
}

const extension: StoryExtension = {
  id: 'sidebar',
  examples: [
    {
      title: 'Complete workspace',
      note: 'Pages, pinned projects, a sidebar tree, hover actions and the icon rail in one column.',
      Demo: CompleteWorkspace,
    },
  ],
}

export default extension
