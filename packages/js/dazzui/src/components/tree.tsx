import { ChevronRight12Regular } from '@fluentui/react-icons'
import type { CSSProperties, KeyboardEvent, ReactNode } from 'react'
import { useCallback, useMemo, useRef, useState } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type TreeSize = 'small' | 'medium'

/** The tree's drawing: the default file tree, or the tighter sidebar tree. */
export type TreeVariant = 'default' | 'sidebar'

export interface TreeNode<T extends string = string> {
  id: T
  label: ReactNode
  /** A glyph before the label — a file's kind, a folder, a status. */
  icon?: ReactNode
  /** A trailing aside: a count, a change marker, a `Badge`. */
  meta?: ReactNode
  disabled?: boolean
  /**
   * Present at all — empty included — marks the node as a branch. An empty
   * array is how an empty folder still draws a twisty and still opens, which
   * is the difference between "nothing in here" and "not a folder".
   */
  children?: readonly TreeNode<T>[]
  /**
   * What type-ahead matches on, where the label is an element rather than a
   * string. A string label needs nothing.
   */
  text?: string
}

export interface TreeProps<T extends string = string> {
  items: readonly TreeNode<T>[]
  /** Controlled open branches. Pair with `onExpandedChange`. */
  expanded?: readonly T[]
  defaultExpanded?: readonly T[]
  onExpandedChange?: (expanded: T[]) => void
  selected?: T | null
  onSelectedChange?: (id: T) => void
  size?: TreeSize
  /**
   * `sidebar` tightens the rows to the sidebar's 12px step, gives them a
   * hairline gap and drops the empty twisty a leaf would otherwise reserve.
   * The default keeps a file tree's columns aligned across leaves and branches.
   */
  variant?: TreeVariant
  /**
   * Which ramp the selected row is washed with. The hover comes off the
   * `normal` recipe either way — that one reads the theme's surface roles
   * rather than a ramp, which is what keeps a row's hover neutral while its
   * selection carries the accent.
   */
  tint?: Tint
  /** Names the tree for a reader who arrives at it out of context. */
  'aria-label'?: string
  className?: string
}

interface Row<T extends string> {
  node: TreeNode<T>
  level: number
  parent: T | undefined
  /** Position and count among its siblings, which a flat tree has to state. */
  index: number
  siblings: number
  branch: boolean
  open: boolean
}

function labelText<T extends string>(node: TreeNode<T>) {
  return (node.text ?? (typeof node.label === 'string' ? node.label : ''))
    .trim()
    .toLowerCase()
}

/**
 * The visible rows, in the order the eye reads them.
 *
 * A tree is nested and its keyboard is not: every arrow key moves between
 * rows that may be at any depth, so the flattening is not an implementation
 * shortcut but the model the navigation is actually written against.
 */
function flatten<T extends string>(
  items: readonly TreeNode<T>[],
  open: ReadonlySet<T>,
  level = 1,
  parent?: T
): Row<T>[] {
  return items.flatMap((node, index) => {
    const branch = node.children !== undefined
    const isOpen = branch && open.has(node.id)
    const row: Row<T> = {
      node,
      level,
      parent,
      index,
      siblings: items.length,
      branch,
      open: isOpen,
    }
    return isOpen
      ? [row, ...flatten(node.children ?? [], open, level + 1, node.id)]
      : [row]
  })
}

/**
 * A hierarchy the reader walks.
 *
 * This is the one component in the package with no Base UI part under it, and
 * it is worth saying why the behaviour is written out rather than borrowed: a
 * tree's keyboard is not a list's. `ArrowRight` opens a shut branch and
 * descends into an open one; `ArrowLeft` shuts an open branch and climbs out
 * of a leaf; the whole tree is one tab stop, so focus roves inside it rather
 * than tabbing through a hundred files. None of that falls out of a
 * `Collapsible` per node — a tree of disclosures is a hundred tab stops and
 * no way across them.
 *
 * A branch and a leaf answer a click differently on purpose. Clicking a
 * folder opens it *and* selects it, because in a file tree those are one
 * gesture; clicking the twisty alone opens it and leaves the selection where
 * it was, which is how a reader looks inside a folder without losing the file
 * they were reading.
 */
export function Tree<T extends string = string>({
  items,
  expanded,
  defaultExpanded,
  onExpandedChange,
  selected,
  onSelectedChange,
  size = 'medium',
  variant = 'default',
  tint = 'primary',
  className,
  ...props
}: TreeProps<T>) {
  const [ownExpanded, setOwnExpanded] = useState<readonly T[]>(
    defaultExpanded ?? []
  )
  const open = useMemo(
    () => new Set(expanded ?? ownExpanded),
    [expanded, ownExpanded]
  )
  const rows = useMemo(() => flatten(items, open), [items, open])

  /* The roving tab stop. It starts on the selection so that tabbing into a
     tree lands where the reader left off rather than at the top of a file
     list they have already scrolled past. */
  const [active, setActive] = useState<T | undefined>(
    () => selected ?? items[0]?.id
  )
  const refs = useRef(new Map<T, HTMLDivElement | null>())
  const typed = useRef<{ prefix: string; at: number }>({ prefix: '', at: 0 })

  const focusRow = useCallback((id: T) => {
    setActive(id)
    refs.current.get(id)?.focus()
  }, [])

  const setOpen = useCallback(
    (id: T, next: boolean) => {
      const value = new Set(open)
      if (next) value.add(id)
      else value.delete(id)
      const list = [...value]
      if (expanded === undefined) setOwnExpanded(list)
      onExpandedChange?.(list)
    },
    [expanded, onExpandedChange, open]
  )

  /* Shutting a folder can take the roving tab stop with it, and a tree with
     no `tabIndex={0}` anywhere cannot be tabbed into at all. So the stop
     falls back to the first row rather than to nothing. */
  const stop = rows.some(row => row.node.id === active)
    ? active
    : rows[0]?.node.id

  const at = (id: T) => rows.findIndex(row => row.node.id === id)

  const move = (from: number, by: number) => {
    const next = rows[from + by]
    if (next && !next.node.disabled) focusRow(next.node.id)
  }

  const onKeyDown = (row: Row<T>) => (event: KeyboardEvent<HTMLDivElement>) => {
    const here = at(row.node.id)
    /* Only the keys the tree actually consumed are swallowed. Preventing the
       default on everything would take Escape away from a dialog the tree is
       inside, and ⌘A away from the page. */
    let handled = true
    switch (event.key) {
      case 'ArrowDown':
        move(here, 1)
        break
      case 'ArrowUp':
        move(here, -1)
        break
      case 'ArrowRight':
        /* Open, then descend — two presses rather than one, so a reader
           opening a folder does not also lose their place inside it. */
        if (row.branch && !row.open) setOpen(row.node.id, true)
        else if (row.open) move(here, 1)
        break
      case 'ArrowLeft':
        if (row.branch && row.open) setOpen(row.node.id, false)
        else if (row.parent !== undefined) focusRow(row.parent)
        break
      case 'Home':
        if (rows[0]) focusRow(rows[0].node.id)
        break
      case 'End': {
        const last = rows[rows.length - 1]
        if (last) focusRow(last.node.id)
        break
      }
      case 'Enter':
      case ' ':
        if (!row.node.disabled) onSelectedChange?.(row.node.id)
        if (row.branch) setOpen(row.node.id, !row.open)
        break
      default: {
        /* Type-ahead over the *visible* rows only: a name inside a shut
           folder is not on screen, and jumping to something the reader
           cannot see is worse than not jumping. */
        if (event.key.length !== 1 || event.metaKey || event.ctrlKey) {
          handled = false
          break
        }
        const now = Date.now()
        const duration = getComputedStyle(event.currentTarget)
          .getPropertyValue('--tree-typeahead-window')
          .trim()
        const typeaheadWindow =
          parseFloat(duration) * (duration.endsWith('ms') ? 1 : 1000)
        const prefix =
          now - typed.current.at < typeaheadWindow
            ? typed.current.prefix + event.key.toLowerCase()
            : event.key.toLowerCase()
        typed.current = { prefix, at: now }
        const order = [...rows.slice(here + 1), ...rows.slice(0, here + 1)]
        const hit = order.find(
          candidate =>
            !candidate.node.disabled &&
            labelText(candidate.node).startsWith(prefix)
        )
        if (hit) focusRow(hit.node.id)
        break
      }
    }
    if (handled) event.preventDefault()
  }

  return (
    <div
      role="tree"
      /* Every `--control-color-*` is declared inside `[data-tint]`, so the
         ramp has to be bound here or the rows would inherit a dead value.
         It is a prop rather than a constant for the reason `Sidebar`'s rows
         take one: a column of selectable rows is exactly where a window says
         which accent it runs on. */
      data-tint={tint}
      data-size={size}
      data-variant={variant}
      className={cx('dz-tree', className)}
      {...props}
    >
      {rows.map(row => {
        const id = row.node.id
        return (
          <div
            key={id}
            ref={element => {
              refs.current.set(id, element)
            }}
            role="treeitem"
            aria-level={row.level}
            aria-posinset={row.index + 1}
            aria-setsize={row.siblings}
            aria-expanded={row.branch ? row.open : undefined}
            aria-selected={selected === id}
            aria-disabled={row.node.disabled || undefined}
            tabIndex={id === stop ? 0 : -1}
            data-branch={row.branch}
            data-selected={selected === id}
            className="dz-tree__item"
            /* Depth is a number the stylesheet multiplies by one step, so the
               indent is a decision made once rather than a length repeated
               per level. */
            style={{ '--tree-depth': row.level - 1 } as CSSProperties}
            onFocus={() => setActive(id)}
            onKeyDown={onKeyDown(row)}
            onClick={() => {
              if (row.node.disabled) return
              onSelectedChange?.(id)
              if (row.branch) setOpen(id, !row.open)
            }}
          >
            <span
              aria-hidden
              className="dz-tree__twisty"
              onClick={event => {
                if (!row.branch || row.node.disabled) return
                /* The twisty is the one target that opens without selecting,
                   so it has to stop the row's own handler. */
                event.stopPropagation()
                setOpen(id, !row.open)
              }}
            >
              {row.branch ? <ChevronRight12Regular /> : null}
            </span>
            {row.node.icon ? (
              <span aria-hidden className="dz-tree__icon">
                {row.node.icon}
              </span>
            ) : null}
            <span className="dz-tree__label">{row.node.label}</span>
            {row.node.meta ? (
              <span className="dz-tree__meta">{row.node.meta}</span>
            ) : null}
          </div>
        )
      })}
    </div>
  )
}
