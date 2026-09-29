import {
  Button,
  PreferenceGroup,
  PreferenceRow,
  Preferences,
  PreferenceSection,
  Select,
  Switch,
} from '@dazzlabs/dazzui-react'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const LANGUAGES = [
  { value: 'en', label: 'English' },
  { value: 'zh', label: 'Chinese' },
]

function General() {
  return (
    <PreferenceSection label="General">
      <PreferenceRow title="Language" subtitle="What the interface is set in">
        <Select
          size="small"
          options={LANGUAGES}
          defaultValue="en"
          aria-label="Language"
        />
      </PreferenceRow>
      <PreferenceRow title="Launch at login">
        <Switch defaultChecked aria-label="Launch at login" />
      </PreferenceRow>
      <PreferenceRow title="Check for updates">
        <Button size="small">Check now</Button>
      </PreferenceRow>
    </PreferenceSection>
  )
}

const story: Story = {
  id: 'preferences',
  name: 'Preferences',
  summary: 'Settings, a row to each, in the sections they belong to.',
  Preview: () => (
    <Col width="20rem">
      <Preferences>
        <PreferenceSection label="General">
          <PreferenceRow title="Launch at login">
            <Switch defaultChecked aria-label="Launch at login" />
          </PreferenceRow>
          <PreferenceRow title="Send crash reports">
            <Switch aria-label="Send crash reports" />
          </PreferenceRow>
        </PreferenceSection>
      </Preferences>
    </Col>
  ),
  examples: [
    {
      title: 'A section',
      Demo: () => (
        <Col width="32rem">
          <Preferences>
            <General />
          </Preferences>
        </Col>
      ),
    },
    {
      title: 'A group of sections',
      Demo: () => (
        <Col width="32rem">
          <Preferences>
            <PreferenceGroup
              title="Application"
              description="How the app behaves on this computer."
            >
              <General />
              <PreferenceSection
                label="Privacy"
                footer="Reports never include your files."
              >
                <PreferenceRow title="Send crash reports">
                  <Switch aria-label="Send crash reports" />
                </PreferenceRow>
              </PreferenceSection>
            </PreferenceGroup>
          </Preferences>
        </Col>
      ),
    },
    {
      title: 'Rows that open something',
      Demo: () => (
        <Col width="32rem">
          <Preferences>
            <PreferenceSection label="Account">
              <PreferenceRow
                title="Profile"
                subtitle="Name and picture"
                onOpen={() => {}}
              />
              <PreferenceRow title="Billing" onOpen={() => {}} />
            </PreferenceSection>
          </Preferences>
        </Col>
      ),
    },
  ],
}

export default story
