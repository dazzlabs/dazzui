/**
 * The twelve themes the token graph carries, in the order the toolbar lists
 * them — the six families, light before dark. Studio Light is the default
 * and is what `:root` holds, so it sets no attribute at all — selecting it
 * has to *remove* `data-theme` rather than write `studio-light`, or none of
 * the theme blocks would match and the page would look right for the wrong
 * reason.
 *
 * Both the preview and the parity extractor read this list, which is what
 * keeps the toolbar and the reference data on the same themes.
 */
export const THEMES = [
  { value: 'studio-light', title: 'Studio Light' },
  { value: 'studio-dark', title: 'Studio Dark' },
  { value: 'bright-light', title: 'Bright Light' },
  { value: 'bright-dark', title: 'Bright Dark' },
  { value: 'frost-light', title: 'Frost Light' },
  { value: 'frost-dark', title: 'Frost Dark' },
  { value: 'graphite-light', title: 'Graphite Light' },
  { value: 'graphite-dark', title: 'Graphite Dark' },
  { value: 'ember-light', title: 'Ember Light' },
  { value: 'ember-dark', title: 'Ember Dark' },
  { value: 'nocturne-light', title: 'Nocturne Light' },
  { value: 'nocturne-dark', title: 'Nocturne Dark' },
] as const

/** The two appearances a theme comes in. */
export type Mode = 'light' | 'dark'

export const MODES: readonly { value: Mode; title: string }[] = [
  { value: 'light', title: 'Light' },
  { value: 'dark', title: 'Dark' },
]

/**
 * A theme *style* — a family — and the theme it is in each mode. The
 * toolbar picks the two separately: the style, then the mode. Every style
 * today comes in both; `modes` stays partial so one that does not can say
 * so, and the toolbar disables the mode it lacks.
 */
export interface ThemeStyle {
  value: string
  title: string
  modes: Partial<Record<Mode, string>>
}

/** The styles, grouped the way the toolbar lists them. */
export interface ThemeStyleGroup {
  title: string
  styles: readonly ThemeStyle[]
}

/** A style that comes in both modes, as `<value>-light` / `<value>-dark`. */
const paired = (value: string, title: string): ThemeStyle => ({
  value,
  title,
  modes: { light: `${value}-light`, dark: `${value}-dark` },
})

/**
 * Every style the toolbar offers. The first group is the twelve in
 * `THEMES`; the other two are the imported themes — the desktops of
 * `ds/themes/desktops.yaml` and Omarchy's of
 * `ds/themes/omarchy.yaml`, in their files' order. The imported
 * ones are not in `THEMES`, since the parity extractor and the golden suite
 * measure the drawn families the ports are held to, and an imported theme is
 * derived from three colours rather than drawn.
 */
export const THEME_STYLES: readonly ThemeStyleGroup[] = [
  {
    title: 'DazzUI',
    styles: [
      paired('studio', 'Studio'),
      paired('bright', 'Bright'),
      paired('frost', 'Frost'),
      paired('graphite', 'Graphite'),
      paired('ember', 'Ember'),
      paired('nocturne', 'Nocturne'),
    ],
  },
  {
    title: 'Desktops',
    styles: [
      paired('macos27', 'macOS 27 Golden Gate'),
      paired('macos15', 'macOS 15 Sequoia'),
      paired('windows11', 'Windows 11'),
      paired('ubuntu', 'Ubuntu'),
      paired('debian', 'Debian (GNOME)'),
      paired('fedora', 'Fedora (GNOME)'),
      paired('kde', 'KDE Plasma'),
    ],
  },
  {
    title: 'Omarchy',
    styles: [
      paired('omarchy-tokyo-night', 'Tokyo Night'),
      paired('omarchy-catppuccin', 'Catppuccin'),
      paired('omarchy-catppuccin-latte', 'Catppuccin Latte'),
      paired('omarchy-ethereal', 'Ethereal'),
      paired('omarchy-everforest', 'Everforest'),
      paired('omarchy-flexoki-light', 'Flexoki Light'),
      paired('omarchy-gruvbox', 'Gruvbox'),
      paired('omarchy-hackerman', 'Hackerman'),
      paired('omarchy-kanagawa', 'Kanagawa'),
      paired('omarchy-last-horizon', 'Last Horizon'),
      paired('omarchy-lumon', 'Lumon'),
      paired('omarchy-lupine', 'Lupine'),
      paired('omarchy-matte-black', 'Matte Black'),
      paired('omarchy-miasma', 'Miasma'),
      paired('omarchy-nord', 'Nord'),
      paired('omarchy-osaka-jade', 'Osaka Jade'),
      paired('omarchy-retro-82', 'Retro 82'),
      paired('omarchy-ristretto', 'Ristretto'),
      paired('omarchy-rose-pine', 'Rosé Pine'),
      paired('omarchy-solitude', 'Solitude'),
      paired('omarchy-vantablack', 'Vantablack'),
      paired('omarchy-white', 'White'),
    ],
  },
]

const STYLES = THEME_STYLES.flatMap(group => group.styles)

/** Every theme the toolbar can reach, as `data-theme` values. */
export const ALL_THEMES: readonly string[] = STYLES.flatMap(style =>
  MODES.flatMap(mode => style.modes[mode.value] ?? [])
)

/** The style and mode a theme is, or undefined for a theme no style has. */
export function splitTheme(
  theme: string
): { style: ThemeStyle; mode: Mode } | undefined {
  for (const style of STYLES) {
    for (const { value: mode } of MODES) {
      if (style.modes[mode] === theme) return { style, mode }
    }
  }
  return undefined
}

/**
 * The theme for a style in a mode — or, when the style does not come in
 * that mode, in the one it does, so changing style never lands on nothing.
 */
export function joinTheme(styleValue: string, mode: Mode): string {
  const style = STYLES.find(entry => entry.value === styleValue) ?? STYLES[0]
  return style.modes[mode] ?? Object.values(style.modes)[0]!
}

export type ThemeValue = (typeof THEMES)[number]['value']

/** The default theme: the one that clears `data-theme` instead of setting it. */
export const DEFAULT_THEME: ThemeValue = 'studio-light'

/** Point the document at a theme the way the token stylesheet expects. */
export function applyTheme(root: HTMLElement, theme: string) {
  if (theme === DEFAULT_THEME) root.removeAttribute('data-theme')
  else root.setAttribute('data-theme', theme)
}
