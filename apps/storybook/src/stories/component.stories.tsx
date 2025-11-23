import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, TokenTable } from '../components/gallery'
import { byPrefix, type Token } from '../lib/tokens'

const meta: Meta = {
  title: 'Tokens/Component',
}

export default meta
type Story = StoryObj<typeof meta>

const SIZE_ORDER = ['tiny', 'small', 'medium', 'large', 'big']

/** Shared tokens first, then one group per control size. */
function rank(prefix: string, token: Token): number {
  return SIZE_ORDER.indexOf(token.name.slice(prefix.length).split('-')[0])
}

function tokensFor(component: string): Token[] {
  const prefix = `${component}-`
  return byPrefix(prefix).sort(
    (a, b) => rank(prefix, a) - rank(prefix, b) || a.name.localeCompare(b.name)
  )
}

function ComponentPage({
  component,
  description,
}: {
  component: string
  description: string
}) {
  return (
    <Page title={component} description={description}>
      <Section title="Tokens">
        <TokenTable
          tokens={tokensFor(component)}
          labelOf={token => token.name.slice(component.length + 1)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  )
}

export const AlertBanner: Story = {
  render: () => (
    <ComponentPage
      component="alert-banner"
      description="Padding, radius, and the two type styles a banner composes: a title and a description."
    />
  ),
}

export const Badge: Story = {
  render: () => (
    <ComponentPage
      component="badge"
      description="A badge borrows the padding of the control size one step below it, so it reads as a label rather than a control."
    />
  ),
}

export const Button: Story = {
  render: () => (
    <ComponentPage
      component="button"
      description="Height, padding, radius, and label type style per button size."
    />
  ),
}

export const Card: Story = {
  render: () => (
    <ComponentPage
      component="card"
      description="Reserved for card tokens; nothing is defined yet."
    />
  ),
}

export const Checkbox: Story = {
  render: () => (
    <ComponentPage
      component="checkbox"
      description="The box itself is square, so a single size token drives both axes."
    />
  ),
}

export const Kbd: Story = {
  render: () => (
    <ComponentPage
      component="kbd"
      description="Keyboard hints carry their own surface, content, and border colors on top of the shared sizing scale."
    />
  ),
}
