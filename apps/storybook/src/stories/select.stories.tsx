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

function Languages() {
  return (
    <>
      <option value="zh">Chinese (Simplified)</option>
      <option value="ja">Japanese</option>
      <option value="de">German</option>
      <option value="fr">French</option>
    </>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Select"
      description="A choice out of a list too long to lay out. It stays a native `<select>`, so it gets the platform's own menu, its keyboard handling, its type-ahead, its `<optgroup>`s and its behaviour on a phone for free — only the chevron is drawn here, because the native arrow ignores padding. It reads the TextField namespace rather than owning one: looking exactly like an input is the design, and a second set of numbers could only drift from it."
    >
      <Section
        title="States"
        description="The same four a TextField has, drawn the same way. An erroring select is re-tinted onto the danger ramp rather than given a second palette."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            <Specimen label="default">
              <Select defaultValue="zh">
                <Languages />
              </Select>
            </Specimen>
            <Specimen label="disabled">
              <Select disabled defaultValue="zh">
                <Languages />
              </Select>
            </Specimen>
            <Specimen label="error">
              <Select state="error" defaultValue="zh">
                <Languages />
              </Select>
            </Specimen>
          </Stack>
        </div>
      </Section>

      <Section
        title="Sizes"
        description="The chevron keeps its distance from the trailing edge, and the text keeps its distance from the chevron, at every size. `size` here is the control profile — the native attribute of that name, which sets the number of visible rows, is displaced by it."
      >
        <div style={{ maxWidth: WIDTH }}>
          <Stack>
            {SIZES.map(size => (
              <Specimen key={size} label={size}>
                <Select defaultValue="zh" size={size}>
                  <Languages />
                </Select>
              </Specimen>
            ))}
          </Stack>
        </div>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select defaultValue="zh">
        <Languages />
      </Select>
    </div>
  ),
}

export const Disabled: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select disabled defaultValue="zh">
        <Languages />
      </Select>
    </div>
  ),
}

export const ErrorState: Story = {
  name: 'Error',
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select state="error" defaultValue="zh">
        <Languages />
      </Select>
    </div>
  ),
}

export const Placeholder: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select defaultValue="">
        <option value="" disabled>
          Choose a language…
        </option>
        <Languages />
      </Select>
    </div>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width={WIDTH}>
      {SIZES.map(size => (
        <Select key={size} defaultValue="zh" size={size}>
          <Languages />
        </Select>
      ))}
    </Col>
  ),
}

export const BesideAButton: Story = {
  render: () => (
    <Col width={WIDTH}>
      {SIZES.map(size => (
        <ActionBar key={size}>
          <Select defaultValue="zh" size={size}>
            <Languages />
          </Select>
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
        <Select key={tint} defaultValue="zh" tint={tint}>
          <Languages />
        </Select>
      ))}
    </Col>
  ),
}

export const Grouped: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Select defaultValue="zh">
        <optgroup label="Recently used">
          <option value="zh">Chinese (Simplified)</option>
          <option value="ja">Japanese</option>
        </optgroup>
        <optgroup label="All languages">
          <option value="de">German</option>
          <option value="fr">French</option>
          <option value="es">Spanish</option>
          <option value="pt">Portuguese</option>
        </optgroup>
      </Select>
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
            value={language}
            onChange={event => setLanguage(event.target.value)}
          >
            <Languages />
          </Select>
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
          <Select defaultValue="zh" size="small">
            <Languages />
          </Select>
        </PreferenceRow>
        <PreferenceRow title="Check for updates">
          <Select defaultValue="weekly" size="small">
            <option value="daily">Daily</option>
            <option value="weekly">Weekly</option>
            <option value="never">Never</option>
          </Select>
        </PreferenceRow>
      </PreferenceSection>
    </div>
  ),
}
