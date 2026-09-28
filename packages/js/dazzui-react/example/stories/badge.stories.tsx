import { defineStory } from '../src/lib/scene'
import scenes from './badge.scene.json'
import {
  Badge,
  SidebarMenu,
  SidebarMenuBadge,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof Badge> = {
  title: 'Components/Badge',
  component: Badge,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Tinted: Story = defineStory(scenes.Tinted)

export const Filled: Story = defineStory(scenes.Filled)

export const Outlined: Story = defineStory(scenes.Outlined)

export const Raised: Story = defineStory(scenes.Raised)

/* The three recipes on the paper, then the paper chip on the card it is
   made for — `raised` on the bare canvas would be a label with no box. */
export const AllVariants: Story = defineStory(scenes.AllVariants)

export const States: Story = defineStory(scenes.States)

export const Tints: Story = defineStory(scenes.Tints)

export const Sizes: Story = defineStory(scenes.Sizes)

export const WithAGlyph: Story = defineStory(scenes.WithAGlyph)

export const Counts: Story = defineStory(scenes.Counts)

export const InATable: Story = defineStory(scenes.InATable)

export const InAPreferenceRow: Story = defineStory(scenes.InAPreferenceRow)

export const InANavRow: Story = {
  render: () => (
    <div style={{ width: '13rem' }}>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton active>
            <span>Needs review</span>
          </SidebarMenuButton>
          <SidebarMenuBadge>24</SidebarMenuBadge>
        </SidebarMenuItem>
        <SidebarMenuItem>
          <SidebarMenuButton>
            <span>Done</span>
          </SidebarMenuButton>
          <SidebarMenuBadge>168</SidebarMenuBadge>
        </SidebarMenuItem>
      </SidebarMenu>
    </div>
  ),
}
