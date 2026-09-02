import { defineConfig } from '@terrazzo/cli'
import css from '@terrazzo/plugin-css'
import contrastPlugin from './plugins/contrast.mjs'
import controlPlugin from './plugins/control.mjs'
import flutterPlugin from './plugins/flutter.mjs'
import gpuiPlugin from './plugins/gpui.mjs'
import swiftUIPlugin from './plugins/swiftui.mjs'

/**
 * The four themes, as three modes over one default.
 *
 * Studio Light is the default and needs no mode: it is what `:root` holds.
 * The other three are selected by `data-theme`, and the modes compose rather
 * than repeat — `bright` is selected by both Bright themes and `dark` by both
 * dark ones, so a value shared along either axis is written once and
 * `bright-dark` carries only what is true of that corner alone.
 *
 * The order is load bearing. All of these are single attribute selectors, so
 * they have equal specificity and the later block wins: `bright` first for
 * the pill corners and the ink accent, then `dark` so Bright Dark takes the
 * dark surfaces and the dark recipe shades rather than Bright Light's warm
 * paper, then `bright-dark` for the acid accent that is its alone.
 *
 * The selector is an attribute rather than a media query because the theme is
 * a choice the app makes, not one the OS makes for it; a host that wants to
 * follow the OS sets the attribute from `prefers-color-scheme` itself.
 */
const modeSelectors = [
  {
    mode: 'bright',
    selectors: ["[data-theme='bright-light']", "[data-theme='bright-dark']"],
    scheme: 'light',
  },
  {
    mode: 'dark',
    selectors: ["[data-theme='studio-dark']", "[data-theme='bright-dark']"],
    scheme: 'dark',
  },
  {
    mode: 'bright-dark',
    selectors: ["[data-theme='bright-dark']"],
    scheme: 'dark',
  },
]

export default defineConfig({
  tokens: [
    './tokens/global/_base.yaml',
    './tokens/global/color.yaml',
    './tokens/global/control.yaml',
    './tokens/global/effect.yaml',
    './tokens/global/focus.yaml',
    './tokens/global/frame.yaml',
    './tokens/global/motion.yaml',
    './tokens/global/radius.yaml',
    './tokens/global/spacing.yaml',
    './tokens/global/stroke.yaml',
    './tokens/global/wash.yaml',
    './tokens/global/typography.yaml',
    './tokens/component/checkbox.yaml',
    './tokens/component/dialog.yaml',
    './tokens/component/menu.yaml',
    './tokens/component/progress.yaml',
    './tokens/component/radio.yaml',
    './tokens/component/segmented-control.yaml',
    './tokens/component/shortcut-recorder.yaml',
    './tokens/component/toast.yaml',
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
    flutterPlugin({ modeSelectors }),
    swiftUIPlugin({ modeSelectors }),
    gpuiPlugin({ modeSelectors }),
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
