# Spotify theme

Spotify is an imported application family, offered under **Applications** in
the playgrounds as `spotify-dark` and `spotify-light`. Its definition lives
in `ds/themes/applications.yaml`; `pnpm tokens` emits every implementation,
and `pnpm codegen` carries the selector into every desktop example.

## Research

Reviewed on 2026-10-14:

- [Spotify's desktop redesign](https://newsroom.spotify.com/2023-06-20/spotify-desktop-experience-redesign-your-library-now-playing-views-customize/)
  separates the library, browsing content and now-playing view into panels.
  This theme translates that hierarchy into DazzUI's existing surface roles.
- [Better in Black, by Spotify Design](https://app.notion.com/better-in-black-rethinking-our-most-important-buttons-by-spotify-design-328ef7ee2259424e8f31ddb44f2427cf)
  explains the bright green action with black content, chosen for readability,
  and the move away from uppercase button labels.
- [Spotify's design guidelines](https://developer.spotify.com/documentation/design)
  identify green as the recognizable brand accent, specify 8px artwork
  corners on large devices, and recommend the platform's sans-serif font.

The dark interpretation uses a black canvas, `#121212` main surface,
`#1f1f1f` secondary surface, `#b3b3b3` secondary ink and `#1ed760`
accent with black content. Buttons and search controls are capsules; cards
and floating panels use 8px corners. These are a Spotify-inspired mapping,
not a claim of pixel-for-pixel replication of a particular client release.
The existing DazzUI shell, glyphs, font families, interactions and recipe
states remain shared. Control heights of 24/32/40/48px and a 14px label
translate the client's roomier density into the four DazzUI profiles.

Some DazzUI roles have no direct Spotify counterpart: the recessed column,
inset, sunken and raised steps are adaptations chosen to retain the shared
surface separation guard. Unspecified roles and the accent ramp are derived.
An imported source may specify `colors` for existing semantic roles; those
override derivation, while `background`, `foreground` and `on-accent` own
the main surface, main ink and filled label. The hover wash is derived
against the actual overridden surfaces. Unknown roles fail the build.

**Light is a DazzUI adaptation**, with white paper and a darker green
(`#107c38`) for legibility. It is not presented as an official Spotify
light appearance. Both halves pass the same contrast and surface guards
as the other themes.

## Preview

React: run `pnpm example`, then open
`http://127.0.0.1:6006/?theme=spotify-dark`.

Flutter: from `packages/dart/dazzui/example`, run
`flutter run -d macos --dart-define=PLAYGROUND_THEME=spotify-dark` (or
`-d windows`). Remote runs use `fastforge run --host macbook-pro -p macos`
or `--host windows-laptop -p windows` with the same define.

WinUI 3: build the example and start `Example.exe --theme spotify-dark`.
Select **Applications → Spotify** in any desktop example to switch between
the two appearances.

The Flutter gallery scales its preview tracks with the theme's regular
control height. When the window cannot fit three columns, it folds a side
panel; the toolbar can exchange the list and theme panel. This keeps the
roomier controls visible in the Mac example's 800px opening viewport.

## Verification

Verified on 2026-10-14:

- Local React: running on port 6006; no runtime page errors, with reference
  readings for 25 scenes in each appearance.
- `macbook-pro` and `windows-laptop`: Flutter package and example analysis,
  151 package tests, seven gallery tests at 800/1280/1360px, and 176 collected
  scenes in each appearance. Both native examples run with Spotify Dark,
  with no overflow in their live render trees.
- `windows-laptop`: WinUI 3 built and rendered all 51 component pages plus
  the overview in both appearances (104 renders), with button geometry,
  live state and native interaction checks. Its dark example runs in the
  signed-in desktop session.
- Sampled fill, ink, corner and font-size readings match React: 520 values
  per Flutter host and 170 in WinUI. This is a targeted theme check, not a
  full desktop conformance verdict across every measured property.

Local screenshots and readings are kept in the ignored
`tools/conformance/data/spotify/` directory. Token contrast guards,
codegen drift checks, TypeScript, formatting and lint checks passed; lint
retains its existing Swift generator warning.
