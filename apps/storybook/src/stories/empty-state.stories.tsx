import { Button, Card, EmptyState } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof EmptyState> = {
  title: 'Components/EmptyState',
  component: EmptyState,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="EmptyState"
      description="What a pane shows before it has content: one muted line and at most one action. The surrounding chrome already names the pane, so there is no heading, no explainer copy, and no illustration."
    >
      <Section title="On its own">
        <Card variant="sunken">
          <EmptyState title="No documents yet" />
        </Card>
      </Section>

      <Section
        title="With an action"
        description="The action row takes the control gap rather than the block gap, because it holds controls."
      >
        <Card variant="sunken">
          <EmptyState
            title="No glossary terms match this filter"
            action={<Button>Clear filter</Button>}
          />
        </Card>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('empty-state')}
          labelOf={token => token.name.slice('empty-state-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
