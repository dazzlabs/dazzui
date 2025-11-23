import { defineConfig } from '@terrazzo/cli'
import css from '@terrazzo/plugin-css'
import flutterPlugin from './plugins/flutter.mjs'

export default defineConfig({
  tokens: [
    './tokens/global/_base.json',
    './tokens/global/color.json',
    './tokens/global/effect.json',
    './tokens/global/radius.json',
    './tokens/global/spacing.json',
    './tokens/global/typography.json',
    './tokens/component/alert-banner.json',
    './tokens/component/badge.json',
    './tokens/component/button.json',
    './tokens/component/card.json',
    './tokens/component/checkbox.json',
    './tokens/component/kbd.json',
    './tokens/theme/default/dark.json',
    './tokens/theme/default/light.json',
  ],
  plugins: [css({ filename: 'style.css' }), flutterPlugin()],
  outDir: './dist/',
  lint: {
    /** @see https://terrazzo.app/docs/cli/lint */
  },
})
