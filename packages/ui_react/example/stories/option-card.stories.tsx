import { defineStory } from '../src/lib/scene'
import scenes from './option-card.scene.json'
import { OptionCard } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof OptionCard> = {
  title: 'Components/OptionCard',
  component: OptionCard,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Selected: Story = defineStory(scenes.Selected)

export const Unselected: Story = defineStory(scenes.Unselected)

export const AGroup: Story = defineStory(scenes.AGroup)

export const WithoutADescription: Story = defineStory(
  scenes.WithoutADescription
)

export const Disabled: Story = defineStory(scenes.Disabled)

export const Tints: Story = defineStory(scenes.Tints)

export const InAGrid: Story = defineStory(scenes.InAGrid)
