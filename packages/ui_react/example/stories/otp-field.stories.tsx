import { defineStory } from '../src/lib/scene'
import scenes from './otp-field.scene.json'
import { OtpField } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

const meta: Meta<typeof OtpField> = {
  title: 'Components/OtpField',
  component: OtpField,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Bare: Story = defineStory(scenes.Bare)

export const Filled: Story = defineStory(scenes.Filled)

export const Grouped: Story = defineStory(scenes.Grouped)

export const Sizes: Story = defineStory(scenes.Sizes)

export const Masked: Story = defineStory(scenes.Masked)

export const Disabled: Story = defineStory(scenes.Disabled)

export const InAForm: Story = defineStory(scenes.InAForm)
