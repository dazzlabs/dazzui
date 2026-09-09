import { ActionBar, Button, Card, Icon } from '@dazzlabs/dazzui-react'

import { ShieldGlyph } from '../data/icons'
import type { Answer, PendingAsk } from '../lib/player'
import { DiffView } from './diff-view'

export interface PermissionCardProps {
  ask: PendingAsk
  onAnswer: (answer: Answer) => void
}

/**
 * The gate in front of a risky operation, drawn in the thread where it came
 * up rather than in a sheet over the window.
 *
 * A modal would take the rest of the window away for a decision that is about
 * one line of it — and the reader usually wants to scroll up and check the
 * plan before answering, which a modal is exactly the thing that prevents.
 * The safe answer is the quiet one: allowing is the filled button because it
 * is what the reader came to do, and denying is one click away either way.
 */
export function PermissionCard({ ask, onAnswer }: PermissionCardProps) {
  return (
    <Card className="wb-turn wb-permission" variant="raised">
      <span className="wb-permission__head">
        <span data-tint="warning" className="wb-mark">
          <Icon icon={ShieldGlyph} size={16} />
        </span>
        {ask.question}
      </span>

      <code className="wb-permission__command">{ask.target}</code>
      <p className="wb-permission__detail">{ask.detail}</p>

      {ask.lines ? (
        <div className="wb-permission__preview">
          <DiffView lines={ask.lines} />
        </div>
      ) : null}

      <ActionBar align="start">
        <Button size="small" variant="filled" onClick={() => onAnswer('once')}>
          Allow
        </Button>
        <Button
          size="small"
          tint="neutral"
          variant="plain"
          onClick={() => onAnswer('deny')}
        >
          Deny
        </Button>
        <Button
          size="small"
          tint="neutral"
          variant="plain"
          onClick={() => onAnswer('always')}
        >
          Always allow this session
        </Button>
      </ActionBar>
    </Card>
  )
}
