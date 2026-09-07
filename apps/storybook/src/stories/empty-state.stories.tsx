import {
  ActionBar,
  Button,
  Card,
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
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Grid, Page, Section } from '../components/gallery'
import { Col, Frame } from '../components/stage'

const meta: Meta<typeof EmptyState> = {
  title: 'Components/EmptyState',
  component: EmptyState,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="EmptyState"
      description="What a pane shows before it has content: one muted line and at most one action. The surrounding chrome already names the pane, so there is no heading, no explainer copy, and no illustration. It centres in whatever it is given, so it needs no height of its own."
    >
      <Section
        title="With and without an action"
        description="The action row takes the control gap rather than the block gap, because it holds controls. One action is the norm; two is the most it should ever carry, and only when neither is obviously the way out."
      >
        <Grid min="18rem">
          <Card variant="sunken">
            <EmptyState title="No documents yet" />
          </Card>
          <Card variant="sunken">
            <EmptyState
              title="No glossary terms match this filter"
              action={<Button>Clear filter</Button>}
            />
          </Card>
        </Grid>
      </Section>

      <Section
        title="Wording"
        description="An empty pane and an empty result are different states and read differently: one says nothing has been added, the other says a filter matched nothing. Copy is the only thing carrying that difference, so it has to be exact."
      >
        <Grid min="16rem">
          <Card variant="sunken">
            <EmptyState
              title="No glossaries yet"
              action={<Button variant="filled">New glossary</Button>}
            />
          </Card>
          <Card variant="sunken">
            <EmptyState
              title="No terms match “placeholder”"
              action={<Button>Clear search</Button>}
            />
          </Card>
          <Card variant="sunken">
            <EmptyState title="Everything is translated" />
          </Card>
        </Grid>
      </Section>
    </Page>
  ),
}

export const OnItsOwn: Story = {
  render: () => (
    <Card variant="sunken" style={{ width: '24rem' }}>
      <EmptyState title="No documents yet" />
    </Card>
  ),
}

export const WithAnAction: Story = {
  render: () => (
    <Card variant="sunken" style={{ width: '24rem' }}>
      <EmptyState
        title="No glossary terms match this filter"
        action={<Button>Clear filter</Button>}
      />
    </Card>
  ),
}

export const WithTwoActions: Story = {
  render: () => (
    <Card variant="sunken" style={{ width: '24rem' }}>
      <EmptyState
        title="No documents yet"
        action={
          <ActionBar>
            <Button variant="filled">Import a file</Button>
            <Button variant="plain">Paste text</Button>
          </ActionBar>
        }
      />
    </Card>
  ),
}

export const NothingYet: Story = {
  render: () => (
    <Card variant="sunken" style={{ width: '24rem' }}>
      <EmptyState
        title="No glossaries yet"
        action={<Button variant="filled">New glossary</Button>}
      />
    </Card>
  ),
}

export const NothingMatched: Story = {
  render: () => (
    <Card variant="sunken" style={{ width: '24rem' }}>
      <EmptyState
        title="No terms match “placeholder”"
        action={<Button>Clear search</Button>}
      />
    </Card>
  ),
}

export const NothingToDo: Story = {
  render: () => (
    <Card variant="sunken" style={{ width: '24rem' }}>
      <EmptyState title="Everything is translated" />
    </Card>
  ),
}

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
