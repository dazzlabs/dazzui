import { Code } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof Code> = {
  title: 'Components/Code',
  component: Code,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Code"
      description="A run of code inside a sentence — a flag, a path, an identifier. It is a wash and a face, not a box: the chip corner and the half-step inset keep it separable from the prose around it without breaking the line's rhythm, which is what a border at this size always does."
    >
      <Section
        title="In a sentence"
        description="The face is the code face read straight from base.font.code — the one place in this package a base primitive is named, because the code face is not a role. The size steps down one from the prose it sits in: a monospace face at the same nominal size always reads larger, and the run would ride above the line."
      >
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
            Pass <Code>--features runtime-shaders</Code> on a Mac without the
            Metal toolchain, then run <Code>cargo test</Code> from{' '}
            <Code>packages/ui_gpui</Code>.
          </p>
        </Col>
      </Section>

      <Section
        title="What it is not"
        description="A run of code is not a KeyCap and not a Badge. A KeyCap is a key you press and is drawn as one; a Badge is an annotation about something else. This is the thing itself, quoted."
      >
        <Stack>
          <Specimen label="an identifier">
            <Code>ThemeVariables</Code>
          </Specimen>
          <Specimen label="a path">
            <Code>packages/ds/tokens/global/_base.yaml</Code>
          </Specimen>
          <Specimen label="a flag">
            <Code>--no-verify</Code>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Inline: Story = {
  render: () => <Code>pnpm parity</Code>,
}

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
        Pass <Code>--features runtime-shaders</Code> on a Mac without the Metal
        toolchain, then run <Code>cargo test</Code> from{' '}
        <Code>packages/ui_gpui</Code>.
      </p>
    </Col>
  ),
}
