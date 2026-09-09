import {
  Badge,
  Button,
  Callout,
  Card,
  EmptyState,
  Icon,
} from '@dazzlabs/dazzui-react'

import {
  ArchiveGlyph,
  ArrowRightGlyph,
  DiffGlyph,
  WarningGlyph,
} from '../data/icons'
import {
  type Attention,
  type InboxFilter,
  type Notice,
  type Task,
} from '../lib/project'
import { STATUS_LABEL, StatusDot } from './status-dot'

/** What a notice offers, which follows from what kind of thing it is. */
export function noticeActions(notice: Notice): string[] {
  switch (notice.kind) {
    case 'conflict':
      return ['Show both sides', 'Pause the later one', 'Ignore']
    case 'archived':
      return ['Clean up now', 'Keep it']
    case 'checks':
      return ['Auto-fix', 'Open the pull request']
  }
}

export interface InboxViewProps {
  /** Every session across the open projects that is asking for a person. */
  asking: readonly Task[]
  /** What the window has noticed across the open projects. */
  notices: readonly Attention[]
  /** Which slice the sidebar has selected. */
  filter: InboxFilter
  onOpen: (task: Task) => void
  onReview: (task: Task) => void
  onNotice: (item: Attention, action: string) => void
}

/**
 * The inbox: everything across the open projects that is waiting on a person,
 * in one list, newest reason first.
 *
 * Every workbench that runs agents in parallel grows one of these — a review
 * queue, a triage list, an inbox — and for the same reason: the sessions are
 * where the work is, but which of them needs *you* is a question no session
 * can answer for itself. Three kinds of thing land here. A session blocked
 * on an answer, a session that has finished and is holding a diff nobody has
 * read, and a notice the window raised on its own — two sessions about to
 * collide, a worktree left behind by a merged pull request. Each row carries
 * the one move that clears it.
 */
export function InboxView({
  asking,
  notices,
  filter,
  onOpen,
  onReview,
  onNotice,
}: InboxViewProps) {
  const waiting = asking.filter(task => task.session.status === 'waiting')
  const review = asking.filter(task => task.session.status === 'review')
  const total = asking.length + notices.length

  const showWaiting = filter === 'all' || filter === 'waiting'
  const showReview = filter === 'all' || filter === 'review'
  const showNotices = filter === 'all' || filter === 'notices'

  const shown =
    (showWaiting ? waiting.length : 0) +
    (showReview ? review.length : 0) +
    (showNotices ? notices.length : 0)

  const row = (task: Task, verb: 'answer' | 'review') => {
    const { session, project } = task
    return (
      <Card key={`${project.id}-${session.id}`} className="wb-inbox__row">
        <StatusDot size="small" status={session.status} />
        <span className="wb-inbox__body">
          <span className="wb-truncate wb-card__title">{session.title}</span>
          <span
            data-tint={verb === 'answer' ? 'warning' : undefined}
            className="wb-inbox__why"
          >
            {verb === 'answer'
              ? `Waiting for approval · ${session.blockedOn ?? 'a permission'}`
              : `${STATUS_LABEL[session.status]} · ${session.when}`}
          </span>
        </span>
        <Badge size="small" tint="neutral">
          {project.name.split('/').pop()}
        </Badge>
        {session.worktree ? (
          <span className="wb-task__where wb-mono">{session.worktree}</span>
        ) : null}
        <span className="wb-view__gap" />
        {verb === 'review' ? (
          <Button size="small" onClick={() => onReview(task)}>
            <Icon icon={DiffGlyph} size={13} />
            Review the diff
          </Button>
        ) : null}
        <Button
          size="small"
          tint="neutral"
          variant={verb === 'review' ? 'plain' : 'outlined'}
          onClick={() => onOpen(task)}
        >
          Open session
          <Icon icon={ArrowRightGlyph} size={12} />
        </Button>
      </Card>
    )
  }

  return (
    <div className="wb-view">
      <header className="wb-view__head">
        <h1 className="wb-view__title">Inbox</h1>
        {waiting.length > 0 ? (
          <Badge size="small" tint="warning" variant="tinted">
            {waiting.length} {waiting.length === 1 ? 'needs' : 'need'} an answer
          </Badge>
        ) : null}
        {review.length > 0 ? (
          <Badge size="small" variant="tinted">
            {review.length} to review
          </Badge>
        ) : null}
        {notices.length > 0 ? (
          <Badge size="small" tint="neutral">
            {notices.length} {notices.length === 1 ? 'notice' : 'notices'}
          </Badge>
        ) : null}
        <span className="wb-view__gap" />
        <span className="wb-card__quiet">
          {total === 0
            ? 'Nothing is waiting on you.'
            : 'Across every open project, oldest first.'}
        </span>
      </header>

      <div className="wb-view__body">
        {shown === 0 ? (
          <EmptyState
            title={
              total === 0
                ? 'Nothing is waiting on you. Every session is either working or done.'
                : 'Nothing in this slice of the inbox.'
            }
          />
        ) : null}

        {showWaiting && waiting.length > 0 ? (
          <section className="wb-inbox__section">
            <span className="wb-label">Needs an answer · {waiting.length}</span>
            {waiting.map(task => row(task, 'answer'))}
          </section>
        ) : null}

        {showReview && review.length > 0 ? (
          <section className="wb-inbox__section">
            <span className="wb-label">Ready to review · {review.length}</span>
            {review.map(task => row(task, 'review'))}
          </section>
        ) : null}

        {showNotices && notices.length > 0 ? (
          <section className="wb-inbox__section">
            <span className="wb-label">Notices · {notices.length}</span>
            {notices.map(item => (
              <Callout
                key={item.notice.id}
                action={
                  <span className="wb-inbox__actions">
                    {noticeActions(item.notice).map((action, index) => (
                      <Button
                        key={action}
                        size="tiny"
                        tint={index === 0 ? 'primary' : 'neutral'}
                        variant={index === 0 ? 'tinted' : 'plain'}
                        onClick={() => onNotice(item, action)}
                      >
                        {action}
                      </Button>
                    ))}
                  </span>
                }
                icon={
                  <Icon
                    icon={
                      item.notice.kind === 'archived'
                        ? ArchiveGlyph
                        : WarningGlyph
                    }
                    size={15}
                  />
                }
                size="medium"
                tint={item.notice.kind === 'archived' ? 'neutral' : 'warning'}
                title={item.notice.title}
              >
                {item.notice.body}
                {item.notice.path ? (
                  <>
                    {' '}
                    <code className="wb-mono">{item.notice.path}</code>
                  </>
                ) : null}
                <span className="wb-inbox__where">
                  {' '}
                  · {item.project.name.split('/').pop()}
                </span>
              </Callout>
            ))}
          </section>
        ) : null}
      </div>
    </div>
  )
}
