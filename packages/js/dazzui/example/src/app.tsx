import { TooltipProvider } from '@dazzlabs/dazzui'
import { useEffect } from 'react'

import { usePage, useQuery } from './lib/location'
import { applyTheme, parseTheme, themeId } from './lib/themes'
import { Shell } from './components/shell'

export function App() {
  const [themeValue, setThemeValue] = useQuery('theme')
  const [page, open] = usePage()
  const theme = parseTheme(themeValue)
  const id = themeId(theme)

  useEffect(() => {
    applyTheme(document.documentElement, parseTheme(id))
  }, [id])

  return (
    <TooltipProvider>
      <Shell
        page={page}
        open={open}
        theme={theme}
        setTheme={next => setThemeValue(themeId(next))}
      />
    </TooltipProvider>
  )
}
