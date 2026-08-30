import type { Preview } from '@storybook/react-vite'

import '@dazzlabs/dazzui/style.css'
import '@dazzlabs/dazzui/components.css'
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
          'Components',
          [
            'Button',
            'IconButton',
            'Selection',
            'Field',
            'Pickers',
            'ActionBar',
            'AlertBanner',
            'Badge',
            'Card',
            'Dialog',
            'Divider',
            'EmptyState',
            'Icon',
            'Kbd',
            'Menu',
            'OptionCard',
            'Preference',
            'Progress',
            'SectionLabel',
            'ShortcutRecorder',
            'Spinner',
            'StepList',
            'Table',
            'Toast',
          ],
          'Base',
        ],
      },
    },
  },
}

export default preview
