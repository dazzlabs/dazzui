import { defineStory } from '../src/lib/scene'
import scenes from './search-field.scene.json'
import { SearchField } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof SearchField> = {
  title: 'Components/SearchField',
  component: SearchField,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Empty: Story = defineStory(scenes.Empty)

export const WithAQuery: Story = defineStory(scenes.WithAQuery)

export const WithAHint: Story = defineStory(scenes.WithAHint)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Tints: Story = defineStory(scenes.Tints)

export const Dismissable: Story = defineStory(scenes.Dismissable)

export const FilteringAList: Story = defineStory(scenes.FilteringAList)

export const InAPaneHeader: Story = defineStory(scenes.InAPaneHeader)
