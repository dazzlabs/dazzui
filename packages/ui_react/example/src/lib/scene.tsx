import {
  evaluateValue,
  type SceneValue,
} from '../../../../../tools/playgrounds/schema/value.mjs'
import { validateScene } from '../../../../../tools/playgrounds/schema/scene.mjs'
import { createElement, useEffect, useState, type ReactNode } from 'react'
import * as UI from '@dazzlabs/dazzui-react'
import { ArrowDownload20Regular, Circle20Filled } from '@fluentui/react-icons'
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
      on?: Record<string, { set: string; value: string | number | boolean }>
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
      if (prop === 'open')
        props.onOpenChange = (value: boolean) => assign(name, value)
    }
    if (node.on?.press)
      props.onClick = () => assign(node.on!.press.set, node.on!.press.value)
    const children = node.children?.map((child, i) =>
      render(child, `${key}-${i}`)
    )
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
            ...(node.id ? { 'data-scene-id': node.id } : {}),
            style: {
              width: props.width,
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
          color:
            props.tone === 'muted' ? 'var(--color-content-muted)' : undefined,
          fontSize: props.fontSize
            ? `var(--${String(props.fontSize).replaceAll('.', '-')}-font-size)`
            : undefined,
          fontFamily: props.mono ? 'var(--base-font-code)' : undefined,
        }
        delete props.tone
        delete props.fontSize
        delete props.mono
        return createElement('span', props, children)
      case 'Switch':
        props['aria-label'] = props.label
        delete props.label
        break
      case 'Icon':
        props.icon =
          props.name === 'Circle20Filled'
            ? Circle20Filled
            : ArrowDownload20Regular
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

export function defineStory(
  scene: Scene,
  options: Omit<StoryObj, 'render'> = {}
): StoryObj {
  validateScene(scene)
  return { ...options, render: () => <SceneView scene={scene} /> }
}
