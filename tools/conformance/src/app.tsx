import { EmptyState, Spinner, TooltipProvider } from '@dazzlabs/dazzui'
import { useEffect } from 'react'

import { Shell } from './components/shell'
import { useLoaded } from './lib/data'
import { useLens } from './lib/lens'
import { usePage } from './lib/location'
import type { Loaded } from './lib/model'
import { applyTheme, themeId } from './lib/themes'

export function App() {
  const state = useLoaded()
  if (!state) {
    return (
      <div className="g-centre">
        <Spinner aria-label="Loading the run" />
      </div>
    )
  }
  if ('error' in state) {
    return (
      <div className="g-centre">
        <EmptyState title={`No run to show: ${state.error}`} />
      </div>
    )
  }
  return <Window loaded={state.loaded} />
}

function Window({ loaded }: { loaded: Loaded }) {
  const lens = useLens(loaded)
  const [page, open] = usePage()
  const id = themeId(lens.theme)

  useEffect(() => {
    applyTheme(document.documentElement, lens.theme)
  }, [id])

  return (
    <TooltipProvider>
      <Shell loaded={loaded} lens={lens} page={page} open={open} />
    </TooltipProvider>
  )
}
