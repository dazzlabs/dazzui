# Playground generation

Source: `packages/ui_react/example/stories/`. See the React example README for
authoring, host automation, commands, and the current migration boundary.

- `catalog.mjs` reads TSX declarations and preserves stable IDs and source order.
- `schema/value.mjs` validates and evaluates shared conditional content and derived form properties.
- `schema/scene.mjs` validates portable content, state and checkpoints.
- `migrate-static.mjs` imports supported static TSX into data; imperative code is rejected.
- `completeness.mjs` enforces the final all-story/all-target acceptance gate.
- `swift-scene.mjs` compiles shared nodes, bindings and actions into SwiftUI views.
- `ios-scene.mjs` maps the shared content, state, slots, actions and timers to system SwiftUI controls for the mobile gallery.
- `android.mjs` emits the shared Android asset catalog and expression character set; the Compose host interprets these with native controls and a controllable frame clock.
- `native.mjs` emits native scene data/builders and a per-platform worklist.
- `format.mjs` runs Dart formatting before output comparison.
- `generate.mjs` writes outputs or checks for drift with `--check`.
- `outputs.json` records owned files and their hashes. Obsolete owned files are
  removed only if their content still matches the generator's last output.
- `coverage.json` is generated coverage, not a claim of visual equivalence.

Never edit emitted sources to change a specimen. Extend the schema and adapter,
edit the scene source, run generation, and test the consuming example. Unknown
components, properties, bindings, targets, duplicate IDs and unrecognized exports
fail generation. Native hosts keep using their existing implementations for
stories not yet covered; those IDs remain visible in the worklist.

The pipeline currently emits a data-backed Flutter adapter, typed SwiftUI
scenes, iOS and Android system-control catalogs, and static Button and Badge
builders for WinUI 3. Completing the remaining scene and
adapter work is required before all native example content has a single source.

The GPUI and Inset compilers (`gpui-scene.mjs`, `native-catalog.mjs`,
`inset-scene.mjs`) live on the `platform/gpui` branch with the two packages
they emit into.

All content and interactions must be shared declarations. Story-specific native
fixtures are migration debt, never an allowed escape hatch. `pnpm playgrounds:complete`
is the strict completion check; `pnpm playgrounds:check` only checks output freshness.
