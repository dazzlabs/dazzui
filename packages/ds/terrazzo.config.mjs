import { defineConfig } from '@terrazzo/cli'
import css from '@terrazzo/plugin-css'
import controlPlugin from './plugins/control.mjs'
import flutterPlugin from './plugins/flutter.mjs'

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
    './tokens/global/_design.yaml',
    './tokens/global/color.yaml',
    './tokens/global/control.yaml',
    './tokens/global/effect.yaml',
    './tokens/global/focus.yaml',
    './tokens/global/frame.yaml',
    './tokens/global/radius.yaml',
    './tokens/global/spacing.yaml',
    './tokens/global/typography.yaml',
    './tokens/component/checkbox.yaml',
    './tokens/component/dialog.yaml',
    './tokens/component/radio.yaml',
    './tokens/component/shortcut-recorder.yaml',
    './tokens/component/toggle.yaml',
    './tokens/theme/default/colors.yaml',
  ],
  plugins: [
    css({ filename: 'style.css', modeSelectors }),
    controlPlugin({ modeSelectors }),
    flutterPlugin({ modeSelectors }),
  ],
  outDir: './dist/',
  lint: {
    /** @see https://terrazzo.app/docs/cli/lint */
  },
})
