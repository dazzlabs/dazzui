import {
  Badge,
  Button,
  Divider,
  FormField,
  Icon,
  Progress,
  SectionLabel,
  Sidebar,
  SidebarContent,
  Switch,
  TextField,
} from '@dazzlabs/dazzui-react'
import { BranchGlyph } from '../data/icons'
import type { Issue, Project, Pull, View } from '../lib/project'
import type { Settings } from '../lib/settings'

/**
 * The window the run is working inside, and what was in it before the first
 * message: the system prompt and the project files already read. A single run
 * barely moves a real context window, so the number is printed beside the bar
 * rather than left to it.
 */
const CONTEXT_LIMIT = 64_000
const CONTEXT_BASE = 21_400

function thousands(value: number): string {
  return value >= 1000 ? `${(value / 1000).toFixed(1)}k` : String(value)
}

function Stat({ label, children }: { label: string; children: string }) {
  return (
    <div className="ca-stat">
      <span className="ca-stat__label">{label}</span>
      <span className="ca-stat__value">{children}</span>
    </div>
  )
}

function Nothing({ children }: { children: string }) {
  return (
    <section className="ca-panel">
      <span className="ca-panel__note">{children}</span>
    </section>
  )
}

function SessionPanel({
  tokens,
  project,
  settings,
  update,
  trusted,
}: {
  tokens: number
  project: Project
  settings: Settings
  update: (patch: Partial<Settings>) => void
  trusted: readonly string[]
}) {
  const used = CONTEXT_BASE + tokens
  const percent = Math.min(100, Math.round((used / CONTEXT_LIMIT) * 100))

  return (
    <>
      <section className="ca-panel">
        <SectionLabel faint>Context window</SectionLabel>
        <Progress
          size="medium"
          tint={percent > 80 ? 'warning' : 'primary'}
          value={percent}
        />
        <span className="ca-panel__note">
          {thousands(used)} of {thousands(CONTEXT_LIMIT)} tokens · {percent}%
        </span>
      </section>

      <Divider tone="muted" />

      <section className="ca-panel">
        <SectionLabel faint>Session</SectionLabel>
        <Stat label="Model">{settings.model}</Stat>
        <Stat label="Mode">{settings.mode}</Stat>
        <Stat label="Branch">{project.branch}</Stat>
      </section>

      <Divider tone="muted" />

      {/* The same values the settings page edits, put where they are needed
          mid-run. Two switches wearing one name would be two settings. */}
      <section className="ca-panel">
        <SectionLabel faint>Permissions</SectionLabel>
        <div className="ca-switch">
          <span className="ca-switch__label">Run commands</span>
          <Switch
            aria-label="Run commands"
            checked={settings.runCommands}
            size="small"
            onCheckedChange={runCommands => update({ runCommands })}
          />
        </div>
        <div className="ca-switch">
          <span className="ca-switch__label">Fetch the web</span>
          <Switch
            aria-label="Fetch the web"
            checked={settings.fetchWeb}
            size="small"
            onCheckedChange={fetchWeb => update({ fetchWeb })}
          />
        </div>
        <span className="ca-panel__note">
          {!settings.askBeforeWrite
            ? 'Writes never ask — turned off in Settings.'
            : trusted.length === 0
              ? 'Every write asks first.'
              : `Allowed for this session: ${trusted.join(', ')}.`}
        </span>
      </section>
    </>
  )
}

function IssuePanel({
  issue,
  onStart,
}: {
  issue: Issue
  onStart: (issue: Issue) => void
}) {
  return (
    <>
      <section className="ca-panel">
        <SectionLabel faint>Issue #{issue.number}</SectionLabel>
        <h2 className="ca-detail__title">{issue.title}</h2>
        <span className="ca-list__meta">
          {issue.labels.map(label => (
            <Badge key={label} size="small" tint="info" variant="outlined">
              {label}
            </Badge>
          ))}
        </span>
        <p className="ca-detail__body">{issue.body}</p>
      </section>

      <Divider tone="muted" />

      <section className="ca-panel">
        <Stat label="Author">{issue.author}</Stat>
        <Stat label="Opened">{issue.when}</Stat>
        <Stat label="Comments">{String(issue.comments)}</Stat>
      </section>

      <Button fullWidth variant="filled" onClick={() => onStart(issue)}>
        Start a session
      </Button>
    </>
  )
}

function PullPanel({
  pull,
  onStart,
}: {
  pull: Pull
  onStart: (pull: Pull) => void
}) {
  return (
    <>
      <section className="ca-panel">
        <SectionLabel faint>Pull request #{pull.number}</SectionLabel>
        <h2 className="ca-detail__title">{pull.title}</h2>
        <span className="ca-list__meta">
          <Badge
            size="small"
            tint={pull.state === 'draft' ? 'neutral' : 'info'}
          >
            {pull.state}
          </Badge>
          {pull.state === 'merged' ? null : (
            <Badge
              size="small"
              tint={
                pull.checks === 'passing'
                  ? 'success'
                  : pull.checks === 'failing'
                    ? 'danger'
                    : 'neutral'
              }
            >
              {pull.checks}
            </Badge>
          )}
        </span>
        <p className="ca-detail__body">{pull.body}</p>
      </section>

      <Divider tone="muted" />

      <section className="ca-panel">
        <span className="ca-sidebar__branch">
          <Icon icon={BranchGlyph} size={12} />
          <span className="ca-truncate">{pull.branch}</span>
        </span>
        <Stat label="Files">{String(pull.files)}</Stat>
        <Stat label="Added">{`+${pull.added}`}</Stat>
        <Stat label="Removed">{`-${pull.removed}`}</Stat>
        <Stat label="Author">{pull.author}</Stat>
      </section>

      <Button fullWidth variant="filled" onClick={() => onStart(pull)}>
        Review in a session
      </Button>
    </>
  )
}

function CommitPanel({
  staged,
  message,
  onMessageChange,
  onCommit,
}: {
  staged: number
  message: string
  onMessageChange: (message: string) => void
  onCommit: () => void
}) {
  return (
    <section className="ca-panel">
      <SectionLabel faint>Commit</SectionLabel>
      <FormField
        hint={
          staged === 0
            ? 'Stage a file first.'
            : `${staged} ${staged === 1 ? 'file' : 'files'} staged.`
        }
        label="Message"
      >
        <TextField
          multiline
          placeholder="Say what changed, and why"
          size="small"
          value={message}
          onChange={event => onMessageChange(event.target.value)}
        />
      </FormField>
      <Button
        disabled={staged === 0 || message.trim().length === 0}
        fullWidth
        variant="filled"
        onClick={onCommit}
      >
        Commit
      </Button>
    </section>
  )
}

function SettingsPanel({ onReset }: { onReset: () => void }) {
  return (
    <section className="ca-panel">
      <SectionLabel faint>This window</SectionLabel>
      <span className="ca-panel__note">
        Settings live in memory for as long as the window is open. There is
        nothing behind them to write to — reloading the page puts every one of
        them back.
      </span>
      <Button fullWidth size="small" onClick={onReset}>
        Restore defaults
      </Button>
    </section>
  )
}

export interface DetailAsideProps {
  view: View
  project: Project
  tokens: number
  settings: Settings
  update: (patch: Partial<Settings>) => void
  onReset: () => void
  trusted: readonly string[]
  issue: Issue | null
  pull: Pull | null
  staged: number
  message: string
  onMessageChange: (message: string) => void
  onCommit: () => void
  onStartFromIssue: (issue: Issue) => void
  onStartFromPull: (pull: Pull) => void
}

/**
 * The trailing column, which is about whatever the main one is showing: the
 * run's context while a session is open, the selected issue or pull request
 * while a list is, and the commit while the working tree is.
 *
 * It is a `Sidebar` on the other edge — the same column at the aside's own
 * width, holding fields rather than rows.
 */
export function DetailAside({
  view,
  project,
  tokens,
  settings,
  update,
  onReset,
  trusted,
  issue,
  pull,
  staged,
  message,
  onMessageChange,
  onCommit,
  onStartFromIssue,
  onStartFromPull,
}: DetailAsideProps) {
  return (
    <Sidebar collapsible="none" side="right" className="ca-aside">
      <SidebarContent className="ca-aside__body">
        {view.kind === 'settings' ? (
          <SettingsPanel onReset={onReset} />
        ) : view.kind === 'session' ? (
          <SessionPanel
            project={project}
            settings={settings}
            tokens={tokens}
            trusted={trusted}
            update={update}
          />
        ) : view.pane === 'issues' ? (
          issue ? (
            <IssuePanel issue={issue} onStart={onStartFromIssue} />
          ) : (
            <Nothing>
              Pick an issue to read it here, and to start from it.
            </Nothing>
          )
        ) : view.pane === 'pulls' ? (
          pull ? (
            <PullPanel pull={pull} onStart={onStartFromPull} />
          ) : (
            <Nothing>
              Pick a pull request to see its checks and its size.
            </Nothing>
          )
        ) : (
          <CommitPanel
            message={message}
            staged={staged}
            onCommit={onCommit}
            onMessageChange={onMessageChange}
          />
        )}
      </SidebarContent>
    </Sidebar>
  )
}
