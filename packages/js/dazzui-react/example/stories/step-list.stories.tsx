import { defineStory } from '../src/lib/scene'
import scenes from './step-list.scene.json'
import { StepList } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof StepList> = {
  title: 'Components/StepList',
  component: StepList,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const InFlight: Story = defineStory(scenes.InFlight)

export const Statuses: Story = defineStory(scenes.Statuses)

export const Finished: Story = defineStory(scenes.Finished)

export const WithoutDetail: Story = defineStory(scenes.WithoutDetail)

export const TimedDetail: Story = defineStory(scenes.TimedDetail)

export const Tints: Story = defineStory(scenes.Tints)

export const Advancing: Story = defineStory(scenes.Advancing)
