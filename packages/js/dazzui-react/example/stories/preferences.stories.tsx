import { Globe20Regular as Globe } from '@fluentui/react-icons'
import {
  Badge,
  Button,
  Divider,
  Icon,
  TextField,
  PreferenceGroup,
  PreferenceRow,
  Preferences,
  PreferenceSection,
  SegmentedControl,
  Select,
  ShortcutRecorder,
  Switch,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, TokenTable } from '../src/components/gallery'
import { componentTokens } from '../src/lib/tokens'

/* A field fills its container, which is right in a form and wrong on a row's
   trailing edge: there it takes a measure of its own. */
const FIELD = { width: 'calc(var(--spacing-20) * 2 + var(--spacing-4))' }

const meta: Meta<typeof Preferences> = {
  title: 'Components/Preferences',
  component: Preferences,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Preferences"
      description="The settings column: a `Preferences` root, and the groups, sections and rows that run down it. The spacing is the whole design: rows sit close together, a heading sits further from the section above it than from its own rows, a group title stands further still, and one group stands furthest from the next — so a heading always reads as belonging to what follows it. The root owns the width, which is capped rather than filling the pane: a row puts its label on one edge and its control on the other, and past that width the two stop reading as one line."
    >
      <Section
        title="Rows"
        description="A subtitle is for what the title cannot say in three words; an icon is for rows that name a thing rather than a setting; `onOpen` makes the row a button and grows the hover wash that says so, while the rest stay inert. The trailing slot takes any control — everything sharing the control height lands on one edge without the row being told what it holds."
      >
        <Preferences>
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
              <Select
                defaultValue="weekly"
                options={[
                  { value: 'daily', label: 'Daily' },
                  { value: 'weekly', label: 'Weekly' },
                ]}
                size="small"
              />
            </PreferenceRow>
            <PreferenceRow
              title="Opens something"
              subtitle="Hover it — the wash says it is a button"
              onOpen={() => {}}
            />
          </PreferenceSection>
        </Preferences>
      </Section>

      <Section
        title="Sections and groups"
        description="A section can carry a label, an action on the heading line, and a muted footer under its rows — each optional. The first run of a page usually goes without a label, because the page title has already named it. A group is a titled run of sections: sections stay one size everywhere, and the level above them is a title that outranks them rather than a section shrunk to fit underneath one, so a heading means the same thing wherever it is met."
      >
        <Preferences>
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
        </Preferences>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('preferences')}
          labelOf={token => token.name.slice('preferences-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const ASection: Story = {
  render: () => (
    <Preferences>
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
    </Preferences>
  ),
}

export const RowShapes: Story = {
  render: () => (
    <Preferences>
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
    </Preferences>
  ),
}

export const Controls: Story = {
  render: () => {
    const [launch, setLaunch] = useState(true)
    const [sounds, setSounds] = useState(false)
    const [appearance, setAppearance] = useState('system')

    return (
      <Preferences>
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
          <PreferenceRow title="Default target language">
            <TextField defaultValue="Chinese" size="small" style={FIELD} />
          </PreferenceRow>
        </PreferenceSection>
      </Preferences>
    )
  },
}

export const SectionHeadings: Story = {
  render: () => (
    <Preferences>
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
    </Preferences>
  ),
}

export const Unlabelled: Story = {
  render: () => (
    <Preferences>
      <PreferenceSection>
        <PreferenceRow title="Launch at login">
          <Switch aria-label="Launch at login" checked />
        </PreferenceRow>
        <PreferenceRow title="Show in the menu bar">
          <Switch aria-label="Show in the menu bar" checked />
        </PreferenceRow>
      </PreferenceSection>
    </Preferences>
  ),
}

export const AGroup: Story = {
  render: () => (
    <Preferences>
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
    </Preferences>
  ),
}

export const ASettingsPage: Story = {
  render: () => {
    const [launch, setLaunch] = useState(true)
    const [menuBar, setMenuBar] = useState(true)
    const [appearance, setAppearance] = useState('system')
    const [beta, setBeta] = useState(false)
    const [onPaste, setOnPaste] = useState(true)
    const [sync, setSync] = useState(true)
    const [translate, setTranslate] = useState('⌥⇧2')
    const [capture, setCapture] = useState('')

    return (
      <Preferences>
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
            <PreferenceRow title="Show in the menu bar">
              <Switch
                aria-label="Show in the menu bar"
                checked={menuBar}
                onCheckedChange={setMenuBar}
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
            <PreferenceRow title="Language">
              <Select
                defaultValue="system"
                options={[
                  { value: 'system', label: 'System' },
                  { value: 'en', label: 'English' },
                  { value: 'zh', label: '中文' },
                ]}
                size="small"
              />
            </PreferenceRow>
          </PreferenceSection>
          <PreferenceSection
            label="Updates"
            footer="The beta channel gets every build the day it is cut."
          >
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
          </PreferenceSection>
          <PreferenceSection
            label="Storage"
            action={
              <Button size="small" tint="neutral" variant="plain">
                Show in Finder
              </Button>
            }
          >
            <PreferenceRow
              title="Storage location"
              subtitle="~/Library/Application Support"
              onOpen={() => {}}
            />
            <PreferenceRow title="Cache" subtitle="1.2 GB across 3 projects">
              <Button size="small" tint="neutral">
                Clear
              </Button>
            </PreferenceRow>
          </PreferenceSection>
        </PreferenceGroup>
        <Divider />
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
            <PreferenceRow title="Add a provider" onOpen={() => {}} />
          </PreferenceSection>
          <PreferenceSection label="Behaviour">
            <PreferenceRow title="Default target language">
              <TextField defaultValue="Chinese" size="small" style={FIELD} />
            </PreferenceRow>
            <PreferenceRow
              title="Translate on paste"
              subtitle="Anything pasted into a segment is sent as it lands"
            >
              <Switch
                aria-label="Translate on paste"
                checked={onPaste}
                onCheckedChange={setOnPaste}
              />
            </PreferenceRow>
          </PreferenceSection>
          <PreferenceSection
            label="Shortcuts"
            action={
              <Button size="small" tint="neutral" variant="plain">
                Reset all
              </Button>
            }
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
        <Divider />
        <PreferenceGroup title="Account">
          <PreferenceSection>
            <PreferenceRow title="Signed in as" subtitle="mara@acme.example">
              <Button size="small" tint="neutral">
                Sign out
              </Button>
            </PreferenceRow>
            <PreferenceRow
              title="Sync settings"
              subtitle="Across every device signed in to this account"
            >
              <Switch
                aria-label="Sync settings"
                checked={sync}
                onCheckedChange={setSync}
              />
            </PreferenceRow>
            <PreferenceRow title="Plan" subtitle="Renews 1 October">
              <Badge tint="primary">Pro</Badge>
            </PreferenceRow>
          </PreferenceSection>
        </PreferenceGroup>
      </Preferences>
    )
  },
}
