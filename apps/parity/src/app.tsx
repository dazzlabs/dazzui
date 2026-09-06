import { PillTabs, SegmentedControl } from '@dazzlabs/dazzui-react'
import { useEffect, useMemo, useState } from 'react'
import data from 'virtual:parity'

import { Overview } from './components/overview'
import { ParitySidebar } from './components/parity-sidebar'
import { SpecimenList } from './components/specimen-card'
import {
  driftingFields,
  THEMES,
  type PlatformId,
  type Specimen,
  type ThemeId,
} from './lib/model'
import { applyTheme } from './lib/theme'

type Scope = 'all' | PlatformId

/** Where the viewer is pointed: the overview, or one component's specimens. */
type Selection = { kind: 'overview' } | { kind: 'component'; className: string }

export function App() {
  const [theme, setTheme] = useState<ThemeId>('studio-light')
  const [scope, setScope] = useState<Scope>('all')
  const [query, setQuery] = useState('')
  const [selection, setSelection] = useState<Selection>({ kind: 'overview' })

  // The viewer wears the theme it is showing values for, so the crop of
  // the React rendering and the numbers beside it are of the same thing.
  useEffect(() => applyTheme(theme), [theme])

  const platforms = useMemo(
    () =>
      data.platforms.filter(
        platform => scope === 'all' || platform.id === scope
      ),
    [scope]
  )

  /** Drifting checks per component, for the sidebar's counts. */
  const counts = useMemo(() => {
    const out = new Map<string, number>()
    for (const specimen of data.specimens) {
      let n = 0
      for (const platform of platforms)
        n += driftingFields(specimen, platform).size
      out.set(specimen.className, (out.get(specimen.className) ?? 0) + n)
    }
    return out
  }, [platforms])

  const classNames = useMemo(
    () => [...new Set(data.specimens.map(s => s.className))].sort(),
    []
  )

  const shown: Specimen[] = useMemo(() => {
    if (selection.kind !== 'component') return []
    return data.specimens.filter(s => s.className === selection.className)
  }, [selection])

  return (
    <div className="pv-root">
      <ParitySidebar
        classNames={classNames}
        counts={counts}
        query={query}
        onQueryChange={setQuery}
        selected={selection.kind === 'component' ? selection.className : null}
        onSelectOverview={() => setSelection({ kind: 'overview' })}
        onSelect={className => setSelection({ kind: 'component', className })}
      />
      <div className="pv-main">
        <div className="pv-toolbar">
          <span className="pv-toolbar__title">Parity</span>
          <PillTabs<Scope>
            items={[
              { value: 'all', label: 'All platforms' },
              ...data.platforms.map(platform => ({
                value: platform.id,
                label: platform.label,
                count: countDrift(platform.id),
              })),
            ]}
            value={scope}
            onValueChange={setScope}
          />
          <span className="pv-toolbar__spacer" />
          <SegmentedControl<ThemeId>
            items={THEMES.map(t => ({ value: t.id, label: t.label }))}
            value={theme}
            onValueChange={setTheme}
            size="small"
          />
        </div>
        <div className="pv-content">
          {selection.kind === 'overview' ? (
            <Overview
              data={data}
              platforms={platforms}
              onOpen={className =>
                setSelection({ kind: 'component', className })
              }
            />
          ) : (
            <SpecimenList
              data={data}
              specimens={shown}
              platforms={platforms}
              theme={theme}
              query={query}
            />
          )}
        </div>
      </div>
    </div>
  )
}

function countDrift(id: PlatformId): number {
  const platform = data.platforms.find(p => p.id === id)
  return platform ? platform.drift.length : 0
}
