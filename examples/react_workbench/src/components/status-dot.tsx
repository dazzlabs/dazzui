import { Icon } from '@dazzlabs/dazzui-react'

import {
  CheckGlyph,
  ClockGlyph,
  DiffGlyph,
  EyeGlyph,
  HandGlyph,
  PauseGlyph,
} from '../data/icons'
import type { SessionStatus } from '../lib/project'

export interface StatusDotProps {
  status: SessionStatus
  /** `small` is the dot a tab or a row carries; `medium` heads a page. */
  size?: 'small' | 'medium'
}

/**
 * What a session's state looks like, in one mark.
 *
 * Running is the only one that moves, and it is a plain dot rather than a
 * glyph — the pulse is what says "now", and a glyph beside it would be a
 * second thing to read. The other six are glyphs, because a colour alone
 * cannot tell six states apart for a reader who cannot see the colour. The
 * two that need a person take a tint; the rest stay in the faint ink, so a
 * column of sessions reads as two hands raised among quiet rows.
 */
export function StatusDot({ status, size = 'medium' }: StatusDotProps) {
  const px = size === 'small' ? 12 : 14

  if (status === 'running') {
    return <span aria-hidden data-size={size} className="wb-pulse" />
  }

  return (
    <span
      aria-hidden
      data-status={status}
      data-tint={
        status === 'waiting'
          ? 'warning'
          : status === 'review'
            ? 'primary'
            : status === 'done'
              ? 'success'
              : undefined
      }
      className="wb-mark"
    >
      <Icon
        icon={
          status === 'waiting'
            ? HandGlyph
            : status === 'review'
              ? DiffGlyph
              : status === 'watching'
                ? EyeGlyph
                : status === 'queued'
                  ? ClockGlyph
                  : status === 'paused'
                    ? PauseGlyph
                    : CheckGlyph
        }
        size={px}
      />
    </span>
  )
}

/** The same seven states, in words, for the line under a title. */
export const STATUS_LABEL: Record<SessionStatus, string> = {
  running: 'Running',
  waiting: 'Waiting for you',
  review: 'Ready to review',
  watching: 'Watching',
  queued: 'Queued',
  paused: 'Paused',
  done: 'Done',
}
