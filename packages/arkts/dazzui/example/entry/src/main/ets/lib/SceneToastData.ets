import { DazzToastOptions, DazzToastAction, DazzTint } from '@dazzlabs/dazzui'
import { SceneValue } from './SceneTypes'
import { SceneState, EMPTY_VALUE, text } from './SceneState'

/** Plain portable toast options; rich React nodes remain compiler gaps. */
export function sceneToastOptions(value: SceneValue, state: SceneState, run: (action: SceneValue) => void): DazzToastOptions {
  const fields = state.evaluate(value).fields ?? {}
  const read = (key: string): SceneValue => state.evaluate(fields[key] ?? EMPTY_VALUE)
  const action = state.evaluate(fields['action'] ?? EMPTY_VALUE).fields
  const nativeAction: DazzToastAction | undefined = action ? {
    label: text(state.evaluate(action['label'] ?? EMPTY_VALUE)),
    onAction: () => run(action['onClick'] ?? EMPTY_VALUE),
  } : undefined
  return {
    id: fields['id'] ? text(read('id')) : undefined,
    title: text(read('title')),
    tint: fields['tint'] ? text(read('tint')) as DazzTint : undefined,
    icon: fields['icon'] ? text(read('icon')) : undefined,
    action: nativeAction,
    dismissable: read('dismissable').boolean,
    dismissLabel: fields['dismissLabel'] ? text(read('dismissLabel')) : undefined,
    timeout: read('timeout').number,
    priority: fields['priority'] ? text(read('priority')) as 'low' | 'high' : undefined,
    onClose: fields['onClose'] ? () => run(fields['onClose']) : undefined,
  }
}
