import { Combobox, FormField, Select } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof Combobox> = {
  title: 'Components/Combobox',
  component: Combobox,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

const TIMEZONES = [
  { value: 'utc', label: 'UTC' },
  { value: 'lisbon', label: 'Europe/Lisbon' },
  { value: 'london', label: 'Europe/London' },
  { value: 'berlin', label: 'Europe/Berlin' },
  { value: 'shanghai', label: 'Asia/Shanghai' },
  { value: 'tokyo', label: 'Asia/Tokyo' },
  { value: 'sydney', label: 'Australia/Sydney' },
  { value: 'new_york', label: 'America/New_York' },
  { value: 'sao_paulo', label: 'America/Sao_Paulo' },
]

const GROUPED = [
  {
    label: 'Europe',
    options: [
      { value: 'lisbon', label: 'Europe/Lisbon' },
      { value: 'london', label: 'Europe/London' },
      { value: 'berlin', label: 'Europe/Berlin' },
    ],
  },
  {
    label: 'Asia',
    options: [
      { value: 'shanghai', label: 'Asia/Shanghai' },
      { value: 'tokyo', label: 'Asia/Tokyo' },
    ],
  },
  {
    label: 'Americas',
    options: [
      { value: 'new_york', label: 'America/New_York' },
      { value: 'sao_paulo', label: 'America/Sao_Paulo' },
    ],
  },
]

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Combobox"
      description="A choice out of a list too long to scroll: the reader types to narrow it, and picks one of what is left. It is a Select with a query in front of it, and that is the only question worth asking when choosing between them — a list of six is a Select, a list of six hundred is this. Both end in one of the options; neither lets a reader enter something that is not on the list, which is what separates them from an Autocomplete."
    >
      <Section
        title="Sizes"
        description="The box is the TextField's — text-field.css names this group alongside `.dz-text-field` — and the list is the shared popup sheet, so a combobox in a form is the same box as the field beside it and the same panel a Menu opens."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <div style={{ width: '14rem' }}>
                <Combobox options={TIMEZONES} size={size} defaultValue="utc" />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Grouped"
        description="Grouped or flat, never both: Base UI reads the whole list one way or the other, so a mixed array would silently lose either the groups or the loose options. The filtering runs inside the groups and drops the ones left empty."
      >
        <div style={{ width: '16rem' }}>
          <Combobox options={GROUPED} placeholder="Search time zones…" />
        </div>
      </Section>

      <Section
        title="Against a Select"
        description="Side by side the rule is easy: if the reader can find the option by looking, it is a Select. If they would have to scroll to find it, give them a query."
      >
        <Col width="16rem">
          <Select
            options={[
              { value: 'private', label: 'Private' },
              { value: 'public', label: 'Public' },
            ]}
            defaultValue="private"
          />
          <Combobox options={TIMEZONES} placeholder="Search time zones…" />
        </Col>
      </Section>

      <Section
        title="States"
        description="Erroring re-tints the control to danger, exactly as a TextField does. The empty line is prose in a panel of rows, so it takes the row's inset and the faint ink rather than looking like a row that cannot be picked."
      >
        <Stack>
          <Specimen label="error">
            <div style={{ width: '14rem' }}>
              <Combobox options={TIMEZONES} state="error" />
            </div>
          </Specimen>
          <Specimen label="disabled">
            <div style={{ width: '14rem' }}>
              <Combobox options={TIMEZONES} disabled defaultValue="utc" />
            </div>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Bare: Story = {
  render: () => (
    <div style={{ width: '16rem' }}>
      <Combobox options={TIMEZONES} placeholder="Search time zones…" />
    </div>
  ),
}

export const Grouped: Story = {
  render: () => (
    <div style={{ width: '16rem' }}>
      <Combobox options={GROUPED} placeholder="Search time zones…" />
    </div>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width="16rem">
      {SIZES.map(size => (
        <Combobox
          key={size}
          size={size}
          options={TIMEZONES}
          defaultValue="utc"
        />
      ))}
    </Col>
  ),
}

export const States: Story = {
  render: () => (
    <Col width="16rem">
      <Combobox options={TIMEZONES} defaultValue="utc" />
      <Combobox options={TIMEZONES} state="error" />
      <Combobox options={TIMEZONES} disabled defaultValue="utc" />
    </Col>
  ),
}

export const InAForm: Story = {
  render: () => {
    const [zone, setZone] = useState<string | null>('london')
    return (
      <div style={{ width: '18rem' }}>
        <FormField
          label="Time zone"
          hint={zone === null ? 'Pick one' : 'Used for every schedule'}
          state={zone === null ? 'error' : 'default'}
        >
          <Combobox
            options={GROUPED}
            value={zone}
            onValueChange={setZone}
            state={zone === null ? 'error' : 'default'}
          />
        </FormField>
      </div>
    )
  },
}
