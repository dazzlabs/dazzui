import { defineConfig } from '@terrazzo/cli'
import css from '@terrazzo/plugin-css'
import controlPlugin from './plugins/control.mjs'
import flutterPlugin from './plugins/flutter.mjs'

export default defineConfig({
  tokens: [
    './tokens/global/_base.json',
    './tokens/global/color.json',
    './tokens/global/effect.json',
    './tokens/global/focus.json',
    './tokens/global/radius.json',
    './tokens/global/spacing.json',
    './tokens/global/typography.json',
    './tokens/component/alert-banner.json',
    './tokens/component/badge.json',
    './tokens/component/button.json',
    './tokens/component/card.json',
    './tokens/component/checkbox.json',
    './tokens/component/dialog.json',
    './tokens/component/empty-state.json',
    './tokens/component/field.json',
    './tokens/component/icon-button.json',
    './tokens/component/kbd.json',
    './tokens/component/menu.json',
    './tokens/component/option-card.json',
    './tokens/component/pill-tabs.json',
    './tokens/component/preference.json',
    './tokens/component/progress.json',
    './tokens/component/radio.json',
    './tokens/component/search-field.json',
    './tokens/component/section-label.json',
    './tokens/component/segmented-control.json',
    './tokens/component/shortcut-recorder.json',
    './tokens/component/spinner.json',
    './tokens/component/step-list.json',
    './tokens/component/table.json',
    './tokens/component/toast.json',
    './tokens/component/toggle.json',
    './tokens/theme/default/dark.json',
    './tokens/theme/default/light.json',
  ],
  plugins: [css({ filename: 'style.css' }), controlPlugin(), flutterPlugin()],
  outDir: './dist/',
  lint: {
    /** @see https://terrazzo.app/docs/cli/lint */
  },
})
