import type { Preview } from '@storybook/react-vite'

import '@dazzlabs/dazzui-react/style.css'
import '@dazzlabs/dazzui-react/components.css'
import '../src/styles/preview.css'

const preview: Preview = {
  parameters: {
    layout: 'fullscreen',
    controls: { disable: true },
    options: {
      storySort: {
        // Tokens before Components, because the components are drawn from
        // them; and within Tokens, the semantic roles a reader actually uses
        // before the primitives those roles alias.
        order: [
          'Tokens',
          [
            'Color',
            'Typography',
            'Spacing',
            'Radius',
            'Effect',
            'Control',
            'Primitives',
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
        ],
      },
    },
  },
}

export default preview
