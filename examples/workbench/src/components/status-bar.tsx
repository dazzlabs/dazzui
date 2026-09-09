import { Button, Icon, Menu, Progress } from '@dazzlabs/dazzui-react'

import {
  BranchGlyph,
  ChevronGlyph,
  ShieldGlyph,
  TreeGlyph,
} from '../data/icons'
import type { RunStatus } from '../lib/project'
import { type Approval, APPROVALS } from '../lib/settings'
import { RunLight } from './pull-view'

/** Where the meter turns: suggest compacting, then compact on its own. */
const CONTEXT_WARN = 75
const CONTEXT_HIGH = 90

export interface StatusBarProps {
  branch: string
  /** How many files the working tree differs by. */
  changed: number
  /** The worktree the open session runs in; null means the main checkout. */
  worktree: string | null
  /** The state of the checks on the open session's pull request, if any. */
  ci: { status: RunStatus; pull: number } | null
  onOpenPull: () => void
  /** How full the context window is, 0–100. */
  context: number
  approval: Approval
  onApproval: (approval: Approval) => void
}

/** What the CI light says, in two words. */
function ciLabel(status: RunStatus): string {
  switch (status) {
    case 'passed':
      return 'CI green'
    case 'failed':
      return 'CI failed'
    case 'running':
      return 'CI running'
    case 'queued':
      return 'CI queued'
    case 'skipped':
      return 'CI skipped'
  }
}

/**
 * The status bar: one line across the foot of the window with what the open
 * session stands on — its branch, its worktree, its checks — and the two
 * numbers that govern what it can do next: how much context is left, and
 * what it is allowed to do without asking.
 *
 * It is set in the code face because every word on it is an identifier, and
 * it is the one place the permission mode can be changed without opening a
 * sheet: a reader who has just been asked three times in a row wants the
 * switch where the asking happens, not two pages away.
 */
export function StatusBar({
  branch,
  changed,
  worktree,
  ci,
  onOpenPull,
  context,
  approval,
  onApproval,
}: StatusBarProps) {
  const policy = APPROVALS.find(entry => entry.value === approval)
  const tint =
    context >= CONTEXT_HIGH
      ? 'danger'
      : context >= CONTEXT_WARN
        ? 'warning'
        : 'primary'

  return (
    <footer className="wb-status">
      <span className="wb-status__item" data-accent>
        <Icon icon={BranchGlyph} size={12} />
        {branch}
      </span>
      <span
        className="wb-status__item"
        title="Files changed in the working tree"
      >
        ±{changed}
      </span>
      <span className="wb-status__item">
        <Icon icon={TreeGlyph} size={12} />
        {worktree ?? 'main checkout'}
      </span>
      {ci ? (
        <button
          className="wb-status__item wb-status__button"
          title={`Checks on #${ci.pull}`}
          type="button"
          onClick={onOpenPull}
        >
          <RunLight status={ci.status} />
          {ciLabel(ci.status)} · #{ci.pull}
        </button>
      ) : null}

      <span className="wb-status__gap" />

      <span
        className="wb-status__item wb-status__context"
        title={
          context >= CONTEXT_HIGH
            ? 'The history will be compacted on the next message'
            : context >= CONTEXT_WARN
              ? 'Worth compacting the history soon'
              : 'How full the context window is'
        }
      >
        Context {context}%
        <Progress
          aria-label="Context window"
          className="wb-status__meter"
          size="small"
          tint={tint}
          value={context}
        />
        {context >= CONTEXT_WARN ? (
          <span data-tint={tint} className="wb-status__note">
            {context >= CONTEXT_HIGH ? 'compacting next' : 'compact soon'}
          </span>
        ) : null}
      </span>

      <Menu
        align="end"
        items={APPROVALS.map(entry => ({
          label: entry.label,
          checked: entry.value === approval,
          onSelect: () => onApproval(entry.value),
        }))}
        side="top"
        trigger={
          <Button
            className="wb-status__policy"
            size="tiny"
            tint="neutral"
            variant="plain"
          >
            <Icon icon={ShieldGlyph} size={12} />
            Permissions: {policy?.label ?? approval}
            <Icon icon={ChevronGlyph} size={10} className="wb-status__caret" />
          </Button>
        }
      />
    </footer>
  )
}
