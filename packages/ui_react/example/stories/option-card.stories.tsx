import { Badge, OptionCard, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { type ReactNode, useState } from 'react'

import { Grid, Page, Section } from '../src/components/gallery'

const meta: Meta<typeof OptionCard> = {
  title: 'Components/OptionCard',
  component: OptionCard,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const FORMATS = [
  { id: 'docx', title: 'DOCX', description: 'Keeps styles and comments.' },
  { id: 'pdf', title: 'PDF', description: 'Fixed layout, no further editing.' },
  { id: 'md', title: 'Markdown', description: 'Plain text, images linked.' },
]

function Group({ children }: { children: ReactNode }) {
  return (
    <div
      role="radiogroup"
      style={{ display: 'flex', gap: '0.5rem', flexWrap: 'wrap' }}
    >
      {children}
    </div>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="OptionCard"
      description="The shape a small set of mutually exclusive choices takes when each needs a line of explanation and a radio row would not carry it. Wrap a set in a radiogroup. The description is optional, and a set that does not need one is really a SegmentedControl."
    >
      <Section
        title="Picking one"
        description="Selection is the tinted recipe with the outlined recipe's border on top: the fill says which one is chosen, and the border makes it survive a glance across its neighbours. An option that cannot be picked stays in the row rather than being dropped, so the set keeps its shape and the reason can be written into the description."
      >
        <Group>
          <OptionCard
            title="DOCX"
            description="Keeps styles and comments."
            selected
          />
          <OptionCard title="PDF" description="Fixed layout, no editing." />
          <OptionCard
            disabled
            title="InDesign"
            description="Needs a paid plan."
          />
        </Group>
      </Section>

      <Section
        title="Tints"
        description="The selected treatment across every ramp. A set normally keeps one tint: the colour says “chosen”, and if it varied per card it would be saying something about the card instead."
      >
        <Group>
          {TINTS.map(tint => (
            <OptionCard key={tint} title={tint} selected tint={tint} />
          ))}
        </Group>
      </Section>
    </Page>
  ),
}

export const Selected: Story = {
  render: () => (
    <OptionCard
      title="DOCX"
      description="Keeps styles and comments."
      selected
    />
  ),
}

export const Unselected: Story = {
  render: () => (
    <OptionCard title="PDF" description="Fixed layout, no further editing." />
  ),
}

export const AGroup: Story = {
  render: () => {
    const [selected, setSelected] = useState('docx')
    return (
      <Group>
        {FORMATS.map(format => (
          <OptionCard
            key={format.id}
            title={format.title}
            description={format.description}
            selected={selected === format.id}
            onClick={() => setSelected(format.id)}
          />
        ))}
      </Group>
    )
  },
}

export const WithoutADescription: Story = {
  render: () => {
    const [quality, setQuality] = useState('balanced')
    return (
      <Group>
        {['Fast', 'Balanced', 'Careful'].map(label => (
          <OptionCard
            key={label}
            title={label}
            selected={quality === label.toLowerCase()}
            onClick={() => setQuality(label.toLowerCase())}
          />
        ))}
      </Group>
    )
  },
}

export const Disabled: Story = {
  render: () => (
    <Group>
      <OptionCard
        title="DOCX"
        description="Keeps styles and comments."
        selected
      />
      <OptionCard title="PDF" description="Fixed layout, no editing." />
      <OptionCard disabled title="InDesign" description="Needs a paid plan." />
    </Group>
  ),
}

export const Tints: Story = {
  render: () => (
    <Group>
      {TINTS.map(tint => (
        <OptionCard key={tint} title={tint} selected tint={tint} />
      ))}
    </Group>
  ),
}

export const InAGrid: Story = {
  render: () => {
    const [plan, setPlan] = useState('pro')
    const plans = [
      {
        id: 'free',
        title: 'Free',
        description: '20,000 characters a month, one provider.',
      },
      {
        id: 'pro',
        title: 'Pro',
        description: 'Unlimited characters, every provider, shared glossaries.',
      },
      {
        id: 'team',
        title: 'Team',
        description: 'Everything in Pro, plus review workflows and seats.',
      },
    ]

    return (
      <div role="radiogroup" style={{ width: '44rem' }}>
        <Grid min="13rem">
          {plans.map(entry => (
            <OptionCard
              key={entry.id}
              title={
                <span
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: 'var(--spacing-2)',
                  }}
                >
                  {entry.title}
                  {entry.id === 'pro' ? (
                    <Badge size="small" tint="primary">
                      Current
                    </Badge>
                  ) : null}
                </span>
              }
              description={entry.description}
              selected={plan === entry.id}
              onClick={() => setPlan(entry.id)}
            />
          ))}
        </Grid>
      </div>
    )
  },
}
