import { defineStory } from '../src/lib/scene'
import scenes from './key-cap.scene.json'
import { Button, KeyCap, Menu, SearchField } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

const meta: Meta<typeof KeyCap> = {
  title: 'Components/KeyCap',
  component: KeyCap,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Hint: Story = defineStory(scenes.Hint)

export const Strong: Story = defineStory(scenes.Strong)

export const Key: Story = defineStory(scenes.Key)

export const AllVariants: Story = defineStory(scenes.AllVariants)

export const Sizes: Story = defineStory(scenes.Sizes)

export const AChord: Story = defineStory(scenes.AChord)

export const AJoinedChord: Story = defineStory(scenes.AJoinedChord)

export const WordKeys: Story = defineStory(scenes.WordKeys)

export const InProse: Story = {
  render: () => (
    <div style={{ maxWidth: '26rem' }}>
      <p style={{ margin: 0 }}>
        Open the command palette with <KeyCap size="small">⌘K</KeyCap>, then
        type a document name. Press <KeyCap size="small">Esc</KeyCap> to close.
      </p>
    </div>
  ),
}

export const AsASearchHint: Story = {
  render: () => {
    const [query, setQuery] = useState('')
    return (
      <div style={{ width: '18rem' }}>
        <SearchField
          hint={<KeyCap size="small">⌘F</KeyCap>}
          placeholder="Search terms"
          value={query}
          onValueChange={setQuery}
        />
      </div>
    )
  },
}

export const InAMenu: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={({ open }) => (
        <Button tint="neutral" variant={open ? 'filled' : 'normal'}>
          Options
        </Button>
      )}
      items={[
        { label: 'Duplicate', shortcut: <KeyCap size="small">⌘D</KeyCap> },
        { label: 'Rename', shortcut: <KeyCap size="small">↩</KeyCap> },
        { label: 'Export…', shortcut: <KeyCap size="small">⌘E</KeyCap> },
      ]}
    />
  ),
}

/* The settings page lists a shortcut as the keys themselves, not as a hint. */
export const InASettingsRow: Story = defineStory(scenes.InASettingsRow)
