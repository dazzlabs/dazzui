import {
  ActionBar,
  Button,
  Dialog,
  DialogBody,
  DialogFooter,
  DialogHeader,
  FormField,
  Icon,
  KeyCap,
  SegmentedControl,
  TextField,
} from '@dazzlabs/dazzui-react'
import { type FormEvent, useState } from 'react'

import { IssueGlyph } from '../data/icons'
import type { Issue, Project } from '../lib/project'
import { type Mode, MODES } from '../lib/settings'

/** What a session opened from an issue starts with. */
export interface SessionPlan {
  worktree: string
  branch: string
  mode: Mode
}

/** The first three words of a title, as a branch would carry them. */
export function slugOf(title: string): string {
  return title
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .split('-')
    .filter(Boolean)
    .slice(0, 3)
    .join('-')
}

export interface StartSessionDialogProps {
  issue: Issue
  project: Project
  /** The mode the window is set to, which the sheet offers first. */
  mode: Mode
  onCreate: (plan: SessionPlan) => void
  onClose: () => void
}

/**
 * The confirmation between an issue and the session it becomes.
 *
 * Everything on it has a default — the worktree is named after the issue,
 * the branch is cut from the project's base, the mode is the window's — so
 * the common case is one keystroke: Return creates it. The sheet exists for
 * the other case, where the reader wants a different branch name or wants
 * the agent to only plan, and would otherwise have to open the session and
 * then undo what it did on its own.
 */
export function StartSessionDialog({
  issue,
  project,
  mode: initialMode,
  onCreate,
  onClose,
}: StartSessionDialogProps) {
  const slug = `${issue.number}-${slugOf(issue.title)}`
  const [worktree, setWorktree] = useState(`worktrees/${slug}`)
  const [branch, setBranch] = useState(`feat/${slug}`)
  const [mode, setMode] = useState<Mode>(initialMode)
  const chosen = MODES.find(entry => entry.value === mode)

  const submit = (event: FormEvent) => {
    event.preventDefault()
    if (!worktree.trim() || !branch.trim()) return
    onCreate({ worktree: worktree.trim(), branch: branch.trim(), mode })
  }

  return (
    <Dialog
      open
      className="wb-start"
      width="30rem"
      onOpenChange={open => {
        if (!open) onClose()
      }}
    >
      <form className="wb-start__form" onSubmit={submit}>
        <DialogHeader
          subtitle={
            <span className="wb-start__issue">
              <Icon icon={IssueGlyph} size={12} />#{issue.number} {issue.title}
            </span>
          }
          title="New session"
        />

        <DialogBody className="wb-start__body">
          <FormField
            hint="A checkout of its own, so it never collides with what another session is writing."
            label="Worktree"
          >
            <TextField
              autoFocus
              className="wb-mono"
              size="small"
              value={worktree}
              onChange={event => setWorktree(event.target.value)}
            />
          </FormField>

          <FormField hint={`Cut from ${project.branch}.`} label="Branch">
            <TextField
              className="wb-mono"
              size="small"
              value={branch}
              onChange={event => setBranch(event.target.value)}
            />
          </FormField>

          <FormField hint={chosen?.hint} label="Mode">
            <SegmentedControl
              aria-label="Mode"
              items={MODES.map(entry => ({
                value: entry.value,
                label: entry.label,
              }))}
              size="small"
              stretch
              value={mode}
              onValueChange={setMode}
            />
          </FormField>

          <span className="wb-card__quiet">
            The issue’s title, body and {issue.comments}{' '}
            {issue.comments === 1 ? 'comment' : 'comments'} go in as the first
            context.
          </span>
        </DialogBody>

        <DialogFooter>
          <ActionBar align="end">
            <Button
              size="small"
              tint="neutral"
              type="button"
              variant="plain"
              onClick={onClose}
            >
              Cancel
            </Button>
            <Button size="small" type="submit" variant="filled">
              Create
              <KeyCap size="small" variant="hint">
                ⏎
              </KeyCap>
            </Button>
          </ActionBar>
        </DialogFooter>
      </form>
    </Dialog>
  )
}
