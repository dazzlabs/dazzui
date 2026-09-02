import type { Project } from '../lib/project'

/**
 * Three projects, because the interesting cases are all in the differences
 * between them: one with a remote and work in flight, one on a feature branch
 * that has fallen behind, and one with no provider at all — which is what puts
 * the issue and pull panes in their empty state.
 *
 * The first is this repository, and every path, registry and command in it is
 * real. A prototype that invents a project has to invent its file paths too,
 * and invented paths are the first thing that reads as fake.
 */
export const PROJECTS: Project[] = [
  {
    id: 'ui',
    name: 'dazzlabs/ui',
    path: '~/Projects/dazzlabs/ui',
    branch: 'main',
    ahead: 2,
    behind: 0,
    provider: 'github',

    sessions: [
      {
        id: 'ui-tooltip',
        title: 'Add a Tooltip component',
        when: 'Now',
        seed: [],
      },
      {
        id: 'ui-focus-ring',
        title: 'Focus ring on the dark canvas',
        when: '2 hours ago',
        seed: [
          {
            id: 'focus-1',
            kind: 'user',
            text: 'The focus ring is almost invisible on studio-dark. Is that a token or a component bug?',
          },
          {
            id: 'focus-2',
            kind: 'agent',
            text: 'A token. --focus-ring is the primary ramp at an alpha that carries a per-theme mode, and the dark modes already lift it — 0.45 on the light canvas, 0.5 and 0.55 on the two dark ones. Nothing in the components decides this, so the fix is one value in the graph rather than a rule per control.',
            shown: Number.MAX_SAFE_INTEGER,
          },
          {
            id: 'focus-3',
            kind: 'tool',
            tool: 'Grep',
            target: 'focus-ring-alpha',
            status: 'done',
            meta: '3 matches in 1 file',
            expanded: false,
            output: {
              kind: 'matches',
              matches: [
                {
                  path: 'packages/ui_react/src/generated/style.css',
                  line: 291,
                  text: '--focus-ring-alpha: 0.45;',
                },
                {
                  path: 'packages/ui_react/src/generated/style.css',
                  line: 519,
                  text: '--focus-ring-alpha: 0.5;',
                },
                {
                  path: 'packages/ui_react/src/generated/style.css',
                  line: 561,
                  text: '--focus-ring-alpha: 0.55;',
                },
              ],
            },
          },
          {
            id: 'focus-4',
            kind: 'notice',
            tint: 'warning',
            title: 'Generated file — do not edit',
            body: 'Those three lines are written by packages/ds. The value to change is the mode on focus.ring.alpha in the token source; editing style.css directly is undone by the next build.',
          },
        ],
      },
      {
        id: 'ui-key-cap',
        title: 'Move KeyCap onto the size profile',
        when: 'Yesterday',
        seed: [
          {
            id: 'key-cap-1',
            kind: 'user',
            text: 'Should KeyCap read the control size profile like Button does?',
          },
          {
            id: 'key-cap-2',
            kind: 'agent',
            text: 'No — and the convention says so outright. The profile bindings are for controls, and KeyCap is an annotation: it never reaches control height. It belongs with Badge and Card, reading the profile steps and typography faces directly per size block. I left it where it was.',
            shown: Number.MAX_SAFE_INTEGER,
          },
        ],
      },
      {
        id: 'ui-rail',
        title: 'Port Rail to Flutter',
        when: 'Tuesday',
        seed: [
          {
            id: 'rail-1',
            kind: 'user',
            text: 'Port the Rail component to packages/ui_flutter.',
          },
          {
            id: 'rail-2',
            kind: 'agent',
            text: 'Done, as lib/src/widgets/rail.dart with RailGroup beside it. The colour axis is tint on the Dart side too — WidgetTint, never the kind axis dazzui-flutter used — and the width comes from the generated frame metrics rather than a constant.',
            shown: Number.MAX_SAFE_INTEGER,
          },
          {
            id: 'rail-3',
            kind: 'tool',
            tool: 'Bash',
            target: 'flutter test',
            status: 'done',
            meta: '11.2s',
            expanded: false,
            output: { kind: 'lines', lines: ['00:11 +42: All tests passed!'] },
          },
        ],
      },
    ],

    issues: [
      {
        number: 142,
        title: 'Tooltip is missing from the React package',
        state: 'open',
        labels: ['component', 'good first issue'],
        author: 'sunhee',
        when: '2 days ago',
        comments: 3,
        body: 'Every icon-only control in the shell needs a hover hint and there is nothing in the library to draw one with. It should be a pair like the rest — tooltip.tsx and tooltip.css — named in both registries, drawn on the inverted surface at the popover corner.',
      },
      {
        number: 139,
        title: 'Focus ring is hard to see on the dark canvas',
        state: 'open',
        labels: ['tokens', 'accessibility'],
        author: 'marek',
        when: '4 days ago',
        comments: 6,
        body: 'On studio-dark the ring is very nearly invisible against surface-muted. The alpha already carries a per-theme mode, so this is one value in the graph rather than a rule per control.',
      },
      {
        number: 137,
        title: 'Rail has no Flutter twin yet',
        state: 'open',
        labels: ['flutter'],
        author: 'lijy91',
        when: 'last week',
        comments: 1,
        body: 'The React package has Rail, RailGroup and Aside; ui_flutter has none of the three. The frame metrics are already generated into lib/src/generated, so the widths are there to read.',
      },
      {
        number: 131,
        title: 'Document why the ink ramp lifts at 700',
        state: 'open',
        labels: ['docs'],
        author: 'ana',
        when: '2 weeks ago',
        comments: 2,
        body: 'Everyone reads base.color.ink as a lightness ramp and is then surprised that 700 is lighter than 600. It is a role ladder — Bright’s primary action is ink navy whose hover lifts — and that belongs in the README rather than in everyone’s head.',
      },
      {
        number: 128,
        title: 'Drop the Tailwind sync from the ds README',
        state: 'closed',
        labels: ['docs'],
        author: 'lijy91',
        when: '3 weeks ago',
        comments: 4,
        body: 'The sync that used to write _base.yaml is gone. The README still describes it as the way values arrive, which is now the opposite of true.',
      },
    ],

    pulls: [
      {
        number: 144,
        title: 'Add Tooltip to ui_react',
        state: 'draft',
        branch: 'agent/tooltip',
        checks: 'running',
        files: 4,
        added: 57,
        removed: 0,
        author: 'forge',
        when: 'just now',
        body: 'Opened from this session. Two new files and the two registry lines that ship them. Storybook page still to come.',
      },
      {
        number: 141,
        title: 'Lift the focus ring alpha on the dark canvas',
        state: 'open',
        branch: 'fix/focus-ring-dark',
        checks: 'passing',
        files: 1,
        added: 3,
        removed: 3,
        author: 'marek',
        when: '3 days ago',
        body: 'Raises the dark modes to 0.5 and 0.55. No component changes — the ring is one token and every control reads it.',
      },
      {
        number: 136,
        title: 'Port Rail and RailGroup to Flutter',
        state: 'open',
        branch: 'flutter/rail',
        checks: 'failing',
        files: 6,
        added: 214,
        removed: 8,
        author: 'lijy91',
        when: 'last week',
        body: 'Widget tests pass locally; the golden test for the group gap fails on CI, which is the difference between the two Flutter versions rather than the widget.',
      },
      {
        number: 130,
        title: 'Rewrite the ds README',
        state: 'merged',
        branch: 'docs/ds-readme',
        checks: 'passing',
        files: 1,
        added: 46,
        removed: 62,
        author: 'lijy91',
        when: '3 weeks ago',
        body: 'Says what is true now: every token source is hand-authored, _base.yaml included.',
      },
    ],

    changes: [
      {
        path: 'AGENTS.md',
        status: 'modified',
        added: 2,
        removed: 1,
        staged: false,
        hunk: [
          {
            tone: 'context',
            before: 210,
            after: 210,
            text: '- One folder-level file pair per component: `src/components/<name>.tsx` and',
          },
          {
            tone: 'removed',
            before: 211,
            text: '  `<name>.css`, both listed in their registries.',
          },
          {
            tone: 'added',
            after: 211,
            text: '  `<name>.css`, both listed in their registries (`components/index.ts`,',
          },
          { tone: 'added', after: 212, text: '  `styles/components.css`).' },
        ],
      },
    ],
  },

  {
    id: 'beyondtranslate',
    name: 'dazzlabs/beyondtranslate',
    path: '~/Projects/dazzlabs/beyondtranslate',
    branch: 'feat/glossary-import',
    ahead: 1,
    behind: 3,
    provider: 'github',

    sessions: [
      {
        id: 'bt-glossary',
        title: 'CSV glossary import',
        when: 'Yesterday',
        seed: [
          {
            id: 'bt-1',
            kind: 'user',
            text: 'Empty rows in an imported CSV become empty glossary entries. Skip them.',
          },
          {
            id: 'bt-2',
            kind: 'agent',
            text: 'The parser was treating a trailing newline as a row. It now drops rows whose source term is blank after trimming, and reports the count so an import that silently halves is visible rather than surprising.',
            shown: Number.MAX_SAFE_INTEGER,
          },
        ],
      },
      {
        id: 'bt-shell',
        title: 'What is left in packages/ui',
        when: 'Last week',
        seed: [],
      },
    ],

    issues: [
      {
        number: 88,
        title: 'Glossary import keeps empty rows',
        state: 'open',
        labels: ['bug', 'glossary'],
        author: 'ana',
        when: '3 days ago',
        comments: 2,
        body: 'A CSV with a trailing newline imports one blank entry per file. They are invisible in the list until a lookup matches the empty string.',
      },
      {
        number: 85,
        title: 'WindowFrame still lives here',
        state: 'open',
        labels: ['design-system'],
        author: 'lijy91',
        when: 'last week',
        comments: 5,
        body: 'Every domain-free component has moved to dazzlabs/ui. What is left is the desktop shell — WindowFrame, Sidebar, MiniWindow, BrowserFrame — which is still pending a decision either way.',
      },
      {
        number: 80,
        title: 'Segment list jumps while a translation streams',
        state: 'open',
        labels: ['bug'],
        author: 'marek',
        when: '2 weeks ago',
        comments: 8,
        body: 'The row grows as the text arrives, and the list is anchored to the top rather than to the row being read.',
      },
    ],

    pulls: [
      {
        number: 91,
        title: 'Import glossaries from CSV',
        state: 'open',
        branch: 'feat/glossary-import',
        checks: 'passing',
        files: 9,
        added: 341,
        removed: 12,
        author: 'ana',
        when: 'yesterday',
        body: 'Adds the importer, the column mapping sheet, and the duplicate strategy. Blank rows are dropped and counted.',
      },
      {
        number: 89,
        title: 'Move WindowFrame to dazzlabs/ui',
        state: 'draft',
        branch: 'chore/shell-migration',
        checks: 'running',
        files: 14,
        added: 402,
        removed: 388,
        author: 'lijy91',
        when: '4 days ago',
        body: 'Opened to see how much of the shell is actually domain-free. The BrowserFrame is not; the rest might be.',
      },
    ],

    changes: [
      {
        path: 'lib/src/glossary/csv_importer.dart',
        status: 'modified',
        added: 8,
        removed: 2,
        staged: false,
        hunk: [
          {
            tone: 'context',
            before: 41,
            after: 41,
            text: '    for (final row in rows) {',
          },
          {
            tone: 'removed',
            before: 42,
            text: '      entries.add(GlossaryEntry.fromRow(row));',
          },
          {
            tone: 'added',
            after: 42,
            text: '      if (row.source.trim().isEmpty) {',
          },
          { tone: 'added', after: 43, text: '        skipped += 1;' },
          { tone: 'added', after: 44, text: '        continue;' },
          { tone: 'added', after: 45, text: '      }' },
          {
            tone: 'added',
            after: 46,
            text: '      entries.add(GlossaryEntry.fromRow(row));',
          },
        ],
      },
      {
        path: 'test/glossary/csv_importer_test.dart',
        status: 'modified',
        added: 14,
        removed: 0,
        staged: false,
        hunk: [
          {
            tone: 'added',
            after: 88,
            text: "    test('drops blank rows and counts them', () {",
          },
          {
            tone: 'added',
            after: 89,
            text: "      final result = importCsv('a,b\\n\\n');",
          },
          {
            tone: 'added',
            after: 90,
            text: '      expect(result.entries, hasLength(1));',
          },
          {
            tone: 'added',
            after: 91,
            text: '      expect(result.skipped, 1);',
          },
          { tone: 'added', after: 92, text: '    });' },
        ],
      },
    ],
  },

  {
    id: 'forge-cli',
    name: 'forge-cli',
    path: '~/Projects/forge-cli',
    branch: 'main',
    ahead: 0,
    behind: 0,
    provider: null,

    sessions: [
      {
        id: 'cli-scaffold',
        title: 'Scaffold the CLI',
        when: 'Monday',
        seed: [],
      },
    ],
    issues: [],
    pulls: [],

    changes: [
      {
        path: 'src/main.rs',
        status: 'modified',
        added: 3,
        removed: 1,
        staged: false,
        hunk: [
          {
            tone: 'context',
            before: 12,
            after: 12,
            text: 'fn main() -> anyhow::Result<()> {',
          },
          { tone: 'removed', before: 13, text: '    println!("forge");' },
          { tone: 'added', after: 13, text: '    let args = Args::parse();' },
          {
            tone: 'added',
            after: 14,
            text: '    tracing_subscriber::fmt::init();',
          },
          { tone: 'added', after: 15, text: '    run(args)' },
        ],
      },
    ],
  },
]
