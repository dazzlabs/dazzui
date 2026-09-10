import {
  Callout,
  EmptyState,
  Icon,
  OptionCard,
  ToastViewport,
  useToast,
} from '@dazzlabs/dazzui-react'
import { useCallback, useEffect, useMemo, useRef, useState } from 'react'

import { type Command, CommandPalette } from './components/command-palette'
import { ContextSidebar } from './components/context-sidebar'
import { Composer } from './components/composer'
import { DockPanel } from './components/dock-panel'
import { GitView } from './components/git-view'
import { InboxView } from './components/inbox-view'
import { type CheckRef, checkKey, GithubView } from './components/github-view'
import { ReleaseView } from './components/release-view'
import { ReviewView } from './components/review-view'
import { SettingsDialog } from './components/settings-dialog'
import {
  type SessionPlan,
  slugOf,
  StartSessionDialog,
} from './components/start-session-dialog'
import { StatusBar } from './components/status-bar'
import { TestsView } from './components/tests-view'
import { ThreadView } from './components/thread-view'
import { WelcomeView } from './components/welcome-view'
import { WindowFrame } from './components/window-frame'
import {
  BranchGlyph,
  ChatGlyph,
  CheckGlyph,
  DiffGlyph,
  FlaskGlyph,
  FolderOpenGlyph,
  GearGlyph,
  GithubGlyph,
  IssueGlyph,
  PlusGlyph,
  PullGlyph,
  RocketShipGlyph,
  ShieldGlyph,
  SparkleGlyph,
  TrayGlyph,
} from './data/icons'
import { OPEN_PROJECTS, PROJECTS } from './data/projects'
import {
  checkRun,
  commentRun,
  issueRun,
  SUGGESTIONS,
  suggestFromIssue,
  TOOLTIP_RUN,
} from './data/run'
import { usePlayer } from './lib/player'
import {
  type Attention,
  type CheckRun,
  checksStatus,
  type GithubFace,
  type InboxFilter,
  type Issue,
  needsYou,
  openIssues,
  type Project,
  type Pull,
  type Session,
  type SessionStatus,
  shortName,
  type Task,
  type View,
  VIEWS,
  workingTree,
  type WorkingChange,
  type Worktree,
} from './lib/project'
import {
  APPROVALS,
  isConnected,
  type Mode,
  MODES,
  useSettings,
} from './lib/settings'
import {
  type Appearance,
  APPEARANCES,
  applyTheme,
  FAMILIES,
  type Family,
  watchSystem,
} from './lib/theme'
import {
  type Beat,
  changedFiles,
  type Entry,
  estimateTokens,
} from './lib/transcript'

/**
 * The window the run is working inside, and what was in it before the first
 * message: the system prompt and the project files already read. A single run
 * barely moves a real context window, so the base is most of the number.
 */
const CONTEXT_LIMIT = 64_000
const CONTEXT_BASE = 21_400

/** The open session's state is the player's, not the fixture's. */
function liveStatus(
  session: Session,
  entries: readonly Entry[],
  running: boolean,
  asking: boolean
): SessionStatus {
  if (asking) return 'waiting'
  if (running) return 'running'
  // A run that has finished and left a rule under itself is holding: on a
  // diff, if it gathered one, and otherwise on whatever the rule names.
  if (entries.some(entry => entry.kind === 'divider')) {
    return entries.some(entry => entry.kind === 'review') ? 'review' : 'paused'
  }
  return session.status
}

/** True where a beat would change something. */
function writes(beat: Beat): boolean {
  if (beat.kind === 'ask' || beat.kind === 'review') return true
  return beat.kind === 'tool' && beat.tool !== 'Read' && beat.tool !== 'Grep'
}

/**
 * The run as the mode allows it. Ask stops before the first thing that
 * would write, and says so; Plan stops once the plan is on the page and
 * waits; Agent is the run as scripted; Auto is the run with nothing in it
 * that stops to ask.
 */
function shape(beats: Beat[], mode: Mode): Beat[] {
  switch (mode) {
    case 'ask': {
      const kept: Beat[] = []
      for (const beat of beats) {
        if (writes(beat)) break
        if (beat.kind === 'plan' || beat.kind === 'step') continue
        if (beat.kind === 'finish') break
        kept.push(beat)
      }
      return [
        ...kept,
        {
          kind: 'finish',
          toast: 'Answered. Nothing was written.',
          after: 'Answered in Ask mode · nothing was written',
        },
      ]
    }
    case 'plan': {
      const at = beats.findIndex(beat => beat.kind === 'plan')
      if (at === -1) return beats
      return [
        ...beats.slice(0, at + 1).filter(beat => !writes(beat)),
        {
          kind: 'finish',
          toast: 'The plan is drafted and waiting for a go.',
          after: 'Waiting for you to approve the plan',
        },
      ]
    }
    case 'auto':
      return beats.filter(beat => beat.kind !== 'ask')
    default:
      return beats
  }
}

export function App() {
  const { settings, update, reset } = useSettings()

  /* What the window has open, and where it is looking. */
  const [openIds, setOpenIds] = useState<string[]>(OPEN_PROJECTS)
  const [projectId, setProjectId] = useState(OPEN_PROJECTS[0])
  const [view, setView] = useState<View>('chat')
  const [palette, setPalette] = useState(false)
  const [settingsOpen, setSettingsOpen] = useState(false)
  /** The issue a session is about to be opened from, while the sheet is up. */
  const [starting, setStarting] = useState<Issue | null>(null)

  /* What each page has selected. A page keeps its place while the reader is
     elsewhere, which is the whole reason the sidebar carries these. */
  const [sessionId, setSessionId] = useState<string>(PROJECTS[0].sessions[0].id)
  const [face, setFace] = useState<GithubFace>('issues')
  const [issueFilter, setIssueFilter] = useState('open')
  const [inboxFilter, setInboxFilter] = useState<InboxFilter>('all')
  const [pullNumber, setPullNumber] = useState<number | null>(null)
  const [issueNumber, setIssueNumber] = useState<number | null>(null)
  const [checkId, setCheckId] = useState<string | null>(null)

  const [draft, setDraft] = useState('')
  /** `startNow` is declared before the function it calls; the ref bridges them. */
  const startFromIssueRef = useRef<(issue: Issue, plan: SessionPlan) => void>(
    () => undefined
  )
  const [message, setMessage] = useState('')
  /** Sessions opened in this window, in front of the fixture's. */
  const [opened, setOpened] = useState<Record<string, Session[]>>({})
  /** Staging overrides, keyed `project:path`; absent means the fixture decides. */
  const [staging, setStaging] = useState<Record<string, boolean>>({})
  /** Inbox notices the reader has answered, by id. */
  const [dismissed, setDismissed] = useState<string[]>([])

  const fixture = PROJECTS.find(entry => entry.id === projectId) ?? PROJECTS[0]
  const project: Project = useMemo(
    () => ({
      ...fixture,
      sessions: [...(opened[fixture.id] ?? []), ...fixture.sessions],
      inbox: fixture.inbox.filter(notice => !dismissed.includes(notice.id)),
    }),
    [fixture, opened, dismissed]
  )

  const projects = useMemo(
    () =>
      openIds
        .map(id => PROJECTS.find(entry => entry.id === id))
        .filter((entry): entry is Project => entry !== undefined)
        .map(entry => ({
          ...entry,
          sessions: [...(opened[entry.id] ?? []), ...entry.sessions],
          inbox: entry.inbox.filter(notice => !dismissed.includes(notice.id)),
        })),
    [openIds, opened, dismissed]
  )

  const toast = useToast()
  const say = useCallback(
    (title: string) => toast.add({ title, tint: 'info' }),
    [toast]
  )
  const connected = isConnected(settings, project)

  /* A finished run reports into the queue, which owns how long it stays and
     which one wins when two land together. */
  const player = usePlayer(project.sessions[0].seed, text =>
    toast.add({
      title: text,
      tint: 'success',
      icon: <Icon icon={CheckGlyph} size={16} />,
      action: { label: 'Review', onClick: () => setView('review') },
    })
  )

  const session =
    project.sessions.find(entry => entry.id === sessionId) ??
    project.sessions[0]
  const status = liveStatus(
    session,
    player.entries,
    player.running,
    player.ask !== null
  )

  /** The project's sessions, with the open one's state coming off the run. */
  const sessions = useMemo(
    () =>
      project.sessions.map(entry =>
        entry.id === session.id ? { ...entry, status } : entry
      ),
    [project.sessions, session.id, status]
  )

  const written = useMemo(() => changedFiles(player.entries), [player.entries])
  const changes = useMemo(
    () => workingTree(project, written),
    [project, written]
  )
  const tokens = useMemo(() => estimateTokens(player.entries), [player.entries])
  const context = Math.min(
    100,
    Math.round(((CONTEXT_BASE + tokens) / CONTEXT_LIMIT) * 100)
  )

  /** The pull request the open session is pushing to, if it has one. */
  const sessionPull =
    project.pulls.find(entry => entry.number === session.pull) ?? null

  /** The one the pull request page is showing. */
  const pull =
    project.pulls.find(entry => entry.number === pullNumber) ??
    sessionPull ??
    project.pulls[0] ??
    null

  /** Every open project's sessions, which is what makes the queue a queue. */
  const tasks: Task[] = useMemo(
    () =>
      projects.flatMap(entry =>
        (entry.id === project.id ? sessions : entry.sessions).map(item => ({
          session: item,
          project: entry,
        }))
      ),
    [projects, project.id, sessions]
  )

  /** The ones asking for a person, which is what the inbox is. */
  const asking = useMemo(
    () => tasks.filter(task => needsYou(task.session.status)),
    [tasks]
  )

  /** And what the window has noticed about them. */
  const inbox: Attention[] = useMemo(
    () =>
      projects.flatMap(entry =>
        entry.inbox.map(notice => ({ notice, project: entry }))
      ),
    [projects]
  )

  /** The pull request page, on one of them. */
  const showPull = useCallback((number: number | null) => {
    setPullNumber(number)
    setFace('pulls')
    setView('github')
  }, [])

  /** The issue the GitHub page is reading: the chosen one, or the first open. */
  const issue: Issue | null =
    project.issues.find(entry => entry.number === issueNumber) ??
    openIssues(project)[0] ??
    null

  /** The check it is reading, on the checks face. */
  const check: CheckRef | null = useMemo(() => {
    const all = project.pulls
      .filter(entry => entry.state !== 'merged')
      .flatMap(entry => entry.runs.map(run => ({ pull: entry, run })))
    return (
      all.find(entry => checkKey(entry.pull, entry.run) === checkId) ??
      all.find(entry => entry.run.status === 'failed') ??
      all[0] ??
      null
    )
  }, [project.pulls, checkId])

  /** A session from an issue with every default taken: what S does. */
  const startNow = useCallback(
    (target: Issue) => {
      const slug = `${target.number}-${slugOf(target.title)}`
      startFromIssueRef.current(target, {
        worktree: `worktrees/${slug}`,
        branch: `feat/${slug}`,
        mode: settings.mode,
      })
    },
    [settings.mode]
  )

  const stageKey = (change: WorkingChange) => `${project.id}:${change.path}`
  const isStaged = (change: WorkingChange) =>
    staging[stageKey(change)] ?? change.staged
  const staged = changes.filter(isStaged).length

  /**
   * What an empty session offers. A project with issues offers its own, since
   * that is where a real session's subject comes from; one without falls back
   * to the three things you can ask of any codebase.
   */
  const suggestions = useMemo(() => {
    const issues = connected ? openIssues(project).slice(0, 3) : []
    return issues.length > 0 ? issues.map(suggestFromIssue) : SUGGESTIONS
  }, [connected, project])

  /**
   * The one thing the message goes out attached to. There is usually more
   * than one candidate — the issue the session came from, every file it has
   * written — and showing them all turns the composer's row into a list the
   * reader has to parse. The newest file wins, because it is what the next
   * message is almost always about; the issue stands in before there is one.
   */
  const reference = useMemo(() => {
    const last = written[written.length - 1]?.path ?? project.changes[0]?.path
    if (last) return last.split('/').pop() ?? last
    return session.issue ? `issue #${session.issue}` : null
  }, [session.issue, written, project.changes])

  /** The beats a run plays under the approval policy in force. */
  const gated = useCallback(
    (beats: Beat[]) =>
      settings.approval === 'auto'
        ? beats.filter(beat => beat.kind !== 'ask')
        : beats,
    [settings.approval]
  )

  const openSession = useCallback(
    (id: string) => {
      const next = project.sessions.find(entry => entry.id === id)
      if (!next) return
      setSessionId(id)
      setView('chat')
      setDraft('')
      player.reset(next.seed)
    },
    [project.sessions, player]
  )

  const openProject = useCallback(
    (id: string) => {
      const next = PROJECTS.find(entry => entry.id === id)
      if (!next) return
      const first = (opened[id] ?? [])[0] ?? next.sessions[0]
      setOpenIds(list => (list.includes(id) ? list : [...list, id]))
      setProjectId(id)
      setSessionId(first.id)
      setPullNumber(null)
      setMessage('')
      setDraft('')
      setView('chat')
      player.reset(first.seed)
    },
    [opened, player]
  )

  /**
   * A session from any open project, opened where it lives. Looked up on the
   * task rather than on the window's current project, since the project may
   * be the one being switched to.
   */
  const openTask = useCallback(
    (task: Task) => {
      setOpenIds(list =>
        list.includes(task.project.id) ? list : [...list, task.project.id]
      )
      setProjectId(task.project.id)
      setSessionId(task.session.id)
      setPullNumber(null)
      setDraft('')
      setView('chat')
      player.reset(task.session.seed)
    },
    [player]
  )

  /** Put a fresh session in front of the project's, and open it. */
  const addSession = useCallback(
    (fresh: Session) => {
      setOpened(map => ({
        ...map,
        [project.id]: [fresh, ...(map[project.id] ?? [])],
      }))
      setSessionId(fresh.id)
      setView('chat')
      setDraft('')
      player.reset([])
    },
    [project.id, player]
  )

  const newSession = useCallback(() => {
    addSession({
      id: `${project.id}-new-${Date.now()}`,
      title: 'New session',
      when: 'Now',
      status: 'running',
      tools: 0,
      branch: project.branch,
      steps: [],
      seed: [],
    })
  }, [project.id, project.branch, addSession])

  /**
   * The one move the issue list makes: an issue becomes a session. The sheet
   * confirms the worktree, the branch and the mode first; Return takes the
   * defaults, and S on the issue list skips the sheet altogether.
   */
  const startFromIssue = useCallback(
    (issue: Issue, plan: SessionPlan) => {
      setStarting(null)
      addSession({
        id: `${project.id}-issue-${issue.number}`,
        title: issue.title,
        when: 'Now',
        status: 'running',
        tools: 0,
        issue: issue.number,
        branch: plan.branch,
        worktree: plan.worktree,
        steps: [],
        seed: [],
      })
      player.start(
        '',
        gated(
          shape(
            issue.number === 142 ? TOOLTIP_RUN : issueRun(issue, project),
            plan.mode
          )
        ),
        { silent: true, askEvery: settings.approval === 'always' }
      )
    },
    [project, player, gated, settings.approval, addSession]
  )
  startFromIssueRef.current = startFromIssue

  /** A failed check becomes a session the same way an issue does. */
  const autoFix = useCallback(
    (target: Pull, run: CheckRun) => {
      addSession({
        id: `${project.id}-check-${target.number}-${slugOf(run.name)}`,
        title: `Fix ${run.name} on #${target.number}`,
        when: 'Now',
        status: 'running',
        tools: 0,
        branch: target.branch,
        pull: target.number,
        worktree: `worktrees/${target.number}-${slugOf(run.name)}`,
        steps: [],
        seed: [],
      })
      player.start('', gated(checkRun(target, run)), { silent: true })
    },
    [project.id, player, gated, addSession]
  )

  /** A review comment is the next message, sent from where it was written. */
  const comment = useCallback(
    (change: WorkingChange, text: string) => {
      setView('chat')
      player.start(
        `On ${change.path}: ${text}`,
        gated(commentRun(change.path, text)),
        { askEvery: settings.approval === 'always' }
      )
    },
    [player, gated, settings.approval]
  )

  /** What was accepted is staged, and the commit page is opened on it. */
  const commitAccepted = useCallback(
    (accepted: readonly WorkingChange[]) => {
      setStaging(map => ({
        ...map,
        ...Object.fromEntries(
          changes.map(change => [
            `${project.id}:${change.path}`,
            accepted.some(entry => entry.path === change.path),
          ])
        ),
      }))
      setMessage(
        session.issue
          ? `${session.title}\n\nCloses #${session.issue}.`
          : session.title
      )
      setView('git')
      say(
        `Staged ${accepted.length} ${accepted.length === 1 ? 'file' : 'files'}. The message is drafted; commit when it reads right.`
      )
    },
    [changes, project.id, session.issue, session.title, say]
  )

  const connect = useCallback(
    () => update({ connected: { ...settings.connected, [project.id]: true } }),
    [update, settings.connected, project.id]
  )

  const dismiss = useCallback(
    (id: string) => setDismissed(list => [...list, id]),
    []
  )

  const cleanWorktree = useCallback(
    (tree: Worktree) => {
      say(`Removed ${tree.path}.`)
      for (const notice of project.inbox) {
        if (notice.path === tree.path) dismiss(notice.id)
      }
    },
    [say, project.inbox, dismiss]
  )

  /** Every notice's verbs, resolved against what they are about. */
  const answerNotice = useCallback(
    (item: Attention, action: string) => {
      const { notice } = item
      if (item.project.id !== project.id) openProject(item.project.id)

      switch (action) {
        case 'Show both sides':
          setView('review')
          return
        case 'Open the pull request':
          showPull(notice.pull ?? null)
          return
        case 'Auto-fix': {
          const target = item.project.pulls.find(
            entry => entry.number === notice.pull
          )
          const run = target?.runs.find(entry => entry.status === 'failed')
          if (target && run) autoFix(target, run)
          dismiss(notice.id)
          return
        }
        case 'Clean up now':
          say(`Removed ${notice.path}.`)
          dismiss(notice.id)
          return
        case 'Pause the later one':
          say('Paused “Name the settings family after its root”.')
          dismiss(notice.id)
          return
        default:
          dismiss(notice.id)
      }
    },
    [project.id, openProject, autoFix, dismiss, say, showPull]
  )

  /* The theme is applied from the settings rather than by the control that
     changed it, so every way of changing it — the sheet, the palette, a
     reset, the system's own light changing under a System appearance — is
     one path. */
  useEffect(() => {
    applyTheme(settings.family, settings.appearance)
    if (settings.appearance !== 'system') return
    return watchSystem(() => applyTheme(settings.family, settings.appearance))
  }, [settings.family, settings.appearance])

  /* ⌘1–7 reach the pages in the order the sidebar lists them, ⌘0 the welcome
     screen, and the three the settings page prints. A window with nine pages
     needs them: without, every move is a trip to the sidebar. */
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (!(event.metaKey || event.ctrlKey)) return

      if (event.key === 'k') {
        event.preventDefault()
        setPalette(open => !open)
        return
      }
      if (event.key === 'p') {
        event.preventDefault()
        setView('welcome')
        return
      }
      if (event.key === 'n') {
        event.preventDefault()
        newSession()
        return
      }
      if (event.key === '0') {
        event.preventDefault()
        setView('welcome')
        return
      }
      /* ⌘, is what a desktop app answers to for settings, and it is what a
         browser keeps for itself — Chrome opens its own preferences before
         the page ever hears the key. ⌘9 is the page's own way in: the seven
         pages take ⌘1–7, so the ninth number is free and settings is what a
         reader would expect to find under it. */
      if (event.key === ',' || event.key === '9') {
        event.preventDefault()
        setSettingsOpen(open => !open)
        return
      }

      const index = Number(event.key)
      if (Number.isNaN(index) || index < 1 || index > VIEWS.length) return
      event.preventDefault()
      setView(VIEWS[index - 1])
    }

    document.addEventListener('keydown', onKey)
    return () => document.removeEventListener('keydown', onKey)
  }, [newSession])

  /** Everything the window can do, as the palette lists it. */
  const commands: Command[] = useMemo(() => {
    const glyphs = {
      chat: ChatGlyph,
      inbox: TrayGlyph,
      review: DiffGlyph,
      git: BranchGlyph,
      tests: FlaskGlyph,
      github: GithubGlyph,
      release: RocketShipGlyph,
      welcome: FolderOpenGlyph,
    } as const

    const labels: Record<View, string> = {
      chat: 'Chat',
      inbox: 'Inbox',
      review: 'Review changes',
      git: 'Git',
      tests: 'Tests',
      github: 'GitHub',
      release: 'Release',
      welcome: 'Welcome',
    }

    return [
      ...VIEWS.map((target, index) => ({
        id: `view-${target}`,
        group: 'Go to',
        label: labels[target],
        glyph: glyphs[target],
        keys: `⌘${index + 1}`,
        run: () => setView(target),
      })),
      {
        id: 'github-issues',
        group: 'GitHub',
        label: 'Issues',
        glyph: IssueGlyph,
        run: () => {
          setFace('issues')
          setView('github')
        },
      },
      {
        id: 'github-pulls',
        group: 'GitHub',
        label: 'Pull requests',
        glyph: PullGlyph,
        run: () => showPull(null),
      },
      {
        id: 'settings',
        group: 'Open',
        label: 'Settings',
        glyph: GearGlyph,
        keys: '⌘,',
        run: () => setSettingsOpen(true),
      },
      {
        id: 'session-new',
        group: 'Session',
        label: 'New session',
        glyph: PlusGlyph,
        keys: '⌘N',
        run: newSession,
      },
      ...sessions.map(entry => ({
        id: `session-${entry.id}`,
        group: 'Session',
        label: entry.title,
        glyph: ChatGlyph,
        run: () => openSession(entry.id),
      })),
      ...MODES.map(entry => ({
        id: `mode-${entry.value}`,
        group: 'Mode',
        label: `${entry.label} — ${entry.hint}`,
        glyph: SparkleGlyph,
        run: () => update({ mode: entry.value }),
      })),
      ...APPEARANCES.map(entry => ({
        id: `appearance-${entry.value}`,
        group: 'Appearance',
        label: entry.label,
        glyph: GearGlyph,
        run: () => update({ appearance: entry.value as Appearance }),
      })),
      ...FAMILIES.map(entry => ({
        id: `family-${entry.value}`,
        group: 'Theme',
        label: entry.label,
        glyph: GearGlyph,
        run: () => update({ family: entry.value as Family }),
      })),
      ...APPROVALS.map(entry => ({
        id: `approval-${entry.value}`,
        group: 'Permissions',
        label: entry.label,
        glyph: ShieldGlyph,
        run: () => update({ approval: entry.value }),
      })),
      ...PROJECTS.map(entry => ({
        id: `project-${entry.id}`,
        group: 'Project',
        label: shortName(entry.name),
        glyph: FolderOpenGlyph,
        run: () => openProject(entry.id),
      })),
    ]
  }, [sessions, newSession, openSession, openProject, update, showPull])

  const empty = (
    <div className="wb-empty">
      <EmptyState
        title={`What should the agent do in ${shortName(project.name)}?`}
      />

      <div
        aria-label="Suggestions"
        className="wb-empty__cards"
        role="radiogroup"
      >
        {suggestions.map(suggestion => (
          <OptionCard
            key={suggestion.prompt}
            description={suggestion.description}
            selected={draft === suggestion.prompt}
            title={suggestion.title}
            onClick={() => setDraft(suggestion.prompt)}
          />
        ))}
      </div>

      <Callout size="small" tint="info" title="Prototype">
        Every message replays the same scripted run. Nothing is sent anywhere,
        and no file is written.
      </Callout>
    </div>
  )

  const page = (() => {
    switch (view) {
      case 'chat':
        return (
          <>
            <ThreadView
              ask={player.ask}
              composer={
                <Composer
                  draft={draft}
                  mode={settings.mode}
                  model={settings.model}
                  reference={reference}
                  running={player.running}
                  onDraftChange={setDraft}
                  onModeChange={mode => update({ mode })}
                  onModelChange={model => update({ model })}
                  onSend={prompt =>
                    player.start(
                      prompt,
                      gated(shape(TOOLTIP_RUN, settings.mode)),
                      { askEvery: settings.approval === 'always' }
                    )
                  }
                  onStop={player.stop}
                />
              }
              empty={empty}
              entries={player.entries}
              session={{ ...session, status }}
              onAnswer={player.answer}
              onPause={() => (player.running ? player.stop() : undefined)}
              onReview={() => setView('review')}
              onToggle={player.toggle}
            />
            {settings.dock ? (
              <DockPanel
                changes={changes}
                entries={player.entries}
                pull={sessionPull}
                onOpenPull={() => showPull(sessionPull?.number ?? null)}
                onReview={() => setView('review')}
              />
            ) : null}
          </>
        )

      case 'inbox':
        return (
          <InboxView
            asking={asking}
            filter={inboxFilter}
            notices={inbox}
            onNotice={answerNotice}
            onOpen={openTask}
            onReview={task => {
              openTask(task)
              setView('review')
            }}
          />
        )

      case 'review':
        return (
          <ReviewView
            branch={session.branch ?? project.branch}
            changes={changes}
            pull={sessionPull?.number ?? null}
            onComment={comment}
            onCommit={commitAccepted}
            onDone={(accepted, reverted) =>
              say(
                reverted === 0
                  ? `Accepted ${accepted} ${accepted === 1 ? 'file' : 'files'}.`
                  : `Reverted ${reverted} ${reverted === 1 ? 'file' : 'files'}.`
              )
            }
          />
        )

      case 'git':
        return (
          <GitView
            changes={changes}
            isStaged={isStaged}
            issue={session.issue ?? null}
            message={message}
            project={project}
            onCleanWorktree={cleanWorktree}
            onCommit={() => {
              setMessage('')
              say(
                `Committed ${staged} ${staged === 1 ? 'file' : 'files'} on ${session.branch ?? project.branch}.`
              )
            }}
            onCommitPush={() => {
              setMessage('')
              say(
                `Committed ${staged} ${staged === 1 ? 'file' : 'files'} and pushed ${session.branch ?? project.branch} to origin.`
              )
            }}
            onNewWorktree={() =>
              say(
                'A worktree is created when a session opens; start one from an issue.'
              )
            }
            onMessageChange={setMessage}
            onOpenSession={openSession}
            onPull={() => say(`Pulled ${project.branch} — already up to date.`)}
            onPush={() =>
              say(
                `Pushed ${project.ahead} ${project.ahead === 1 ? 'commit' : 'commits'} to origin.`
              )
            }
            onStage={(change, next) =>
              setStaging(map => ({ ...map, [stageKey(change)]: next }))
            }
            onWriteMessage={() =>
              setMessage(
                'fix(tokens): lift the focus ring alpha on the dark canvas'
              )
            }
          />
        )

      case 'tests':
        return (
          <TestsView
            project={project}
            onFix={failure => {
              say(`A session is taking ${failure.suite}.`)
              setView('chat')
            }}
            onRun={what => say(`Running ${what}…`)}
          />
        )

      case 'github':
        return (
          <GithubView
            check={check}
            connected={connected}
            face={face}
            issue={issue}
            issueFilter={issueFilter}
            mergeMethod={settings.mergeMethod}
            mode={settings.mode}
            project={project}
            pull={pull}
            sessions={sessions}
            onAgentReply={target =>
              say(`The agent is drafting a reply on #${target.number}.`)
            }
            onAutoFix={autoFix}
            onConnect={connect}
            onLog={(target, run) =>
              say(`Opening the log of ${run.name} on #${target.number}.`)
            }
            onMerge={target => say(`Merged #${target.number}.`)}
            onMergeMethod={mergeMethod => update({ mergeMethod })}
            onOpenSession={openSession}
            onPlan={setStarting}
            onSelectCheck={setCheckId}
            onSelectIssue={setIssueNumber}
            onSelectPull={showPull}
            onStart={startNow}
          />
        )

      case 'release':
        return (
          <ReleaseView
            project={project}
            onChannel={(channel, action) => say(`${action} — ${channel.name}.`)}
            onRun={() => say(`Running the ${project.release?.active} release…`)}
          />
        )

      case 'welcome':
        return (
          <WelcomeView
            open={openIds}
            projects={PROJECTS}
            onOpen={openProject}
          />
        )
    }
  })()

  return (
    <div className="wb-root">
      <WindowFrame
        current={view === 'welcome' ? null : project}
        projects={projects}
        sidebar={
          view === 'welcome' ? undefined : (
            <ContextSidebar
              changes={changes}
              connected={connected}
              face={face}
              inbox={inbox}
              inboxFilter={inboxFilter}
              issueFilter={issueFilter}
              openSession={session.id}
              project={project}
              sessions={sessions}
              tasks={tasks}
              view={view}
              onFace={setFace}
              onInboxFilter={setInboxFilter}
              onIssueFilter={setIssueFilter}
              onNewSession={newSession}
              onOpenSession={openSession}
              onOpenSettings={() => setSettingsOpen(true)}
              onOpenView={setView}
            />
          )
        }
        statusBar={
          view === 'welcome' ? undefined : (
            <StatusBar
              approval={settings.approval}
              branch={session.branch ?? project.branch}
              changed={changes.length}
              ci={
                sessionPull
                  ? {
                      status: checksStatus(sessionPull),
                      pull: sessionPull.number,
                    }
                  : null
              }
              context={context}
              worktree={session.worktree ?? null}
              onApproval={approval => update({ approval })}
              onOpenPull={() => showPull(sessionPull?.number ?? null)}
            />
          )
        }
        onOpen={() => setView('welcome')}
        onPalette={() => setPalette(true)}
        onSelect={openProject}
      >
        {page}
      </WindowFrame>

      {settingsOpen ? (
        <SettingsDialog
          connected={entry => isConnected(settings, entry)}
          project={project}
          projects={projects}
          settings={settings}
          update={update}
          onClose={() => setSettingsOpen(false)}
          onConnect={(entry, next) =>
            update({ connected: { ...settings.connected, [entry.id]: next } })
          }
          onNotify={say}
          onReset={reset}
        />
      ) : null}

      {starting ? (
        <StartSessionDialog
          issue={starting}
          mode={settings.mode}
          project={project}
          onClose={() => setStarting(null)}
          onCreate={plan => startFromIssue(starting, plan)}
        />
      ) : null}

      {palette ? (
        <CommandPalette commands={commands} onClose={() => setPalette(false)} />
      ) : null}

      <ToastViewport />
    </div>
  )
}
