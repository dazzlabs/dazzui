import type { Family } from '@dazzlabs/dazzui-mobile'
import { useSyncExternalStore } from 'react'

/**
 * Every family the mobile sheet carries, grouped as the desktop example
 * groups them. A family is a pair; which half shows is the mode, and
 * `system` leaves that to the OS, as the native packages do.
 */
export interface FamilyEntry {
  value: Family
  title: string
}

export interface FamilyGroup {
  title: string
  families: readonly FamilyEntry[]
}

export const FAMILY_GROUPS: readonly FamilyGroup[] = [
  {
    title: 'DazzUI',
    families: [
      { value: 'studio', title: 'Studio' },
      { value: 'bright', title: 'Bright' },
      { value: 'frost', title: 'Frost' },
      { value: 'graphite', title: 'Graphite' },
      { value: 'ember', title: 'Ember' },
      { value: 'nocturne', title: 'Nocturne' },
    ],
  },
  {
    title: 'Desktops',
    families: [
      { value: 'macos27', title: 'macOS 27 Golden Gate' },
      { value: 'macos15', title: 'macOS 15 Sequoia' },
      { value: 'windows11', title: 'Windows 11' },
      { value: 'ubuntu', title: 'Ubuntu' },
      { value: 'debian', title: 'Debian (GNOME)' },
      { value: 'fedora', title: 'Fedora (GNOME)' },
      { value: 'kde', title: 'KDE Plasma' },
    ],
  },
  {
    title: 'Omarchy',
    families: [
      { value: 'omarchy-tokyo-night', title: 'Tokyo Night' },
      { value: 'omarchy-catppuccin', title: 'Catppuccin' },
      { value: 'omarchy-catppuccin-latte', title: 'Catppuccin Latte' },
      { value: 'omarchy-ethereal', title: 'Ethereal' },
      { value: 'omarchy-everforest', title: 'Everforest' },
      { value: 'omarchy-flexoki-light', title: 'Flexoki Light' },
      { value: 'omarchy-gruvbox', title: 'Gruvbox' },
      { value: 'omarchy-hackerman', title: 'Hackerman' },
      { value: 'omarchy-kanagawa', title: 'Kanagawa' },
      { value: 'omarchy-last-horizon', title: 'Last Horizon' },
      { value: 'omarchy-lumon', title: 'Lumon' },
      { value: 'omarchy-lupine', title: 'Lupine' },
      { value: 'omarchy-matte-black', title: 'Matte Black' },
      { value: 'omarchy-miasma', title: 'Miasma' },
      { value: 'omarchy-nord', title: 'Nord' },
      { value: 'omarchy-osaka-jade', title: 'Osaka Jade' },
      { value: 'omarchy-retro-82', title: 'Retro 82' },
      { value: 'omarchy-ristretto', title: 'Ristretto' },
      { value: 'omarchy-rose-pine', title: 'Rosé Pine' },
      { value: 'omarchy-solitude', title: 'Solitude' },
      { value: 'omarchy-vantablack', title: 'Vantablack' },
      { value: 'omarchy-white', title: 'White' },
    ],
  },
]

export const FAMILIES = FAMILY_GROUPS.flatMap(group => group.families)

export const MODES = [
  { value: 'system', label: 'System' },
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
] as const

export type Mode = (typeof MODES)[number]['value']

export interface Theme {
  family: Family
  mode: Mode
}

export const DEFAULT_THEME: Theme = { family: 'studio', mode: 'system' }

export const PLATFORMS = [
  { value: 'both', label: 'Both' },
  { value: 'ios', label: 'iOS' },
  { value: 'android', label: 'Android' },
] as const

export type Platforms = (typeof PLATFORMS)[number]['value']

export function parseFamily(value: string | null): Family {
  return FAMILIES.find(family => family.value === value)?.value ?? 'studio'
}

export function parseMode(value: string | null): Mode {
  return MODES.find(mode => mode.value === value)?.value ?? 'system'
}

export function parsePlatforms(value: string | null): Platforms {
  return PLATFORMS.find(p => p.value === value)?.value ?? 'both'
}

const query = () => window.matchMedia('(prefers-color-scheme: dark)')

/** Whether the OS is dark right now, re-read when it changes. */
export function useSystemDark() {
  return useSyncExternalStore(
    notify => {
      const media = query()
      media.addEventListener('change', notify)
      return () => media.removeEventListener('change', notify)
    },
    () => query().matches
  )
}

/**
 * The window around the phones is drawn with the desktop kit, whose
 * themes are chosen whole; `system` resolves to the half the OS is
 * showing. Studio Light is what the desktop sheet's `:root` holds, so it
 * is applied by removing `data-theme` rather than writing it.
 */
export function applyTheme(
  root: HTMLElement,
  theme: Theme,
  systemDark: boolean
) {
  const mode =
    theme.mode === 'system' ? (systemDark ? 'dark' : 'light') : theme.mode
  const id = `${theme.family}-${mode}`
  if (id === 'studio-light') root.removeAttribute('data-theme')
  else root.setAttribute('data-theme', id)
}
