import {
  Badge,
  Button,
  Card,
  EmptyState,
  Icon,
  Menu,
  Progress,
} from '@dazzlabs/dazzui-react'
import { useState } from 'react'

import {
  ChevronGlyph,
  EyeGlyph,
  PlayGlyph,
  RefreshGlyph,
  SparkleGlyph,
} from '../data/icons'
import type { Project, TestFailure } from '../lib/project'
import { RunLight } from './pull-view'

export interface TestsViewProps {
  project: Project
  onFix: (failure: TestFailure) => void
  onRun: (what: string) => void
}

/**
 * Tests: the four numbers, then every failure with enough of its output to
 * act on, then what has run lately.
 *
 * A failure is a card rather than a row because it carries the thing that
 * makes it fixable — the assertion as the runner printed it, and the agent's
 * reading of why. The button beside it is the same one the issue list has:
 * this is a thing to hand over, not only a thing to read.
 */
export function TestsView({ project, onFix, onRun }: TestsViewProps) {
  const [only, setOnly] = useState('failures')
  const tests = project.tests

  if (!tests) {
    return (
      <div className="wb-view">
        <header className="wb-view__head">
          <h1 className="wb-view__title">Tests</h1>
        </header>
        <div className="wb-view__body">
          <EmptyState title="This project has no test command configured." />
        </div>
      </div>
    )
  }

  return (
    <div className="wb-view">
      <header className="wb-view__head">
        <h1 className="wb-view__title">Tests</h1>
        {tests.failed > 0 ? (
          <Badge size="small" tint="danger" variant="tinted">
            {tests.failed} failed
          </Badge>
        ) : (
          <Badge size="small" tint="success" variant="tinted">
            All green
          </Badge>
        )}
        <Badge size="small" tint="neutral">
          {tests.passed} passed · {tests.coverage}% covered
        </Badge>
        <span className="wb-view__gap" />
        <Menu
          align="end"
          items={[
            {
              label: 'Only failures',
              checked: only === 'failures',
              onSelect: () => setOnly('failures'),
            },
            {
              label: 'Every suite',
              checked: only === 'all',
              onSelect: () => setOnly('all'),
            },
          ]}
          trigger={
            <Button size="small" tint="neutral" variant="outlined">
              {only === 'failures' ? 'Only failures' : 'Every suite'}
              <Icon icon={ChevronGlyph} size={11} className="wb-caret-down" />
            </Button>
          }
        />
        <Button size="small" tint="neutral" variant="plain">
          <Icon icon={EyeGlyph} size={14} />
          Watch
        </Button>
        <Button
          size="small"
          variant="filled"
          onClick={() => onRun('everything')}
        >
          <Icon icon={PlayGlyph} size={13} />
          Run all
        </Button>
      </header>

      <div className="wb-view__body">
        <div className="wb-grid" data-columns="4">
          <Card variant="raised">
            <span data-tint="success" className="wb-stat">
              {tests.passed}
            </span>
            <span className="wb-card__quiet">passed</span>
          </Card>
          <Card variant="raised">
            <span
              data-tint={tests.failed > 0 ? 'danger' : undefined}
              className="wb-stat"
            >
              {tests.failed}
            </span>
            <span className="wb-card__quiet">failed</span>
          </Card>
          <Card variant="raised">
            <span className="wb-stat">{tests.skipped}</span>
            <span className="wb-card__quiet">skipped</span>
          </Card>
          <Card variant="raised">
            <span className="wb-stat">{tests.coverage}%</span>
            <Progress
              aria-label="Line coverage"
              size="small"
              value={tests.coverage}
            />
            <span className="wb-card__quiet">line coverage</span>
          </Card>
        </div>

        {tests.failures.length === 0 ? (
          <EmptyState title="Every test passes. The last run was clean." />
        ) : null}

        {tests.failures.map(failure => (
          <Card key={failure.name} size="large" variant="raised">
            <span className="wb-card__head">
              <RunLight status="failed" />
              <span className="wb-truncate wb-mono wb-card__title">
                {failure.suite} › {failure.name}
              </span>
              <span className="wb-view__gap" />
              <Button size="small" onClick={() => onFix(failure)}>
                <Icon icon={SparkleGlyph} size={13} />
                Hand to agent
              </Button>
              <Button
                size="small"
                tint="neutral"
                variant="plain"
                onClick={() => onRun(failure.suite)}
              >
                <Icon icon={RefreshGlyph} size={13} />
                Re-run
              </Button>
            </span>

            <div className="wb-terminal__body wb-terminal__body--flat">
              {failure.output.map((line, index) => (
                <div key={index}>{line}</div>
              ))}
            </div>

            <span className="wb-card__quiet">
              Agent’s reading: {failure.analysis}
              {failure.pull
                ? ` Same point as the review on #${failure.pull}.`
                : ''}
            </span>
          </Card>
        ))}

        <Card size="large" variant="raised">
          <span className="wb-label">Recent runs</span>
          <ul className="wb-rows">
            {tests.runs.map(run => (
              <li key={run.label}>
                <RunLight status={run.status} />
                <span className="wb-truncate">{run.label}</span>
                <span className="wb-rows__meta">
                  {run.when} · {run.took}
                </span>
              </li>
            ))}
          </ul>
        </Card>
      </div>
    </div>
  )
}
