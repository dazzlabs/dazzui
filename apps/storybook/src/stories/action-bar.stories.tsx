import { ActionBar, Badge } from '@dazzlabs/dazzui'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Stack } from '../components/gallery'

const meta: Meta<typeof ActionBar> = {
  title: 'Components/ActionBar',
  component: ActionBar,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="ActionBar"
      description="A row of controls kept on one rhythm. It owns no colour and no box — only the gap — so it takes the shared control gap rather than a token of its own."
    >
      <Section
        title="Alignment"
        description="`end` is a dialog footer's usual shape; `between` splits a secondary action away from the confirming pair."
      >
        <Stack>
          <ActionBar>
            <Badge variant="filled">Save</Badge>
            <Badge variant="outlined" tint="neutral">
              Cancel
            </Badge>
          </ActionBar>
          <ActionBar align="end">
            <Badge variant="outlined" tint="neutral">
              Cancel
            </Badge>
            <Badge variant="filled">Save</Badge>
          </ActionBar>
          <ActionBar align="between">
            <Badge variant="outlined" tint="danger">
              Delete
            </Badge>
            <Badge variant="filled">Save</Badge>
          </ActionBar>
        </Stack>
      </Section>
    </Page>
  ),
}
