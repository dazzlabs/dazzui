import { defineConfig } from '@terrazzo/cli'
import css from '@terrazzo/plugin-css'
import contrastPlugin from './plugins/contrast.mjs'
import composePlugin from './plugins/compose.mjs'
import controlPlugin from './plugins/control.mjs'
import flutterPlugin from './plugins/flutter.mjs'
import gpuiPlugin from './plugins/gpui.mjs'
import gtk4Plugin from './plugins/gtk4.mjs'
import harmonyPlugin from './plugins/harmony.mjs'
import harmonyMobilePlugin from './plugins/harmony-mobile.mjs'
import mobileCSSPlugin from './plugins/mobile-css.mjs'
import runtimePlugin from './plugins/runtime.mjs'
import swiftUIPlugin from './plugins/swiftui.mjs'
import winUI3Plugin from './plugins/winui3.mjs'
import { importedSelectors } from './scripts/themes.mjs'

/**
 * Six drawn families over one default, and the imported ones — Omarchy,
 * desktops and applications, each light and dark.
 *
 * Bright Light is the default and needs no mode: it is what `:root` holds.
 * Every other theme is selected by `data-theme`, and the modes compose rather
 * than repeat — a family mode is selected by both of its themes and `dark` by
 * every dark one, so a value shared along either axis is written once and a
 * `<family>-dark` mode carries only what is true of that corner alone.
 *
 * `studio` is the baseline rather than a family mode, and it is selected by
 * every theme but Bright's pair. Those are the values the design was drawn at
 * first, and every other theme — the drawn families and the imported ones
 * alike — is written as a re-pointing of them rather than as a second whole
 * palette. Carrying that in one mode is what lets `:root` move to Bright
 * without a copy of Studio in every theme: a token Bright moves keeps
 * Studio's value under `studio`, the family mode after it re-points what that
 * family changes, and nothing else moves. Studio's own two themes are simply
 * where nothing overrides the baseline again.
 *
 * The order is load bearing. All of these are single attribute selectors, so
 * they have equal specificity and the later block wins: the baseline first,
 * for the tokens the default re-points; then the family modes, for the
 * corners and the accent each family re-points; then `dark`, so a dark theme
 * takes the dark surfaces, the lifted status ramps and the dark recipe shades
 * rather than its family's light paper; then the four dark corners, each for
 * the accent that is its alone.
 *
 * Frost is the one family whose accent is written once: a teal reads
 * correctly in both directions, so `frost` carries the whole ramp and
 * `frost-dark` re-points only surfaces and ink, where the other three each
 * invert or lift their ramp in the dark; Nocturne's blurple is written once
 * the same way. Graphite and Nocturne are the two families that move
 * geometry — each carries the tighter corner ladder for both of its themes.
 *
 * The selector is an attribute rather than a media query because the theme is
 * a choice the app makes, not one the OS makes for it; a host that wants to
 * follow the OS sets the attribute from `prefers-color-scheme` itself.
 *
 * The imported themes are read off another system rather than drawn:
 * Omarchy's twenty-two, `omarchy-<name>-*`, and the desktops' — `macos27-*`,
 * `macos15-*`, `windows11-*`, `ubuntu-*`, `debian-*`, `fedora-*`, `kde-*`. They come from
 * `themes/*.yaml` and take the same three places in the order: a metrics
 * mode beside the families (Hyprland's square corner, AppKit's capsule,
 * Adwaita's 9px button), `dark` for the dark ones, and a mode per theme
 * last, carrying the roles `scripts/themes.mjs` derives from its palette.
 *
 * These are the six drawn families. `bright` is the default and is applied by
 * holding no attribute at all, so its pair is the only one the baseline
 * leaves alone.
 */
const DRAWN = ['studio', 'bright', 'frost', 'graphite', 'ember', 'nocturne']

const themeSelector = theme => `[data-theme='${theme}']`

/**
 * The baseline's selectors: every theme but Bright's pair, which is the
 * default and so is not re-pointed at all. Studio's own themes are in it.
 */
const BASELINE = [
  ...DRAWN.filter(family => family !== 'bright').flatMap(family => [
    themeSelector(`${family}-light`),
    themeSelector(`${family}-dark`),
  ]),
  ...importedSelectors.corners.map(corner => corner.selectors[0]),
]

export const modeSelectors = [
  {
    mode: 'studio',
    selectors: BASELINE,
    scheme: 'light',
  },
  {
    mode: 'frost',
    selectors: ["[data-theme='frost-light']", "[data-theme='frost-dark']"],
    scheme: 'light',
  },
  {
    mode: 'graphite',
    selectors: [
      "[data-theme='graphite-light']",
      "[data-theme='graphite-dark']",
    ],
    scheme: 'light',
  },
  {
    mode: 'ember',
    selectors: ["[data-theme='ember-light']", "[data-theme='ember-dark']"],
    scheme: 'light',
  },
  {
    mode: 'nocturne',
    selectors: [
      "[data-theme='nocturne-light']",
      "[data-theme='nocturne-dark']",
    ],
    scheme: 'light',
  },
  ...importedSelectors.families,
  {
    mode: 'dark',
    selectors: [
      ...DRAWN.map(family => `[data-theme='${family}-dark']`),
      ...importedSelectors.dark,
    ],
    scheme: 'dark',
  },
  {
    mode: 'bright-dark',
    selectors: ["[data-theme='bright-dark']"],
    scheme: 'dark',
  },
  {
    mode: 'frost-dark',
    selectors: ["[data-theme='frost-dark']"],
    scheme: 'dark',
  },
  {
    mode: 'graphite-dark',
    selectors: ["[data-theme='graphite-dark']"],
    scheme: 'dark',
  },
  {
    mode: 'ember-dark',
    selectors: ["[data-theme='ember-dark']"],
    scheme: 'dark',
  },
  {
    mode: 'nocturne-dark',
    selectors: ["[data-theme='nocturne-dark']"],
    scheme: 'dark',
  },
  ...importedSelectors.corners,
]

/**
 * The themes the mobile packages carry: those that come in pairs. Their
 * generators hand a host a family and let the OS pick its light or dark
 * half, so a theme without a `-light`/`-dark` counterpart would have nowhere
 * to go and stays on the desktop platforms, where a theme is chosen whole.
 * Every theme comes in pairs today; the filter keeps a future single-mode
 * import from breaking the mobile builds. Bright Light is `:root` and pairs
 * with `bright-dark`.
 */
const themeOf = selector => /\[data-theme='([^']+)'\]/.exec(selector)?.[1]
const allThemes = new Set([
  'bright-light',
  ...modeSelectors.flatMap(entry => entry.selectors.map(themeOf)),
])
const paired = theme => {
  const match = /^(.+)-(light|dark)$/.exec(theme)
  if (!match) return false
  const other = match[2] === 'light' ? 'dark' : 'light'
  return allThemes.has(`${match[1]}-${other}`)
}
const pairedModeSelectors = modeSelectors
  .map(entry => ({
    ...entry,
    selectors: entry.selectors.filter(selector => paired(themeOf(selector))),
  }))
  .filter(entry => entry.selectors.length > 0)

export default defineConfig({
  tokens: [
    './tokens/global/_base.yaml',
    './tokens/global/color.yaml',
    './tokens/global/control.yaml',
    './tokens/global/effect.yaml',
    './tokens/global/focus.yaml',
    './tokens/global/frame.yaml',
    './tokens/global/icon.yaml',
    './tokens/global/motion.yaml',
    './tokens/global/radius.yaml',
    './tokens/global/spacing.yaml',
    './tokens/global/stroke.yaml',
    './tokens/global/wash.yaml',
    './tokens/global/typography.yaml',
    './tokens/component/calendar.yaml',
    './tokens/component/checkbox.yaml',
    './tokens/component/date-range-picker.yaml',
    './tokens/component/dialog.yaml',
    './tokens/component/drawer.yaml',
    './tokens/component/icon-button.yaml',
    './tokens/component/menu.yaml',
    './tokens/component/preferences.yaml',
    './tokens/component/preview-card.yaml',
    './tokens/component/progress.yaml',
    './tokens/component/radio.yaml',
    './tokens/component/search-field.yaml',
    './tokens/component/segmented-control.yaml',
    './tokens/component/shortcut-recorder.yaml',
    './tokens/component/slider.yaml',
    './tokens/component/toast.yaml',
    './tokens/component/tabs.yaml',
    './tokens/component/tooltip.yaml',
    './tokens/component/tree.yaml',
    './tokens/component/switch.yaml',
  ],
  plugins: [
    css({
      filename: 'style.css',
      modeSelectors,
      /**
       * Everything under `control.color` belongs to the control plugin: the
       * shade and alpha leaves are build-time inputs it resolves, and even
       * the literal-colour leaves are re-emitted by it beside the resolved
       * recipes. Letting the CSS plugin emit them too put ninety-odd inert
       * numbers into every page.
       */
      exclude: [
        'control.color.**',
        /**
         * The same rule, for the recipes that live outside `control.color`.
         * A shade and an alpha here are build-time inputs the control plugin
         * resolves into `--focus-ring`, `--focus-glow`, `--dialog-scrim` and
         * the baked press alphas. Emitting the raw inputs beside the finished
         * values is not just eight dead lines: it publishes a lever that does
         * nothing. A consumer setting `--focus-ring-alpha` would reasonably
         * expect the ring to move, and it would not.
         */
        'focus.ring.*',
        'focus.glow.*',
        'control.pressed.*',
        'dialog.scrim.alpha',
      ],
      /**
       * Emit hex-6/hex-8 instead of `color(srgb 0.9647058823529412 …)`.
       * The Flutter plugin already collapses every ramp — the oklch ones
       * included — to 8-digit hex `Color`s, so hex precision is the floor
       * the design already ships at; this keeps the CSS at the same one.
       */
      legacyHex: true,
    }),
    contrastPlugin({ modeSelectors }),
    controlPlugin({ modeSelectors }),
    runtimePlugin({ modeSelectors }),
    flutterPlugin({ modeSelectors }),
    swiftUIPlugin({ modeSelectors, mobileModeSelectors: pairedModeSelectors }),
    winUI3Plugin({ modeSelectors }),
    gpuiPlugin({ modeSelectors }),
    gtk4Plugin({ modeSelectors }),
    harmonyPlugin({ modeSelectors }),
    harmonyMobilePlugin({ modeSelectors: pairedModeSelectors }),
    composePlugin({ modeSelectors: pairedModeSelectors }),
    mobileCSSPlugin({ modeSelectors: pairedModeSelectors }),
  ],
  outDir: './dist/',
  lint: {
    /**
     * @see https://terrazzo.app/docs/cli/lint
     *
     * Empty on purpose: Terrazzo 0.10 lints each token file with only that
     * file's own tokens, so a contrast rule pairing the semantic roles
     * cannot run here. `plugins/contrast.mjs` does that job instead, on the
     * merged graph and across every theme rather than the default alone.
     */
  },
})
