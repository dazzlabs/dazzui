import { defineStory } from '../src/lib/scene'
import scenes from './callout.scene.json'
import { Callout } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof Callout> = {
  title: 'Components/Callout',
  component: Callout,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Info: Story = defineStory(scenes.Info)

export const Success: Story = defineStory(scenes.Success)

export const Warning: Story = defineStory(scenes.Warning)

export const Danger: Story = defineStory(scenes.Danger)

export const Tints: Story = defineStory(scenes.Tints)

export const WithoutATitle: Story = defineStory(scenes.WithoutATitle)

export const WithAnIcon: Story = defineStory(scenes.WithAnIcon)

export const WithAnAction: Story = defineStory(scenes.WithAnAction)

export const Sizes: Story = defineStory(scenes.Sizes)

export const InAPane: Story = defineStory(scenes.InAPane)
