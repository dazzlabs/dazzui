/**
 * Every theme the token graph carries, as families that each come in light
 * and dark: the six drawn ones, then the imported ones in the order of
 * `ds/themes/desktops.yaml` and `ds/themes/omarchy.yaml`. Bright Light is
 * what `:root` holds, so it is applied by removing `data-theme` rather than
 * writing it, and it is the list's first family because it is the default.
 */
export interface Family {
  value: string
  title: string
}

export interface FamilyGroup {
  title: string
  families: readonly Family[]
}

export const FAMILY_GROUPS: readonly FamilyGroup[] = [
  {
    title: 'DazzUI',
    families: [
      { value: 'bright', title: 'Bright' },
      { value: 'studio', title: 'Studio' },
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
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
] as const

export type Mode = (typeof MODES)[number]['value']

export interface Theme {
  family: string
  mode: Mode
}

export const DEFAULT_THEME: Theme = { family: 'bright', mode: 'light' }

export const themeId = ({ family, mode }: Theme) => `${family}-${mode}`

/** A family's name contains hyphens of its own, so the mode is the last part. */
export function parseTheme(id: string | null): Theme {
  const cut = (id ?? '').lastIndexOf('-')
  const family = (id ?? '').slice(0, cut)
  const mode = (id ?? '').slice(cut + 1)
  if (!FAMILIES.some(story => story.value === family)) return DEFAULT_THEME
  if (mode !== 'light' && mode !== 'dark') return DEFAULT_THEME
  return { family, mode }
}

export function applyTheme(root: HTMLElement, theme: Theme) {
  const id = themeId(theme)
  if (id === themeId(DEFAULT_THEME)) root.removeAttribute('data-theme')
  else root.setAttribute('data-theme', id)
}
