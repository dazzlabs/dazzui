import type { DiffLine, DiffTone } from '../lib/transcript'

/** The column a hunk is read down: what changed, before what it says. */
const SIGN: Record<DiffTone, string> = {
  context: '',
  added: '+',
  removed: '-',
}

/**
 * Which ramp a line is washed with. Context takes none, so it stays on the
 * card it sits on rather than being tinted a third neutral.
 */
const TINT: Record<DiffTone, string | undefined> = {
  context: undefined,
  added: 'success',
  removed: 'danger',
}

export interface DiffViewProps {
  lines: readonly DiffLine[]
}

/**
 * A unified diff.
 *
 * The washes are the `tinted` recipe rather than colours of its own — an added
 * line is the success ramp at the alpha the system already uses for a tinted
 * chip, and its marker takes that recipe's content grade, a step darker than
 * the fill. That is what keeps a diff legible on the paper canvas and on the
 * near-black one without a single per-theme value here.
 */
export function DiffView({ lines }: DiffViewProps) {
  return (
    <div className="wb-diff">
      {lines.map((line, index) => (
        <div
          // Diff lines have no identity of their own: two blank context lines
          // in one hunk are the same line, and the list never reorders.
          key={index}
          data-tint={TINT[line.tone]}
          data-tone={line.tone}
          className="wb-diff__line"
        >
          <span aria-hidden className="wb-diff__gutter">
            {line.before ?? ''}
          </span>
          <span aria-hidden className="wb-diff__gutter">
            {line.after ?? ''}
          </span>
          <span aria-hidden className="wb-diff__sign">
            {SIGN[line.tone]}
          </span>
          <span className="wb-diff__text">{line.text || ' '}</span>
        </div>
      ))}
    </div>
  )
}
