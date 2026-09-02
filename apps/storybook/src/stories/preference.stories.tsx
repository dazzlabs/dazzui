import { Globe20Regular as Globe } from '@fluentui/react-icons'
import {
  Badge,
  Button,
  Divider,
  Icon,
  TextField,
  PreferenceGroup,
  PreferenceRow,
  PreferenceSection,
  SegmentedControl,
  Select,
  ShortcutRecorder,
  Switch,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof PreferenceSection> = {
  title: 'Components/Preference',
  component: PreferenceSection,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const WIDTH = '30rem'

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Preference"
      description="The settings column. The spacing is the whole design: rows sit close together, a heading sits further from the section above it than from its own rows, and a group title stands further still — so a heading always reads as belonging to what follows it."
    >
      <Section
        title="Rows"
        description="A subtitle is for what the title cannot say in three words; an icon is for rows that name a thing rather than a setting; `onOpen` makes the row a button and grows the hover wash that says so, while the rest stay inert. The trailing slot takes any control — everything sharing the control height lands on one edge without the row being told what it holds."
      >
        <div style={{ maxWidth: WIDTH }}>
          <PreferenceSection label="General">
            <PreferenceRow title="Title only">
              <Switch aria-label="Title only" checked />
            </PreferenceRow>
            <PreferenceRow
              title="With a subtitle"
              subtitle="The second line, for what three words cannot carry"
            >
              <Switch aria-label="With a subtitle" checked={false} />
            </PreferenceRow>
            <PreferenceRow
              icon={<Icon icon={Globe} size={20} />}
              title="With an icon"
              subtitle="For a row that names a thing"
            >
              <Badge tint="success">Connected</Badge>
            </PreferenceRow>
            <PreferenceRow title="With a select">
              <Select defaultValue="weekly" size="small">
                <option value="daily">Daily</option>
                <option value="weekly">Weekly</option>
              </Select>
            </PreferenceRow>
            <PreferenceRow
              title="Opens something"
              subtitle="Hover it — the wash says it is a button"
              onOpen={() => {}}
            />
          </PreferenceSection>
        </div>
      </Section>

      <Section
        title="Sections and groups"
        description="A section can carry a label, an action on the heading line, and a muted footer under its rows — each optional. The first run of a page usually goes without a label, because the page title has already named it. A group is a titled run of sections: sections stay one size everywhere, and the level above them is a title that outranks them rather than a section shrunk to fit underneath one, so a heading means the same thing wherever it is met."
      >
        <div style={{ maxWidth: WIDTH }}>
          <PreferenceGroup
            title="Translation"
            description="How segments are sent, and what comes back."
            action={<Button size="small">Reset</Button>}
          >
            <PreferenceSection label="Providers">
              <PreferenceRow title="Primary" subtitle="DeepL">
                <Badge tint="success">Connected</Badge>
              </PreferenceRow>
              <PreferenceRow title="Fallback" subtitle="Google Translate">
                <Badge tint="neutral">Idle</Badge>
              </PreferenceRow>
            </PreferenceSection>
            <Divider />
            <PreferenceSection
              label="Shortcuts"
              footer="A shortcut already taken by another app will not fire here."
            >
              <PreferenceRow title="Translate selection">
                <ShortcutRecorder value="⌥⇧2" onValueChange={() => {}} />
              </PreferenceRow>
              <PreferenceRow title="Capture and translate">
                <ShortcutRecorder value="" onValueChange={() => {}} />
              </PreferenceRow>
            </PreferenceSection>
          </PreferenceGroup>
        </div>
      </Section>
    </Page>
  ),
}

export const ASection: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <PreferenceSection label="General">
        <PreferenceRow
          title="Launch at login"
          subtitle="Starts minimised in the menu bar"
        >
          <Badge tint="success">On</Badge>
        </PreferenceRow>
        <PreferenceRow title="Check for updates">
          <Badge tint="neutral" variant="outlined">
            Weekly
          </Badge>
        </PreferenceRow>
        <PreferenceRow
          title="Storage location"
          subtitle="~/Library/Application Support"
          onOpen={() => {}}
        />
      </PreferenceSection>
    </div>
  ),
}

export const RowShapes: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <PreferenceSection>
        <PreferenceRow title="Title only">
          <Switch aria-label="Title only" checked />
        </PreferenceRow>
        <PreferenceRow
          title="With a subtitle"
          subtitle="The second line, for what three words cannot carry"
        >
          <Switch aria-label="With a subtitle" checked={false} />
        </PreferenceRow>
        <PreferenceRow
          icon={<Icon icon={Globe} size={20} />}
          title="With an icon"
          subtitle="For a row that names a thing"
        >
          <Badge tint="success">Connected</Badge>
        </PreferenceRow>
        <PreferenceRow
          title="Opens something"
          subtitle="Hover it — the wash says it is a button"
          onOpen={() => {}}
        />
      </PreferenceSection>
    </div>
  ),
}

export const Controls: Story = {
  render: () => {
    const [launch, setLaunch] = useState(true)
    const [sounds, setSounds] = useState(false)
    const [appearance, setAppearance] = useState('system')

    return (
      <div style={{ width: WIDTH }}>
        <PreferenceSection label="General">
          <PreferenceRow
            title="Launch at login"
            subtitle="Starts minimised in the menu bar"
          >
            <Switch
              aria-label="Launch at login"
              checked={launch}
              onCheckedChange={setLaunch}
            />
          </PreferenceRow>
          <PreferenceRow title="Play a sound when a job finishes">
            <Switch
              aria-label="Play a sound"
              checked={sounds}
              onCheckedChange={setSounds}
            />
          </PreferenceRow>
          <PreferenceRow title="Appearance">
            <SegmentedControl
              aria-label="Appearance"
              items={[
                { value: 'light', label: 'Light' },
                { value: 'dark', label: 'Dark' },
                { value: 'system', label: 'System' },
              ]}
              size="small"
              value={appearance}
              onValueChange={setAppearance}
            />
          </PreferenceRow>
          <PreferenceRow title="Check for updates">
            <Select defaultValue="weekly" size="small">
              <option value="daily">Daily</option>
              <option value="weekly">Weekly</option>
              <option value="never">Never</option>
            </Select>
          </PreferenceRow>
          <PreferenceRow title="Default target language">
            <TextField defaultValue="Chinese" size="small" />
          </PreferenceRow>
        </PreferenceSection>
      </div>
    )
  },
}

export const SectionHeadings: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <PreferenceSection
        label="Shortcuts"
        action={<Button size="small">Reset all</Button>}
        footer="A shortcut already taken by another app will not fire here."
      >
        <PreferenceRow title="Translate selection">
          <ShortcutRecorder value="⌥⇧2" onValueChange={() => {}} />
        </PreferenceRow>
        <PreferenceRow title="Capture and translate">
          <ShortcutRecorder value="" onValueChange={() => {}} />
        </PreferenceRow>
      </PreferenceSection>
    </div>
  ),
}

export const Unlabelled: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <PreferenceSection>
        <PreferenceRow title="Launch at login">
          <Switch aria-label="Launch at login" checked />
        </PreferenceRow>
        <PreferenceRow title="Show in the menu bar">
          <Switch aria-label="Show in the menu bar" checked />
        </PreferenceRow>
      </PreferenceSection>
    </div>
  ),
}

export const AGroup: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <PreferenceGroup
        title="Translation"
        description="How segments are sent, and what comes back."
        action={<Button size="small">Reset</Button>}
      >
        <PreferenceSection label="Providers">
          <PreferenceRow title="Primary" subtitle="DeepL">
            <Badge tint="success">Connected</Badge>
          </PreferenceRow>
          <PreferenceRow title="Fallback" subtitle="Google Translate">
            <Badge tint="neutral">Idle</Badge>
          </PreferenceRow>
        </PreferenceSection>
        <Divider />
        <PreferenceSection
          label="Shortcuts"
          footer="A shortcut already taken by another app will not fire here."
        >
          <PreferenceRow title="Translate selection">
            <ShortcutRecorder value="⌥⇧2" onValueChange={() => {}} />
          </PreferenceRow>
          <PreferenceRow title="Capture and translate">
            <ShortcutRecorder value="" onValueChange={() => {}} />
          </PreferenceRow>
        </PreferenceSection>
      </PreferenceGroup>
    </div>
  ),
}

export const ASettingsPage: Story = {
  render: () => {
    const [launch, setLaunch] = useState(true)
    const [beta, setBeta] = useState(false)
    const [translate, setTranslate] = useState('⌥⇧2')
    const [capture, setCapture] = useState('')

    return (
      <Col gap="var(--spacing-8)" width={WIDTH}>
        <PreferenceGroup
          title="General"
          description="How the app behaves on this device."
        >
          <PreferenceSection>
            <PreferenceRow
              title="Launch at login"
              subtitle="Starts minimised in the menu bar"
            >
              <Switch
                aria-label="Launch at login"
                checked={launch}
                onCheckedChange={setLaunch}
              />
            </PreferenceRow>
            <PreferenceRow
              title="Join the beta channel"
              subtitle="Updates arrive earlier and break more often"
            >
              <Switch
                aria-label="Join the beta channel"
                checked={beta}
                onCheckedChange={setBeta}
              />
            </PreferenceRow>
            <PreferenceRow
              title="Storage location"
              subtitle="~/Library/Application Support"
              onOpen={() => {}}
            />
          </PreferenceSection>
        </PreferenceGroup>

        <PreferenceGroup
          title="Translation"
          description="How segments are sent, and what comes back."
          action={<Button size="small">Reset</Button>}
        >
          <PreferenceSection label="Providers">
            <PreferenceRow
              icon={<Icon icon={Globe} size={20} />}
              title="DeepL"
              subtitle="Primary"
            >
              <Badge tint="success">Connected</Badge>
            </PreferenceRow>
            <PreferenceRow
              icon={<Icon icon={Globe} size={20} />}
              title="Google Translate"
              subtitle="Fallback"
            >
              <Badge tint="neutral">Idle</Badge>
            </PreferenceRow>
          </PreferenceSection>
          <Divider />
          <PreferenceSection
            label="Shortcuts"
            footer="A shortcut already taken by another app will not fire here."
          >
            <PreferenceRow title="Translate selection">
              <ShortcutRecorder
                value={translate}
                onValueChange={setTranslate}
              />
            </PreferenceRow>
            <PreferenceRow title="Capture and translate">
              <ShortcutRecorder value={capture} onValueChange={setCapture} />
            </PreferenceRow>
          </PreferenceSection>
        </PreferenceGroup>
      </Col>
    )
  },
}
