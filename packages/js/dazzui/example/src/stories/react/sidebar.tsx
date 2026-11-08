import {
  Icon,
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarGroupAction,
  SidebarHeader,
  SidebarInset,
  SidebarInput,
  SidebarRail,
  SidebarMenu,
  SidebarMenuBadge,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuAction,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
  SidebarProvider,
  SidebarTrigger,
} from '@dazzlabs/dazzui'
import {
  Add20Regular,
  MoreHorizontal20Regular,
  Document20Regular,
} from '@fluentui/react-icons'
import { useState } from 'react'
import { Col, Frame, Row, Text } from '../../components/stage'
import type { StoryExtension } from '../../lib/story'

function NestedRows() {
  const [page, setPage] = useState('Inbox')
  return (
    <Frame width="100%" height="18rem">
      <SidebarProvider>
        <Sidebar collapsible="icon">
          <SidebarContent>
            <SidebarGroup>
              <SidebarGroupLabel>Library</SidebarGroupLabel>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton active>Documents</SidebarMenuButton>
                  <SidebarMenuBadge>3</SidebarMenuBadge>
                  <SidebarMenuSub>
                    <SidebarMenuSubItem>
                      <SidebarMenuSubButton
                        active={page === 'Inbox'}
                        onClick={() => setPage('Inbox')}
                      >
                        Inbox
                      </SidebarMenuSubButton>
                    </SidebarMenuSubItem>
                    <SidebarMenuSubItem>
                      <SidebarMenuSubButton
                        size="medium"
                        variant="tinted"
                        tint="success"
                        active={page === 'Archive'}
                        onClick={() => setPage('Archive')}
                      >
                        Archive
                      </SidebarMenuSubButton>
                    </SidebarMenuSubItem>
                    <SidebarMenuSubItem>
                      <SidebarMenuSubButton disabled>
                        Deleted
                      </SidebarMenuSubButton>
                    </SidebarMenuSubItem>
                  </SidebarMenuSub>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroup>
          </SidebarContent>
        </Sidebar>
        <SidebarInset>
          <SidebarHeader>
            <Row gap="2">
              <SidebarTrigger />
              <Text strong>{page}</Text>
            </Row>
          </SidebarHeader>
        </SidebarInset>
      </SidebarProvider>
    </Frame>
  )
}

function RowActions() {
  const [status, setStatus] = useState('Choose an action')
  return (
    <Frame width="100%" height="14rem">
      <SidebarProvider>
        <Sidebar collapsible="icon">
          <SidebarContent>
            <SidebarGroup>
              <SidebarGroupLabel>Library</SidebarGroupLabel>
              <SidebarGroupAction
                label="Add document"
                onClick={() => setStatus('Added document')}
              >
                <Icon icon={Add20Regular} />
              </SidebarGroupAction>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    icon={<Icon icon={Document20Regular} />}
                    onClick={() => setStatus('Documents')}
                  >
                    Documents
                  </SidebarMenuButton>
                  <SidebarMenuAction
                    label="More document actions"
                    showOnHover
                    onClick={() => setStatus('Document actions')}
                  >
                    <Icon icon={MoreHorizontal20Regular} />
                  </SidebarMenuAction>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    size="large"
                    icon={<Icon icon={Document20Regular} />}
                  >
                    Shared library
                  </SidebarMenuButton>
                  <SidebarMenuAction
                    label="Shared library actions"
                    size="small"
                    variant="tinted"
                    tint="success"
                    onClick={() => setStatus('Shared library actions')}
                  >
                    <Icon icon={MoreHorizontal20Regular} />
                  </SidebarMenuAction>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroup>
          </SidebarContent>
        </Sidebar>
        <SidebarInset>
          <SidebarHeader>
            <Row gap="2">
              <SidebarTrigger />
              <Text strong>{status}</Text>
            </Row>
          </SidebarHeader>
        </SidebarInset>
      </SidebarProvider>
    </Frame>
  )
}

function InputAndRail() {
  const [query, setQuery] = useState('')
  return (
    <Frame width="100%" height="14rem">
      <SidebarProvider>
        <Sidebar collapsible="icon">
          <SidebarHeader>
            <SidebarInput
              aria-label="Filter library"
              placeholder="Filter library"
              value={query}
              onValueChange={setQuery}
            />
          </SidebarHeader>
          <SidebarContent>
            <SidebarGroup>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton icon={<Icon icon={Document20Regular} />}>
                    Documents
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroup>
          </SidebarContent>
          <SidebarRail />
        </Sidebar>
        <SidebarInset>
          <SidebarHeader>
            <Row gap="2">
              <SidebarTrigger />
              <Text>{query}</Text>
            </Row>
          </SidebarHeader>
        </SidebarInset>
      </SidebarProvider>
    </Frame>
  )
}
function SidebarFields() {
  return (
    <Col width="20rem" gap="3">
      <SidebarInput
        aria-label="Small default"
        defaultValue="Small by default"
      />
      <SidebarInput
        aria-label="Medium override"
        size="medium"
        defaultValue="Medium override"
      />
      <SidebarInput
        aria-label="Error field"
        state="error"
        defaultValue="Needs attention"
      />
      <SidebarInput
        aria-label="Read only field"
        readOnly
        defaultValue="Read only"
        mono
      />
      <SidebarInput
        aria-label="Disabled field"
        disabled
        defaultValue="Disabled"
      />
      <SidebarInput
        aria-label="Multiline field"
        multiline
        rows={3}
        defaultValue="A longer note"
      />
    </Col>
  )
}

const extension: StoryExtension = {
  id: 'sidebar',
  platforms: ['gpui'],
  examples: [
    {
      title: 'Nested navigation',
      note: 'Nested rows share the row recipes and disappear when the column collapses to icons.',
      Demo: NestedRows,
    },
    {
      title: 'Group and row actions',
      note: 'Row actions appear on hover or focus. Collapsing the column removes its actions.',
      Demo: RowActions,
    },
    {
      title: 'Input and edge toggle',
      note: 'The input defaults to small. The edge strip toggles the column without joining Tab traversal.',
      Demo: InputAndRail,
    },
    { title: 'Sidebar input states', Demo: SidebarFields },
  ],
}
export default extension
