import {
  ActionBar,
  Button,
  FormField,
  PreferenceRow,
  PreferenceSection,
  Select,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof Select> = {
  title: 'Components/Select',
  component: Select,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

const WIDTH = '22rem'

const LANGUAGES = [
  { value: 'zh', label: 'Chinese (Simplified)' },
  { value: 'ja', label: 'Japanese' },
  { value: 'de', label: 'German' },
  { value: 'fr', label: 'French' },
]

const GROUPED = [
  { label: 'Recently used', options: LANGUAGES.slice(0, 2) },
  {
    label: 'All languages',
    options: [
      ...LANGUAGES.slice(2),
      { value: 'es', label: 'Spanish' },
      { value: 'pt', label: 'Portuguese' },
    ],
  },
]

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Select"
      description="A choice out of a list too long to lay out. The trigger reads the TextField namespace rather than owning one — looking exactly like an input is the design, and a second set of numbers could only drift from it — and the list is the same floating sheet a Menu opens, so the two match. It takes its options as data rather than as children, because the rows are drawn here rather than by the platform."
    >
      <Section
        title="States"
        description="The same four a TextField has, drawn the same way. An erroring select is re-tinted onto the danger ramp rather than given a second palette. The trigger lifts while its list is open as well as while it holds the focus: the trigger of an open popup is not idle."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            <Specimen label="default">
              <Select defaultValue="zh" options={LANGUAGES} />
            </Specimen>
            <Specimen label="disabled">
              <Select disabled defaultValue="zh" options={LANGUAGES} />
            </Specimen>
            <Specimen label="error">
              <Select defaultValue="zh" options={LANGUAGES} state="error" />
            </Specimen>
            <Specimen label="placeholder">
              <Select options={LANGUAGES} placeholder="Choose a language…" />
            </Specimen>
          </Stack>
        </div>
      </Section>

      <Section
        title="Sizes"
        description="The chevron keeps its distance from the trailing edge, and the value keeps its distance from the chevron, at every size. The list is at least as wide as the trigger it dropped from, so a long label never has to be read in a narrower column than it was chosen in."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            {SIZES.map(size => (
              <Specimen key={size} label={size}>
                <Select defaultValue="zh" options={LANGUAGES} size={size} />
              </Specimen>
            ))}
          </Stack>
        </div>
      </Section>

      <Section
        title="Groups"
        description="An entry with its own `options` becomes a heading and the rows under it, which is what `<optgroup>` used to do. The heading takes the caption voice on the row's own inset, so it reads as a divider with a name rather than as an option nobody can pick."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Select defaultValue="zh" options={GROUPED} />
        </div>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select defaultValue="zh" options={LANGUAGES} />
    </div>
  ),
}

export const Disabled: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select disabled defaultValue="zh" options={LANGUAGES} />
    </div>
  ),
}

export const ErrorState: Story = {
  name: 'Error',
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select defaultValue="zh" options={LANGUAGES} state="error" />
    </div>
  ),
}

export const Placeholder: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select options={LANGUAGES} placeholder="Choose a language…" />
    </div>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width={WIDTH}>
      {SIZES.map(size => (
        <Select key={size} defaultValue="zh" options={LANGUAGES} size={size} />
      ))}
    </Col>
  ),
}

export const BesideAButton: Story = {
  render: () => (
    <Col width={WIDTH}>
      {SIZES.map(size => (
        <ActionBar key={size}>
          <Select defaultValue="zh" options={LANGUAGES} size={size} />
          <Button size={size} variant="filled">
            Apply
          </Button>
        </ActionBar>
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col width={WIDTH}>
      {TINTS.map(tint => (
        <Select key={tint} defaultValue="zh" options={LANGUAGES} tint={tint} />
      ))}
    </Col>
  ),
}

export const Grouped: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select defaultValue="zh" options={GROUPED} />
    </div>
  ),
}

export const InAFormField: Story = {
  render: () => {
    const [language, setLanguage] = useState('zh')
    return (
      <div style={{ width: WIDTH }}>
        <FormField
          label="Target language"
          hint="Used when a document does not name one."
        >
          <Select
            options={LANGUAGES}
            value={language}
            onValueChange={setLanguage}
          />
        </FormField>
      </div>
    )
  },
}

export const InASettingsRow: Story = {
  render: () => (
    <div style={{ width: '30rem' }}>
      <PreferenceSection label="Translation">
        <PreferenceRow
          title="Target language"
          subtitle="Used when a document does not name one"
        >
          <Select defaultValue="zh" options={LANGUAGES} size="small" />
        </PreferenceRow>
        <PreferenceRow title="Check for updates">
          <Select
            defaultValue="weekly"
            options={[
              { value: 'daily', label: 'Daily' },
              { value: 'weekly', label: 'Weekly' },
              { value: 'never', label: 'Never' },
            ]}
            size="small"
          />
        </PreferenceRow>
      </PreferenceSection>
    </div>
  ),
}
