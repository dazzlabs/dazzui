import type { Preview } from '@storybook/react-vite'

import '@dazzlabs/dazzui-tokens/style.css'
import '../src/styles/preview.css'

const preview: Preview = {
  parameters: {
    layout: 'fullscreen',
    controls: { disable: true },
    options: {
      storySort: {
        order: [
          'Tokens',
          [
            'Color',
            'Typography',
            'Spacing',
            'Radius',
            'Effect',
            'Control',
            'Component',
          ],
          'Base',
        ],
      },
    },
  },
}

export default preview
