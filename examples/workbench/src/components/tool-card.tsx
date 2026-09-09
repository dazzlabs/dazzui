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

const GLYPHS: Record<ToolName, IconGlyph> = {
  Read: DocGlyph,
  Grep: SearchGlyph,
  Write: PlusGlyph,
  Edit: PencilGlyph,
  Bash: TerminalGlyph,
  Task: SparkleGlyph,
}

/** What a tool call did, in the fewest words the row can be read by. */
function summary(entry: ToolEntry): string {
  switch (entry.tool) {
    case 'Write':
      return `Create ${entry.target}`
    case 'Edit':
      return `Edit ${entry.target}`
    case 'Read':
      return `Read ${entry.target}`
    case 'Grep':
      return `Search ${entry.target}`
    case 'Task':
      return entry.target
    case 'Bash':
      return entry.target
  }
}

function Output({ output }: { output: ToolOutput }) {
  if (output.kind === 'diff') return <DiffView lines={output.lines} />

  if (output.kind === 'lines') {
    return <pre className="wb-output">{output.lines.join('\n')}</pre>
  }

  return (
    <div className="wb-matches">
      {output.matches.map(match => (
        <div key={`${match.path}:${match.line}`} className="wb-match">
          <span className="wb-match__where">
            {match.path.split('/').pop()}:{match.line}
          </span>
          <span className="wb-match__text">{match.text}</span>
        </div>
      ))}
    </div>
  )
}

export interface ToolCardProps {
  entry: ToolEntry
  onToggle: () => void
}

/**
 * One tool call, drawn as the thing it is rather than as a generic row.
 *
 * A command is a terminal: dark, in every theme, because that is what a
 * terminal is and a light one reads as a quotation of one. Everything else is
 * a card on the page's own paper — a file edit is the diff itself, and a read
 * or a search is a line with its answer folded under it.
 */
export function ToolCard({ entry, onToggle }: ToolCardProps) {
  const output = entry.output
  const meta =
    output?.kind === 'diff'
      ? (formatDiffMeta(output.lines) ?? entry.meta)
      : entry.meta

  if (entry.tool === 'Bash') {
    return (
      <div className="wb-turn wb-terminal">
        <div className="wb-terminal__head">
          <Icon icon={TerminalGlyph} size={13} />
          Terminal
          <span className="wb-terminal__gap" />
          {entry.status === 'running' ? (
            <Spinner size="small" onAccent />
          ) : (
            <span className="wb-terminal__meta">
              {entry.status === 'stopped'
                ? 'Interrupted'
                : `Done · ${meta ?? 'ok'}`}
            </span>
          )}
        </div>
        <div className="wb-terminal__body">
          <div className="wb-terminal__command">$ {entry.target}</div>
          {output?.kind === 'lines'
            ? output.lines.map((line, index) => <div key={index}>{line}</div>)
            : null}
        </div>
      </div>
    )
  }

  const foldable = Boolean(output) && output?.kind !== 'diff'

  return (
    <div className="wb-turn wb-tool" data-open={entry.expanded}>
      <button
        aria-expanded={foldable ? entry.expanded : undefined}
        className="wb-tool__head"
        disabled={!foldable}
        type="button"
        onClick={foldable ? onToggle : undefined}
      >
        <Icon icon={GLYPHS[entry.tool]} size={14} />
        <span className="wb-truncate wb-tool__name">{summary(entry)}</span>
        {entry.status === 'running' ? <Spinner size="small" /> : null}
        {entry.status === 'stopped' ? (
          <Badge size="small" tint="danger">
            {entry.meta ?? 'stopped'}
          </Badge>
        ) : null}
        {entry.status === 'done' && output?.kind === 'diff' ? (
          <span className="wb-counts">
            <span data-tint="success" className="wb-count">
              +{output.lines.filter(line => line.tone === 'added').length}
            </span>
            <span data-tint="danger" className="wb-count">
              −{output.lines.filter(line => line.tone === 'removed').length}
            </span>
          </span>
        ) : entry.status === 'done' && meta ? (
          <span className="wb-tool__meta">{meta}</span>
        ) : null}
      </button>

      {output && (output.kind === 'diff' || entry.expanded) ? (
        <div className="wb-tool__body">
          <Output output={output} />
        </div>
      ) : null}
    </div>
  )
}
