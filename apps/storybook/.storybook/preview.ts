import type { Preview } from '@storybook/react-vite'

import '@dazzlabs/dazzui-react/style.css'
import '@dazzlabs/dazzui-react/components.css'
import '../src/styles/preview.css'

/**
 * The four themes the token graph carries. Studio Light is the default and
 * is what `:root` holds, so it sets no attribute at all — selecting it has
 * to *remove* `data-theme` rather than write `studio-light`, or the Bright
 * and dark blocks would simply not match and the page would look right for
 * the wrong reason.
 */
const THEMES = [
  { value: 'studio-light', title: 'Studio Light' },
  { value: 'studio-dark', title: 'Studio Dark' },
  { value: 'bright-light', title: 'Bright Light' },
  { value: 'bright-dark', title: 'Bright Dark' },
]

const preview: Preview = {
  globalTypes: {
    theme: {
      description: 'Design system theme',
      toolbar: {
        title: 'Theme',
        icon: 'paintbrush',
        items: THEMES,
        dynamicTitle: true,
      },
    },
  },
  initialGlobals: {
    theme: 'studio-light',
  },
  decorators: [
    (Story, context) => {
      const theme = context.globals.theme ?? 'studio-light'
      const root = document.documentElement

      if (theme === 'studio-light') root.removeAttribute('data-theme')
      else root.setAttribute('data-theme', theme)

      return Story()
    },
  ],
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
