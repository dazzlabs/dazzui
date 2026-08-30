import { Select } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Select> = {
  title: 'Components/Select',
  component: Select,
}

export default meta
type Story = StoryObj<typeof meta>

function Languages() {
  return (
    <>
      <option value="zh">Chinese (Simplified)</option>
      <option value="ja">Japanese</option>
      <option value="de">German</option>
    </>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="Select"
      description="A choice out of a list too long to lay out. It stays a native `<select>`, so it gets the platform's own menu, its keyboard handling and its behaviour on a phone for free — only the chevron is drawn here, because the native arrow ignores padding."
    >
      <Section title="States">
        <div style={{ maxWidth: '22rem' }}>
          <Stack>
            <Select defaultValue="zh">
              <Languages />
            </Select>
            <Select disabled defaultValue="zh">
              <Languages />
            </Select>
            <Select state="error" defaultValue="zh">
              <Languages />
            </Select>
          </Stack>
        </div>
      </Section>

      <Section
        title="Sizes"
        description="The chevron keeps its distance from the trailing edge, and the text keeps its distance from the chevron, at every size."
      >
        <div style={{ maxWidth: '22rem' }}>
          <Stack>
            {(['small', 'medium', 'large'] as const).map(size => (
              <Specimen key={size} label={size}>
                <Select defaultValue="zh" size={size}>
                  <Languages />
                </Select>
              </Specimen>
            ))}
          </Stack>
        </div>
      </Section>

      <Section
        title="Tokens"
        description="A Select reads the Input namespace rather than owning one: looking exactly like an input is the design, and a second set of numbers could only drift from it."
      >
        <TokenTable
          tokens={componentTokens('input')}
          labelOf={token => token.name.slice('input-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
