import { defineStory } from '../src/lib/scene'
import scenes from './divider.scene.json'
import { Divider } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof Divider> = {
  title: 'Components/Divider',
  component: Divider,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Horizontal: Story = defineStory(scenes.Horizontal)

export const Muted: Story = defineStory(scenes.Muted)

export const Tones: Story = defineStory(scenes.Tones)

export const Vertical: Story = defineStory(scenes.Vertical)

export const InAToolbar: Story = defineStory(scenes.InAToolbar)

export const InACard: Story = defineStory(scenes.InACard)

export const BetweenRows: Story = defineStory(scenes.BetweenRows)
