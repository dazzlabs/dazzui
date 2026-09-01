import {
  AlertBanner,
  Card,
  Icon,
  SectionLabel,
  Step,
  StepList,
} from '@dazzlabs/dazzui-react'
import { type ReactNode, useEffect, useRef } from 'react'

import { SparkleGlyph } from '../data/icons'
import type { Entry } from '../lib/transcript'
import { ToolCard } from './tool-card'

/**
 * How near the foot counts as being at the foot. Below this the column follows
 * the run; above it the reader has scrolled back on purpose and is left alone.
 */
const STICK = 80

function Lane({
  role,
  mark,
  children,
}: {
  role: 'user' | 'agent'
  mark?: ReactNode
  children: ReactNode
}) {
  return (
    <div
      data-role={role}
      data-tint={role === 'agent' ? 'primary' : undefined}
      className="ca-lane"
    >
      <span aria-hidden className="ca-lane__mark">
        {mark}
      </span>
      <div className="ca-lane__body">{children}</div>
    </div>
  )
}

export interface TranscriptViewProps {
  entries: readonly Entry[]
  /** The initials on the mark beside the user's own turns. */
  initials: string
  /** Shown instead of the column while the session is still empty. */
  empty: ReactNode
  onToggle: (id: string) => void
}

export function TranscriptView({
  entries,
  initials,
  empty,
  onToggle,
}: TranscriptViewProps) {
  const scroller = useRef<HTMLDivElement>(null)
  /** Whether the reader was at the foot when the last entry landed. */
  const following = useRef(true)

  useEffect(() => {
    const node = scroller.current
    if (!node || !following.current) return
    node.scrollTop = node.scrollHeight
  }, [entries])

  return (
    <div
      ref={scroller}
      className="ca-transcript"
      onScroll={event => {
        const node = event.currentTarget
        const left = node.scrollHeight - node.scrollTop - node.clientHeight
        following.current = left < STICK
      }}
    >
      {entries.length === 0 ? empty : null}

      {entries.map(entry => {
        switch (entry.kind) {
          case 'user':
            return (
              <Lane key={entry.id} role="user" mark={initials}>
                <p className="ca-turn-text">{entry.text}</p>
              </Lane>
            )

          case 'agent': {
            const typing = entry.shown < entry.text.length
            return (
              <Lane
                key={entry.id}
                role="agent"
                mark={<Icon icon={SparkleGlyph} size={14} />}
              >
                <p className="ca-turn-text">
                  {entry.text.slice(0, entry.shown)}
                  {typing ? <span aria-hidden className="ca-caret" /> : null}
                </p>
              </Lane>
            )
          }

          case 'plan':
            return (
              <Lane key={entry.id} role="agent">
                {/* The container size, not the small one: `card.small` takes
                    the control corner, and Bright draws that as a pill — a
                    plan is a panel, and a panel in a lozenge is not one. */}
                <Card size="medium" variant="sunken">
                  <SectionLabel faint>Plan</SectionLabel>
                  <StepList>
                    {entry.steps.map(step => (
                      <Step
                        key={step.label}
                        label={step.label}
                        meta={step.meta}
                        status={step.status}
                      />
                    ))}
                  </StepList>
                </Card>
              </Lane>
            )

          case 'tool':
            return (
              <Lane key={entry.id} role="agent">
                <ToolCard entry={entry} onToggle={() => onToggle(entry.id)} />
              </Lane>
            )

          case 'notice':
            return (
              <Lane key={entry.id} role="agent">
                <AlertBanner size="small" tint={entry.tint} title={entry.title}>
                  {entry.body}
                </AlertBanner>
              </Lane>
            )
        }
      })}
    </div>
  )
}
