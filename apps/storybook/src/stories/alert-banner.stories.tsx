import { AlertBanner, Spinner, TINTS } from '@dazzlabs/dazzui'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Stack, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof AlertBanner> = {
  title: 'Components/AlertBanner',
  component: AlertBanner,
}

export default meta
type Story = StoryObj<typeof meta>

const COPY: Record<string, { title: string; description: string }> = {
  primary: {
    title: 'Translation memory is syncing',
    description:
      'Segments you edit now are queued and upload when it finishes.',
  },
  neutral: {
    title: 'Export is about 3.6 MB',
    description: 'It will be saved to your Downloads folder.',
  },
  info: {
    title: 'Two glossaries apply to this document',
    description: 'Terms defined in both resolve to whichever ranks higher.',
  },
  success: {
    title: 'All 15 pages translated',
    description: 'Layout and formulas were preserved.',
  },
  warning: {
    title: 'Tables keep their original layout',
    description: 'Text inside them is translated, but column widths are not.',
  },
  danger: {
    title: 'The provider rejected the last request',
    description: 'Check the API key in Settings, then run the job again.',
  },
}

export const Overview: Story = {
  render: () => (
    <Page
      title="AlertBanner"
      description="A tinted notice. The token graph gives it two type roles — a title and a description — so it is built for both, and drops the title when a one-line notice is all that is needed."
    >
      <Section
        title="Tints"
        description="A banner always wears the tinted treatment; anything louder would be a dialog, not a banner. So the ramp is the only color axis."
      >
        <Stack>
          {TINTS.map(tint => (
            <AlertBanner
              key={tint}
              tint={tint}
              title={COPY[tint]?.title}
              action={<Spinner size="small" tint={tint} />}
            >
              {COPY[tint]?.description}
            </AlertBanner>
          ))}
        </Stack>
      </Section>

      <Section
        title="Without a title"
        description="The description carries it alone — bt's one-line callout."
      >
        <Stack>
          <AlertBanner tint="neutral">
            Testing the connection — 1.4s elapsed.
          </AlertBanner>
        </Stack>
      </Section>

      <Section title="Sizes">
        <Stack>
          {(['small', 'medium', 'large'] as const).map(size => (
            <AlertBanner
              key={size}
              size={size}
              tint="info"
              title={`Size ${size}`}
            >
              Padding, radius, gap, and both type roles step together.
            </AlertBanner>
          ))}
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('alert-banner')}
          labelOf={token => token.name.slice('alert-banner-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
