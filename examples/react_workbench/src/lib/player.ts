import { useCallback, useEffect, useRef, useState } from 'react'

import type { Beat, Entry, PlanStep, ToolEntry } from './transcript'

/**
 * How the reply is typed out. The tick only decides how often the column is
 * repainted — how much has arrived is worked out from the clock, so a timer
 * the browser has throttled (a backgrounded tab clamps these to whole seconds)
 * reveals the text in fewer, larger steps rather than at a fraction of speed.
 */
const MS_PER_CHARACTER = 6
const TICK = 16

/** The pause between one move and the next, so the column does not stampede. */
const BEAT_GAP = 320

/** A tool call with nothing declared takes about this long to come back. */
const DEFAULT_WORK = 700

export type Answer = 'once' | 'always' | 'deny'

/** The question the modal is asking, lifted out of the beat that raised it. */
export type PendingAsk = Extract<Beat, { kind: 'ask' }>

const wait = (ms: number) =>
  new Promise<void>(resolve => setTimeout(resolve, ms))

export interface StartOptions {
  /** Ask on every gate, standing yes or not — the `ask-all` permission mode. */
  askEvery?: boolean
  /** Leave the person's turn out: the run was started from an issue, not typed. */
  silent?: boolean
}

export interface Player {
  entries: Entry[]
  /** True from the first beat until the last one lands, or until Stop. */
  running: boolean
  /** The permission the run is blocked on, or null. */
  ask: PendingAsk | null
  /** Tools this session has been given a standing yes for. */
  trusted: string[]
  start: (prompt: string, beats: Beat[], options?: StartOptions) => void
  answer: (value: Answer) => void
  stop: () => void
  reset: (entries?: Entry[]) => void
  toggle: (id: string) => void
}

/**
 * Replays a scripted run into a transcript.
 *
 * The loop is a plain `for await` rather than a chain of timers because a run
 * has to be able to *stop* in the middle — on a denied permission, on the
 * Stop button, on a session being switched out from under it. Every await
 * checks the generation it started in, so a superseded run unwinds instead of
 * writing into the transcript that replaced it.
 *
 * What a finished run has to report goes out through `onFinish` rather than
 * into state here, because a toast's lifetime belongs to the toast queue —
 * a message the player held on to would be one the queue had already timed
 * out and removed.
 */
export function usePlayer(
  seed: Entry[] = [],
  onFinish?: (message: string) => void
): Player {
  const [entries, setEntries] = useState<Entry[]>(seed)
  const [running, setRunning] = useState(false)
  const [ask, setAsk] = useState<PendingAsk | null>(null)
  const [trusted, setTrusted] = useState<string[]>([])

  /* Read through a ref so a caller's inline callback does not re-make the
     run loop on every render. */
  const finish = useRef(onFinish)
  finish.current = onFinish

  /** Bumped by every start, stop and unmount; the run checks it after awaits. */
  const generation = useRef(0)
  const resolveAsk = useRef<((value: Answer) => void) | null>(null)
  const counter = useRef(0)

  const nextId = () => `entry-${(counter.current += 1)}`

  useEffect(
    () => () => {
      generation.current += 1
      resolveAsk.current?.('deny')
    },
    []
  )

  const patch = useCallback(
    (id: string, change: (entry: Entry) => Entry) =>
      setEntries(list =>
        list.map(entry => (entry.id === id ? change(entry) : entry))
      ),
    []
  )

  const start = useCallback(
    (prompt: string, beats: Beat[], options: StartOptions = {}) => {
      const run = (generation.current += 1)
      const alive = () => generation.current === run

      if (!options.silent) {
        setEntries(list => [
          ...list,
          { id: nextId(), kind: 'user', text: prompt },
        ])
      }
      setRunning(true)

      // The plan entry the `step` beats reach back into. A run has one at
      // most, so the id is easier to carry than a search through the list.
      let planId: string | null = null

      // A local copy, so an "always" answer takes effect inside this run
      // rather than one render later.
      const trustedTools = new Set(trusted)

      const say = async (text: string) => {
        const id = nextId()
        setEntries(list => [...list, { id, kind: 'agent', text, shown: 0 }])

        const started = performance.now()
        for (;;) {
          await wait(TICK)
          if (!alive()) return

          const elapsed = performance.now() - started
          const shown = Math.min(
            text.length,
            Math.ceil(elapsed / MS_PER_CHARACTER)
          )
          patch(id, entry =>
            entry.kind === 'agent' ? { ...entry, shown } : entry
          )
          if (shown >= text.length) return
        }
      }

      const call = async (beat: Extract<Beat, { kind: 'tool' }>) => {
        const id = nextId()
        const row: ToolEntry = {
          id,
          kind: 'tool',
          tool: beat.tool,
          target: beat.target,
          status: 'running',
          expanded: false,
        }
        setEntries(list => [...list, row])

        await wait(beat.work ?? DEFAULT_WORK)
        if (!alive()) return
        patch(id, entry =>
          entry.kind === 'tool'
            ? {
                ...entry,
                status: 'done',
                meta: beat.meta,
                output: beat.output,
                expanded: beat.expanded ?? false,
              }
            : entry
        )
      }

      const apply = async (beat: Beat): Promise<'go' | 'stop'> => {
        switch (beat.kind) {
          case 'say':
            await say(beat.text)
            return 'go'

          case 'plan': {
            const id = nextId()
            const steps: PlanStep[] = beat.steps.map((step, index) => ({
              ...step,
              status: index === 0 ? 'active' : 'idle',
            }))
            planId = id
            setEntries(list => [...list, { id, kind: 'plan', steps }])
            return 'go'
          }

          case 'step': {
            if (planId) {
              patch(planId, entry =>
                entry.kind === 'plan'
                  ? {
                      ...entry,
                      steps: entry.steps.map((step, index) => ({
                        ...step,
                        status:
                          index <= beat.index
                            ? 'done'
                            : index === beat.index + 1
                              ? 'active'
                              : 'idle',
                      })),
                    }
                  : entry
              )
            }
            return 'go'
          }

          case 'tool':
            await call(beat)
            return 'go'

          case 'context':
            setEntries(list => [
              ...list,
              {
                id: nextId(),
                kind: 'context',
                number: beat.number,
                title: beat.title,
                body: beat.body,
                meta: beat.meta,
              },
            ])
            return 'go'

          case 'review':
            setEntries(list => [...list, { id: nextId(), kind: 'review' }])
            return 'go'

          case 'ask': {
            // A tool this session already trusts never asks twice — which is
            // the whole point of "Allow always", and the only way to see it
            // work is to script a second write after the first.
            if (!options.askEvery && trustedTools.has(beat.tool)) return 'go'

            setAsk(beat)
            const answered = await new Promise<Answer>(resolve => {
              resolveAsk.current = resolve
            })
            resolveAsk.current = null
            setAsk(null)
            if (!alive()) return 'stop'

            if (answered === 'always') {
              trustedTools.add(beat.tool)
              setTrusted([...trustedTools])
            }
            if (answered !== 'deny') {
              // The answer stays in the thread where the question was: a
              // reader scrolling back should find what they allowed.
              setEntries(list => [
                ...list,
                {
                  id: nextId(),
                  kind: 'allowed',
                  tool: beat.tool,
                  target: beat.target,
                  note: beat.done ?? 'done',
                },
              ])
              return 'go'
            }

            setEntries(list => [
              ...list,
              {
                id: nextId(),
                kind: 'notice',
                tint: 'warning',
                title: 'Stopped — permission denied',
                body: `${beat.tool} was not allowed to touch ${beat.target}. Nothing was written; send another message to pick this back up.`,
              },
            ])
            return 'stop'
          }

          case 'notice':
            setEntries(list => [
              ...list,
              {
                id: nextId(),
                kind: 'notice',
                tint: beat.tint,
                title: beat.title,
                body: beat.body,
              },
            ])
            return 'go'

          case 'finish':
            if (beat.after) {
              const text = beat.after
              setEntries(list => [
                ...list,
                { id: nextId(), kind: 'divider', text },
              ])
            }
            finish.current?.(beat.toast)
            return 'stop'
        }
      }

      void (async () => {
        for (const beat of beats) {
          if (!alive()) return
          const carry = await apply(beat)
          if (carry === 'stop') break
          await wait(BEAT_GAP)
        }
        if (alive()) setRunning(false)
      })()
    },
    [patch, trusted]
  )

  const answer = useCallback((value: Answer) => {
    resolveAsk.current?.(value)
  }, [])

  const stop = useCallback(() => {
    generation.current += 1
    resolveAsk.current?.('deny')
    resolveAsk.current = null
    setAsk(null)
    setRunning(false)
    setEntries(list =>
      list.map(entry =>
        entry.kind === 'tool' && entry.status === 'running'
          ? { ...entry, status: 'stopped', meta: 'interrupted' }
          : entry
      )
    )
  }, [])

  const reset = useCallback((next: Entry[] = []) => {
    generation.current += 1
    resolveAsk.current?.('deny')
    resolveAsk.current = null
    setAsk(null)
    setRunning(false)
    setEntries(next)
  }, [])

  const toggle = useCallback(
    (id: string) =>
      setEntries(list =>
        list.map(entry =>
          entry.id === id && entry.kind === 'tool'
            ? { ...entry, expanded: !entry.expanded }
            : entry
        )
      ),
    []
  )

  return {
    entries,
    running,
    ask,
    trusted,
    start,
    answer,
    stop,
    reset,
    toggle,
  }
}
