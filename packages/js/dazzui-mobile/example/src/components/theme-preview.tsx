import type { Family } from '@dazzlabs/dazzui-mobile'

/**
 * A theme in miniature: a phone of its own — the canvas, a card on it, two
 * lines of its ink and a capsule in its accent. Every colour is asked for
 * by the `data-family` and `data-appearance` the preview carries, read off
 * the mobile sheet, so it is right in whatever theme the window is in.
 */
export function ThemePreview({
  family,
  mode,
}: {
  family: Family
  mode: 'light' | 'dark'
}) {
  return (
    <span
      className="g-preview"
      data-family={family}
      data-appearance={mode}
      aria-hidden
    >
      <span className="g-preview__paper">
        <span className="g-preview__ink" />
        <span className="g-preview__ink" />
        <span className="g-preview__accent" />
      </span>
    </span>
  )
}
