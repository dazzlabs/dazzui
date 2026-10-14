import * as kit from '@dazzlabs/dazzui'
import type { ComponentType } from 'react'

import * as stage from '../components/stage'

/*
 * Finds a drawn scene's specimens by the walk codegen made to address them
 * (tools/codegen/scenes.mjs): pre-order over the elements the story wrote —
 * an element, then the elements among its props in the order they were
 * written, then its children. A helper component the story defines is
 * followed into what it rendered, as codegen inlines it.
 *
 * The walk reads React's own tree: a fiber keeps the props object of the
 * element it was made from, so an element the story wrote leads straight to
 * the fiber that drew it, and from there to its DOM node.
 */

/** The parts of a fiber this reads. React's internals, kept to a minimum. */
interface Fiber {
  tag: number
  elementType: unknown
  memoizedProps: unknown
  stateNode: unknown
  child: Fiber | null
  sibling: Fiber | null
  return: Fiber | null
}

interface Element {
  type: unknown
  props: Record<string, unknown>
}

const HOST = new Set([5, 26, 27])
const TEXT = 6

const KIT = new Map<unknown, string>()
for (const [name, value] of Object.entries(kit))
  if (!KIT.has(value)) KIT.set(value, name)
const STAGE = new Set<unknown>(Object.values(stage))
const FRAGMENT = Symbol.for('react.fragment')

const isElement = (value: unknown): value is Element =>
  typeof value === 'object' &&
  value !== null &&
  '$$typeof' in value &&
  (value.$$typeof === Symbol.for('react.transitional.element') ||
    value.$$typeof === Symbol.for('react.element'))

const isPlain = (value: unknown): value is object =>
  typeof value === 'object' &&
  value !== null &&
  (Object.getPrototypeOf(value) === Object.prototype ||
    Object.getPrototypeOf(value) === null)

function children(fiber: Fiber): Fiber[] {
  const list = []
  for (let child = fiber.child; child; child = child.sibling) list.push(child)
  return list
}

/** The root fiber a container holds, as React last committed it. */
function rootOf(container: HTMLElement): Fiber {
  const key = Object.keys(container).find(k =>
    k.startsWith('__reactContainer$')
  )
  if (!key) throw new Error('The container holds no React root')
  const fiber = (container as unknown as Record<string, Fiber>)[key]!
  return (fiber.stateNode as { current: Fiber }).current
}

/** The first DOM node a fiber draws, or null for one that draws none. */
function hostOf(fiber: Fiber): HTMLElement | null {
  if (HOST.has(fiber.tag)) return fiber.stateNode as HTMLElement
  for (const child of children(fiber)) {
    const host = hostOf(child)
    if (host) return host
  }
  return null
}

export interface Found {
  /** The kit's export name, as codegen names the element. */
  component: string
  node: HTMLElement | null
}

/** Every specimen of a drawn scene, in address order. */
export function walk(container: HTMLElement, Demo: ComponentType): Found[] {
  /* Every fiber by the props object it was made from; the outermost one
     where a memo or forwardRef passes the same object down. The same
     element drawn twice has one entry per drawing, taken in turn. */
  const byProps = new Map<unknown, Fiber[]>()
  let demo: Fiber | undefined
  const index = (fiber: Fiber) => {
    if (fiber.elementType === Demo && !demo) demo = fiber
    const props = fiber.memoizedProps
    if (
      props &&
      typeof props === 'object' &&
      fiber.return?.memoizedProps !== props
    ) {
      const list = byProps.get(props) ?? []
      list.push(fiber)
      byProps.set(props, list)
    }
    children(fiber).forEach(index)
  }
  index(rootOf(container))
  if (!demo) throw new Error('The scene was not drawn')

  const found: Found[] = []
  const used = new Set<Fiber>()
  const declared = new Set<unknown>()
  const remembered = new WeakSet<object>()
  const remember = (value: unknown): void => {
    if (!value || typeof value !== 'object' || remembered.has(value)) return
    remembered.add(value)
    if (isElement(value)) {
      declared.add(value.props)
      remember(value.props)
    } else if (Array.isArray(value)) value.forEach(remember)
    else if (isPlain(value)) Object.values(value).forEach(remember)
  }

  /* The fiber an element made: by its props object, or — where the kit
     clones an element it was handed, as a trigger is merged with the
     popup's own props — by type, inside the specimen that holds it, the
     one keeping the most of the element's prop values. */
  const fiberOf = (element: Element, scope: Fiber) => {
    let fiber = byProps.get(element.props)?.find(f => !used.has(f))
    if (!fiber) {
      const want = Object.entries(element.props)
      let best = want.length ? 0 : -1
      const search = (candidate: Fiber) => {
        if (candidate.elementType === element.type && !used.has(candidate)) {
          // A sibling with the same glyph or label is its own declaration,
          // not a clone of this one. In particular, a closed tree branch
          // must not steal a later visible file's icon and shift addresses.
          if (
            candidate.memoizedProps !== element.props &&
            declared.has(candidate.memoizedProps)
          )
            return
          const have = candidate.memoizedProps as Element['props']
          const score = want.filter(([k, v]) => have?.[k] === v).length
          if (score > best) {
            best = score
            fiber = candidate
          }
        }
        children(candidate).forEach(search)
      }
      children(scope).forEach(search)
    }
    if (fiber) used.add(fiber)
    return fiber
  }

  const props = (element: Element, scope: Fiber) => {
    for (const [name, value] of Object.entries(element.props))
      if (name !== 'children') inProps(value, scope)
    inChildren(element.props.children, scope)
  }

  const inProps = (value: unknown, scope: Fiber): void => {
    if (Array.isArray(value)) return value.forEach(v => inProps(v, scope))
    if (isElement(value)) return visit(value, scope)
    if (isPlain(value)) Object.values(value).forEach(v => inProps(v, scope))
  }

  const inChildren = (value: unknown, scope: Fiber): void => {
    if (Array.isArray(value)) return value.forEach(v => inChildren(v, scope))
    if (isElement(value)) visit(value, scope)
  }

  const visit = (element: Element, scope: Fiber): void => {
    remember(element)
    const { type } = element
    if (KIT.has(type)) {
      const fiber = fiberOf(element, scope)
      const specimen: Found = {
        component: KIT.get(type)!,
        node: fiber ? hostOf(fiber) : null,
      }
      const start = found.push(specimen)
      props(element, fiber ?? scope)
      // A provider, or a wrapper that lends its trigger a behaviour, draws
      // nothing of its own: its first node is a specimen inside it.
      if (found.slice(start).some(inner => inner.node === specimen.node))
        specimen.node = null
      return
    }
    if (STAGE.has(type) || type === FRAGMENT) return props(element, scope)
    // A helper the story defines, or a glyph: whatever it rendered.
    const fiber = fiberOf(element, scope)
    if (fiber) return children(fiber).forEach(f => rendered(f, fiber))
    // Content not drawn yet — a popup's, before it opens — still holds
    // addresses. The helper is called for the elements it would return;
    // one that needs a hook to run cannot be, and its scene fails.
    if (typeof type === 'function') {
      let returned: unknown
      try {
        returned = (type as (props: unknown) => unknown)(element.props)
      } catch (error) {
        throw new Error(
          `${(type as { name?: string }).name || 'A helper'} is not drawn and cannot be called: ${(error as Error).message}`
        )
      }
      inChildren(returned, scope)
    }
  }

  const rendered = (fiber: Fiber, scope: Fiber): void => {
    if (HOST.has(fiber.tag) || fiber.tag === TEXT) return
    const type = fiber.elementType
    if (typeof type === 'function' || (typeof type === 'object' && type))
      return visit(
        { type, props: fiber.memoizedProps as Element['props'] },
        scope
      )
    children(fiber).forEach(f => rendered(f, scope))
  }

  const top = demo
  children(top).forEach(f => rendered(f, top))
  return found
}
