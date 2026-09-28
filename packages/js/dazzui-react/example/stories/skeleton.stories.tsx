import { defineStory } from '../src/lib/scene'
import scenes from './skeleton.scene.json'
import { Skeleton } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof Skeleton> = {
  title: 'Components/Skeleton',
  component: Skeleton,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Shapes: Story = defineStory(scenes.Shapes)

export const Text: Story = defineStory(scenes.Text)

export const Lines: Story = defineStory(scenes.Lines)

export const AListRow: Story = defineStory(scenes.AListRow)

export const InACard: Story = defineStory(scenes.InACard)
