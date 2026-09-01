import { Badge, Icon, type IconGlyph, Spinner } from '@dazzlabs/dazzui-react'

import {
  DocGlyph,
  PencilGlyph,
  PlusGlyph,
  SearchGlyph,
  SparkleGlyph,
  TerminalGlyph,
} from '../data/icons'
import {
  formatDiffMeta,
  type ToolEntry,
  type ToolName,
  type ToolOutput,
} from '../lib/transcript'
import { DiffView } from './diff-view'
import { Fold } from './fold'

const GLYPHS: Record<ToolName, IconGlyph> = {
  Read: DocGlyph,
  Grep: SearchGlyph,
  Write: PlusGlyph,
  Edit: PencilGlyph,
  Bash: TerminalGlyph,
  Task: SparkleGlyph,
}

function Output({ output }: { output: ToolOutput }) {
  if (output.kind === 'diff') return <DiffView lines={output.lines} />

  if (output.kind === 'lines') {
    return <pre className="ca-output">{output.lines.join('\n')}</pre>
  }

  return (
    <div>
      {output.matches.map(match => (
        <div key={`${match.path}:${match.line}`} className="ca-match">
          <span className="ca-match__where">
            {match.path.split('/').pop()}:{match.line}
          </span>
          <span className="ca-match__text">{match.text}</span>
        </div>
      ))}
    </div>
  )
}

export interface ToolCardProps {
  entry: ToolEntry
  onToggle: () => void
}

/** One tool call: what was run against what, with the answer folded away. */
export function ToolCard({ entry, onToggle }: ToolCardProps) {
  const output = entry.output
  const meta =
    output?.kind === 'diff'
      ? (formatDiffMeta(output.lines) ?? entry.meta)
      : entry.meta

  return (
    <Fold
      open={entry.expanded}
      summary={
        <>
          <Icon icon={GLYPHS[entry.tool]} size={14} />
          <span className="ca-tool__name">{entry.tool}</span>
          <span className="ca-tool__target">{entry.target}</span>

          {entry.status === 'running' ? <Spinner size="small" /> : null}
          {entry.status === 'stopped' ? (
            <Badge size="small" tint="danger">
              {entry.meta ?? 'stopped'}
            </Badge>
          ) : null}
          {entry.status === 'done' && meta ? (
            <span className="ca-tool__meta">{meta}</span>
          ) : null}
        </>
      }
      onToggle={output ? onToggle : undefined}
    >
      {output ? <Output output={output} /> : null}
    </Fold>
  )
}
