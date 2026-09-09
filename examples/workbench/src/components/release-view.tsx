import {
  Badge,
  Button,
  Card,
  EmptyState,
  Icon,
  Progress,
  Table,
  TableCell,
  TableHead,
  TableRow,
} from '@dazzlabs/dazzui-react'

import { DownloadGlyph, PlayGlyph, SparkleGlyph } from '../data/icons'
import type { Channel, Project } from '../lib/project'
import { RunLight } from './pull-view'

/** What a channel offers, which follows from what the channel is. */
function actions(channel: Channel): string[] {
  switch (channel.id) {
    case 'app-store':
      return ['View submission', 'Agent writes the notes']
    case 'play-store':
      return ['Raise to 50%', 'Pause rollout']
    case 'github':
      return ['Preview the draft']
    case 'npm':
    case 'pub':
    case 'crates':
      return ['View on the registry']
    default:
      return ['Configure']
  }
}

export interface ReleaseViewProps {
  project: Project
  onRun: () => void
  onChannel: (channel: Channel, action: string) => void
}

/**
 * Release: one pipeline from a build to a shop window.
 *
 * The three blocks are the three questions a release raises in order — is it
 * built, is it out, and what exactly went out. The jobs are a table because
 * they are a grid of the same four facts; the channels are cards because each
 * one is a different kind of thing with different verbs.
 */
export function ReleaseView({ project, onRun, onChannel }: ReleaseViewProps) {
  const release = project.release

  if (!release) {
    return (
      <div className="wb-view">
        <header className="wb-view__head">
          <h1 className="wb-view__title">Release</h1>
        </header>
        <div className="wb-view__body">
          <EmptyState title="Nothing is configured to ship from this project yet." />
        </div>
      </div>
    )
  }

  const done = release.jobs.filter(job => job.status === 'passed').length
  const percent = Math.round((done / release.jobs.length) * 100)

  return (
    <div className="wb-view">
      <header className="wb-view__head">
        <h1 className="wb-view__title">Release {release.version}</h1>
        <Badge size="small" variant="tinted">
          {release.active} · running
        </Badge>
        <span className="wb-view__gap" />
        <Button size="small" tint="neutral" variant="plain">
          Edit config
        </Button>
        <Button size="small" variant="filled" onClick={onRun}>
          <Icon icon={PlayGlyph} size={13} />
          Run release
        </Button>
      </header>

      <div className="wb-view__body">
        <Card size="large" variant="raised">
          <span className="wb-card__head">
            <span className="wb-label">Pipeline</span>
            <span className="wb-card__quiet">
              package → publish · {done} of {release.jobs.length} done
            </span>
            <span className="wb-view__gap" />
            <Badge size="small" tint="neutral">
              <span className="wb-mono">{release.command}</span>
            </Badge>
          </span>

          <Progress aria-label="Release progress" value={percent} />

          <Table className="wb-jobs">
            <TableHead>
              <TableCell head className="wb-jobs__kind">
                Step
              </TableCell>
              <TableCell head className="wb-jobs__name">
                Job
              </TableCell>
              <TableCell head className="wb-jobs__target">
                Target
              </TableCell>
              <TableCell head align="end" className="wb-jobs__status">
                Status
              </TableCell>
            </TableHead>
            {release.jobs.map(job => (
              <TableRow key={`${job.kind}-${job.name}`}>
                <TableCell className="wb-jobs__kind">
                  <Badge
                    size="small"
                    tint={job.kind === 'publish' ? 'primary' : 'neutral'}
                    variant={job.kind === 'publish' ? 'tinted' : 'filled'}
                  >
                    {job.kind}
                  </Badge>
                </TableCell>
                <TableCell className="wb-jobs__name">
                  <RunLight status={job.status} />
                  <span className="wb-truncate">{job.name}</span>
                </TableCell>
                <TableCell className="wb-jobs__target">
                  <span className="wb-mono">{job.target}</span>
                </TableCell>
                <TableCell align="end" className="wb-jobs__status">
                  {job.status === 'queued'
                    ? 'waiting for the build'
                    : (job.took ?? job.status)}
                </TableCell>
              </TableRow>
            ))}
          </Table>

          <div className="wb-terminal__body wb-terminal__body--flat">
            {release.log.map((line, index) => (
              <div
                key={index}
                className={index === 0 ? 'wb-terminal__command' : undefined}
              >
                {line}
              </div>
            ))}
          </div>
        </Card>

        <div className="wb-grid" data-columns="3">
          {release.channels.map(channel => (
            <Card key={channel.id} size="large" variant="raised">
              <span className="wb-card__head">
                <span className="wb-card__title">{channel.name}</span>
                <Badge
                  size="small"
                  tint={
                    channel.tone === 'live'
                      ? 'success'
                      : channel.tone === 'moving'
                        ? 'warning'
                        : 'neutral'
                  }
                  variant={channel.tone === 'idle' ? 'filled' : 'tinted'}
                >
                  {channel.state}
                </Badge>
              </span>
              <span className="wb-card__note">{channel.live}</span>
              <span className="wb-card__quiet">{channel.note}</span>
              <span className="wb-channel__actions">
                {actions(channel).map((action, index) => (
                  <Button
                    key={action}
                    size="tiny"
                    tint="neutral"
                    variant={index === 0 ? 'outlined' : 'plain'}
                    onClick={() => onChannel(channel, action)}
                  >
                    {action.startsWith('Agent') ? (
                      <Icon icon={SparkleGlyph} size={12} />
                    ) : null}
                    {action}
                  </Button>
                ))}
              </span>
            </Card>
          ))}
        </div>

        <Card size="large" variant="raised">
          <span className="wb-label">Artifacts · dist/</span>
          <ul className="wb-rows">
            {release.artifacts.map(artifact => (
              <li key={artifact.name}>
                <Badge size="small" variant="outlined">
                  {artifact.format}
                </Badge>
                <span className="wb-truncate wb-mono">{artifact.name}</span>
                <span className="wb-rows__meta">{artifact.size}</span>
                <Button size="tiny" tint="neutral" variant="plain">
                  <Icon icon={DownloadGlyph} size={12} />
                  Download
                </Button>
              </li>
            ))}
          </ul>
        </Card>
      </div>
    </div>
  )
}
