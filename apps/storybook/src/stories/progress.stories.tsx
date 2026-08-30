import { Progress, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Progress> = {
  title: 'Components/Progress',
  component: Progress,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="Progress"
      description="A neutral track with a tinted fill. The track is a token because it stays neutral whatever the fill is tinted with; the fill's width is data, so it is the one thing set inline."
    >
      <Section title="Values">
        <Stack>
          {[12, 48, 100].map(value => (
            <Specimen key={value} label={`${value}%`}>
              <div style={{ flex: 1 }}>
                <Progress value={value} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Indeterminate"
        description="Omit `value` when the work has no measurable end. The bar sweeps instead of filling, and reports no value to assistive tech."
      >
        <Stack>
          <Specimen label="no value">
            <div style={{ flex: 1 }}>
              <Progress />
            </div>
          </Specimen>
        </Stack>
      </Section>

      <Section title="Sizes">
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <Specimen key={size} label={size}>
              <div style={{ flex: 1 }}>
                <Progress size={size} value={64} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section title="Tints">
        <Stack>
          {TINTS.map(tint => (
            <Specimen key={tint} label={tint}>
              <div style={{ flex: 1 }}>
                <Progress tint={tint} value={64} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('progress')}
          labelOf={token => token.name.slice('progress-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
