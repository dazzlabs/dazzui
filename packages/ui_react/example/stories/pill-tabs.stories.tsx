import { defineStory } from '../src/lib/scene'
import scenes from './pill-tabs.scene.json'
import { PillTabs } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof PillTabs> = {
  title: 'Components/PillTabs',
  component: PillTabs,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Default: Story = defineStory(scenes.Default)

export const WithCounts: Story = defineStory(scenes.WithCounts)

export const Disabled: Story = defineStory(scenes.Disabled)

export const Tints: Story = defineStory(scenes.Tints)

export const Wrapping: Story = defineStory(scenes.Wrapping)

export const InAPaneHeader: Story = defineStory(scenes.InAPaneHeader)
