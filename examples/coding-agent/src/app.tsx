import {
  AlertBanner,
  Button,
  EmptyState,
  Icon,
  OptionCard,
  Toast,
  ToastViewport,
} from '@dazzlabs/dazzui-react'
import { useMemo, useState } from 'react'

import { ChangesView } from './components/changes-view'
import { Composer } from './components/composer'
import { DetailAside } from './components/detail-aside'
import { IssuesView } from './components/issues-view'
import { PermissionDialog } from './components/permission-dialog'
import { ProjectRail } from './components/project-rail'
import { ProjectSidebar } from './components/project-sidebar'
import { PullsView } from './components/pulls-view'
import { SettingsRail } from './components/settings-rail'
import { SettingsView } from './components/settings-view'
import { TranscriptView } from './components/transcript-view'
import { WindowFrame } from './components/window-frame'
import { CheckGlyph } from './data/icons'
import { PROJECTS } from './data/projects'
import { SUGGESTIONS, suggestFromIssue, TOOLTIP_RUN } from './data/run'
import { usePlayer } from './lib/player'
import {
  type Issue,
  openIssues,
  type ProjectPane,
  type Pull,
  type SettingsSection,
  type View,
  workingTree,
  type WorkingChange,
} from './lib/project'
import { isConnected, useSettings } from './lib/settings'
import { applyTheme, type Theme } from './lib/theme'
import { changedFiles, estimateTokens } from './lib/transcript'

/** The initials on the mark beside the person's own turns. */
const INITIALS = 'LJ'

export function App() {
  const { settings, update, reset } = useSettings()
  const [projectId, setProjectId] = useState(PROJECTS[0].id)
  const [view, setView] = useState<View>({
    kind: 'session',
    id: PROJECTS[0].sessions[0].id,
  })
  const [sidebarOpen, setSidebarOpen] = useState(true)
  const [draft, setDraft] = useState('')
  const [issueNumber, setIssueNumber] = useState<number | null>(null)
  const [pullNumber, setPullNumber] = useState<number | null>(null)
  const [message, setMessage] = useState('')
  /** Staging overrides, keyed `project:path`; absent means the fixture decides. */
  const [staging, setStaging] = useState<Record<string, boolean>>({})

  const project = PROJECTS.find(entry => entry.id === projectId) ?? PROJECTS[0]
  const player = usePlayer(project.sessions[0].seed)
  const connected = isConnected(settings, project)

  const written = useMemo(() => changedFiles(player.entries), [player.entries])
  const changes = useMemo(
    () => workingTree(project, written),
    [project, written]
  )
  const tokens = useMemo(() => estimateTokens(player.entries), [player.entries])

  /**
   * What an empty session offers. A project with issues offers its own, since
   * that is where a real session's subject comes from; one without falls back
   * to the three things you can ask of any codebase.
   */
  const suggestions = useMemo(() => {
    const issues = connected ? openIssues(project).slice(0, 3) : []
    return issues.length > 0 ? issues.map(suggestFromIssue) : SUGGESTIONS
  }, [connected, project])

  const stageKey = (change: WorkingChange) => `${project.id}:${change.path}`
  const isStaged = (change: WorkingChange) =>
    staging[stageKey(change)] ?? change.staged
  const staged = changes.filter(isStaged).length

  const chooseTheme = (theme: Theme) => {
    applyTheme(theme)
    update({ theme })
  }

  const restore = () => {
    applyTheme('studio-light')
    reset()
  }

  const openSession = (id: string) => {
    const next = project.sessions.find(session => session.id === id)
    if (!next) return
    setView({ kind: 'session', id })
    setDraft('')
    player.reset(next.seed)
  }

  const openProject = (id: string) => {
    const next = PROJECTS.find(entry => entry.id === id)
    if (!next) return
    setProjectId(id)
    setView({ kind: 'session', id: next.sessions[0].id })
    setIssueNumber(null)
    setPullNumber(null)
    setMessage('')
    setDraft('')
    player.reset(next.sessions[0].seed)
  }

  const openPane = (pane: ProjectPane) => setView({ kind: 'pane', pane })

  const openSettings = (section: SettingsSection) =>
    setView({ kind: 'settings', section })

  /** Both list panes hand their subject to a fresh session, pre-addressed. */
  const startFrom = (subject: string) => {
    setView({ kind: 'session', id: project.sessions[0].id })
    player.reset([])
    setDraft(subject)
  }

  return (
    <div className="ca-root">
      <WindowFrame
        ahead={project.ahead}
        behind={project.behind}
        branch={project.branch}
        project={project.name}
        sidebar={
          sidebarOpen ? (
            <ProjectSidebar
              active={project}
              changed={changes.length}
              projects={PROJECTS}
              settingsOpen={view.kind === 'settings'}
              onOpenSettings={() => openSettings('general')}
              onSelect={openProject}
            />
          ) : null
        }
        sidebarOpen={sidebarOpen}
        theme={settings.theme}
        onSidebarToggle={() => setSidebarOpen(open => !open)}
        onThemeChange={chooseTheme}
      >
        {view.kind === 'settings' ? (
          <SettingsRail
            section={view.section}
            onClose={() => openSession(project.sessions[0].id)}
            onOpen={openSettings}
          />
        ) : (
          <ProjectRail
            changed={changes.length}
            connected={connected}
            project={project}
            view={view}
            onNewSession={() => {
              setView({ kind: 'session', id: project.sessions[0].id })
              setDraft('')
              player.reset([])
            }}
            onOpenPane={openPane}
            onOpenSession={openSession}
          />
        )}

        <main className="ca-main">
          {view.kind === 'settings' ? (
            <SettingsView
              projects={PROJECTS}
              section={view.section}
              settings={settings}
              update={update}
              onConnect={(id, next) =>
                update({ connected: { ...settings.connected, [id]: next } })
              }
              onThemeChange={chooseTheme}
            />
          ) : view.kind === 'session' ? (
            <>
              <TranscriptView
                entries={player.entries}
                initials={INITIALS}
                empty={
                  <div className="ca-empty">
                    <EmptyState
                      title={`What should Forge do in ${project.name}?`}
                    />

                    <div
                      aria-label="Suggestions"
                      className="ca-empty__cards"
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

                    <AlertBanner size="small" tint="info" title="Prototype">
                      Every message replays the same scripted run. Nothing is
                      sent anywhere, and no file is written.
                    </AlertBanner>
                  </div>
                }
                onToggle={player.toggle}
              />

              <Composer
                asks={settings.askBeforeWrite}
                draft={draft}
                mode={settings.mode}
                model={settings.model}
                running={player.running}
                onDraftChange={setDraft}
                onModeChange={mode => update({ mode })}
                onModelChange={model => update({ model })}
                onSend={prompt =>
                  player.start(
                    prompt,
                    // The permission sheet is a setting, so turning it off has
                    // to take the beat out of the run rather than answer it.
                    settings.askBeforeWrite
                      ? TOOLTIP_RUN
                      : TOOLTIP_RUN.filter(beat => beat.kind !== 'ask')
                  )
                }
                onStop={player.stop}
              />
            </>
          ) : view.pane === 'issues' ? (
            <IssuesView
              connected={connected}
              project={project}
              selected={issueNumber}
              onConnect={() =>
                update({
                  connected: { ...settings.connected, [project.id]: true },
                })
              }
              onSelect={(issue: Issue) => setIssueNumber(issue.number)}
            />
          ) : view.pane === 'pulls' ? (
            <PullsView
              connected={connected}
              project={project}
              selected={pullNumber}
              onConnect={() =>
                update({
                  connected: { ...settings.connected, [project.id]: true },
                })
              }
              onSelect={(pull: Pull) => setPullNumber(pull.number)}
            />
          ) : (
            <ChangesView
              branch={project.branch}
              changes={changes}
              isStaged={isStaged}
              onCommit={() => setMessage('')}
              onStage={(change, next) =>
                setStaging(map => ({ ...map, [stageKey(change)]: next }))
              }
            />
          )}
        </main>

        <DetailAside
          issue={
            project.issues.find(entry => entry.number === issueNumber) ?? null
          }
          message={message}
          project={project}
          pull={
            project.pulls.find(entry => entry.number === pullNumber) ?? null
          }
          settings={settings}
          staged={staged}
          tokens={tokens}
          trusted={player.trusted}
          update={update}
          view={view}
          onCommit={() => setMessage('')}
          onMessageChange={setMessage}
          onReset={restore}
          onStartFromIssue={issue =>
            startFrom(`Fix #${issue.number} — ${issue.title}`)
          }
          onStartFromPull={pull =>
            startFrom(`Review #${pull.number} — ${pull.title}`)
          }
        />
      </WindowFrame>

      {player.ask ? (
        <PermissionDialog ask={player.ask} onAnswer={player.answer} />
      ) : null}

      <ToastViewport>
        {player.toast ? (
          <Toast
            action={
              <Button
                size="tiny"
                variant="plain"
                onClick={() => {
                  player.dismissToast()
                  openPane('changes')
                }}
              >
                Review
              </Button>
            }
            icon={<Icon icon={CheckGlyph} size={16} />}
            tint="success"
            onDismiss={player.dismissToast}
          >
            {player.toast}
          </Toast>
        ) : null}
      </ToastViewport>
    </div>
  )
}
