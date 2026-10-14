import { useEffect, useState } from 'react'

import type { Loaded, Readings, Run, Scenes, Tolerances } from './model'

async function json<T>(path: string): Promise<T> {
  const response = await fetch(path, { cache: 'no-store' })
  if (!response.ok) throw new Error(`${path}: ${response.status}`)
  return response.json() as Promise<T>
}

const revision = (run: Run) => run.revision ?? run.collected

/** Reuse unchanged platforms when the run publishes an incremental update. */
export async function load(
  previous?: Loaded,
  published?: Run
): Promise<Loaded> {
  const [scenes, run, tolerances] = await Promise.all([
    json<Scenes>('/scenes.json'),
    published ?? json<Run>('/run.json'),
    json<Tolerances>('/tolerances.json'),
  ])
  const list = await Promise.all(
    run.platforms.map(platform => {
      const before = previous?.run.snapshots?.[platform]
      const after = run.snapshots?.[platform]
      return before &&
        after &&
        before.collected === after.collected &&
        before.commit === after.commit &&
        previous?.readings[platform]
        ? previous.readings[platform]
        : json<Readings>(`/${platform}/readings.json`)
    })
  )
  if (revision(await json<Run>('/run.json')) !== revision(run))
    throw new Error('The run changed while loading; retrying')
  const readings = Object.fromEntries(list.map(r => [r.platform, r]))
  const reference =
    list.find(r => r.role === 'reference')?.platform ?? run.platforms[0]!
  const implementations = run.platforms.filter(p => p !== reference)
  return { scenes, run, readings, tolerances, reference, implementations }
}

export function useLoaded() {
  const [state, setState] = useState<
    { loaded: Loaded } | { error: string } | undefined
  >()
  useEffect(() => {
    let current: Loaded | undefined
    let cancelled = false
    let timer: ReturnType<typeof setTimeout>
    async function refresh() {
      try {
        const run = await json<Run>('/run.json')
        if (!current || revision(run) !== revision(current.run)) {
          const loaded = await load(current, run)
          if (!cancelled) {
            current = loaded
            setState({ loaded })
          }
        }
      } catch (error) {
        if (!cancelled && !current)
          setState({ error: (error as Error).message })
      } finally {
        if (!cancelled) timer = setTimeout(refresh, 1500)
      }
    }
    void refresh()
    return () => {
      cancelled = true
      clearTimeout(timer)
    }
  }, [])
  return state
}

/** A platform's image, resolved against its own directory. */
export const imageOf = (platform: string, path: string | undefined) =>
  path ? `/${platform}/${path}` : undefined
