import {
  Badge,
  Button,
  FieldValue,
  FormField,
  Icon,
  PreferenceGroup,
  PreferenceRow,
  Preferences,
  PreferenceSection,
  SegmentedControl,
  Select,
  ShortcutRecorder,
  Switch,
  TextField,
} from '@dazzlabs/dazzui-react'

import { FolderGlyph } from '../data/icons'
import type { Project, SettingsSection } from '../lib/project'
import {
  clashes,
  isConnected,
  MODELS,
  type Mode,
  type Settings,
  SHORTCUTS,
} from '../lib/settings'
import { THEMES, type Theme } from '../lib/theme'

const PARALLEL = [1, 2, 4, 8]

const TITLES: Record<SettingsSection, { title: string; description: string }> =
  {
    general: {
      title: 'General',
      description: 'How the window looks, and what it opens on.',
    },
    agent: {
      title: 'Agent',
      description:
        'Which model answers, and how much it is allowed to do at once.',
    },
    permissions: {
      title: 'Permissions',
      description:
        'What the agent may do without asking. Reading and searching never ask; everything that changes something does.',
    },
    integrations: {
      title: 'Integrations',
      description:
        'Where a project’s issues and pull requests come from. A project with no provider still works — it just has two fewer panes.',
    },
    shortcuts: {
      title: 'Shortcuts',
      description:
        'Click a binding and press the combination. Escape backs out, and ⌫ clears it.',
    },
  }

function General({
  settings,
  onThemeChange,
}: {
  settings: Settings
  // Appearance is the one setting with a side effect outside the store — the
  // attribute on the document — so it goes back through the app rather than
  // being written here.
  onThemeChange: (theme: Theme) => void
}) {
  return (
    <>
      <PreferenceSection label="Appearance">
        <PreferenceRow
          subtitle="Studio is the workhorse; Bright is louder."
          title="Theme"
        >
          <Select
            aria-label="Theme"
            className="ca-settings__select"
            options={THEMES}
            size="small"
            value={settings.theme}
            onValueChange={onThemeChange}
          />
        </PreferenceRow>
      </PreferenceSection>

      <PreferenceSection
        footer="Forge is a prototype of an interface, not an agent. Nothing here reaches the network, and no file is written."
        label="About"
      >
        <PreferenceRow title="Version">
          <FieldValue className="ca-settings__value" mono size="small">
            0.1.0
          </FieldValue>
        </PreferenceRow>
        <PreferenceRow title="Built on">
          <FieldValue className="ca-settings__value" mono size="small">
            @dazzlabs/dazzui-react
          </FieldValue>
        </PreferenceRow>
      </PreferenceSection>
    </>
  )
}

function Agent({
  settings,
  update,
}: {
  settings: Settings
  update: (patch: Partial<Settings>) => void
}) {
  return (
    <>
      <PreferenceSection label="Model">
        <PreferenceRow
          subtitle="What a new session starts on"
          title="Default model"
        >
          <Select
            aria-label="Default model"
            className="ca-settings__select"
            options={MODELS.map(model => ({ value: model, label: model }))}
            size="small"
            value={settings.model}
            onValueChange={model => update({ model })}
          />
        </PreferenceRow>

        <PreferenceRow
          subtitle="Ask reads and explains; Build writes."
          title="Default mode"
        >
          <SegmentedControl
            aria-label="Default mode"
            items={[
              { value: 'ask' as const, label: 'Ask' },
              { value: 'build' as const, label: 'Build' },
            ]}
            size="small"
            value={settings.mode}
            onValueChange={(mode: Mode) => update({ mode })}
          />
        </PreferenceRow>
      </PreferenceSection>

      <PreferenceSection
        footer="More calls in flight finish sooner and are harder to follow. Four is the point where the transcript still reads as a sequence."
        label="Execution"
      >
        <PreferenceRow
          subtitle="How many tool calls may run at once"
          title="Tools in flight"
        >
          <Select
            aria-label="Tools in flight"
            className="ca-settings__select"
            options={PARALLEL.map(count => ({
              value: String(count),
              label: String(count),
            }))}
            size="small"
            value={String(settings.parallel)}
            onValueChange={parallel => update({ parallel: Number(parallel) })}
          />
        </PreferenceRow>

        <PreferenceRow
          subtitle="Otherwise the run stops at the first failure"
          title="Keep going after a failed command"
        >
          <Switch
            aria-label="Keep going after a failed command"
            checked={settings.continueOnFailure}
            size="small"
            onCheckedChange={continueOnFailure => update({ continueOnFailure })}
          />
        </PreferenceRow>
      </PreferenceSection>
    </>
  )
}

function Permissions({
  settings,
  update,
}: {
  settings: Settings
  update: (patch: Partial<Settings>) => void
}) {
  return (
    <>
      <PreferenceSection
        footer="Turning off “ask before writing” is a standing yes for the whole window, not for one session. The sheet in front of the first write is the only place a diff is shown before it lands."
        label="Tools"
      >
        <PreferenceRow
          subtitle="Read, Grep and Task never change anything"
          title="Reading and searching"
        >
          <Badge size="small" tint="success">
            Always allowed
          </Badge>
        </PreferenceRow>

        <PreferenceRow
          subtitle="Ask before the first write of a session"
          title="Writing files"
        >
          <Switch
            aria-label="Ask before writing files"
            checked={settings.askBeforeWrite}
            size="small"
            onCheckedChange={askBeforeWrite => update({ askBeforeWrite })}
          />
        </PreferenceRow>

        <PreferenceRow title="Running commands">
          <Switch
            aria-label="Running commands"
            checked={settings.runCommands}
            size="small"
            onCheckedChange={runCommands => update({ runCommands })}
          />
        </PreferenceRow>

        <PreferenceRow title="Fetching the web">
          <Switch
            aria-label="Fetching the web"
            checked={settings.fetchWeb}
            size="small"
            onCheckedChange={fetchWeb => update({ fetchWeb })}
          />
        </PreferenceRow>
      </PreferenceSection>

      <PreferenceSection label="Allowlist">
        <FormField
          hint="One command per line. A prefix match is enough — “pnpm” allows every pnpm script."
          label="Commands that never ask"
        >
          <TextField
            className="ca-settings__allowlist"
            disabled={!settings.runCommands}
            mono
            multiline
            size="small"
            value={settings.allowlist}
            onChange={event => update({ allowlist: event.target.value })}
          />
        </FormField>
      </PreferenceSection>
    </>
  )
}

function Integrations({
  projects,
  settings,
  onConnect,
}: {
  projects: readonly Project[]
  settings: Settings
  onConnect: (id: string, connected: boolean) => void
}) {
  return (
    <PreferenceSection
      footer="Connecting is what fills the Issues and Pull requests panes. Disconnecting leaves the project’s sessions and its working tree exactly where they were."
      label="Projects"
    >
      {projects.map(project => {
        const connected = isConnected(settings, project)
        return (
          <PreferenceRow
            key={project.id}
            icon={<Icon icon={FolderGlyph} size={16} />}
            subtitle={project.path}
            title={project.name}
          >
            <Badge size="small" tint={connected ? 'success' : 'neutral'}>
              {connected ? 'GitHub' : 'No remote'}
            </Badge>
            <Button
              size="small"
              variant={connected ? 'recessed' : 'tinted'}
              onClick={() => onConnect(project.id, !connected)}
            >
              {connected ? 'Disconnect' : 'Connect'}
            </Button>
          </PreferenceRow>
        )
      })}
    </PreferenceSection>
  )
}

function Shortcuts({
  settings,
  update,
}: {
  settings: Settings
  update: (patch: Partial<Settings>) => void
}) {
  const taken = clashes(settings.shortcuts)

  return (
    <PreferenceSection label="Keyboard">
      {SHORTCUTS.map(shortcut => {
        const value = settings.shortcuts[shortcut.id]
        return (
          <PreferenceRow
            key={shortcut.id}
            subtitle={'where' in shortcut ? shortcut.where : undefined}
            title={shortcut.label}
          >
            <ShortcutRecorder
              aria-label={shortcut.label}
              // Two rows bound to the same keys both read as wrong, because
              // from either row that is exactly what is true.
              state={taken.has(value) ? 'error' : 'default'}
              value={value}
              onValueChange={next =>
                update({
                  shortcuts: { ...settings.shortcuts, [shortcut.id]: next },
                })
              }
            />
          </PreferenceRow>
        )
      })}
    </PreferenceSection>
  )
}

export interface SettingsViewProps {
  section: SettingsSection
  settings: Settings
  projects: readonly Project[]
  update: (patch: Partial<Settings>) => void
  onThemeChange: (theme: Theme) => void
  onConnect: (id: string, connected: boolean) => void
}

/**
 * The settings page.
 *
 * One `PreferenceGroup` per section, which is what carries the spacing: a
 * heading sits closer to its own rows than to the section above it, so it
 * always reads as belonging to what follows. The `Preferences` root is what
 * caps the column rather than letting it fill the pane; the app only lends it
 * a class, as a hook for the controls it sizes.
 */
export function SettingsView({
  section,
  settings,
  projects,
  update,
  onThemeChange,
  onConnect,
}: SettingsViewProps) {
  const heading = TITLES[section]

  return (
    <div className="ca-view">
      <header className="ca-view__head">
        <div className="ca-view__titles">
          <h1 className="ca-view__title">Settings</h1>
          <p className="ca-view__subtitle">{heading.title}</p>
        </div>
      </header>

      <div className="ca-view__body" data-inset="true">
        <Preferences className="ca-settings">
          <PreferenceGroup
            description={heading.description}
            title={heading.title}
          >
            {section === 'general' ? (
              <General settings={settings} onThemeChange={onThemeChange} />
            ) : null}
            {section === 'agent' ? (
              <Agent settings={settings} update={update} />
            ) : null}
            {section === 'permissions' ? (
              <Permissions settings={settings} update={update} />
            ) : null}
            {section === 'integrations' ? (
              <Integrations
                projects={projects}
                settings={settings}
                onConnect={onConnect}
              />
            ) : null}
            {section === 'shortcuts' ? (
              <Shortcuts settings={settings} update={update} />
            ) : null}
          </PreferenceGroup>
        </Preferences>
      </div>
    </div>
  )
}
