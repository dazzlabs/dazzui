import { defineStory } from '../src/lib/scene'
import scenes from './button.scene.json'
import { Button } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Normal: Story = defineStory(scenes.Normal)

export const Recessed: Story = defineStory(scenes.Recessed)

export const Filled: Story = defineStory(scenes.Filled)

export const Tinted: Story = defineStory(scenes.Tinted)

export const Outlined: Story = defineStory(scenes.Outlined)

export const Plain: Story = defineStory(scenes.Plain)

export const AllVariants: Story = defineStory(scenes.AllVariants)

export const Disabled: Story = defineStory(scenes.Disabled)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Tints: Story = defineStory(scenes.Tints)

export const WithAnIcon: Story = defineStory(scenes.WithAnIcon)

export const WithAShortcut: Story = defineStory(scenes.WithAShortcut)

export const Busy: Story = defineStory(scenes.Busy)

export const FullWidth: Story = defineStory(scenes.FullWidth)

export const Destructive: Story = defineStory(scenes.Destructive)

export const InAnActionBar: Story = defineStory(scenes.InAnActionBar)
