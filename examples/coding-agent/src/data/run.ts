import type { Issue } from '../lib/project'
import type { Beat, DiffLine } from '../lib/transcript'

/** A whole new file, as a diff: every line added, numbered from one. */
function created(lines: string[]): DiffLine[] {
  return lines.map((text, index) => ({
    tone: 'added',
    after: index + 1,
    text,
  }))
}

const TOOLTIP_TSX = created([
  "import type { ComponentPropsWithoutRef, ReactNode } from 'react'",
  '',
  "import { cx } from '../lib/cx'",
  '',
  "export type TooltipSide = 'top' | 'bottom'",
  '',
  "export interface TooltipProps extends ComponentPropsWithoutRef<'span'> {",
  '  /** The line the tip carries. One line — it is a hint, not a panel. */',
  '  label: ReactNode',
  '  side?: TooltipSide',
  '  children: ReactNode',
  '}',
  '',
  '/** A hover hint for a control whose glyph is not its own label. */',
  'export function Tooltip({',
  '  label,',
  "  side = 'top',",
  '  className,',
  '  children,',
  '  ...props',
  '}: TooltipProps) {',
  '  return (',
  "    <span className={cx('dz-tooltip', className)} {...props}>",
  '      {children}',
  '      <span role="tooltip" data-side={side} className="dz-tooltip__bubble">',
  '        {label}',
  '      </span>',
  '    </span>',
  '  )',
  '}',
])

const TOOLTIP_CSS = created([
  '/*',
  ' * A hint, not a panel: one line of the caption face on the inverted',
  ' * surface, at the popover corner the rest of the overlays share.',
  ' */',
  '.dz-tooltip {',
  '  position: relative;',
  '  display: inline-flex;',
  '}',
  '',
  '.dz-tooltip__bubble {',
  '  position: absolute;',
  '  z-index: 30;',
  '  inset-inline-start: 50%;',
  '  translate: -50% 0;',
  '  padding-block: var(--spacing-1);',
  '  padding-inline: var(--spacing-2);',
  '  border-radius: var(--radius-small);',
  '  background-color: var(--color-content);',
  '  color: var(--color-surface);',
  '  white-space: nowrap;',
  '  opacity: 0;',
  '  pointer-events: none;',
  '  font-family: var(--caption-small-font-family);',
  '  font-size: var(--caption-small-font-size);',
  '  transition: opacity var(--motion-duration) var(--motion-easing);',
  '}',
])

const INDEX_EDIT: DiffLine[] = [
  {
    tone: 'context',
    before: 128,
    after: 128,
    text: "export { Toast, type ToastProps } from './toast'",
  },
  {
    tone: 'context',
    before: 129,
    after: 129,
    text: "export { Switch, type SwitchProps, type SwitchSize } from './switch'",
  },
  {
    tone: 'added',
    after: 130,
    text: "export { Tooltip, type TooltipProps, type TooltipSide } from './tooltip'",
  },
]

const STYLES_EDIT: DiffLine[] = [
  {
    tone: 'context',
    before: 51,
    after: 51,
    text: "@import '../components/toast.css';",
  },
  {
    tone: 'context',
    before: 52,
    after: 52,
    text: "@import '../components/switch.css';",
  },
  {
    tone: 'added',
    after: 53,
    text: "@import '../components/tooltip.css';",
  },
]

/**
 * The run the composer replays. It is the repository working on itself, which
 * is the honest thing for an example to demonstrate: every path, registry and
 * command below is one this workspace really has.
 */
export const TOOLTIP_RUN: Beat[] = [
  {
    kind: 'say',
    text: 'I will follow whichever existing component is closest rather than inventing a shape. Badge is the one — it is not interactive, so it reads the size profile steps directly instead of binding them. Let me check both registries first.',
  },
  {
    kind: 'plan',
    steps: [
      { label: 'Read the two registries', meta: '2 files' },
      { label: 'Draft tooltip.tsx and tooltip.css' },
      { label: 'Register the pair' },
      { label: 'Typecheck and lint' },
    ],
  },
  {
    kind: 'tool',
    tool: 'Read',
    target: 'packages/ui_react/src/components/index.ts',
    work: 640,
    meta: '129 lines',
    output: {
      kind: 'lines',
      lines: [
        "export { Table, TableCell, TableHead, TableRow } from './table'",
        "export { Toast, type ToastProps } from './toast'",
        "export { Switch, type SwitchProps, type SwitchSize } from './switch'",
      ],
    },
  },
  {
    kind: 'tool',
    tool: 'Grep',
    target: "@import '../components/",
    work: 520,
    meta: '33 matches in 1 file',
    output: {
      kind: 'matches',
      matches: [
        {
          path: 'packages/ui_react/src/styles/components.css',
          line: 31,
          text: "@import '../components/badge.css';",
        },
        {
          path: 'packages/ui_react/src/styles/components.css',
          line: 51,
          text: "@import '../components/toast.css';",
        },
        {
          path: 'packages/ui_react/src/styles/components.css',
          line: 52,
          text: "@import '../components/switch.css';",
        },
      ],
    },
  },
  {
    kind: 'say',
    text: 'Both registries are explicit lists rather than globs, so a component only ships once it is named in each. That is two edits after the two new files.',
  },
  { kind: 'step', index: 0 },
  {
    kind: 'ask',
    tool: 'Write',
    target: 'packages/ui_react/src/components/tooltip.tsx',
    question: 'Create packages/ui_react/src/components/tooltip.tsx?',
    detail:
      'The file does not exist yet, so the whole of it is new. Allowing Write once covers this file; allowing it always covers every file this session writes.',
    lines: TOOLTIP_TSX.slice(0, 12),
  },
  {
    kind: 'tool',
    tool: 'Write',
    target: 'packages/ui_react/src/components/tooltip.tsx',
    work: 900,
    expanded: true,
    output: { kind: 'diff', lines: TOOLTIP_TSX },
  },
  {
    kind: 'tool',
    tool: 'Write',
    target: 'packages/ui_react/src/components/tooltip.css',
    work: 780,
    output: { kind: 'diff', lines: TOOLTIP_CSS },
  },
  { kind: 'step', index: 1 },
  {
    kind: 'tool',
    tool: 'Edit',
    target: 'packages/ui_react/src/components/index.ts',
    work: 560,
    expanded: true,
    output: { kind: 'diff', lines: INDEX_EDIT },
  },
  {
    kind: 'tool',
    tool: 'Edit',
    target: 'packages/ui_react/src/styles/components.css',
    work: 500,
    output: { kind: 'diff', lines: STYLES_EDIT },
  },
  { kind: 'step', index: 2 },
  {
    kind: 'tool',
    tool: 'Bash',
    target: 'pnpm typecheck',
    work: 1700,
    meta: '3.4s',
    expanded: true,
    output: {
      kind: 'lines',
      lines: [
        '> dazzui@0.0.0 typecheck',
        '> pnpm -r typecheck',
        '',
        'packages/ds        typecheck  Done',
        'packages/ui_react  typecheck  Done',
        'apps/storybook     typecheck  Done',
      ],
    },
  },
  {
    kind: 'tool',
    tool: 'Bash',
    target: 'pnpm lint',
    work: 950,
    meta: '0.9s',
    output: {
      kind: 'lines',
      lines: [
        'Found 1 warning and 0 errors.',
        'Finished in 96ms on 214 files with 102 rules using 8 threads.',
      ],
    },
  },
  {
    kind: 'notice',
    tint: 'info',
    title: 'One warning left standing',
    body: 'The warning is the one in packages/ds that predates the lint setup, not anything this change introduced.',
  },
  { kind: 'step', index: 3 },
  {
    kind: 'say',
    text: 'Tooltip is in. It draws on the inverted surface at the popover corner, takes a side, and is registered in both lists — so it ships with the next build rather than needing one. The Storybook page is the piece still missing; say the word and I will add Components/Tooltip beside the rest.',
  },
  {
    kind: 'finish',
    toast: 'Tooltip added — 4 files touched.',
  },
]

export interface Suggestion {
  title: string
  description: string
  prompt: string
}

/**
 * What the empty state offers a project with no issues to work from — or none
 * this window can see, which is the same thing from here.
 */
export const SUGGESTIONS: Suggestion[] = [
  {
    title: 'Explain this project',
    description: 'What it builds, how it is laid out, and where to start.',
    prompt:
      'Walk me through this project — what it builds and how it is laid out.',
  },
  {
    title: 'Review the working tree',
    description: 'Read what is uncommitted and say whether it holds together.',
    prompt:
      'Review everything in the working tree and tell me what is wrong with it.',
  },
  {
    title: 'Add a test',
    description: 'Cover the last change with a test that would have caught it.',
    prompt: 'Add a test covering the most recent change in the working tree.',
  },
]

/**
 * The same, drawn from an issue: where a real session's subject comes from.
 *
 * An issue body runs as long as it likes; a card in a row of three cannot, so
 * only the first sentence makes it across.
 */
export function suggestFromIssue(issue: Issue): Suggestion {
  const stop = issue.body.indexOf('. ')
  return {
    title: `#${issue.number} ${issue.title}`,
    description: stop === -1 ? issue.body : issue.body.slice(0, stop + 1),
    prompt: `Fix #${issue.number} — ${issue.title}`,
  }
}
