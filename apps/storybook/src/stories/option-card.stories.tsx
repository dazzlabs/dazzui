import { OptionCard, TINTS } from '@dazzlabs/dazzui'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof OptionCard> = {
  title: 'Components/OptionCard',
  component: OptionCard,
}

export default meta
type Story = StoryObj<typeof meta>

const FORMATS = [
  { id: 'docx', title: 'DOCX', description: 'Keeps styles and comments.' },
  { id: 'pdf', title: 'PDF', description: 'Fixed layout, no further editing.' },
  { id: 'md', title: 'Markdown', description: 'Plain text, images linked.' },
]

function FormatPicker() {
  const [selected, setSelected] = useState('docx')

  return (
    <div role="radiogroup" style={{ display: 'flex', gap: '0.5rem' }}>
      {FORMATS.map(format => (
        <OptionCard
          key={format.id}
          title={format.title}
          description={format.description}
          selected={selected === format.id}
          onClick={() => setSelected(format.id)}
        />
      ))}
    </div>
  )
}

export const Overview: Story = {
  render: () => (
    <Page
      title="OptionCard"
      description="The shape a small set of mutually exclusive choices takes when each needs a line of explanation and a radio row would not carry it. Wrap a set in a radiogroup."
    >
      <Section
        title="Picking one"
        description="Selection is the tinted recipe with the outlined recipe's border on top: the fill says which one is chosen, and the border makes it survive a glance across its neighbours."
      >
        <FormatPicker />
      </Section>

      <Section
        title="Tints"
        description="The selected treatment across every ramp."
      >
        <div style={{ display: 'flex', gap: '0.5rem', flexWrap: 'wrap' }}>
          {TINTS.map(tint => (
            <OptionCard key={tint} title={tint} selected tint={tint} />
          ))}
        </div>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('option-card')}
          labelOf={token => token.name.slice('option-card-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
