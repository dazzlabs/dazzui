import {
  evaluateValue,
  type SceneValue,
} from '../../../../../tools/playgrounds/schema/value.mjs'
import {
  ITEMS,
  validateScene,
} from '../../../../../tools/playgrounds/schema/scene.mjs'
import { createElement, useEffect, useState, type ReactNode } from 'react'
import * as UI from '@dazzlabs/dazzui-react'
import * as Glyphs from '../generated/glyphs'
import * as Gallery from '../components/gallery'
import { Col, Row } from '../components/stage'
import type { StoryObj } from './story'

export type SceneNode =
  | string
  | {
      value?: SceneValue
      component: string
      id?: string
      props?: Record<string, string | number | boolean | undefined>
      derive?: Record<string, SceneValue | undefined>
      bind?: Record<string, string>
      on?: Record<
        string,
        { set?: string; value?: string | number | boolean } | undefined
      >
      children?: SceneNode[]
      slots?: Record<string, SceneNode | undefined>
    }
export interface Scene {
  timers?: { every: number; set: string; value: SceneValue }[]
  state?: Record<string, string | number | boolean>
  content: SceneNode
  checkpoints?: {
    id: string
    steps: {
      action: string
      target?: string
      value?: string
      milliseconds?: number
    }[]
    expect?: Record<string, string | number | boolean>
    expectText?: Partial<Record<string, string>>
    expectDisabled?: Partial<Record<string, boolean>>
  }[]
}

declare global {
  interface Window {
    __SCENE_STATE__?: Record<string, string | number | boolean>
  }
}

function SceneView({ scene }: { scene: Scene }) {
  const [state, setState] = useState(scene.state ?? {})
  useEffect(() => {
    window.__SCENE_STATE__ = state
    return () => {
      delete window.__SCENE_STATE__
    }
  }, [state])
  useEffect(() => {
    const timers = (scene.timers ?? []).map(timer =>
      window.setInterval(() => {
        setState(current => ({
          ...current,
          [timer.set]: evaluateValue(timer.value, current),
        }))
      }, timer.every)
    )
    return () => timers.forEach(timer => window.clearInterval(timer))
  }, [scene])
  function render(node: SceneNode, key: string): ReactNode {
    if (typeof node === 'string') return node
    if (node.component === 'Value')
      return String(evaluateValue(node.value!, state))
    const props: Record<string, unknown> = { ...node.props, key }
    for (const [name, expression] of Object.entries(node.derive ?? {}))
      props[name] = evaluateValue(expression!, state)
    if (node.id) props['data-scene-id'] = node.id
    for (const [name, slot] of Object.entries(node.slots ?? {}))
      props[name] = render(slot!, `${key}-slot-${name}`)
    const assign = (name: string, value: string | number | boolean) =>
      setState(s => ({ ...s, [name]: value }))
    for (const [prop, name] of Object.entries(node.bind ?? {})) {
      props[prop] = state[name]
      if (prop === 'value')
        props.onValueChange = (value: string) => assign(name, value)
      if (prop === 'checked')
        props.onCheckedChange = (value: boolean) => assign(name, value)
      if (prop === 'pressed')
        props.onPressedChange = (value: boolean) => assign(name, value)
      if (prop === 'open')
        props.onOpenChange = (value: boolean) => assign(name, value)
    }
    // Event names map to the React callback each component takes; an empty
    // action declares a live control that assigns nothing.
    const callbacks: Record<string, string> = {
      dismiss: 'onDismiss',
      press: 'onClick',
      select: 'onSelectedChange',
      remove: 'onRemove',
    }
    for (const [event, action] of Object.entries(node.on ?? {}))
      props[callbacks[event]!] =
        action?.set === undefined
          ? () => {}
          : () => assign(action.set!, action.value!)
    for (const name of ['width', 'height', 'maxWidth'])
      if (
        typeof props[name] === 'string' &&
        String(props[name]).startsWith('spacing.')
      )
        props[name] = `var(--${String(props[name]).replace('.', '-')})`
    if (node.component === 'Show')
      return evaluateValue(node.value!, state)
        ? node.children?.map((child, i) => render(child, `${key}-${i}`))
        : null
    // A list component reads its item nodes as data, not as elements.
    const children = ITEMS[node.component as keyof typeof ITEMS]
      ? undefined
      : node.children?.map((child, i) => render(child, `${key}-${i}`))
    switch (node.component) {
      case 'Grid':
        return createElement(Gallery.Grid, {
          ...props,
          min: `${props.min ?? 256}px`,
          children,
        })
      case 'Box':
        return createElement(
          'div',
          {
            key,
            role: props.role,
            ...(node.id ? { 'data-scene-id': node.id } : {}),
            style: {
              width: props.width,
              flexWrap: props.wrap ? 'wrap' : undefined,
              height: props.height,
              maxWidth: props.maxWidth,
              flex: props.grow ? 1 : undefined,
              display: props.display,
              alignItems: props.align,
              justifyContent:
                props.justify === 'between' ? 'space-between' : undefined,
              color:
                props.tone === 'muted'
                  ? 'var(--color-content-muted)'
                  : undefined,
              gap: props.gap
                ? `var(--${String(props.gap).replaceAll('.', '-')})`
                : undefined,
              padding: props.padding
                ? `var(--${String(props.padding).replaceAll('.', '-')})`
                : undefined,
              background: props.surface
                ? `var(--${String(props.surface).replaceAll('.', '-')})`
                : undefined,
              borderRadius: props.radius
                ? `var(--${String(props.radius).replaceAll('.', '-')})`
                : undefined,
            },
          },
          children
        )
      case 'Page':
      case 'Section':
      case 'Stack':
      case 'Specimen':
        return createElement(
          Gallery[node.component] as React.ElementType,
          props,
          children
        )
      case 'Inline':
        props.style = {
          color: props.tone ? `var(--color-content-${props.tone})` : undefined,
          fontSize: props.fontSize
            ? `var(--${String(props.fontSize).replaceAll('.', '-')}-font-size)`
            : undefined,
          fontFamily: props.mono ? 'var(--base-font-code)' : undefined,
        }
        delete props.tone
        delete props.fontSize
        delete props.mono
        return createElement('span', props, children)
      case 'Link':
        // A link has no disabled attribute; it is marked for assistive tech.
        if (props.disabled) props['aria-disabled'] = true
        delete props.disabled
        break
      case 'Switch':
        props['aria-label'] = props.label
        delete props.label
        break
      case 'RadioGroup':
        props['aria-label'] = props.label
        delete props.label
        props.options = itemProps(node)
        if (props.direction === 'row' || props.gap)
          props.style = {
            flexDirection: props.direction,
            gap: props.gap
              ? `var(--${String(props.gap).replace('.', '-')})`
              : undefined,
          }
        delete props.direction
        delete props.gap
        return createElement(UI.RadioGroup, props as never)
      case 'PillTabs':
        props['aria-label'] = props.label
        delete props.label
        props.items = itemProps(node)
        return createElement(UI.PillTabs, props as never)
      case 'ToggleGroup': {
        props['aria-label'] = props.label
        delete props.label
        const items = (node.children ?? []).filter(
          (item): item is Exclude<SceneNode, string> => typeof item !== 'string'
        )
        // A multiple group's selection is each item's own pressed state; a
        // single group holds one value.
        props.items = items.map((item, i) => {
          const { pressed: _, label, ...rest } = item.props ?? {}
          const content = item.children?.map((child, j) =>
            render(child, `${key}-${i}-${j}`)
          )
          return {
            ...rest,
            label: !content?.length
              ? label
              : content.length === 1
                ? content[0]
                : content,
            ...(item.children?.some(child => typeof child !== 'string')
              ? { hint: label }
              : {}),
          }
        })
        if (props.multiple === false) {
          const bound = node.bind?.value
          props.value = [bound ? state[bound] : props.value].filter(Boolean)
          props.onValueChange = (next: string[]) => {
            if (bound) assign(bound, next[0] ?? '')
          }
        } else {
          props.value = items
            .filter(item =>
              item.bind?.pressed
                ? state[item.bind.pressed]
                : item.props?.pressed
            )
            .map(item => item.props!.value)
          props.onValueChange = (next: string[]) => {
            for (const item of items)
              if (item.bind?.pressed)
                assign(
                  item.bind.pressed,
                  next.includes(String(item.props!.value))
                )
          }
        }
        return createElement(UI.ToggleGroup, props as never)
      }
      case 'Slider': {
        const end = node.bind?.end ? state[node.bind.end] : props.end
        const value = node.bind?.value ? state[node.bind.value] : props.value
        props.value = end === undefined ? value : [value, end]
        props.onValueChange = (next: number | number[]) => {
          const [first, second] = Array.isArray(next) ? next : [next]
          if (node.bind?.value) assign(node.bind.value, first!)
          if (node.bind?.end && second !== undefined)
            assign(node.bind.end, second)
        }
        if (props.unit) {
          const unit = props.unit
          props.formatValue = (values: readonly number[]) =>
            `${values[0]}${unit}`
        }
        delete props.end
        delete props.unit
        break
      }
      case 'NumberField': {
        const text = String(props.value ?? '')
        props.value = text === '' ? null : Number(text)
        if (node.bind?.value) {
          const bound = node.bind.value
          props.onValueChange = (next: number | null) =>
            assign(bound, next === null ? '' : String(next))
        }
        break
      }
      case 'SegmentedControl':
        props['aria-label'] = props.label
        delete props.label
        props.items = (node.children ?? []).map(item => {
          if (typeof item === 'string')
            throw new Error('Segmented items are nodes')
          return item.props
        })
        return createElement(UI.SegmentedControl, props as never)
      case 'Icon':
        props.icon = Glyphs[props.name as keyof typeof Glyphs]
        if (!props.icon) throw new Error(`No glyph ${String(props.name)}`)
        if (props.fit)
          return createElement(props.icon as React.ElementType, { key })
        if (typeof props.size === 'string')
          props.size = `var(--${props.size.replaceAll('.', '-')})`
        delete props.name
        break
      case 'Button':
        props.fullWidth = props.expand
        delete props.expand
        break
      case 'ActionBar':
        if (props.pushEnd) props.style = { marginInlineStart: 'auto' }
        delete props.pushEnd
        break
      case 'Row':
        if (typeof props.gap === 'string')
          props.gap = `var(--${props.gap.replaceAll('.', '-')})`
        return createElement(Row, { ...props, children })
      case 'Col':
        if (typeof props.gap === 'string')
          props.gap = `var(--${props.gap.replaceAll('.', '-')})`
        return createElement(Col, { ...props, children })
      case 'Text':
        return (
          <p key={key} className="pg-scene-message">
            {children}
          </p>
        )
    }
    const Component = UI[node.component as keyof typeof UI]
    if (!Component) throw new Error(`No React adapter for ${node.component}`)
    // The generator validates the finite portable prop vocabulary. This cast
    // is only the boundary between its data and React's heterogeneous exports.
    return createElement(Component as React.ElementType, props, children)
  }
  return render(scene.content, 'root')
}

/** A list component's item nodes as the option objects its React API takes. */
function itemProps(node: Exclude<SceneNode, string>) {
  return (node.children ?? []).map(item => {
    if (typeof item === 'string') throw new Error('List items are nodes')
    return item.props
  })
}

export function defineStory(
  scene: Scene,
  options: Omit<StoryObj, 'render'> = {}
): StoryObj {
  validateScene(scene)
  return { ...options, render: () => <SceneView scene={scene} /> }
}
