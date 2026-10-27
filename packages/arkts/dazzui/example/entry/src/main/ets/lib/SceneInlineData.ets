import { SceneValue } from './SceneTypes'
import { SceneState, text } from './SceneState'

export interface InlineRun {
  label: string
  code: boolean
  href: string
  link?: SceneValue
  preview?: SceneValue
  start: number
  end: number
}

/** Preview content is not part of the sentence; only its trigger is. */
export function inlineRuns(node: SceneValue, state: SceneState): InlineRun[] {
  const flatten = (node: SceneValue, code: boolean, href: string, link?: SceneValue): InlineRun[] => {
    const value = state.evaluate(node)
    if (value.kind === 'element') {
      if (value.name === 'PreviewCard') {
        const trigger = value.fields?.['trigger'] ?? { kind: 'null' }
        const runs = flatten(trigger, code, href, link)
        return [{ label: runs.map((run: InlineRun): string => run.label).join(''),
          code, href: text(state.evaluate(trigger.fields?.['href'] ?? { kind: 'null' })),
          preview: value, link: trigger, start: 0, end: 0 }]
      }
      return (value.items ?? []).flatMap((item: SceneValue): InlineRun[] => flatten(item,
        code || value.name === 'Code', value.name === 'Link'
          ? text(state.evaluate(value.fields?.['href'] ?? { kind: 'null' })) : href,
        value.name === 'Link' ? value : link))
    }
    return [{ label: text(value), code, href, link, start: 0, end: 0 }]
  }
  const result = flatten(node, false, '')
  let offset = 0
  for (const run of result) {
    run.start = offset
    offset += run.label.length
    run.end = offset
  }
  return result
}
