import { defineStory } from '../src/lib/scene'
import scenes from './card.scene.json'
import { Card } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof Card> = {
  title: 'Components/Card',
  component: Card,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Raised: Story = defineStory(scenes.Raised)

export const Sunken: Story = defineStory(scenes.Sunken)

export const Outlined: Story = defineStory(scenes.Outlined)

export const Tinted: Story = defineStory(scenes.Tinted)

export const Variants: Story = defineStory(scenes.Variants)

export const Tints: Story = defineStory(scenes.Tints)

export const Sizes: Story = defineStory(scenes.Sizes)

export const WithAnAction: Story = defineStory(scenes.WithAnAction)

export const AsAGrid: Story = defineStory(scenes.AsAGrid)

export const AsAWell: Story = defineStory(scenes.AsAWell)

export const Nested: Story = defineStory(scenes.Nested)
