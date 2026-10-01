# Themes

The twelve drawn themes, the imported ones, and how each platform carries a
theme.

Twelve drawn themes, as six families over one default, plus the imported
themes below — defined once in `terrazzo.config.mjs` and passed to
every plugin so the platforms cannot drift:

| `data-theme`     | modes applied                     |
| ---------------- | --------------------------------- |
| *(none)*         | — Studio Light, `:root`           |
| `studio-dark`    | `dark`                            |
| `bright-light`   | `bright`                          |
| `bright-dark`    | `bright`, `dark`, `bright-dark`   |
| `frost-light`    | `frost`                           |
| `frost-dark`     | `frost`, `dark`                   |
| `graphite-light` | `graphite`                        |
| `graphite-dark`  | `graphite`, `dark`, `graphite-dark` |
| `ember-light`    | `ember`                           |
| `ember-dark`     | `ember`, `dark`, `ember-dark`     |
| `nocturne-light` | `nocturne`                        |
| `nocturne-dark`  | `nocturne`, `dark`, `nocturne-dark` |

The modes compose rather than repeat, so a value shared along the light/dark
axis or along a family is written once. A family whose accent reads correctly
in both directions needs no dark corner at all — Frost is the one that does
not have one, because a teal is a teal on either canvas, where Bright,
Graphite and Ember each invert or lift their ramp and so carry a second
ladder. Nocturne's blurple is written once the same way, and its dark corner
carries only the surfaces and ink of the design system it is read off — the
Tiller mockups' own ground, `#161826` under `#e9e9ed`. The order in that config is load bearing: every selector is a single
attribute and so has equal specificity, and the later block wins — the family
modes first, then `dark`, then the corners.

Graphite and Nocturne are the two families that move geometry: `radius.small`
through `radius.big` and both `frame` corners step down one rung of the same
ladder. Graphite does it to draw a crisper monochrome corner; Nocturne because
the mockups round a row at 7 and a card at 8, which is exactly where that rung
lands.

**The imported themes are read off another system, not drawn.** Beside the
twelve sit two sources under `ds/themes/`:

- `omarchy.yaml` — [Omarchy](https://github.com/basecamp/omarchy)'s
  twenty-two themes from its `colors.toml` files, as
  `omarchy-<name>-light` / `-dark`. Omarchy ships each in one appearance;
  that one is exact, and the other is derived from the same three colours
  (`opposite` in the file, `bothModes` in the script) — the foreground
  becomes the paper, the paper the ink, the accent pressed or lifted to stay
  legible. The derived half is DazzUI's reading, not Omarchy's.
- `desktops.yaml` — seven desktops in light and dark: `macos27-*`,
  `macos15-*`, `windows11-*`, `ubuntu-*`, `debian-*`, `fedora-*`, `kde-*`,
  from the values dazzle's prototype draws its native windows with (AppKit,
  WinUI 3 / Fluent 2, libadwaita, Yaru, Breeze). Debian and Fedora ship
  Adwaita unchanged, so they are the same theme under two names.

A file holds two things. Per theme: a title, a scheme, and the three colours
the source builds a window from — background, foreground (an alpha is
composited over the background, as AppKit's 85% label is), accent — and,
where the source says so, `on-accent`. And `metrics`: the geometry a family
shares, as token path → value, applied under a family mode (`omarchy` —
Hyprland's square corner and the shell's control and type sizes; `macos27` — the capsule; `macos15`; `windows11`;
`adwaita`, which Debian, Fedora and Ubuntu all wear; `kde`). A desktop's
metrics are its own control heights (`control.<size>.size`), its button
inset, and a `type` scale — `[size, leading]` per `base.type` step plus
`label`, the size its controls set their text in — which the script spreads
over every face set at that step, keeping the design's family and weight. A theme applies
its metrics mode, then `dark` if it is a dark one, then a mode named after
the theme itself. Adding a theme is an entry in one of those files; the
config reads its selectors from them, and a metrics path no token file
declares fails the build.

Glyphs use the shared `icon.small / medium / large` scale: 12/16/20px
in the drawn families, Omarchy, macOS and KDE; 14/20/24px in Windows 11
and the GNOME families. These are DazzUI's density decisions, identical in
light and dark. Auxiliary glyphs (chevrons, checks, dismiss) use small;
leading and navigation glyphs use medium. Sized controls bind their icon
through `control.<size>.icon-size`: tiny uses small, small and medium use
medium, large uses large. Marks inside constrained boxes fit that box.
React's `Icon` defaults to its enclosing slot's size, or `icon.medium` when
standalone. An explicit `size` or inline width/height overrides the default.
Slots size raw SVGs as well, without changing avatars or spinners.
Flutter installs the medium default through `IconTheme` and scopes each slot;
SwiftUI's `Icon` reads an inherited slot size before falling back to the theme.
Both preserve explicit sizes and share the same control profiles. Generated
native theme instances resolve spacing aliases to constants, so desktop
metrics reach the native implementations as well as CSS.

Everything else is derived by `scripts/themes.mjs` before the graph is
parsed, alongside the recipe expansion and in the same spirit: nothing is
written back. `surface` is the background exactly; the rest of the surface
ladder steps off it towards the foreground (a dark column steps under it) by
the CIE L* intervals `plugins/contrast.mjs` guards, a little above the
floors; the quieter inks are the foreground mixed back into the paper and
pushed up to 4.5:1; hairlines are the foreground at the source's own
`separator` alpha where it states one (AppKit's 7–8%, Fluent's 8%,
Adwaita's 10%), else at the shared 13/16%, with the floating edge a rung
above, and `border-muted` at 38%; `on-accent` is the source's, else the
paper, else the ink, whichever clears 3:1. The primary ramp is written into
`base.color` under the theme's name — OKLCH through the accent, with the
accent itself at the step `filled` picks (600 on paper, 500 on a dark
ground), so a filled button is the source's accent to the digit — and the
text steps a recipe reads are pushed to 4.5:1 on the paper. The contrast
guard then holds every imported theme exactly as it holds a drawn one.
The face stays the design's own; the scale is the platform's.

**Flutter also reads the Omarchy theme a machine is wearing now, at run
time.** The twenty-two in `omarchy.yaml` are the built-in ones; a user's own
theme, or one installed from a repository, is known only to the machine. So
`packages/dart/dazzui/lib/src/omarchy/derive.dart` transcribes `derive` and
`bothModes` step for step. It lays the derived colours over a generated
Omarchy instance of the same scheme, because everything that is not a colour
is the same for every Omarchy theme of one scheme. `test/src/omarchy_test.dart`
checks it against every generated `omarchy-*` instance in both halves, to the
digit, so a change to `scripts/themes.mjs` that is not carried into the Dart
file fails that test. `OmarchyPalette` reads `background`, `foreground`,
`accent` and `mode` out of `colors.toml`. `readOmarchyPalette` finds the file
under `$XDG_STATE_HOME/omarchy/current/theme/`, or under the older
`~/.config/omarchy/current/theme/`. The palette also carries an
`OmarchyShellStyle`, which holds the sizes the shell sets its text and
controls at. It is read the way the shell reads it: the `[font]` and
`[spacing]` sections of the theme's `shell.toml`, then of
`~/.config/omarchy/shell.toml`, whose keys win. Every type step is
`round(base-size × multiplier)` unless it is pinned. Every stored length
(the spacing ladder, which DazzUI derives its paddings, gaps and boxes from,
plus control heights, a button's side inset, icons, frame and component
widths) is scaled as the shell's `space(px)` scales one: by `[spacing] scale`,
and by `base-size / 12` while `scale-with-font` is on. `control-height` and
`control-padding-x` can be pinned. Corners, strokes and the focus ring stay.
At the defaults this moves nothing from what `omarchy.yaml` gives the
generated themes, and a test checks the two against each other.
`watchOmarchyPalette` watches the `current/` directory, which
`omarchy theme set` swaps whole, and `~/.config/omarchy/` as well. It emits
again on every switch and every edit to the user's `shell.toml`. `OmarchyTheme` is a widget that does both and falls back to
a given theme off Omarchy. On the web these return nothing. The Flutter
example wears the machine's theme by default when there is one, offered as
the first tile under *This machine*, until another tile is picked.

Two things a theme re-sizes needed the native generators to store what they
used to compute, following the rule a re-picked recipe shade already had: a
type style that carries a mode keeps `<style>FontSize` and
`<style>LineHeight` as stored members in Dart (the style stays a getter
over them), and a length that carries a mode but aliases a semantic step
(`control.medium.padding.inline` → `spacing.4`) is stored at its resolved
default in Dart and Swift, so a theme instance has a parameter to pass.
That is only sound while no theme moves the step it aliases — a theme that
did would leave the stored field behind — so both generators fail the build
when a themed length aliases a themed token; such a role is stated against
`base` with a mode per family, as `control.container.radius` and
`control.field.radius` are.

The mobile packages carry only themes that come in `-light`/`-dark` pairs —
they hand a host a family and let the OS pick its half — which every theme
now does, so the desktops and Omarchy reach them as families. The React
mobile package is the third of them and carries a theme the web's way:
`plugins/mobile-css.mjs` writes each family's two halves as
`[data-family]` blocks, the dark one both under `[data-appearance='dark']`
and under a `prefers-color-scheme: dark` media rule fenced to
`:not([data-appearance='light'])`, so `MobileTheme` follows the OS unless
told otherwise, exactly as `mobileTheme` and `DazzTheme` do. Each block
also sets `color-scheme`, which is what lets the glass material pick its
highlight with `light-dark()` rather than carry a dark block of its own. The React
playground picks a theme in two steps, *style* then *mode*
(`THEME_STYLES`, `splitTheme` and `joinTheme` in `themes.ts`; the URL still
carries one `theme`), and a style that lacks a mode has it disabled. The
Flutter playground does the same with a grouped `Select` and a brightness
(`playgroundStyleGroups` in `example/lib/playground_theme.dart`), reading
each theme off the generated `themeVariablesByName` map — every theme by
its `data-theme` name — rather than a case per theme; a test holds that
list to the map both ways, and draws the control pages under every imported
theme. The imported styles stay out of `THEMES` and `PlaygroundTheme`, and
so out of the conformance reference, the golden suite and the every-story test:
the ports are held to the drawn families.

Studio Light sets **no attribute at all** — selecting it means removing
`data-theme`, not writing `studio-light`. Writing it would leave the page
looking right for the wrong reason, since none of the theme blocks would
match. The React playground does exactly this; see `packages/js/dazzui/example/src/lib/themes.ts`.

Flutter has no cascade, so `plugins/flutter.mjs` collapses each theme's mode
chain into a whole `ThemeVariables` instance — `themeVariablesStudioDark`,
`themeVariablesBrightLight`, `themeVariablesBrightDark` — beside the default
`themeVariables`. It reads the themes back out of the same `modeSelectors`,
so adding a theme reaches Dart without the plugin being touched. Swift does
the same thing in its own idiom, an initialiser with defaults and computed
properties. Rust has struct update syntax in a `const`, so
`plugins/gpui.mjs` resolves every theme whole — by the walk WinUI's snapshot
takes, shared in `plugins/resolve.mjs` — and writes each as the fields that
differ from Studio Light's over `..STUDIO_LIGHT`. Only the tint is left
open: a recipe cell that picks a shade off the control's ramp stays a
`Paint::Shade`, resolved where it is drawn.

The Dart class also carries a generated `copyWith` over every stored field,
and each type *face* is one of those fields — `fontUi`, `fontDisplay`, built
from `base.font.*` and read by every style set in them. A design drawn to the
platform's own UI face is `fontFamily: null`, which is right on a Mac and
nothing at all on Windows or in a `flutter test`, so a host has to be able to
say what the kit is set in without re-typing a hundred-parameter constructor.
The components meet it halfway: every ambient style they set is a
`DefaultTextStyle.merge`, never a replacement, so a family set above them
reaches a `Badge`'s label as well as a `Text`'s. The styles read a face's
`resolvedFamily` rather than its `family`, because Flutter on Linux never
consults the fallback list for a glyph the default face lacks: Chinese comes
out of whatever fontconfig picks, a Japanese or a serif cut. There, a face
that asks for the platform's own is set in the last family its stack names
(Noto Sans CJK SC), and its Latin with it.

A recipe that names a semantic role in any state (every variant's
`disabled` does, through `$defaults`) is a computed member on those three
platforms, since it reads roles of the same instance. A *shade* on it that a
theme re-picks cannot be baked into a computed member, so each such shade is
a stored member of its own — `controlColorTintedContentNormalShade`,
`control_color_filled_surface_normal_shade` — which the theme instance
re-points and the computed descriptor reads. Only shades that carry a mode
get one; the rest stay literal. This is what the reference tests caught first:
every dark-theme control colour on Swift and in the first Rust port was one
step off.
