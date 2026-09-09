import {
  Badge,
  Button,
  EmptyState,
  Icon,
  IconButton,
  KeyCap,
  TextField,
} from '@dazzlabs/dazzui-react'
import { useEffect, useState } from 'react'

import {
  ArrowRightGlyph,
  CheckGlyph,
  ChevronGlyph,
  CommentGlyph,
  SparkleGlyph,
  UndoGlyph,
} from '../data/icons'
import type { WorkingChange } from '../lib/project'
import { DiffView } from './diff-view'

/** What the reader has said about a file. Absent means they have not. */
type Verdict = 'accepted' | 'reverted'

export interface ReviewViewProps {
  branch: string
  changes: readonly WorkingChange[]
  /** The pull request the branch is on, which the commit can update. */
  pull: number | null
  onDone: (accepted: number, reverted: number) => void
  /** A note on a file, sent back to the agent as its next instruction. */
  onComment: (change: WorkingChange, text: string) => void
  /** Stage what was accepted and go on to commit it. */
  onCommit: (accepted: readonly WorkingChange[]) => void
}

/** True where the key was pressed in a field, and belongs to it. */
function inField(target: EventTarget | null): boolean {
  return (
    target instanceof HTMLElement &&
    (target.tagName === 'INPUT' ||
      target.tagName === 'TEXTAREA' ||
      target.isContentEditable)
  )
}

/**
 * Review changes: every file the agent has written, with a verdict on each.
 *
 * The unit is the file rather than the hunk, because that is the unit the
 * agent works in — it wrote a file to do a thing, and accepting half of it
 * leaves the thing half done. A file that has been decided folds shut and
 * fades, so what is left to read is always what is still open.
 *
 * A verdict is not the only answer. A comment on a file goes back to the
 * agent as its next instruction rather than into a list nobody reads — the
 * reader is already looking at the line they want changed, and the shortest
 * path from there to a changed line is to say so where they are.
 *
 * The keys — J and K between files, A, X and C on the one under the cursor —
 * are what make a twelve-file review a minute rather than a mouse trip per
 * file. The cursor is the file with the accent edge.
 */
export function ReviewView({
  branch,
  changes,
  pull,
  onDone,
  onComment,
  onCommit,
}: ReviewViewProps) {
  const [verdicts, setVerdicts] = useState<Record<string, Verdict>>({})
  const [open, setOpen] = useState<string[]>(() =>
    changes.slice(0, 1).map(change => change.path)
  )
  const [cursor, setCursor] = useState(0)
  /** The file whose comment box is open, and what is in it. */
  const [commenting, setCommenting] = useState<string | null>(null)
  const [note, setNote] = useState('')

  const added = changes.reduce((sum, change) => sum + change.added, 0)
  const removed = changes.reduce((sum, change) => sum + change.removed, 0)
  const pending = changes.filter(change => !verdicts[change.path])
  const accepted = changes.filter(
    change => verdicts[change.path] === 'accepted'
  )
  const reverted = changes.length - pending.length - accepted.length
  const current = changes[Math.min(cursor, changes.length - 1)]

  const decide = (path: string, verdict: Verdict) => {
    const at = changes.findIndex(change => change.path === path)
    setVerdicts(map => ({ ...map, [path]: verdict }))
    setOpen(list => list.filter(item => item !== path))
    // The cursor moves on to the next file still open, so A A A reads a
    // review to the end without a J in between.
    const next = changes.findIndex(
      (change, index) => index > at && !verdicts[change.path]
    )
    if (next !== -1) {
      const upcoming = changes[next].path
      setCursor(next)
      setOpen(list => (list.includes(upcoming) ? list : [...list, upcoming]))
    }
  }

  const decideAll = (verdict: Verdict) => {
    setVerdicts(
      Object.fromEntries(changes.map(change => [change.path, verdict]))
    )
    setOpen([])
    const count = verdict === 'accepted' ? changes.length : 0
    onDone(count, changes.length - count)
  }

  const toggle = (path: string) =>
    setOpen(list =>
      list.includes(path) ? list.filter(item => item !== path) : [...list, path]
    )

  const send = (change: WorkingChange) => {
    const text = note.trim()
    if (!text) return
    onComment(change, text)
    setNote('')
    setCommenting(null)
  }

  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.metaKey || event.ctrlKey || event.altKey) return
      if (inField(event.target)) return
      if (!current) return

      switch (event.key) {
        case 'j':
          event.preventDefault()
          setCursor(index => Math.min(changes.length - 1, index + 1))
          return
        case 'k':
          event.preventDefault()
          setCursor(index => Math.max(0, index - 1))
          return
        case 'a':
          event.preventDefault()
          if (!verdicts[current.path]) decide(current.path, 'accepted')
          return
        case 'x':
          event.preventDefault()
          if (!verdicts[current.path]) decide(current.path, 'reverted')
          return
        case 'c':
          event.preventDefault()
          setOpen(list =>
            list.includes(current.path) ? list : [...list, current.path]
          )
          setCommenting(current.path)
          return
        case 'Enter':
          event.preventDefault()
          toggle(current.path)
          return
      }
    }
    document.addEventListener('keydown', onKey)
    return () => document.removeEventListener('keydown', onKey)
  })

  return (
    <div className="wb-view">
      <header className="wb-view__head">
        <h1 className="wb-view__title">Review changes</h1>
        <Badge size="small" tint="neutral">
          {branch} · {changes.length} {changes.length === 1 ? 'file' : 'files'}{' '}
          · +{added} −{removed}
        </Badge>
        {changes.length > 0 ? (
          <span className="wb-card__quiet wb-review__tally">
            {accepted.length} accepted · {reverted} reverted · {pending.length}{' '}
            to read
          </span>
        ) : null}
        <span className="wb-view__gap" />
        <Button
          disabled={pending.length === 0}
          size="small"
          tint="neutral"
          variant="plain"
          onClick={() => decideAll('reverted')}
        >
          Revert all
        </Button>
        <Button
          disabled={pending.length === 0}
          size="small"
          variant="filled"
          onClick={() => decideAll('accepted')}
        >
          <Icon icon={CheckGlyph} size={14} />
          Accept all
        </Button>
      </header>

      <div className="wb-view__body">
        {changes.length === 0 ? (
          <EmptyState title="Nothing to review — the working tree matches the last commit." />
        ) : null}

        {changes.map((change, index) => {
          const verdict = verdicts[change.path]
          const shown = open.includes(change.path)
          const isCurrent = current?.path === change.path

          return (
            <section
              key={change.path}
              data-current={isCurrent || undefined}
              data-verdict={verdict}
              className="wb-file"
              onClick={() => setCursor(index)}
            >
              <header className="wb-file__head">
                <IconButton
                  active={shown}
                  className="wb-file__fold"
                  label={shown ? 'Collapse' : 'Expand'}
                  size="tiny"
                  tint="neutral"
                  variant="plain"
                  onClick={() => toggle(change.path)}
                >
                  <Icon icon={ChevronGlyph} size={13} />
                </IconButton>

                <span className="wb-truncate wb-mono wb-file__path">
                  {change.path}
                </span>
                <span className="wb-counts">
                  <span data-tint="success" className="wb-count">
                    +{change.added}
                  </span>
                  <span data-tint="danger" className="wb-count">
                    −{change.removed}
                  </span>
                </span>

                <span className="wb-view__gap" />

                {verdict ? (
                  <Badge
                    size="small"
                    tint={verdict === 'accepted' ? 'success' : 'neutral'}
                    variant="tinted"
                  >
                    {verdict === 'accepted' ? 'Accepted' : 'Reverted'}
                  </Badge>
                ) : (
                  <>
                    <Button
                      aria-pressed={commenting === change.path}
                      size="tiny"
                      tint="neutral"
                      variant={commenting === change.path ? 'tinted' : 'plain'}
                      onClick={() => {
                        setOpen(list =>
                          list.includes(change.path)
                            ? list
                            : [...list, change.path]
                        )
                        setCommenting(
                          commenting === change.path ? null : change.path
                        )
                      }}
                    >
                      <Icon icon={CommentGlyph} size={12} />
                      Comment
                    </Button>
                    <Button
                      size="tiny"
                      tint="neutral"
                      variant="plain"
                      onClick={() => decide(change.path, 'reverted')}
                    >
                      <Icon icon={UndoGlyph} size={12} />
                      Revert
                    </Button>
                    <Button
                      size="tiny"
                      onClick={() => decide(change.path, 'accepted')}
                    >
                      Accept
                    </Button>
                  </>
                )}
              </header>

              {shown ? (
                <div className="wb-file__body">
                  <DiffView lines={change.hunk} />

                  {/* The note goes under the diff it is about, and it is a
                      form: Return sends it, the way the composer does. */}
                  {commenting === change.path ? (
                    <form
                      className="wb-file__comment"
                      onSubmit={event => {
                        event.preventDefault()
                        send(change)
                      }}
                    >
                      <TextField
                        autoFocus
                        aria-label="Comment for the agent"
                        placeholder="Say what should change here… it goes to the agent as its next instruction"
                        size="small"
                        value={note}
                        onChange={event => setNote(event.target.value)}
                        onKeyDown={event => {
                          if (event.key === 'Escape') setCommenting(null)
                        }}
                      />
                      <Button
                        disabled={note.trim().length === 0}
                        size="small"
                        type="submit"
                        variant="filled"
                      >
                        <Icon icon={SparkleGlyph} size={13} />
                        Send to agent
                      </Button>
                      <Button
                        size="small"
                        tint="neutral"
                        type="button"
                        variant="plain"
                        onClick={() => setCommenting(null)}
                      >
                        Cancel
                      </Button>
                    </form>
                  ) : null}
                </div>
              ) : (
                <button
                  className="wb-file__folded"
                  type="button"
                  onClick={() => toggle(change.path)}
                >
                  {change.added} added, {change.removed} removed — expand
                </button>
              )}
            </section>
          )
        })}
      </div>

      {changes.length > 0 ? (
        <footer className="wb-review__foot">
          <span className="wb-review__keys">
            <KeyCap size="small" variant="key">
              J
            </KeyCap>
            <KeyCap size="small" variant="key">
              K
            </KeyCap>
            files
            <span aria-hidden>·</span>
            <KeyCap size="small" variant="key">
              A
            </KeyCap>
            accept
            <span aria-hidden>·</span>
            <KeyCap size="small" variant="key">
              X
            </KeyCap>
            revert
            <span aria-hidden>·</span>
            <KeyCap size="small" variant="key">
              C
            </KeyCap>
            comment
          </span>
          <span className="wb-view__gap" />
          <span className="wb-card__quiet">
            {accepted.length === 0
              ? 'Accept a file to stage it.'
              : `${accepted.length} ${accepted.length === 1 ? 'file' : 'files'} will be staged.`}
          </span>
          <Button
            disabled={accepted.length === 0}
            size="small"
            variant={pending.length === 0 ? 'filled' : 'outlined'}
            onClick={() => onCommit(accepted)}
          >
            {pull ? `Commit and update #${pull}` : 'Commit accepted files'}
            <Icon icon={ArrowRightGlyph} size={13} />
          </Button>
        </footer>
      ) : null}
    </div>
  )
}
