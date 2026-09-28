import type { Project } from '../lib/project'

/**
 * Three projects, because the interesting cases are all in the differences
 * between them: a design-system monorepo that publishes to package
 * registries, a Flutter app that ships to the stores through dazzui, and
 * a small CLI with no remote and nothing to release — which is what puts
 * three views in their empty state.
 *
 * They belong to a sample organisation, `acme`, and to a sample person, Mia
 * Chen, so nothing here reads as anyone's real repository. The shapes are
 * still real ones: the paths, the commands and the failures are the kind a
 * project like each of these actually has, which is what keeps a scripted
 * run from reading as a demo.
 */
export const PROJECTS: Project[] = [
  {
    id: 'atlas',
    name: 'acme/atlas',
    path: '~/Code/atlas',
    branch: 'agent/tooltip',
    ahead: 2,
    behind: 0,
    provider: 'github',
    account: 'mia',
    seen: '10 minutes ago',

    branches: [
      { name: 'agent/tooltip', meta: 'current · ↑2' },
      { name: 'main', meta: 'synced' },
      { name: 'fix/focus-ring-dark', meta: '↑1' },
      { name: 'flutter/rail', meta: '↓5' },
    ],

    sessions: [
      {
        id: 'atlas-tooltip',
        title: 'Add a Tooltip component',
        when: 'Now',
        status: 'running',
        tools: 12,
        worktree: 'worktrees/142-tooltip',
        branch: 'agent/tooltip',
        issue: 142,
        pull: 144,
        steps: [
          { label: 'Read the two registries', meta: '2 files', status: 'done' },
          { label: 'Draft tooltip.tsx and tooltip.css', status: 'done' },
          { label: 'Register the pair', status: 'active' },
          { label: 'Typecheck and lint', status: 'idle' },
        ],
        seed: [],
      },
      {
        id: 'atlas-parity',
        title: 'Fix the card radius drift',
        when: 'Now',
        status: 'waiting',
        tools: 7,
        worktree: 'worktrees/parity',
        branch: 'fix/card-radius',
        blockedOn: 'pnpm parity',
        steps: [
          { label: 'Read the drift list', meta: '8 checks', status: 'done' },
          { label: 'Re-run the extractor', status: 'active' },
          { label: 'Delete the lines it settles', status: 'idle' },
        ],
        seed: [
          {
            id: 'parity-1',
            kind: 'user',
            text: 'The Flutter parity suite says every small card is 12 where React draws 10. Is that the adapter or the widget?',
          },
          {
            id: 'parity-2',
            kind: 'agent',
            text: 'The widget. `card.small` took the container corner in the same change that gave it to React, but the distributed reference predates it — so the drift list is measuring the old React. Re-running the extractor is the fix, not an edit.',
            shown: Number.MAX_SAFE_INTEGER,
          },
        ],
      },
      {
        id: 'atlas-preferences',
        title: 'Name the settings family after its root',
        when: '1 hour ago',
        status: 'review',
        tools: 6,
        worktree: 'worktrees/preferences',
        branch: 'agent/preferences',
        steps: [
          { label: 'Find the parts with no root', status: 'done' },
          { label: 'Add Preferences and rename the family', status: 'done' },
          { label: 'Wait for the review', status: 'active' },
        ],
        seed: [
          {
            id: 'prefs-1',
            kind: 'user',
            text: 'PreferenceGroup, PreferenceSection and PreferenceRow exist but there is no Preferences. Is that a family with no root, or a root that is missing?',
          },
          {
            id: 'prefs-2',
            kind: 'agent',
            text: 'A missing root. The stories and the workbench were each writing the run between groups and the column’s width for themselves, which is exactly what a root owns. Preferences is in, the file and the story page take its name, and the three parts are unchanged.',
            shown: Number.MAX_SAFE_INTEGER,
          },
          { id: 'prefs-3', kind: 'review' },
          {
            id: 'prefs-4',
            kind: 'divider',
            text: 'Waiting for you to review the diff',
          },
        ],
      },
      {
        id: 'atlas-focus-ring',
        title: 'Focus ring on the dark canvas',
        when: '2 hours ago',
        status: 'paused',
        tools: 5,
        worktree: 'worktrees/139-focus-ring',
        branch: 'fix/focus-ring-dark',
        issue: 139,
        pull: 141,
        steps: [
          { label: 'Find where the alpha is decided', status: 'done' },
          { label: 'Lift it under the dark modes', status: 'idle' },
        ],
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
            kind: 'notice',
            tint: 'warning',
            title: 'Generated file — do not edit',
            body: 'The three matches are in packages/react/src/generated/style.css, which packages/tokens writes. The value to change is the mode on focus.ring.alpha in the token source.',
          },
        ],
      },
      {
        id: 'atlas-key-cap',
        title: 'Move KeyCap onto the size profile',
        when: 'Yesterday',
        status: 'done',
        tools: 4,
        branch: 'chore/key-cap-profile',
        steps: [
          { label: 'Read the convention', status: 'done' },
          { label: 'Leave KeyCap where it is', status: 'done' },
        ],
        seed: [
          {
            id: 'key-cap-1',
            kind: 'user',
            text: 'Should KeyCap read the control size profile like Button does?',
          },
          {
            id: 'key-cap-2',
            kind: 'agent',
            text: 'No — and the convention says so outright. The profile bindings are for controls, and KeyCap is an annotation: it never reaches control height. It belongs with Badge and Card, reading the profile steps and typography faces directly per size block.',
            shown: Number.MAX_SAFE_INTEGER,
          },
        ],
      },
    ],

    issues: [
      {
        number: 142,
        title: 'Tooltip is missing from the React package',
        state: 'open',
        label: 'component',
        author: 'sunhee',
        when: '2 days ago',
        comments: 3,
        agentReady: true,
        pull: 144,
        body: 'Every icon-only control in the shell needs a hover hint and there is nothing in the library to draw one with. It should be a pair like the rest — tooltip.tsx and tooltip.css — named in both registries, drawn on the inverted surface at the popover corner.',
      },
      {
        number: 139,
        title: 'Focus ring is hard to see on the dark canvas',
        state: 'open',
        label: 'tokens',
        author: 'marek',
        when: '4 days ago',
        comments: 6,
        agentReady: true,
        pull: 141,
        body: 'On studio-dark the ring is very nearly invisible against surface-muted. The alpha already carries a per-theme mode, so this is one value in the graph rather than a rule per control.',
      },
      {
        number: 137,
        title: 'Rail has no Flutter twin yet',
        state: 'open',
        label: 'flutter',
        author: 'mia',
        when: 'last week',
        comments: 1,
        agentReady: false,
        body: 'The React package has Rail, RailGroup and Aside; the Flutter port has none of the three. The frame metrics are already generated into lib/src/generated, so the widths are there to read.',
      },
      {
        number: 131,
        title: 'Document why the ink ramp lifts at 700',
        state: 'open',
        label: 'docs',
        author: 'ana',
        when: '2 weeks ago',
        comments: 2,
        agentReady: true,
        body: 'Everyone reads base.color.ink as a lightness ramp and is then surprised that 700 is lighter than 600. It is a role ladder — Bright’s primary action is ink navy whose hover lifts — and that belongs in the README rather than in everyone’s head.',
      },
      {
        number: 128,
        title: 'Drop the Tailwind sync from the tokens README',
        state: 'closed',
        label: 'docs',
        author: 'mia',
        when: '3 weeks ago',
        comments: 4,
        agentReady: false,
        pull: 130,
        body: 'The sync that used to write _base.yaml is gone. The README still describes it as the way values arrive, which is now the opposite of true.',
      },
    ],

    pulls: [
      {
        number: 144,
        title: 'Add Tooltip to the React package',
        state: 'draft',
        branch: 'agent/tooltip',
        base: 'main',
        commits: 3,
        files: 4,
        added: 57,
        removed: 0,
        author: 'atlas-bot',
        when: '40 minutes ago',
        session: 'atlas-tooltip',
        fixes: 142,
        body: 'Two new files and the two registry lines that ship them. The Storybook page is still to come.',
        runs: [
          { name: 'build', status: 'passed', took: '1m 12s' },
          { name: 'test (unit)', status: 'passed', took: '3m 04s' },
          {
            name: 'test (flutter parity)',
            status: 'failed',
            took: '2m 40s',
            note: '2 checks drift from drift.txt',
          },
          { name: 'lint', status: 'running', took: 'running…' },
        ],
        timeline: [
          {
            author: 'mia',
            initials: 'MC',
            kind: 'opened',
            body: 'opened this pull request',
            when: '40 minutes ago',
            session: 'atlas-tooltip',
          },
          {
            author: 'sunhee',
            initials: 'SH',
            kind: 'comment',
            body: 'Worth a Storybook page in the same change — an unlisted component is one nobody finds.',
            when: '12 minutes ago',
          },
        ],
      },
      {
        number: 141,
        title: 'Lift the focus ring alpha on the dark canvas',
        state: 'open',
        branch: 'fix/focus-ring-dark',
        base: 'main',
        commits: 1,
        files: 1,
        added: 3,
        removed: 3,
        author: 'marek',
        when: '3 days ago',
        session: 'atlas-focus-ring',
        fixes: 139,
        body: 'Raises the dark modes to 0.5 and 0.55. No component changes — the ring is one token and every control reads it.',
        runs: [
          { name: 'build', status: 'passed', took: '58s' },
          { name: 'test (unit)', status: 'passed', took: '2m 11s' },
          { name: 'lint', status: 'passed', took: '30s' },
        ],
        timeline: [
          {
            author: 'marek',
            initials: 'MK',
            kind: 'opened',
            body: 'opened this pull request',
            when: '3 days ago',
          },
        ],
      },
      {
        number: 130,
        title: 'Rewrite the tokens README',
        state: 'merged',
        branch: 'docs/ds-readme',
        base: 'main',
        commits: 2,
        files: 1,
        added: 46,
        removed: 62,
        author: 'mia',
        when: '3 weeks ago',
        fixes: 128,
        body: 'Says what is true now: every token source is hand-authored, _base.yaml included.',
        runs: [
          { name: 'build', status: 'passed', took: '55s' },
          { name: 'test (unit)', status: 'passed', took: '2m 03s' },
          { name: 'lint', status: 'passed', took: '29s' },
        ],
        timeline: [
          {
            author: 'mia',
            initials: 'MC',
            kind: 'opened',
            body: 'opened this pull request',
            when: '3 weeks ago',
          },
        ],
      },
    ],

    changes: [
      {
        path: 'AGENTS.md',
        status: 'modified',
        added: 2,
        removed: 1,
        staged: true,
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

    history: [
      {
        sha: '4850255',
        message:
          'Give the settings column its root, and name the family after it',
        author: 'mia',
        when: '4 minutes ago',
        agent: true,
      },
      {
        sha: '94a6b2b',
        message: 'Give the small card the container corner',
        author: 'mia',
        when: '2 hours ago',
        agent: true,
      },
      {
        sha: '72ab456',
        message: 'Fold Rail and Aside into one Sidebar',
        author: 'mia',
        when: 'yesterday',
        agent: false,
      },
      {
        sha: 'efa2b4c',
        message: 'Drop the Material dependency from the Flutter port',
        author: 'mia',
        when: 'yesterday',
        agent: false,
      },
    ],

    worktrees: [
      {
        path: 'worktrees/142-tooltip',
        branch: 'agent/tooltip',
        session: 'atlas-tooltip',
      },
      {
        path: 'worktrees/parity',
        branch: 'fix/card-radius',
        session: 'atlas-parity',
      },
      {
        path: 'worktrees/preferences',
        branch: 'agent/preferences',
        session: 'atlas-preferences',
      },
      {
        path: 'worktrees/139-focus-ring',
        branch: 'fix/focus-ring-dark',
        session: 'atlas-focus-ring',
      },
      {
        path: 'worktrees/128-readme',
        branch: 'docs/ds-readme',
        archived: true,
      },
    ],

    inbox: [
      {
        id: 'atlas-conflict-index',
        kind: 'conflict',
        title: 'Two sessions are writing the same file',
        body: '“Add a Tooltip component” and “Name the settings family after its root” both add a line to the component registry. Whichever merges second will conflict.',
        path: 'packages/react/src/components/index.ts',
      },
      {
        id: 'ui-archived-130',
        kind: 'archived',
        title: '#130 merged — the session is archived',
        body: 'The worktree it ran in is still on disk and can be removed.',
        path: 'worktrees/128-readme',
        pull: 130,
      },
    ],

    tests: {
      passed: 46,
      failed: 2,
      skipped: 3,
      coverage: 81,
      suites: [
        { path: 'flutter/example/parity', passed: 2, failed: 2 },
        { path: 'flutter/test', passed: 46, failed: 0 },
        { path: 'gpui/tests/render', passed: 11, failed: 0 },
        { path: 'gpui/tests/parity', passed: 3, failed: 0 },
      ],
      failures: [
        {
          suite: 'parity/reference_test.dart',
          name: 'every specimen the port can build measures as React drew it',
          output: [
            'DRIFT  dz-card size=small tint=primary variant=raised · radius',
            '       studio-light: React 10.0, here 12.0',
            '  8 checks are out of step with drift.txt',
          ],
          analysis:
            'The distributed reference predates the change that gave card.small the container corner, so the drift is in the measurement rather than the widget. `pnpm parity` settles it.',
        },
        {
          suite: 'parity/stories_test.dart',
          name: 'every React story is drawn here, or is listed as pending',
          output: [
            'React draws components-preference--overview and this playground does not',
            'components-preferences--overview is drawn here but is not a React story',
          ],
          analysis:
            'One story id, spelled `preference` on the React side and `preferences` here. The page took the root component’s name and the port did not follow.',
          pull: 144,
        },
      ],
      runs: [
        {
          label: 'flutter test (example) · 2 failed',
          status: 'failed',
          when: '3 minutes ago',
          took: '13.1s',
        },
        {
          label: 'cargo test · agent triggered · all passed',
          status: 'passed',
          when: '25 minutes ago',
          took: '4.2s',
        },
        {
          label: 'CI · typecheck · PR #144 · all passed',
          status: 'passed',
          when: '40 minutes ago',
          took: '3m 04s',
        },
      ],
    },

    release: {
      version: 'v0.2.0',
      active: 'packages',
      configFile: 'package.json · pubspec.yaml · Cargo.toml',
      command: 'pnpm publish -r --access public',
      configs: [
        { name: 'packages', jobs: 6, note: '6 jobs · running' },
        { name: 'canary', jobs: 3, note: '3 jobs · npm tag next' },
      ],
      jobs: [
        {
          kind: 'package',
          name: '@acme/atlas-tokens',
          target: 'npm',
          status: 'passed',
          took: '18s',
        },
        {
          kind: 'package',
          name: '@acme/atlas-react',
          target: 'npm',
          status: 'passed',
          took: '31s',
        },
        {
          kind: 'package',
          name: 'atlas_ui (Dart)',
          target: 'pub',
          status: 'passed',
          took: '44s',
        },
        {
          kind: 'package',
          name: 'atlas_ui (Rust)',
          target: 'crates',
          status: 'running',
          took: 'running…',
        },
        {
          kind: 'publish',
          name: 'npm registry',
          target: 'npm',
          status: 'queued',
        },
        { kind: 'publish', name: 'pub.dev', target: 'pub', status: 'queued' },
      ],
      log: [
        '$ cargo package -p atlas_ui',
        '   Packaging atlas_ui v0.2.0',
        '   Verifying atlas_ui v0.2.0',
        '   Compiling atlas_ui v0.2.0 (target/package/atlas_ui-0.2.0)',
      ],
      channels: [
        {
          id: 'npm',
          name: 'npm',
          state: 'Publishing',
          tone: 'moving',
          live: 'Live 0.1.4 · 0.2.0 staged behind the build',
          note: 'Two packages: @acme/atlas-tokens and @acme/atlas-react.',
        },
        {
          id: 'pub',
          name: 'pub.dev',
          state: 'Queued',
          tone: 'moving',
          live: 'Live 0.1.4 · waiting for the Dart package job',
          note: 'atlas_ui, with the committed generated sources.',
        },
        {
          id: 'crates',
          name: 'crates.io',
          state: 'Not configured',
          tone: 'idle',
          live: 'Never published',
          note: 'The GPUI crate builds but has no owner set on crates.io yet.',
        },
      ],
      artifacts: [
        {
          format: 'TGZ',
          name: 'acme-atlas-tokens-0.2.0.tgz',
          size: '184 KB',
        },
        {
          format: 'TGZ',
          name: 'acme-atlas-react-0.2.0.tgz',
          size: '412 KB',
        },
        {
          format: 'TAR',
          name: 'atlas_ui-0.2.0.tar.gz',
          size: '96 KB',
        },
      ],
    },
  },

  {
    id: 'lingo',
    name: 'acme/lingo',
    path: '~/Code/lingo',
    branch: 'feat/glossary-import',
    ahead: 1,
    behind: 3,
    provider: 'github',
    account: 'mia',
    seen: 'yesterday',

    branches: [
      { name: 'feat/glossary-import', meta: 'current · ↑1 ↓3' },
      { name: 'main', meta: 'synced' },
      { name: 'chore/shell-migration', meta: '↑4' },
    ],

    sessions: [
      {
        id: 'lingo-glossary',
        title: 'CSV glossary import',
        when: '20 minutes ago',
        status: 'running',
        tools: 9,
        worktree: 'worktrees/88-glossary',
        branch: 'feat/glossary-import',
        issue: 88,
        pull: 91,
        steps: [
          { label: 'Reproduce the blank rows', status: 'done' },
          { label: 'Drop them, and count what was dropped', status: 'done' },
          { label: 'Run flutter test', status: 'active' },
        ],
        seed: [
          {
            id: 'lingo-1',
            kind: 'user',
            text: 'Empty rows in an imported CSV become empty glossary entries. Skip them.',
          },
          {
            id: 'lingo-2',
            kind: 'agent',
            text: 'The parser was treating a trailing newline as a row. It now drops rows whose source term is blank after trimming, and reports the count so an import that silently halves is visible rather than surprising.',
            shown: Number.MAX_SAFE_INTEGER,
          },
        ],
      },
      {
        id: 'lingo-watch-91',
        title: 'Watch the checks on #91',
        when: 'yesterday',
        status: 'watching',
        tools: 0,
        branch: 'feat/glossary-import',
        pull: 91,
        steps: [
          { label: 'Wait for build (macos) to finish', status: 'active' },
          { label: 'Report, and fix if it fails', status: 'idle' },
        ],
        seed: [],
      },
      {
        id: 'lingo-dark',
        title: 'Dark mode for the segment list',
        when: 'Queued',
        status: 'queued',
        tools: 0,
        branch: 'feat/dark-mode',
        steps: [{ label: 'Waiting for a free worktree', status: 'idle' }],
        seed: [],
      },
    ],

    issues: [
      {
        number: 88,
        title: 'Glossary import keeps empty rows',
        state: 'open',
        label: 'bug',
        author: 'ana',
        when: '3 days ago',
        comments: 2,
        agentReady: true,
        pull: 91,
        body: 'A CSV with a trailing newline imports one blank entry per file. They are invisible in the list until a lookup matches the empty string.',
      },
      {
        number: 85,
        title: 'WindowFrame still lives here',
        state: 'open',
        label: 'design-system',
        author: 'mia',
        when: 'last week',
        comments: 5,
        agentReady: false,
        body: 'Every domain-free component has moved to acme/atlas. What is left is the desktop shell — WindowFrame, Sidebar, MiniWindow, BrowserFrame — which is still pending a decision either way.',
      },
      {
        number: 80,
        title: 'Segment list jumps while a translation streams',
        state: 'open',
        label: 'bug',
        author: 'marek',
        when: '2 weeks ago',
        comments: 8,
        agentReady: true,
        body: 'The row grows as the text arrives, and the list is anchored to the top rather than to the row being read.',
      },
    ],

    pulls: [
      {
        number: 91,
        title: 'Import glossaries from CSV',
        state: 'open',
        branch: 'feat/glossary-import',
        base: 'main',
        commits: 4,
        files: 9,
        added: 341,
        removed: 12,
        author: 'ana',
        when: 'yesterday',
        session: 'lingo-glossary',
        fixes: 88,
        body: 'Adds the importer, the column mapping sheet, and the duplicate strategy. Blank rows are dropped and counted.',
        runs: [
          { name: 'analyze', status: 'passed', took: '41s' },
          { name: 'test', status: 'passed', took: '3m 12s' },
          { name: 'build (macos)', status: 'running', took: 'running…' },
        ],
        timeline: [
          {
            author: 'ana',
            initials: 'AN',
            kind: 'opened',
            body: 'opened this pull request',
            when: 'yesterday',
          },
        ],
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

    history: [
      {
        sha: 'c9d41e0',
        message: 'Drop blank rows on import, and count them',
        author: 'agent',
        when: '18 minutes ago',
        agent: true,
      },
      {
        sha: '7b2f0aa',
        message: 'Add the column mapping sheet',
        author: 'ana',
        when: 'yesterday',
        agent: false,
      },
      {
        sha: '10e77c3',
        message: 'Start the CSV importer',
        author: 'ana',
        when: '2 days ago',
        agent: false,
      },
    ],

    worktrees: [
      {
        path: 'worktrees/88-glossary',
        branch: 'feat/glossary-import',
        session: 'lingo-glossary',
      },
    ],

    inbox: [],

    tests: {
      passed: 128,
      failed: 0,
      skipped: 1,
      coverage: 74,
      suites: [
        { path: 'test/glossary', passed: 22, failed: 0 },
        { path: 'test/segments', passed: 61, failed: 0 },
        { path: 'test/providers', passed: 45, failed: 0 },
      ],
      failures: [],
      runs: [
        {
          label: 'flutter test · all passed',
          status: 'passed',
          when: '18 minutes ago',
          took: '46.2s',
        },
        {
          label: 'CI · test · PR #91 · all passed',
          status: 'passed',
          when: 'yesterday',
          took: '3m 12s',
        },
      ],
    },

    release: {
      version: 'v2.4.0',
      active: 'production',
      configFile: 'distribute_options.yaml',
      command: 'dazzui release --name production',
      configs: [
        { name: 'production', jobs: 6, note: '6 jobs · running' },
        { name: 'dev', jobs: 2, note: '2 jobs · PGYER beta' },
      ],
      jobs: [
        {
          kind: 'package',
          name: 'macOS DMG',
          target: 'dmg',
          status: 'passed',
          took: '1m 58s',
        },
        {
          kind: 'package',
          name: 'Windows MSIX',
          target: 'msix',
          status: 'passed',
          took: '2m 12s',
        },
        {
          kind: 'package',
          name: 'Android AAB',
          target: 'aab',
          status: 'passed',
          took: '2m 41s',
        },
        {
          kind: 'package',
          name: 'iOS IPA',
          target: 'ipa',
          status: 'running',
          took: 'running…',
        },
        {
          kind: 'publish',
          name: 'App Store',
          target: 'app-store',
          status: 'queued',
        },
        {
          kind: 'publish',
          name: 'GitHub Releases',
          target: 'github',
          status: 'queued',
        },
      ],
      log: [
        '$ dazzui package --platform=android --targets=aab',
        "Running Gradle task 'bundleRelease'...",
        '✓ dist/2.4.0+124/lingo-2.4.0+124-android.aab (28.4 MB) · 2m 41s',
      ],
      channels: [
        {
          id: 'app-store',
          name: 'App Store',
          state: 'In review',
          tone: 'moving',
          live: 'Live v2.3.1 · v2.4.0 submitted a day ago',
          note: 'TestFlight: v2.4.0 (124) with 32 testers.',
        },
        {
          id: 'play-store',
          name: 'Play Store',
          state: '20% rollout',
          tone: 'live',
          live: 'Production track v2.4.0 · staged rollout',
          note: 'Crash-free 99.92% — clears the bar to raise it to 50%.',
        },
        {
          id: 'github',
          name: 'GitHub Releases',
          state: 'Draft',
          tone: 'idle',
          live: 'Live v2.3.1 · v2.4.0 draft generated',
          note: 'Six attachments still to upload; notes summarised from the log.',
        },
        {
          id: 'pgyer',
          name: 'PGYER',
          state: 'Not configured',
          tone: 'idle',
          live: 'Used by the dev config only',
          note: 'The beta channel the dev release publishes an APK to.',
        },
      ],
      artifacts: [
        {
          format: 'DMG',
          name: 'lingo-2.4.0+124-macos.dmg',
          size: '46.2 MB',
        },
        {
          format: 'MSIX',
          name: 'lingo-2.4.0+124-windows.msix',
          size: '38.9 MB',
        },
        {
          format: 'AAB',
          name: 'lingo-2.4.0+124-android.aab',
          size: '28.4 MB',
        },
        {
          format: 'APK',
          name: 'lingo-2.4.0+124-android.apk',
          size: '31.7 MB',
        },
      ],
    },
  },

  {
    id: 'notes-cli',
    name: 'notes-cli',
    path: '~/Code/notes-cli',
    branch: 'main',
    ahead: 0,
    behind: 0,
    provider: null,
    seen: 'last week',

    branches: [{ name: 'main', meta: 'current' }],

    sessions: [
      {
        id: 'cli-scaffold',
        title: 'Scaffold the CLI',
        when: 'Monday',
        status: 'done',
        tools: 3,
        branch: 'main',
        steps: [{ label: 'Parse the arguments', status: 'done' }],
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
          { tone: 'removed', before: 13, text: '    println!("notes");' },
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

    history: [
      {
        sha: '3f1c2d9',
        message: 'Initial commit',
        author: 'mia',
        when: 'Monday',
        agent: false,
      },
    ],

    worktrees: [],
    inbox: [],
    tests: null,
    release: null,
  },
]

/** The projects the window opens with, as tabs. The rest wait in Welcome. */
export const OPEN_PROJECTS = ['atlas', 'lingo']
