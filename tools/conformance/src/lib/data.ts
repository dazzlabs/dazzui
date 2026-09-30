import { useEffect, useState } from 'react'

import type { Loaded, Readings, Run, Scenes, Tolerances } from './model'

async function json<T>(path: string): Promise<T> {
  const response = await fetch(path)
  if (!response.ok) throw new Error(`${path}: ${response.status}`)
  return response.json() as Promise<T>
}

/** The whole run, read once from wherever the server points `/` at. */
export async function load(): Promise<Loaded> {
  const [scenes, run, tolerances] = await Promise.all([
    json<Scenes>('/scenes.json'),
    json<Run>('/run.json'),
    json<Tolerances>('/tolerances.json'),
  ])
  const list = await Promise.all(
    run.platforms.map(platform => json<Readings>(`/${platform}/readings.json`))
  )
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
    load()
      .then(loaded => setState({ loaded }))
      .catch((error: Error) => setState({ error: error.message }))
  }, [])
  return state
}

/** A platform's image, resolved against its own directory. */
export const imageOf = (platform: string, path: string | undefined) =>
  path ? `/${platform}/${path}` : undefined
