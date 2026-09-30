import type { ComponentProps } from 'react'

/*
 * The few glyphs the kit draws itself — a chevron, a check, a clear button,
 * a magnifier, a plus and a minus, a drop-down arrow. SF Symbols and
 * Material Symbols are the platforms' own and cannot ship in a web package,
 * so these are strokes drawn to read as either: round caps and joins, at the
 * weight each OS sets its own glyphs in. A host's content glyphs come from
 * whatever set it uses; the kit never sizes them.
 */

type GlyphProps = ComponentProps<'svg'>

function Glyph({ children, ...props }: GlyphProps) {
  return (
    <svg
      aria-hidden
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={2.4}
      strokeLinecap="round"
      strokeLinejoin="round"
      {...props}
    >
      {children}
    </svg>
  )
}

export function ChevronRight(props: GlyphProps) {
  return (
    <Glyph {...props}>
      <path d="M9 5l7 7-7 7" />
    </Glyph>
  )
}

export function ChevronLeft(props: GlyphProps) {
  return (
    <Glyph {...props}>
      <path d="M15 5l-7 7 7 7" />
    </Glyph>
  )
}

export function ChevronUpDown(props: GlyphProps) {
  return (
    <Glyph {...props}>
      <path d="M8 9.5l4-4 4 4M8 14.5l4 4 4-4" />
    </Glyph>
  )
}

export function DropDown(props: GlyphProps) {
  return (
    <svg aria-hidden viewBox="0 0 24 24" fill="currentColor" {...props}>
      <path d="M7 10l5 5 5-5z" />
    </svg>
  )
}

export function Check(props: GlyphProps) {
  return (
    <Glyph {...props}>
      <path d="M5 12.5l4.5 4.5L19 7.5" />
    </Glyph>
  )
}

export function Close(props: GlyphProps) {
  return (
    <Glyph {...props}>
      <path d="M6.5 6.5l11 11M17.5 6.5l-11 11" />
    </Glyph>
  )
}

/** A filled circle carrying a cross: iOS's `xmark.circle.fill`. */
export function ClearCircle(props: GlyphProps) {
  return (
    <svg aria-hidden viewBox="0 0 24 24" {...props}>
      <circle cx="12" cy="12" r="11" fill="currentColor" />
      <path
        d="M8.2 8.2l7.6 7.6M15.8 8.2l-7.6 7.6"
        stroke="var(--dzm-glyph-contrast, #fff)"
        strokeWidth={2.2}
        strokeLinecap="round"
      />
    </svg>
  )
}

export function Search(props: GlyphProps) {
  return (
    <Glyph {...props}>
      <circle cx="10.5" cy="10.5" r="6.5" />
      <path d="M15.5 15.5L20 20" />
    </Glyph>
  )
}

export function Plus(props: GlyphProps) {
  return (
    <Glyph {...props}>
      <path d="M12 5v14M5 12h14" />
    </Glyph>
  )
}

export function Minus(props: GlyphProps) {
  return (
    <Glyph {...props}>
      <path d="M5 12h14" />
    </Glyph>
  )
}
