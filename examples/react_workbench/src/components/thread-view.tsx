import {
  Badge,
  Button,
  Callout,
  Icon,
  IconButton,
} from '@dazzlabs/dazzui-react'
import { type ReactNode, useEffect, useRef } from 'react'

import {
  CheckCircleGlyph,
  DiffGlyph,
  IssueGlyph,
  MoreGlyph,
  PauseGlyph,
  PlayGlyph,
} from '../data/icons'
import type { Answer, PendingAsk } from '../lib/player'
import type { Session } from '../lib/project'
import type { Entry } from '../lib/transcript'
import { PermissionCard } from './permission-card'
import { STATUS_LABEL } from './status-dot'
import { ToolCard } from './tool-card'

/**
 * How near the foot counts as being at the foot. Below this the column follows
 * the run; above it the reader has scrolled back on purpose and is left alone.
 */
const STICK = 80

/** The plan, as a checklist rather than a stepper: one line per step. */
function Plan({ steps }: { steps: Session['steps'] }) {
  return (
    <ul className="wb-plan">
      {steps.map(step => (
        <li
          key={step.label}
          data-status={step.status}
          className="wb-plan__step"
        >
          <span
            data-tint={step.status === 'done' ? 'success' : undefined}
            className="wb-plan__mark"
          >
            <Icon
              icon={step.status === 'done' ? CheckCircleGlyph : PlayGlyph}
              size={14}
            />
          </span>
          <span>{step.label}</span>
          {step.meta ? (
            <span className="wb-plan__meta">{step.meta}</span>
          ) : null}
        </li>
      ))}
    </ul>
  )
}

export interface ThreadViewProps {
  session: Session
  entries: readonly Entry[]
  /** The permission the run is blocked on, drawn where it came up. */
  ask: PendingAsk | null
  onAnswer: (answer: Answer) => void
  /** Shown instead of the column while the session is still empty. */
  empty: ReactNode
  /** The composer, pinned under the column. */
  composer: ReactNode
  onToggle: (id: string) => void
  onReview: () => void
  onPause: () => void
}

/**
 * The agent thread.
 *
 * What the person said is a bubble on the trailing edge; what the agent said
 * is the page's own text on the leading one. Only one of the two can be
 * ordinary prose, and it should be the longer one — so the agent's turns
 * carry no mark, no bubble and no avatar, and the eye reads them as the page
 * rather than as a chat log.
 */
export function ThreadView({
  session,
  entries,
  ask,
  onAnswer,
  empty,
  composer,
  onToggle,
  onReview,
  onPause,
}: ThreadViewProps) {
  const scroller = useRef<HTMLDivElement>(null)
  /** Whether the reader was at the foot when the last entry landed. */
  const following = useRef(true)

  useEffect(() => {
    const node = scroller.current
    if (!node || !following.current) return
    node.scrollTop = node.scrollHeight
  }, [entries, ask])

  return (
    <section className="wb-thread">
      <header className="wb-thread__head">
        <h1 className="wb-thread__title wb-truncate">{session.title}</h1>
        <Badge
          size="small"
          tint={session.status === 'waiting' ? 'warning' : 'primary'}
          variant="tinted"
        >
          {STATUS_LABEL[session.status]}
        </Badge>
        {session.issue ? (
          <Badge size="small" tint="neutral" variant="outlined">
            <Icon icon={IssueGlyph} size={11} />#{session.issue}
          </Badge>
        ) : null}

        <span className="wb-thread__gap" />

        <Button size="small" tint="neutral" variant="plain" onClick={onPause}>
          <Icon icon={PauseGlyph} size={14} />
          Pause
        </Button>
        <IconButton label="More" size="small" tint="neutral" variant="plain">
          <Icon icon={MoreGlyph} size={15} />
        </IconButton>
      </header>

      <div
        ref={scroller}
        className="wb-transcript"
        onScroll={event => {
          const node = event.currentTarget
          const left = node.scrollHeight - node.scrollTop - node.clientHeight
          following.current = left < STICK
        }}
      >
        {entries.length === 0 && !ask ? empty : null}

        {entries.map(entry => {
          switch (entry.kind) {
            case 'user':
              return (
                <p key={entry.id} className="wb-said">
                  {entry.text}
                </p>
              )

            case 'agent': {
              const typing = entry.shown < entry.text.length
              return (
                <p key={entry.id} className="wb-turn">
                  {entry.text.slice(0, entry.shown)}
                  {typing ? <span aria-hidden className="wb-caret" /> : null}
                </p>
              )
            }

            case 'context':
              return (
                <div key={entry.id} className="wb-turn wb-context">
                  <span className="wb-context__head">
                    <Icon icon={IssueGlyph} size={13} />
                    Context injected · issue #{entry.number}
                  </span>
                  <span className="wb-context__title">{entry.title}</span>
                  <p className="wb-context__body">{entry.body}</p>
                  {entry.meta ? (
                    <span className="wb-context__meta">{entry.meta}</span>
                  ) : null}
                </div>
              )

            case 'plan':
              return (
                <div key={entry.id} className="wb-turn">
                  <Plan steps={entry.steps} />
                </div>
              )

            case 'tool':
              return (
                <ToolCard
                  key={entry.id}
                  entry={entry}
                  onToggle={() => onToggle(entry.id)}
                />
              )

            case 'review':
              return (
                <div key={entry.id} className="wb-turn wb-review-note">
                  <span className="wb-review-note__line">
                    <Icon icon={DiffGlyph} size={14} />
                    The change is ready to read.
                  </span>
                  <Button size="small" variant="outlined" onClick={onReview}>
                    Review changes
                  </Button>
                </div>
              )

            case 'allowed':
              return (
                <p key={entry.id} className="wb-turn wb-allowed">
                  <span data-tint="success" className="wb-mark">
                    <Icon icon={CheckCircleGlyph} size={14} />
                  </span>
                  Allowed <code className="wb-mono">{entry.target}</code>
                  <span data-tint="success" className="wb-allowed__note">
                    {entry.note}
                  </span>
                </p>
              )

            case 'notice':
              return (
                <div key={entry.id} className="wb-turn">
                  <Callout size="small" tint={entry.tint} title={entry.title}>
                    {entry.body}
                  </Callout>
                </div>
              )

            case 'divider':
              return (
                <div key={entry.id} className="wb-rule">
                  {entry.text}
                </div>
              )
          }
        })}

        {ask ? <PermissionCard ask={ask} onAnswer={onAnswer} /> : null}
      </div>

      {composer}
    </section>
  )
}
