import { defineStory } from '../src/lib/scene'
import scenes from './toggle-group.scene.json'
import { ToggleGroup } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof ToggleGroup> = {
  title: 'Components/ToggleGroup',
  component: ToggleGroup,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Glyphs: Story = defineStory(scenes.Glyphs)

export const Words: Story = defineStory(scenes.Words)

export const Single: Story = defineStory(scenes.Single)

export const Variants: Story = defineStory(scenes.Variants)

export const Tints: Story = defineStory(scenes.Tints)

export const Disabled: Story = defineStory(scenes.Disabled)
