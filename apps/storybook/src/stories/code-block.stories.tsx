import { Button, CodeBlock } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof CodeBlock> = {
  title: 'Components/CodeBlock',
  component: CodeBlock,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SAMPLE = `export function Badge({ size = 'medium', tint = 'primary' }: BadgeProps) {
  return <span data-size={size} data-tint={tint} className="dz-badge" />
}`

const SHELL = `pnpm install
pnpm tokens
pnpm parity`

const WIDE = `const themes = ['studio-dark', 'bright-light', 'bright-dark', 'frost-light', 'frost-dark', 'graphite-light']`

export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="CodeBlock"
      description="A block of code, read rather than edited. It takes the card's materials rather than the popup's — this sits in the page, not over it — and its head is drawn only where there is something to put in it. A bar holding one label is furniture that says less than the space it takes."
    >
      <Section
        title="The gutter is a column"
        description="Line numbers live in their own column rather than in the same run as the code, so a selection that crosses ten lines copies ten lines of code and not ten line numbers — which is the single thing a reader does with one of these, and the thing a numbered <pre> gets wrong. The column is sticky, so it stays put while the code scrolls sideways under it."
      >
        <Col width="30rem">
          <CodeBlock
            code={SAMPLE}
            lineNumbers
            title="src/components/badge.tsx"
          />
        </Col>
      </Section>

      <Section
        title="No highlighter"
        description="`language` names the grammar at the trailing edge of the head and does nothing else. This package ships no highlighter, because a grammar per language is a dependency a design system has no opinion about — an app that wants colour renders its own runs instead."
      >
        <Col width="30rem">
          <CodeBlock code={SHELL} copyable language="bash" />
        </Col>
      </Section>

      <Section
        title="Long lines"
        description="Scrolling is the default: a wrapped line of code has lost the one thing indentation was carrying. `wrap` is there for a block in a narrow column where a horizontal scroller would be missed entirely."
      >
        <Col gap="var(--spacing-3)" width="30rem">
          <CodeBlock code={WIDE} language="ts" />
          <CodeBlock code={WIDE} language="ts" wrap />
        </Col>
      </Section>
    </Page>
  ),
}

export const Plain: Story = {
  render: () => (
    <Col width="30rem">
      <CodeBlock code={SHELL} />
    </Col>
  ),
}

export const WithHead: Story = {
  render: () => (
    <Col width="30rem">
      <CodeBlock
        code={SAMPLE}
        language="tsx"
        title="src/components/badge.tsx"
      />
    </Col>
  ),
}

export const LineNumbers: Story = {
  render: () => (
    <Col width="30rem">
      <CodeBlock code={SAMPLE} lineNumbers />
    </Col>
  ),
}

export const Copyable: Story = {
  render: () => (
    <Col width="30rem">
      <CodeBlock code={SHELL} copyable language="bash" />
    </Col>
  ),
}

export const Wrapped: Story = {
  render: () => (
    <Col width="24rem">
      <CodeBlock code={WIDE} language="ts" wrap />
    </Col>
  ),
}

export const WithActions: Story = {
  render: () => (
    <Col width="30rem">
      <CodeBlock
        actions={
          <Button size="tiny" variant="plain">
            Run
          </Button>
        }
        code={SHELL}
        copyable
        title="install.sh"
      />
    </Col>
  ),
}
