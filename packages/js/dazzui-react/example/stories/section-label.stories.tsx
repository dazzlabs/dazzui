import { defineStory } from '../src/lib/scene'
import scenes from './section-label.scene.json'
import {
  Card,
  Divider,
  SectionLabel,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import { Frame } from '../src/components/stage'

const meta: Meta<typeof SectionLabel> = {
  title: 'Components/SectionLabel',
  component: SectionLabel,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Default: Story = defineStory(scenes.Default)

/** The quieter tone, for a column that already has a heading above it. */
export const Faint: Story = defineStory(scenes.Faint)

export const Tints: Story = defineStory(scenes.Tints)

export const InAPanel: Story = defineStory(scenes.InAPanel)

export const InAColumn: Story = {
  render: () => (
    <Frame height="20rem" width="44rem">
      <SidebarProvider>
        <Sidebar collapsible="none">
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
              <SidebarGroupLabel>Library</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  <SidebarMenuItem>
                    <SidebarMenuButton>
                      <span>Translation memory</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton>
                      <span>Archive</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </SidebarContent>
          <SidebarFooter>
            <Card>
              <SectionLabel>Today</SectionLabel>
              <span style={{ fontSize: 'var(--body-small-font-size)' }}>
                148 segments translated
              </span>
            </Card>
          </SidebarFooter>
        </Sidebar>
        <div style={{ flex: 1, minWidth: 0 }} />
        <Sidebar collapsible="none" side="right">
          <SidebarContent
            style={{ gap: 'var(--spacing-5)', padding: 'var(--spacing-4)' }}
          >
            <div>
              <SectionLabel>Source</SectionLabel>
              <p style={{ margin: 'var(--spacing-2) 0 0' }}>handbook.docx</p>
            </div>
            <Divider />
            <div>
              <SectionLabel>Progress</SectionLabel>
              <p style={{ margin: 'var(--spacing-2) 0 0' }}>
                168 of 392 segments
              </p>
            </div>
          </SidebarContent>
        </Sidebar>
      </SidebarProvider>
    </Frame>
  ),
}
