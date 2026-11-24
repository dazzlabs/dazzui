import { IMPORTED } from '../scripts/themes.mjs'

/**
 * Canonical metadata for every theme and family in the token graph.
 *
 * Shared by the web (`themes.ts`), SwiftUI (`ThemeFamily.swift`) and HarmonyOS
 * (`ThemeData.ets`) generators so every platform agrees on family IDs, display
 * titles, grouping, and light/dark pairs.
 */
export const DRAWN_FAMILIES = [
  { family: 'bright', title: 'Bright' },
  { family: 'studio', title: 'Studio' },
  { family: 'frost', title: 'Frost' },
  { family: 'graphite', title: 'Graphite' },
  { family: 'ember', title: 'Ember' },
  { family: 'nocturne', title: 'Nocturne' },
]

const GROUP_BY_FILE = {
  'desktops.yaml': { id: 'desktops', title: 'Desktops' },
  'applications.yaml': { id: 'applications', title: 'Applications' },
  'omarchy.yaml': { id: 'omarchy', title: 'Omarchy' },
}

const GROUP_ORDER = [
  { id: 'drawn', title: 'DazzUI' },
  { id: 'desktops', title: 'Desktops' },
  { id: 'applications', title: 'Applications' },
  { id: 'omarchy', title: 'Omarchy' },
]

function familyTitleOf(theme) {
  return theme.title.replace(/\s+(Light|Dark)(?:\s+\([^)]+\))?$/, '').trim()
}

function themeTitleOf(familyTitle, appearance, rawTitle) {
  if (rawTitle && /\b(Light|Dark)\b/.test(rawTitle)) return rawTitle
  return `${familyTitle} ${appearance === 'dark' ? 'Dark' : 'Light'}`
}

function titleFromId(id) {
  return id
    .split(/[-_\s]+/)
    .filter(Boolean)
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

/**
 * Build the ordered catalog of theme groups, families and individual themes
 * present in `modeSelectors` (plus the `:root` `bright-light` default).
 */
export function buildCatalog(modeSelectors = []) {
  const present = new Set(['bright-light'])
  for (const { selectors = [] } of modeSelectors) {
    for (const selector of selectors) {
      const match = /\[data-theme='([^']+)'\]/.exec(selector)
      if (match) present.add(match[1])
    }
  }

  const importedByTheme = new Map()
  const importedByFamily = new Map()
  for (const source of IMPORTED) {
    const group = GROUP_BY_FILE[source.file] ?? {
      id: 'omarchy',
      title: 'Omarchy',
    }
    for (const theme of source.themes) {
      const family = theme.name.replace(/-(light|dark)$/, '')
      const familyTitle = familyTitleOf(theme)
      importedByTheme.set(theme.name, {
        group: group.id,
        family,
        familyTitle,
        title: themeTitleOf(familyTitle, theme.scheme, theme.title),
      })
      if (!importedByFamily.has(family)) {
        importedByFamily.set(family, {
          family,
          title: familyTitle,
          group: group.id,
        })
      }
    }
  }

  const familyEntries = new Map()
  const addFamily = (family, title, group) => {
    const light = `${family}-light`
    const dark = `${family}-dark`
    if (!present.has(light) || !present.has(dark)) return
    if (familyEntries.has(family)) return
    familyEntries.set(family, {
      family,
      title,
      group,
      light,
      dark,
    })
  }

  // Drawn families first, in canonical order (Bright leads as `:root` default).
  for (const { family, title } of DRAWN_FAMILIES) {
    addFamily(family, title, 'drawn')
  }

  // Imported families in canonical group order: desktops, applications, omarchy.
  for (const groupId of ['desktops', 'applications', 'omarchy']) {
    for (const entry of importedByFamily.values()) {
      if (entry.group === groupId) {
        addFamily(entry.family, entry.title, entry.group)
      }
    }
  }

  // Any remaining paired themes discovered in `modeSelectors`.
  for (const name of present) {
    const match = /^(.+)-(light|dark)$/.exec(name)
    if (!match) continue
    const family = match[1]
    if (!familyEntries.has(family)) {
      addFamily(family, titleFromId(family), 'omarchy')
    }
  }

  const families = [...familyEntries.values()]
  const groups = GROUP_ORDER.map(group => ({
    id: group.id,
    title: group.title,
    families: families
      .filter(entry => entry.group === group.id)
      .map(entry => entry.family),
  })).filter(group => group.families.length > 0)

  const themes = families.flatMap(entry =>
    ['light', 'dark'].map(appearance => {
      const name = appearance === 'dark' ? entry.dark : entry.light
      const imported = importedByTheme.get(name)
      return {
        name,
        family: entry.family,
        appearance,
        group: entry.group,
        title: imported?.title ?? themeTitleOf(entry.title, appearance),
        familyTitle: entry.title,
        isDefault: name === 'bright-light',
      }
    })
  )

  return { groups, families, themes }
}
