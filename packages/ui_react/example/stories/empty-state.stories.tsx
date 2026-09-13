import { defineStory } from '../src/lib/scene'
import scenes from './empty-state.scene.json'
import {
  Button,
  EmptyState,
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SearchField,
  Table,
  TableCell,
  TableHead,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Col, Frame } from '../src/components/stage'

const meta: Meta<typeof EmptyState> = {
  title: 'Components/EmptyState',
  component: EmptyState,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const OnItsOwn: Story = defineStory(scenes.OnItsOwn)

export const WithAnAction: Story = defineStory(scenes.WithAnAction)

export const WithTwoActions: Story = defineStory(scenes.WithTwoActions)

export const NothingYet: Story = defineStory(scenes.NothingYet)

export const NothingMatched: Story = defineStory(scenes.NothingMatched)

export const NothingToDo: Story = defineStory(scenes.NothingToDo)

export const InAList: Story = {
  render: () => {
    const [query, setQuery] = useState('placeholder')
    return (
      <Col width="34rem">
        <SearchField
          placeholder="Search terms"
          value={query}
          onValueChange={setQuery}
        />
        <Table>
          <TableHead>
            <TableCell head>Term</TableCell>
            <TableCell head>Translation</TableCell>
            <TableCell head align="end">
              Hits
            </TableCell>
          </TableHead>
          <EmptyState
            title={query ? `No terms match “${query}”` : 'No terms yet'}
            action={
              query ? (
                <Button onClick={() => setQuery('')}>Clear search</Button>
              ) : undefined
            }
          />
        </Table>
      </Col>
    )
  },
}

export const InAPane: Story = {
  render: () => (
    <Frame height="20rem" width="40rem">
      <SidebarProvider>
        <Sidebar collapsible="none">
          <SidebarContent>
            <SidebarGroup>
              <SidebarGroupContent>
                <SidebarMenu>
                  {['handbook.docx', 'contract.pdf', 'release-notes.md'].map(
                    entry => (
                      <SidebarMenuItem key={entry}>
                        <SidebarMenuButton size="small">
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
        <div
          style={{
            flex: 1,
            minWidth: 0,
            display: 'flex',
            alignItems: 'center',
          }}
        >
          <EmptyState title="Select a document to see its progress" />
        </div>
      </SidebarProvider>
    </Frame>
  ),
}
