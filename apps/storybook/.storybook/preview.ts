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
      /**
       * A comparator rather than the declarative `order` array, because the
       * sorting has to stop at the page: two entries sharing a title are the
       * stories of one component, and returning 0 keeps them in the order
       * they are written in — an overview, then the axes, then the
       * compositions, then the token table. `method: 'alphabetical'` reaches
       * inside a page too, and would open every component on whichever
       * example happens to start with an A.
       *
       * It has to be written inline, in plain JS, reaching nothing outside
       * itself. Storybook lifts this one node out of the file and `eval`s its
       * source on its own to sort the sidebar, so an identifier here is
       * refused outright, a type annotation is a syntax error in that eval,
       * and a constant hoisted to module scope is simply not defined by the
       * time it runs.
       */
      storySort: (a, b) => {
        if (a.title === b.title) return 0

        // Tokens before Components, because the components are drawn from
        // them; and within Tokens, the semantic roles a reader actually uses
        // before the primitives those roles alias. Anything neither list
        // names falls back to alphabetical, which is what sorts the
        // Components group: there is one page per component, so the group is
        // an index you look a name up in.
        const groups = ['Tokens', 'Components']
        const tokenPages = [
          'Color',
          'Typography',
          'Spacing',
          'Radius',
          'Effect',
          'Control',
          'Primitives',
        ]
        const rank = (list, value) => {
          const index = list.indexOf(value)
          return index === -1 ? list.length : index
        }

        const [groupA = '', pageA = ''] = a.title.split('/')
        const [groupB = '', pageB = ''] = b.title.split('/')

        return (
          rank(groups, groupA) - rank(groups, groupB) ||
          groupA.localeCompare(groupB) ||
          rank(tokenPages, pageA) - rank(tokenPages, pageB) ||
          pageA.localeCompare(pageB)
        )
      },
    },
  },
}

export default preview
