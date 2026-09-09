import {
  Badge,
  Button,
  Dialog,
  DialogClose,
  Icon,
  IconButton,
  type IconGlyph,
  KeyCap,
  PillTabs,
  PreferenceRow,
  Preferences,
  PreferenceSection,
  SearchField,
  SegmentedControl,
  Select,
  Switch,
} from '@dazzlabs/dazzui-react'
import { type ReactNode, useState } from 'react'

import {
  CheckCircleGlyph,
  CompassGlyph,
  CrossGlyph,
  DocGlyph,
  ExternalGlyph,
  FolderOpenGlyph,
  GearGlyph,
  GithubGlyph,
  KeyboardGlyph,
  LayersGlyph,
  QueueGlyph,
  RocketShipGlyph,
  ShieldGlyph,
  SparkleGlyph,
  TreeGlyph,
} from '../data/icons'
import type { Project } from '../lib/project'
import {
  APPROVALS,
  type Approval,
  MERGE_METHODS,
  type Mode,
  MODELS,
  MODES,
  PARALLEL,
  type Settings,
  SHORTCUTS,
} from '../lib/settings'
import { APPEARANCES, FAMILIES } from '../lib/theme'

/**
 * The pages of settings, in groups the way the column lists them: what the
 * window is, then what the agent does, then — under their own heading —
 * the projects it has open, each with a page of its own.
 */
const GROUPS: {
  id: string
  label: string
  glyph: IconGlyph
  about: string
  keywords: string
}[][] = [
  [
    {
      id: 'general',
      label: 'General',
      glyph: GearGlyph,
      about: 'The window itself: what it shows, and where it keeps things.',
      keywords:
        'dock sessions worktree location updates version restore defaults',
    },
    {
      id: 'account',
      label: 'Account',
      glyph: GithubGlyph,
      about: 'The account the issues, pull requests and checks are read with.',
      keywords: 'github merge squash rebase scopes disconnect',
    },
    {
      id: 'appearance',
      label: 'Appearance',
      glyph: LayersGlyph,
      about:
        'Which look the window is drawn in, and whether on paper or in the dark.',
      keywords: 'theme dark light system family colour color',
    },
    {
      id: 'shortcuts',
      label: 'Shortcuts',
      glyph: KeyboardGlyph,
      about: 'What this window answers to. Every page is one keystroke away.',
      keywords: 'keys keyboard palette',
    },
  ],
  [
    {
      id: 'models',
      label: 'Models',
      glyph: SparkleGlyph,
      about: 'Which model answers, and how far a message lets it go.',
      keywords: 'claude sonnet opus mode ask plan agent auto',
    },
    {
      id: 'approvals',
      label: 'Approvals',
      glyph: ShieldGlyph,
      about:
        'When the agent stops and asks, and what always asks whatever you set.',
      keywords: 'permissions push network delete allow',
    },
  ],
]

export interface SettingsDialogProps {
  settings: Settings
  update: (patch: Partial<Settings>) => void
  /** The project the window is on, whose page opens first under Projects. */
  project: Project
  /** Every project the window has open, for the column's own heading. */
  projects: readonly Project[]
  /** Whether each project's provider is bound, by id. */
  connected: (project: Project) => boolean
  onConnect: (project: Project, connected: boolean) => void
  onReset: () => void
  onClose: () => void
  /** For the two buttons that would reach the system: a toast, in this prototype. */
  onNotify: (message: string) => void
}

/** A path, set the way a file chooser prints it, with the two things to do with one. */
function PathPanel({
  path,
  onNotify,
}: {
  path: string
  onNotify: (m: string) => void
}) {
  return (
    <div className="wb-settings__inset">
      <Icon
        icon={FolderOpenGlyph}
        size={15}
        className="wb-settings__inset-mark"
      />
      <span className="wb-truncate wb-mono wb-settings__path">{path}</span>
      <IconButton
        label="Copy the path"
        size="small"
        tint="neutral"
        variant="plain"
        onClick={() => onNotify('Path copied.')}
      >
        <Icon icon={DocGlyph} size={14} />
      </IconButton>
      <IconButton
        label="Show in Finder"
        size="small"
        tint="neutral"
        variant="plain"
        onClick={() => onNotify(`Showing ${path} in Finder.`)}
      >
        <Icon icon={ExternalGlyph} size={14} />
      </IconButton>
    </div>
  )
}

/**
 * Settings, as a sheet over the window.
 *
 * They belong to the window rather than to whichever project is open, and
 * they are somewhere a reader goes for a moment and comes back from — so they
 * take the window over rather than becoming a page. Every control writes as
 * it is touched, which is why there is no Save and no Cancel: Escape and the
 * cross both simply close it.
 *
 * The shape is the one every desktop settings window has settled on: a
 * column on the left with who you are, a search field, and the pages in
 * groups — the window's, the agent's, and each open project's under a
 * heading of its own — and on the right the page as a run of rows, the
 * setting's name and what it means on one edge and its control on the other.
 * A control too wide for a row's edge sits under it in an inset panel.
 */
export function SettingsDialog({
  settings,
  update,
  project,
  projects,
  connected,
  onConnect,
  onReset,
  onClose,
  onNotify,
}: SettingsDialogProps) {
  const [section, setSection] = useState<string>('general')
  const [query, setQuery] = useState('')

  const policy = APPROVALS.find(entry => entry.value === settings.approval)
  const mode = MODES.find(entry => entry.value === settings.mode)
  const family = FAMILIES.find(entry => entry.value === settings.family)

  /* The search is over the column: a page whose name or contents match
     stays, the rest fold away, and the first match is opened if the open
     one has gone. */
  const needle = query.trim().toLowerCase()
  const matches = (label: string, keywords: string) =>
    !needle || `${label} ${keywords}`.toLowerCase().includes(needle)
  const groups = GROUPS.map(group =>
    group.filter(entry => matches(entry.label, entry.keywords))
  ).filter(group => group.length > 0)
  const shownProjects = projects.filter(entry =>
    matches(entry.name, `project ${entry.path} ${entry.branch}`)
  )
  const visible = new Set([
    ...groups.flat().map(entry => entry.id),
    ...shownProjects.map(entry => `project:${entry.id}`),
  ])
  const open = visible.has(section)
    ? section
    : (groups[0]?.[0]?.id ??
      (shownProjects[0] ? `project:${shownProjects[0].id}` : section))

  const page = GROUPS.flat().find(entry => entry.id === open)
  const shownProject = open.startsWith('project:')
    ? projects.find(entry => `project:${entry.id}` === open)
    : undefined

  const row = (
    title: ReactNode,
    subtitle: ReactNode,
    control?: ReactNode,
    inset?: ReactNode
  ) => (
    <div className="wb-settings__row">
      <PreferenceRow subtitle={subtitle} title={title}>
        {control}
      </PreferenceRow>
      {inset}
    </div>
  )

  const body = (() => {
    if (shownProject) {
      const bound = connected(shownProject)
      return (
        <PreferenceSection>
          {row(
            'Location',
            'Where the repository is on disk.',
            undefined,
            <PathPanel path={shownProject.path} onNotify={onNotify} />
          )}
          {row(
            'Base branch',
            'What a new session’s branch is cut from, and what a pull request targets.',
            <Select
              aria-label="Base branch"
              className="wb-settings__select"
              options={shownProject.branches.map(branch => ({
                value: branch.name,
                label: branch.name,
              }))}
              size="small"
              value={shownProject.branch}
              onValueChange={() =>
                onNotify(
                  'The base branch is set per session in this prototype.'
                )
              }
            />
          )}
          {row(
            'GitHub',
            shownProject.provider === null
              ? 'This project has no remote, so there is nothing to read issues from.'
              : bound
                ? `Issues, pull requests and checks are read from ${shownProject.name}.`
                : 'Disconnected. The GitHub page says how to reconnect.',
            shownProject.provider === null ? (
              <Button size="small" tint="neutral" variant="outlined">
                Publish to GitHub…
              </Button>
            ) : (
              <>
                <Badge
                  size="small"
                  tint={bound ? 'success' : 'neutral'}
                  variant="tinted"
                >
                  {bound ? 'Connected' : 'Disconnected'}
                </Badge>
                <Button
                  size="small"
                  tint="neutral"
                  variant={bound ? 'plain' : 'outlined'}
                  onClick={() => onConnect(shownProject, !bound)}
                >
                  {bound ? 'Disconnect' : 'Connect'}
                </Button>
              </>
            )
          )}
          {row(
            'Tests',
            shownProject.tests
              ? `${shownProject.tests.suites.length} suites, ${shownProject.tests.passed + shownProject.tests.failed} cases at the last run.`
              : 'No test command is configured. The tests page stays empty.',
            <Badge size="small" tint="neutral">
              {shownProject.tests ? 'Configured' : 'None'}
            </Badge>
          )}
          {row(
            'Release',
            shownProject.release
              ? `Read from ${shownProject.release.configFile}.`
              : 'Nothing is configured to ship from this project.',
            <Badge size="small" tint="neutral">
              {shownProject.release
                ? shownProject.release.configs.length + ' configs'
                : 'None'}
            </Badge>
          )}
        </PreferenceSection>
      )
    }

    switch (open) {
      case 'general':
        return (
          <>
            <div className="wb-settings__inset wb-settings__version">
              <Icon
                icon={CompassGlyph}
                size={28}
                className="wb-settings__inset-mark"
              />
              <span className="wb-settings__version-body">
                <span className="wb-card__title">Workbench 0.0.0</span>
                <span className="wb-card__quiet">
                  A design prototype. Nothing is fetched; every run is scripted.
                </span>
              </span>
              <Button
                size="small"
                tint="neutral"
                variant="outlined"
                onClick={() =>
                  onNotify('Up to date — this build is the only one.')
                }
              >
                Check for updates
              </Button>
            </div>

            <PreferenceSection>
              {row(
                'Dock beside the chat',
                'The changes, the terminal and the pull request, beside the thread. Off, the thread takes the whole pane.',
                <Switch
                  aria-label="Dock beside the chat"
                  checked={settings.dock}
                  onCheckedChange={dock => update({ dock })}
                />
              )}
              {row(
                'Sessions at once',
                'Each running session takes a worktree of its own. Anything over the limit waits, queued, in the sidebar.',
                <SegmentedControl
                  aria-label="Sessions at once"
                  items={PARALLEL.map(count => ({
                    value: String(count),
                    label: String(count),
                  }))}
                  size="small"
                  value={String(settings.parallel)}
                  onValueChange={value => update({ parallel: Number(value) })}
                />
              )}
              {row(
                'Worktrees location',
                'Where each session’s checkout is created, inside the project.',
                <Button
                  size="small"
                  tint="neutral"
                  variant="outlined"
                  onClick={() =>
                    onNotify('The location is fixed in this prototype.')
                  }
                >
                  Change location…
                </Button>,
                <PathPanel
                  path={`${project.path}/worktrees`}
                  onNotify={onNotify}
                />
              )}
            </PreferenceSection>

            <PreferenceSection>
              {row(
                'Restore defaults',
                'Every setting back to what the window opened with. Settings live in memory for as long as it is open.',
                <Button
                  size="small"
                  tint="neutral"
                  variant="outlined"
                  onClick={onReset}
                >
                  Restore
                </Button>
              )}
            </PreferenceSection>
          </>
        )

      case 'account':
        return (
          <PreferenceSection>
            <div className="wb-settings__inset">
              <span aria-hidden className="wb-event__avatar">
                MC
              </span>
              <span className="wb-event__body">
                <span className="wb-card__title">mia</span>
                <span className="wb-card__quiet">
                  acme organisation · repo, issues and pull request scopes
                </span>
              </span>
              <Badge size="small" tint="success" variant="tinted">
                <Icon icon={CheckCircleGlyph} size={11} />
                Signed in
              </Badge>
              <Button
                size="small"
                tint="neutral"
                variant="plain"
                onClick={() => onNotify('Signed out of GitHub.')}
              >
                Sign out
              </Button>
            </div>
            {row(
              'Default merge method',
              'What the merge button on a pull request does unless it is told otherwise.',
              <SegmentedControl
                aria-label="Default merge method"
                items={MERGE_METHODS.map(entry => ({
                  value: entry.value,
                  label: entry.label,
                }))}
                size="small"
                value={settings.mergeMethod}
                onValueChange={mergeMethod => update({ mergeMethod })}
              />
            )}
            {row(
              'Projects on this account',
              'Which of the open projects read their issues and pull requests through it. Each project has a page of its own below.',
              <Badge size="small" tint="neutral">
                {projects.filter(connected).length} of {projects.length}
              </Badge>
            )}
          </PreferenceSection>
        )

      case 'appearance':
        return (
          <PreferenceSection>
            {row(
              'Theme',
              family?.hint,
              undefined,
              <div className="wb-settings__inset">
                <PillTabs
                  aria-label="Theme"
                  items={FAMILIES.map(entry => ({
                    value: entry.value,
                    label: entry.label,
                  }))}
                  value={settings.family}
                  onValueChange={value => update({ family: value })}
                />
              </div>
            )}
            {row(
              'Mode',
              'Light, dark, or whichever the machine is set to — System follows it as it changes.',
              <SegmentedControl
                aria-label="Mode"
                items={APPEARANCES.map(entry => ({
                  value: entry.value,
                  label: entry.label,
                }))}
                size="small"
                value={settings.appearance}
                onValueChange={appearance => update({ appearance })}
              />
            )}
          </PreferenceSection>
        )

      case 'shortcuts':
        return (
          <PreferenceSection footer="On Windows and Linux, ⌃ stands in for ⌘. In a browser tab ⌘, belongs to the browser, so ⌘9 opens this sheet there.">
            {SHORTCUTS.map(shortcut => (
              <PreferenceRow key={shortcut.keys} title={shortcut.label}>
                <KeyCap size="small" variant="key">
                  {shortcut.keys}
                </KeyCap>
              </PreferenceRow>
            ))}
          </PreferenceSection>
        )

      case 'models':
        return (
          <PreferenceSection>
            {row(
              'Default model',
              'What a new session starts on. A session keeps the model it was opened with.',
              <SegmentedControl
                aria-label="Default model"
                items={MODELS.map(entry => ({
                  value: entry.value,
                  label: entry.label,
                }))}
                size="small"
                value={settings.model}
                onValueChange={model => update({ model })}
              />
            )}
            {row(
              'Mode a message starts in',
              mode?.hint,
              <SegmentedControl
                aria-label="Mode"
                items={MODES.map(entry => ({
                  value: entry.value,
                  label: entry.label,
                }))}
                size="small"
                value={settings.mode}
                onValueChange={(value: Mode) => update({ mode: value })}
              />
            )}
          </PreferenceSection>
        )

      case 'approvals':
        return (
          <>
            <PreferenceSection>
              {row(
                'When the agent asks',
                policy?.hint,
                <SegmentedControl
                  aria-label="When the agent asks"
                  items={APPROVALS.map(entry => ({
                    value: entry.value,
                    label: entry.label,
                  }))}
                  size="small"
                  value={settings.approval}
                  onValueChange={(value: Approval) =>
                    update({ approval: value })
                  }
                />
              )}
            </PreferenceSection>

            <PreferenceSection
              footer="Whatever the policy above, these stop for the permission card in the thread — they leave the machine, or they touch something outside the session’s own worktree."
              label="Always asks"
            >
              {[
                [
                  'git push, force and rebase',
                  'Rewrites what other people have.',
                ],
                ['Network requests', 'Leaves the machine.'],
                [
                  'Writes outside the worktree',
                  'Touches another session’s work.',
                ],
                ['Deleting a file', 'Cannot be read back afterwards.'],
                ['Every write to the GitHub API', 'Is visible to everyone.'],
              ].map(([what, why]) => (
                <PreferenceRow
                  key={what}
                  icon={<Icon icon={ShieldGlyph} size={14} />}
                  subtitle={why}
                  title={what}
                >
                  <Badge size="small" tint="warning" variant="tinted">
                    Always
                  </Badge>
                </PreferenceRow>
              ))}
            </PreferenceSection>
          </>
        )

      default:
        return null
    }
  })()

  const navRow = (
    id: string,
    label: string,
    glyph: IconGlyph,
    current: boolean
  ) => (
    <Button
      key={id}
      aria-current={current || undefined}
      className="wb-settings__tab"
      fullWidth
      size="small"
      tint={current ? 'primary' : 'neutral'}
      variant={current ? 'tinted' : 'plain'}
      onClick={() => setSection(id)}
    >
      <Icon icon={glyph} size={15} />
      <span className="wb-truncate">{label}</span>
    </Button>
  )

  return (
    <Dialog
      open
      className="wb-settings"
      width="68rem"
      onOpenChange={next => {
        if (!next) onClose()
      }}
    >
      {/* The column. Buttons rather than a `Sidebar`: that component is a
          window column and reads the window's own context, which a sheet in
          a portal is outside of. */}
      <nav aria-label="Settings" className="wb-settings__nav">
        <div className="wb-settings__who">
          <span aria-hidden className="wb-event__avatar wb-settings__avatar">
            MC
          </span>
          <span className="wb-event__body">
            <span className="wb-card__title">Mia Chen</span>
            <span className="wb-card__quiet">@mia</span>
          </span>
        </div>

        <SearchField
          autoFocus
          placeholder="Search settings…"
          size="small"
          value={query}
          onValueChange={setQuery}
        />

        {groups.map((group, index) => (
          <div key={index} className="wb-settings__group">
            {group.map(entry =>
              navRow(entry.id, entry.label, entry.glyph, entry.id === open)
            )}
          </div>
        ))}

        {shownProjects.length > 0 ? (
          <div className="wb-settings__group">
            <span className="wb-settings__heading">Projects</span>
            {shownProjects.map(entry =>
              navRow(
                `project:${entry.id}`,
                entry.name.split('/').pop() ?? entry.name,
                entry.id === project.id ? FolderOpenGlyph : TreeGlyph,
                `project:${entry.id}` === open
              )
            )}
          </div>
        ) : null}

        {groups.length === 0 && shownProjects.length === 0 ? (
          <span className="wb-card__quiet wb-settings__none">
            Nothing matches “{query.trim()}”.
          </span>
        ) : null}
      </nav>

      <section className="wb-settings__panel">
        <header className="wb-settings__head">
          <h2 className="wb-settings__title">
            {shownProject ? shownProject.name : (page?.label ?? 'Settings')}
          </h2>
          <span className="wb-view__gap" />
          <DialogClose
            render={
              <IconButton
                label="Close"
                size="small"
                tint="neutral"
                variant="plain"
              >
                <Icon icon={CrossGlyph} size={15} />
              </IconButton>
            }
          />
        </header>

        <div className="wb-settings__scroll">
          <Preferences className="wb-settings__prefs">
            {shownProject ? (
              <p className="wb-card__note wb-settings__about">
                <Icon
                  icon={shownProject.release ? RocketShipGlyph : QueueGlyph}
                  size={13}
                />{' '}
                {shownProject.provider
                  ? `${shownProject.name} on GitHub, on ${shownProject.branch}.`
                  : `${shownProject.name}, a local repository with no remote, on ${shownProject.branch}.`}
              </p>
            ) : page ? (
              <p className="wb-card__note wb-settings__about">{page.about}</p>
            ) : null}
            {body}
          </Preferences>
        </div>
      </section>
    </Dialog>
  )
}
