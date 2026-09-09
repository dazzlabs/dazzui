import {
  ActionBar,
  Badge,
  Button,
  Card,
  EmptyState,
  Icon,
  Menu,
  Spinner,
  TextField,
} from '@dazzlabs/dazzui-react'
import { useState } from 'react'

import {
  ArrowRightGlyph,
  BranchGlyph,
  CheckCircleGlyph,
  ClockGlyph,
  CloudGlyph,
  CrossCircleGlyph,
  ExternalGlyph,
  SparkleGlyph,
} from '../data/icons'
import {
  type CheckRun,
  checksStatus,
  type Project,
  type Pull,
  type RunStatus,
} from '../lib/project'
import { MERGE_METHODS } from '../lib/settings'

/** The light beside a check, in the ramp its state belongs to. */
export function RunLight({ status }: { status: RunStatus }) {
  if (status === 'running') return <Spinner size="small" tint="warning" />

  return (
    <span
      data-tint={
        status === 'passed'
          ? 'success'
          : status === 'failed'
            ? 'danger'
            : undefined
      }
      className="wb-mark"
    >
      <Icon
        icon={
          status === 'passed'
            ? CheckCircleGlyph
            : status === 'failed'
              ? CrossCircleGlyph
              : ClockGlyph
        }
        size={14}
      />
    </span>
  )
}

/**
 * The empty state a page about the provider shows when there is no provider
 * to read from. Two cases, and they are not the same one: a project with no
 * remote has nowhere to read issues from, and a project whose account has
 * been disconnected has somewhere and no key.
 */
export function ProviderEmpty({
  project,
  what,
  onConnect,
}: {
  project: Project
  /** `issues` or `pull requests`, for the sentence. */
  what: string
  onConnect: () => void
}) {
  if (project.provider === null) {
    return (
      <EmptyState
        action={
          <Button size="small" tint="neutral" variant="outlined">
            <Icon icon={CloudGlyph} size={14} />
            Publish to GitHub…
          </Button>
        }
        title={`${project.name} has no remote, so there are no ${what} to read. Git still works; push it somewhere to see them here.`}
      />
    )
  }

  return (
    <EmptyState
      action={
        <span className="wb-connect">
          <Button size="small" variant="filled" onClick={onConnect}>
            <Icon icon={CloudGlyph} size={14} />
            Connect GitHub…
          </Button>
          <span className="wb-card__quiet">
            Through gh auth login · needs the repo and checks scopes
          </span>
        </span>
      }
      title={`GitHub isn’t connected, so the ${what} on ${project.name} can’t be read. Everything local still works.`}
    />
  )
}

export interface PullDetailProps {
  project: Project
  pull: Pull
  mergeMethod: string
  onMergeMethod: (method: string) => void
  onMerge: (pull: Pull) => void
  onAgentReply: (pull: Pull) => void
  /** Hand a failed check to the agent. */
  onAutoFix: (pull: Pull, run: CheckRun) => void
  onLog: (pull: Pull, run: CheckRun) => void
}

/**
 * One pull request, whole: what it is against, whether it is green, and who
 * has said what about it.
 *
 * It is the detail pane of the GitHub page, beside the list — a pull request
 * is a thing you read rather than scan. The merge button sits at the foot
 * with the reason it is disabled beside it, so the pane always answers "can
 * this go in yet" without the reader hunting for why not.
 */
export function PullDetail({
  project,
  pull,
  mergeMethod,
  onMergeMethod,
  onMerge,
  onAgentReply,
  onAutoFix,
  onLog,
}: PullDetailProps) {
  const [comment, setComment] = useState('')
  const checks = checksStatus(pull)
  const method = MERGE_METHODS.find(entry => entry.value === mergeMethod)
  const blocked = checks === 'running' || checks === 'failed'

  return (
    <>
      <header className="wb-gh__head">
        <h2 className="wb-gh__title wb-truncate">
          #{pull.number} {pull.title}
        </h2>
        <Badge
          size="small"
          tint={pull.state === 'merged' ? 'primary' : 'info'}
          variant="tinted"
        >
          {pull.state}
        </Badge>
        <span className="wb-view__gap" />
        <Button size="small" tint="neutral" variant="plain">
          Open on GitHub
          <Icon icon={ExternalGlyph} size={13} />
        </Button>
      </header>

      <span className="wb-facts">
        <Badge size="small" tint="neutral">
          <Icon icon={BranchGlyph} size={11} />
          {pull.branch}
        </Badge>
        <Icon icon={ArrowRightGlyph} size={14} />
        <Badge size="small" tint="neutral">
          {pull.base}
        </Badge>
        <span>
          {pull.commits} commits · {pull.files} files · +{pull.added} −
          {pull.removed}
          {pull.fixes ? ` · closes #${pull.fixes}` : ''}
        </span>
      </span>

      <Card size="large" variant="raised">
        <span className="wb-label">Checks</span>
        <ul className="wb-rows">
          {pull.runs.map(run => (
            <li key={run.name}>
              <RunLight status={run.status} />
              <span className="wb-mono">{run.name}</span>
              {run.note ? (
                <span className="wb-card__quiet">— {run.note}</span>
              ) : null}
              <span className="wb-rows__meta">
                {run.status === 'queued' ? 'waiting for a runner' : run.took}
              </span>
              {/* A failed check is a thing to hand over, not only to read:
                  the agent opens a session on the log, the way it does on
                  an issue. */}
              {run.status === 'failed' ? (
                <span className="wb-check__actions">
                  <Button size="tiny" onClick={() => onAutoFix(pull, run)}>
                    <Icon icon={SparkleGlyph} size={12} />
                    Auto-fix
                  </Button>
                  <Button
                    size="tiny"
                    tint="neutral"
                    variant="plain"
                    onClick={() => onLog(pull, run)}
                  >
                    Log
                  </Button>
                </span>
              ) : null}
            </li>
          ))}
        </ul>
      </Card>

      <Card size="large" variant="raised">
        <span className="wb-label">Timeline</span>

        {pull.timeline.map((event, index) => {
          const session = project.sessions.find(
            entry => entry.id === event.session
          )
          return (
            <span key={index} className="wb-event">
              <span aria-hidden className="wb-event__avatar">
                {event.initials}
              </span>
              <span className="wb-event__body">
                <span className="wb-card__note">
                  <strong>{event.author}</strong> {event.body} · {event.when}
                </span>
                {session ? (
                  <span className="wb-card__quiet">
                    Written by the agent, from the session “{session.title}”
                  </span>
                ) : null}
              </span>
            </span>
          )
        })}

        <span className="wb-event__reply">
          <TextField
            aria-label="Comment"
            className="wb-event__field"
            placeholder="Write a comment, or let the agent answer…"
            size="small"
            value={comment}
            onChange={event => setComment(event.target.value)}
          />
          <Button
            size="small"
            variant="plain"
            onClick={() => {
              setComment('')
              onAgentReply(pull)
            }}
          >
            <Icon icon={SparkleGlyph} size={13} />
            Agent replies
          </Button>
        </span>
      </Card>

      <ActionBar align="start">
        <Button
          disabled={blocked || pull.state === 'merged'}
          variant="filled"
          onClick={() => onMerge(pull)}
        >
          Merge pull request
        </Button>
        <Menu
          align="start"
          items={MERGE_METHODS.map(entry => ({
            label: entry.label,
            checked: entry.value === mergeMethod,
            onSelect: () => onMergeMethod(entry.value),
          }))}
          trigger={
            <Button size="medium" tint="neutral" variant="outlined">
              {method?.label ?? 'Squash'} merge
            </Button>
          }
        />
        <span className="wb-card__quiet">
          {pull.state === 'merged'
            ? 'Already merged.'
            : checks === 'failed'
              ? 'A check failed — the tests page has the output.'
              : checks === 'running'
                ? 'Waiting for the running check to finish.'
                : 'Every check is green.'}
        </span>
      </ActionBar>
    </>
  )
}
