import { Aside, Badge, Divider, SectionLabel } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Aside> = {
  title: 'Components/Aside',
  component: Aside,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="Aside"
      description="The trailing information column. It carries what the current selection *is*, where the sidebar and rail carry what you can navigate to — which is why it hangs off the opposite edge and takes no rows."
    >
      <Section
        title="Beside a pane"
        description="It shares the sidebar's surface, so a window reads as content between two quieter edges rather than as three different greys."
      >
        <div
          style={{
            display: 'flex',
            height: '20rem',
            overflow: 'hidden',
            border: 'var(--spacing-px) solid var(--color-border)',
            borderRadius: 'var(--radius-large)',
            background: 'var(--color-surface)',
          }}
        >
          <div style={{ flex: 1, minWidth: 0, padding: 'var(--spacing-5)' }} />
          <Aside>
            <div>
              <SectionLabel>Source</SectionLabel>
              <p style={{ marginTop: 'var(--spacing-2)' }}>handbook.docx</p>
            </div>
            <Divider />
            <div>
              <SectionLabel>Progress</SectionLabel>
              <p style={{ marginTop: 'var(--spacing-2)' }}>
                168 of 392 segments
              </p>
            </div>
            <div>
              <SectionLabel>Provider</SectionLabel>
              <div style={{ marginTop: 'var(--spacing-2)' }}>
                <Badge size="small" tint="success">
                  Connected
                </Badge>
              </div>
            </div>
          </Aside>
        </div>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('aside')}
          labelOf={token => token.name.slice('aside-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
