import { Badge, type Tint } from '@dazzlabs/dazzui-react'

import { type Tally, type Verdict, summarize } from '../lib/verdict'

const TINT: Record<Verdict, Tint> = {
  match: 'success',
  within: 'success',
  off: 'danger',
  unread: 'neutral',
  missing: 'warning',
}

/** A tally, in one word or one number: what a cell says at a glance. */
export function Chip({ tally }: { tally: Tally }) {
  const verdict = summarize(tally)
  const label =
    verdict === 'off'
      ? `${tally.off + tally.missing} off`
      : verdict === 'within'
        ? 'within'
        : verdict

  return (
    <Badge
      size="small"
      variant={verdict === 'match' ? 'outlined' : 'tinted'}
      tint={TINT[verdict]}
      className="g-chip"
      data-verdict={verdict}
    >
      {label}
    </Badge>
  )
}

/** One verdict, for a cell in the property table. */
export function Mark({ verdict }: { verdict: Verdict }) {
  return (
    <span
      className="g-mark"
      data-verdict={verdict}
      title={verdict}
      aria-label={verdict}
    />
  )
}
