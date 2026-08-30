import { ActionBar, Button } from '@dazzlabs/dazzui-react'
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
            <Button variant="filled">Save</Button>
            <Button>Cancel</Button>
          </ActionBar>
          <ActionBar align="end">
            <Button>Cancel</Button>
            <Button variant="filled">Save</Button>
          </ActionBar>
          <ActionBar align="between">
            <Button tint="danger" variant="outlined">
              Delete
            </Button>
            <Button variant="filled">Save</Button>
          </ActionBar>
        </Stack>
      </Section>
    </Page>
  ),
}
