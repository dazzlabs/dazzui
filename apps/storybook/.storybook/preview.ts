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
        // Anything the order below does not name falls back to alphabetical,
        // which is what sorts the Components group: there is one page per
        // component, so the group is an index you look a name up in.
        method: 'alphabetical',
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
        ],
      },
    },
  },
}

export default preview
