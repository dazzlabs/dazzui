import { TooltipProvider } from '@dazzlabs/dazzui'
import { useEffect } from 'react'

import { Shell } from './components/shell'
import { useQuery, useStory } from './lib/location'
import {
  applyTheme,
  parseFamily,
  parseMode,
  parsePlatforms,
  type Theme,
  useSystemDark,
} from './lib/themes'

/**
 * The whole address is the state, as in the desktop example: `?family=`,
 * `?mode=` and `?platforms=` for the panel, the hash for the story.
 */
export function App() {
  const [familyValue, setFamily] = useQuery('family', 'bright')
  const [modeValue, setMode] = useQuery('mode', 'system')
  const [platformsValue, setPlatforms] = useQuery('platforms', 'both')
  const [story, open] = useStory()
  const systemDark = useSystemDark()

  const theme: Theme = {
    family: parseFamily(familyValue),
    mode: parseMode(modeValue),
  }
  const platforms = parsePlatforms(platformsValue)

  useEffect(() => {
    applyTheme(document.documentElement, theme, systemDark)
  }, [theme.family, theme.mode, systemDark])

  return (
    <TooltipProvider>
      <Shell
        story={story}
        open={open}
        theme={theme}
        setTheme={next => {
          setFamily(next.family)
          setMode(next.mode)
        }}
        platforms={platforms}
        setPlatforms={setPlatforms}
      />
    </TooltipProvider>
  )
}
