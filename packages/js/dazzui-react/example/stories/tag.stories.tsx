import { defineStory } from '../src/lib/scene'
import scenes from './tag.scene.json'
import { Tag } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof Tag> = {
  title: 'Components/Tag',
  component: Tag,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Selectable: Story = defineStory(scenes.Selectable)

export const Removable: Story = defineStory(scenes.Removable)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Variants: Story = defineStory(scenes.Variants)

export const Tints: Story = defineStory(scenes.Tints)

export const Disabled: Story = defineStory(scenes.Disabled)

export const AFilterBar: Story = defineStory(scenes.AFilterBar)
