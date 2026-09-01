import type { StepStatus, Tint } from '@dazzlabs/dazzui-react'

/** The tools the prototype's agent is allowed to reach for. */
export type ToolName = 'Read' | 'Grep' | 'Write' | 'Edit' | 'Bash' | 'Task'

export type DiffTone = 'context' | 'added' | 'removed'

export interface DiffLine {
  tone: DiffTone
  /** Line number in the file as it was; absent on an added line. */
  before?: number
  /** Line number in the file as it will be; absent on a removed line. */
  after?: number
  text: string
}

export interface GrepMatch {
  path: string
  line: number
  text: string
}

/**
 * What a tool hands back. Three shapes rather than one blob of text, because
 * the reader wants a diff drawn as a diff and a command's output set in the
 * code face — the same string in both would be the tell of a prototype.
 */
export type ToolOutput =
  | { kind: 'lines'; lines: string[] }
  | { kind: 'diff'; lines: DiffLine[] }
  | { kind: 'matches'; matches: GrepMatch[] }

export type ToolStatus = 'running' | 'done' | 'stopped'

export interface PlanStep {
  label: string
  meta?: string
  status: StepStatus
}

export interface ToolEntry {
  id: string
  kind: 'tool'
  tool: ToolName
  /** The one argument the row is read by — a path, a pattern, a command. */
  target: string
  status: ToolStatus
  /** Trailing detail once it lands: `128 lines`, `1.2s`. */
  meta?: string
  output?: ToolOutput
  expanded: boolean
}

export type Entry =
  | { id: string; kind: 'user'; text: string }
  | { id: string; kind: 'agent'; text: string; shown: number }
  | { id: string; kind: 'plan'; steps: PlanStep[] }
  | ToolEntry
  | { id: string; kind: 'notice'; tint: Tint; title: string; body: string }

/**
 * One move in a scripted run. The player walks these in order, so a script
 * reads top to bottom the way its transcript will.
 */
export type Beat =
  /** The agent types a paragraph. */
  | { kind: 'say'; text: string }
  | { kind: 'plan'; steps: { label: string; meta?: string }[] }
  /** Finish the plan step at `index`, and start the one after it. */
  | { kind: 'step'; index: number }
  | {
      kind: 'tool'
      tool: ToolName
      target: string
      /** How long the call appears to take, in ms. */
      work?: number
      meta?: string
      output?: ToolOutput
      /** Leave a long output folded; a diff is worth opening on arrival. */
      expanded?: boolean
    }
  /** Halt for a permission answer. Nothing after this runs until it lands. */
  | {
      kind: 'ask'
      tool: ToolName
      target: string
      question: string
      detail: string
      lines: DiffLine[]
    }
  | { kind: 'notice'; tint: Tint; title: string; body: string }
  /** Close the run with the toast that reports what it did. */
  | { kind: 'finish'; toast: string }

/** A file the run has written, gathered back out of its tool rows. */
export interface ChangedFile {
  path: string
  added: number
  removed: number
  /** True when nothing was there before: every line of the diff is new. */
  created: boolean
  /** The hunks, concatenated, so a review pane has something to show. */
  lines: DiffLine[]
}

function countDiff(lines: readonly DiffLine[]) {
  return {
    added: lines.filter(line => line.tone === 'added').length,
    removed: lines.filter(line => line.tone === 'removed').length,
  }
}

/** `+18 -4`, or nothing at all when a diff only moved context around. */
export function formatDiffMeta(lines: DiffLine[]): string | undefined {
  const { added, removed } = countDiff(lines)
  if (!added && !removed) return undefined
  return `+${added} -${removed}`
}

/**
 * What the run has written, read back out of the transcript rather than kept
 * beside it. The tool rows already carry every diff, so a second list would
 * only be a chance for the two to disagree.
 */
export function changedFiles(entries: readonly Entry[]): ChangedFile[] {
  const byPath = new Map<string, ChangedFile>()

  for (const entry of entries) {
    if (entry.kind !== 'tool' || entry.status !== 'done') continue
    if (entry.output?.kind !== 'diff') continue

    const counts = countDiff(entry.output.lines)
    const seen = byPath.get(entry.target)
    const lines = [...(seen?.lines ?? []), ...entry.output.lines]
    byPath.set(entry.target, {
      path: entry.target,
      added: (seen?.added ?? 0) + counts.added,
      removed: (seen?.removed ?? 0) + counts.removed,
      created: lines.every(line => line.tone === 'added'),
      lines,
    })
  }

  return [...byPath.values()]
}

/**
 * Four characters to a token, the usual back-of-envelope. Crude on purpose:
 * the meter is here to show a context window filling up, and a prototype that
 * shipped a real tokenizer to draw one bar would be solving the wrong problem.
 */
const CHARS_PER_TOKEN = 4

/** What the transcript weighs, in tokens, tools and all. */
export function estimateTokens(entries: readonly Entry[]): number {
  let characters = 0

  for (const entry of entries) {
    switch (entry.kind) {
      case 'user':
      case 'agent':
        characters += entry.text.length
        break
      case 'notice':
        characters += entry.title.length + entry.body.length
        break
      case 'plan':
        characters += entry.steps.reduce((sum, s) => sum + s.label.length, 0)
        break
      case 'tool': {
        characters += entry.tool.length + entry.target.length
        const output = entry.output
        if (output?.kind === 'lines') {
          characters += output.lines.join('').length
        } else if (output?.kind === 'diff') {
          characters += output.lines.reduce((sum, l) => sum + l.text.length, 0)
        } else if (output?.kind === 'matches') {
          characters += output.matches.reduce(
            (sum, m) => sum + m.text.length,
            0
          )
        }
        break
      }
    }
  }

  return Math.round(characters / CHARS_PER_TOKEN)
}
