import { DazzSize, DazzTint } from './TokenTypes'

export interface DazzTreeBadge {
  label: string
  tint?: DazzTint
  variant?: string
  controlSize?: DazzSize
}

export interface DazzTreeNode {
  id: string
  label: string
  icon?: string
  iconSize?: number
  meta?: string
  badge?: DazzTreeBadge
  disabled?: boolean
  children?: DazzTreeNode[]
  text?: string
}

export interface DazzTreeRow {
  node: DazzTreeNode
  level: number
  parent?: string
  index: number
  siblings: number
  branch: boolean
  open: boolean
}

function visitTree(
  items: DazzTreeNode[], open: Set<string>, level: number, parent?: string
): DazzTreeRow[] {
  const rows: DazzTreeRow[] = []
  items.forEach((node: DazzTreeNode, index: number) => {
    const branch = node.children !== undefined
    const isOpen = branch && open.has(node.id)
    rows.push({
      node, level, parent, index, siblings: items.length, branch, open: isOpen,
    })
    if (isOpen) rows.push(...visitTree(node.children ?? [], open, level + 1, node.id))
  })
  return rows
}

export function flattenTree(
  items: DazzTreeNode[], expanded: string[]
): DazzTreeRow[] { return visitTree(items, new Set(expanded), 1) }

export interface DazzTreeNavigation {
  handled: boolean
  focus?: string
  expand?: boolean
  select?: string
}

/** The reference tree's focus and selection are separate operations. */
export function treeNavigation(
  rows: DazzTreeRow[], id: string, key: number
): DazzTreeNavigation {
  const index = rows.findIndex(row => row.node.id === id)
  const row = rows[index]
  if (!row) return { handled: false }
  const adjacent = (by: number): string | undefined => {
    const next = rows[index + by]
    return next && !next.node.disabled ? next.node.id : undefined
  }
  if (key === 2013) return { handled: true, focus: adjacent(1) }
  if (key === 2012) return { handled: true, focus: adjacent(-1) }
  if (key === 2015) {
    return !row.open && row.branch ? { handled: true, expand: true }
      : { handled: true, focus: row.open ? adjacent(1) : undefined }
  }
  if (key === 2014) {
    return row.branch && row.open ? { handled: true, expand: false }
      : { handled: true, focus: row.parent }
  }
  if (key === 2081) return { handled: true, focus: rows[0]?.node.id }
  if (key === 2082) return { handled: true, focus: rows[rows.length - 1]?.node.id }
  if (key === 2054 || key === 2119 || key === 2050) return {
    handled: true,
    select: row.node.disabled ? undefined : id,
    expand: row.branch ? !row.open : undefined,
  }
  return { handled: false }
}

export function treeTypeahead(
  rows: DazzTreeRow[], id: string, prefix: string
): string | undefined {
  const index = rows.findIndex(row => row.node.id === id)
  const order = rows.slice(index + 1).concat(rows.slice(0, index + 1))
  return order.find(row => !row.node.disabled &&
    (row.node.text ?? row.node.label).trim().toLowerCase().startsWith(prefix)
  )?.node.id
}
