import { defineStory } from '../src/lib/scene'
import scenes from './icon-button.scene.json'
import {
  Add20Regular as Plus,
  Delete20Regular as Trash,
} from '@fluentui/react-icons'
import {
  ActionBar,
  Icon,
  IconButton,
  SearchField,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

const meta: Meta<typeof IconButton> = {
  title: 'Components/IconButton',
  component: IconButton,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Plain: Story = defineStory(scenes.Plain)

export const Filled: Story = defineStory(scenes.Filled)

export const Variants: Story = defineStory(scenes.Variants)

export const Disabled: Story = defineStory(scenes.Disabled)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Tints: Story = defineStory(scenes.Tints)

export const AToolbar: Story = defineStory(scenes.AToolbar)

export const InAListHeader: Story = {
  render: () => {
    const [query, setQuery] = useState('')
    return (
      <div style={{ width: '28rem' }}>
        <ActionBar>
          <div style={{ flex: 1 }}>
            <SearchField
              placeholder="Search terms"
              value={query}
              onValueChange={setQuery}
            />
          </div>
          <IconButton label="New term" tint="primary" variant="filled">
            <Icon icon={Plus} size={16} />
          </IconButton>
          <IconButton label="Delete selected" tint="danger">
            <Icon icon={Trash} size={16} />
          </IconButton>
        </ActionBar>
      </div>
    )
  },
}
