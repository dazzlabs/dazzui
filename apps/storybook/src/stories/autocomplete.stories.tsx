import { Autocomplete, Combobox, FormField } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof Autocomplete> = {
  title: 'Components/Autocomplete',
  component: Autocomplete,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

const BRANCHES = [
  'main',
  'release/4.2',
  'release/4.1',
  'fix/token-drift',
  'fix/popup-layer',
  'feat/slider',
  'feat/drawer',
]

const TAGS = ['design-system', 'tokens', 'flutter', 'gpui', 'swiftui', 'parity']

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Autocomplete"
      description="A text field that suggests as you type. The difference from a Combobox is what the value is: a combobox ends in one of its options and nothing else; an autocomplete's value is whatever was typed, and the list is a shortcut to a likely answer rather than the set of allowed ones. A branch name, a search query, a tag that may not exist yet."
    >
      <Section
        title="Sizes"
        description="It is a TextField wearing the shared popup sheet, and it is literally that: the same box class, the same panel, the same rows. There is no chevron and no clear — nothing here is opened or chosen, and the field is emptied the way any text field is."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <div style={{ width: '14rem' }}>
                <Autocomplete
                  items={BRANCHES}
                  size={size}
                  mono
                  placeholder="Branch"
                />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Against a Combobox"
        description="Ask what happens when the reader types something that is not on the list. If that is an answer, this is the component. If it is a mistake, it is a Combobox."
      >
        <Col width="16rem">
          <Autocomplete
            items={BRANCHES}
            mono
            placeholder="Branch or new name"
          />
          <Combobox
            options={BRANCHES.map(branch => ({
              value: branch,
              label: branch,
            }))}
            placeholder="Existing branch"
          />
        </Col>
      </Section>

      <Section
        title="States"
        description="Erroring re-tints to danger like every other text control, so the border, the wash and the value's ink move together."
      >
        <Stack>
          <Specimen label="error">
            <div style={{ width: '14rem' }}>
              <Autocomplete items={TAGS} state="error" defaultValue="tok" />
            </div>
          </Specimen>
          <Specimen label="disabled">
            <div style={{ width: '14rem' }}>
              <Autocomplete items={TAGS} disabled defaultValue="tokens" />
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
      <Autocomplete items={TAGS} placeholder="Add a tag" />
    </div>
  ),
}

export const Mono: Story = {
  render: () => (
    <div style={{ width: '16rem' }}>
      <Autocomplete items={BRANCHES} mono placeholder="Branch" />
    </div>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width="16rem">
      {SIZES.map(size => (
        <Autocomplete
          key={size}
          size={size}
          items={TAGS}
          placeholder="Add a tag"
        />
      ))}
    </Col>
  ),
}

export const States: Story = {
  render: () => (
    <Col width="16rem">
      <Autocomplete items={TAGS} defaultValue="tokens" />
      <Autocomplete items={TAGS} state="error" defaultValue="tok" />
      <Autocomplete items={TAGS} disabled defaultValue="tokens" />
    </Col>
  ),
}

export const InAForm: Story = {
  render: () => {
    const [branch, setBranch] = useState('feat/')
    const known = BRANCHES.includes(branch)

    return (
      <div style={{ width: '20rem' }}>
        <FormField
          label="Branch"
          hint={
            known ? 'Checks out the existing branch' : 'Creates a new branch'
          }
        >
          <Autocomplete
            mono
            items={BRANCHES}
            value={branch}
            onValueChange={setBranch}
          />
        </FormField>
      </div>
    )
  },
}
