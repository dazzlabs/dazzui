import {
  Book20Regular,
  Document20Regular,
  Settings20Regular,
} from '@fluentui/react-icons'
import {
  Icon,
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuBadge,
  SidebarMenuButton,
  SidebarMenuItem,
  type SidebarProps,
  SidebarProvider,
  SidebarTrigger,
} from '@dazzlabs/dazzui-react'
import { useState } from 'react'

import { Col, Frame, Row, Text } from '../components/stage'
import type { Story } from '../lib/story'

const PANES = [
  { label: 'Documents', icon: Document20Regular, count: 12 },
  { label: 'Glossaries', icon: Book20Regular },
  { label: 'Settings', icon: Settings20Regular },
]

function Window({
  width,
  height,
  ...sidebar
}: SidebarProps & { width?: string; height?: string }) {
  const [pane, setPane] = useState('Documents')
  return (
    <Frame width={width} height={height}>
      <SidebarProvider>
        <Sidebar {...sidebar}>
          <SidebarHeader>
            <Text strong>Workspace</Text>
          </SidebarHeader>
          <SidebarContent>
            <SidebarGroup>
              <SidebarGroupLabel>Library</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  {PANES.map(({ label, icon, count }) => (
                    <SidebarMenuItem key={label}>
                      <SidebarMenuButton
                        icon={<Icon icon={icon} />}
                        tooltip={label}
                        active={pane === label}
                        onClick={() => setPane(label)}
                      >
                        {label}
                      </SidebarMenuButton>
                      {count && <SidebarMenuBadge>{count}</SidebarMenuBadge>}
                    </SidebarMenuItem>
                  ))}
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </SidebarContent>
        </Sidebar>
        <SidebarInset>
          <SidebarHeader>
            <Row gap="2">
              {sidebar.collapsible !== 'none' && <SidebarTrigger />}
              <Text strong>{pane}</Text>
            </Row>
          </SidebarHeader>
        </SidebarInset>
      </SidebarProvider>
    </Frame>
  )
}

const story: Story = {
  id: 'sidebar',
  name: 'Sidebar',
  summary: 'The column a window is navigated from.',
  Preview: () => <Window collapsible="none" width="100%" height="11rem" />,
  examples: [
    {
      title: 'Beside the content',
      note: 'The trigger puts it away; so does ⌘B.',
      Demo: () => <Window />,
    },
    {
      title: 'Collapsing to its icons',
      Demo: () => <Window collapsible="icon" />,
    },
    {
      title: 'Floating and inset',
      Demo: () => (
        <Col gap="4">
          <Window variant="floating" />
          <Window variant="inset" />
        </Col>
      ),
    },
  ],
}

export default story
