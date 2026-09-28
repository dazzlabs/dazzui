import { defineStory } from '../src/lib/scene'
import scenes from './link.scene.json'
import { Link } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { Col } from '../src/components/stage'

const meta: Meta<typeof Link> = {
  title: 'Components/Link',
  component: Link,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = defineStory(scenes.Overview, {
  parameters: { layout: 'fullscreen' },
})

export const Underlines: Story = defineStory(scenes.Underlines)

export const External: Story = defineStory(scenes.External)

export const Tints: Story = defineStory(scenes.Tints)

export const Disabled: Story = defineStory(scenes.Disabled)

export const InProse: Story = {
  render: () => (
    <Col width="26rem">
      <p
        style={{
          margin: 0,
          color: 'var(--color-content)',
          fontFamily: 'var(--body-small-font-family)',
          fontSize: 'var(--body-small-font-size)',
          lineHeight: 'var(--body-small-line-height)',
        }}
      >
        Behaviour comes from <Link href="#">Base UI</Link>, and the drawing is
        ours. Where a control has to reach a form, read{' '}
        <Link href="#">the field conventions</Link> first — or open the{' '}
        <Link external href="https://base-ui.com">
          upstream docs
        </Link>{' '}
        for the parts this package does not wrap.
      </p>
    </Col>
  ),
}
