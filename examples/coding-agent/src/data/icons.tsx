import type { IconGlyph } from '@dazzlabs/dazzui-react'
import type { ReactNode, SVGProps } from 'react'

/**
 * The prototype draws its own glyphs rather than installing an icon set. The
 * design system takes an icon as a prop precisely so a product can bring its
 * own, and an example that pulled in a dependency to draw a chevron would be
 * demonstrating the dependency instead of the system.
 *
 * Every glyph is stroked on the same 20x20 grid at the same weight, so the set
 * reads as one hand. The stroke lives on a `<g>` rather than the `<svg>`,
 * because `.dz-icon` sets `fill: currentColor` on the element itself and an
 * inherited value would paint these outlines solid.
 */
function glyph(path: ReactNode): IconGlyph {
  return (props: SVGProps<SVGSVGElement>) => (
    <svg viewBox="0 0 20 20" {...props}>
      <g
        fill="none"
        stroke="currentColor"
        strokeWidth="1.6"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        {path}
      </g>
    </svg>
  )
}

/** The agent's own mark — it stands in for an avatar on every reply. */
export const SparkleGlyph = glyph(
  <path d="M10 3.5q.7 5.8 6.5 6.5-5.8.7-6.5 6.5-.7-5.8-6.5-6.5 5.8-.7 6.5-6.5Z" />
)

export const ChatGlyph = glyph(
  <path d="M17 11.5a2.5 2.5 0 0 1-2.5 2.5H9l-4 3v-3h-.5A2.5 2.5 0 0 1 2 11.5v-6A2.5 2.5 0 0 1 4.5 3h10A2.5 2.5 0 0 1 17 5.5Z" />
)

export const DocGlyph = glyph(
  <>
    <path d="M11.5 2.5H6A2.5 2.5 0 0 0 3.5 5v10A2.5 2.5 0 0 0 6 17.5h8a2.5 2.5 0 0 0 2.5-2.5V7.5Z" />
    <path d="M11.5 2.5v3.6a1.4 1.4 0 0 0 1.4 1.4h3.6" />
  </>
)

export const FolderGlyph = glyph(
  <path d="M2.5 13.5v-7a2 2 0 0 1 2-2h2.7a2 2 0 0 1 1.4.6l1.2 1.2h5.7a2 2 0 0 1 2 2v5.2a2 2 0 0 1-2 2h-11a2 2 0 0 1-2-2Z" />
)

export const SearchGlyph = glyph(
  <>
    <circle cx="9" cy="9" r="5.5" />
    <path d="m13.2 13.2 4.3 4.3" />
  </>
)

export const TerminalGlyph = glyph(
  <>
    <rect x="2.5" y="3.5" width="15" height="13" rx="2.5" />
    <path d="m6.5 8.5 2.3 2.3-2.3 2.3M11.5 13.1H14" />
  </>
)

export const PencilGlyph = glyph(
  <>
    <path d="M13.4 3.6a2 2 0 1 1 2.8 2.8l-8.7 8.7-3.6 1 1-3.6Z" />
    <path d="m12.2 4.8 2.8 2.8" />
  </>
)

export const CheckGlyph = glyph(<path d="m4.5 10.5 3.6 3.6 7.4-8.2" />)

export const ChevronGlyph = glyph(<path d="m8 5.5 4.5 4.5L8 14.5" />)

export const PlusGlyph = glyph(<path d="M10 4.5v11M4.5 10h11" />)

export const SidebarGlyph = glyph(
  <>
    <rect x="2.5" y="3.5" width="15" height="13" rx="2.5" />
    <path d="M8 3.5v13" />
  </>
)

export const ClipGlyph = glyph(
  <path d="m14.6 9.3-5.1 5.1a3 3 0 0 1-4.3-4.3l6.5-6.5a2 2 0 0 1 2.9 2.9l-6.5 6.5a1 1 0 0 1-1.5-1.5l5.7-5.7" />
)

export const SendGlyph = glyph(<path d="M10 16V4.5M5 9.5 10 4.5l5 5" />)

export const StopGlyph = glyph(
  <rect x="5.5" y="5.5" width="9" height="9" rx="1.5" />
)

export const BranchGlyph = glyph(
  <>
    <circle cx="6" cy="5" r="2" />
    <circle cx="6" cy="15" r="2" />
    <circle cx="14" cy="5" r="2" />
    <path d="M6 7v6M14 7v1.5a4 4 0 0 1-4 4H6" />
  </>
)

export const ShieldGlyph = glyph(
  <path d="M10 2.8 4 5v4.6c0 3.4 2.4 6.5 6 7.6 3.6-1.1 6-4.2 6-7.6V5Z" />
)

export const ClockGlyph = glyph(
  <>
    <circle cx="10" cy="10" r="7" />
    <path d="M10 5.8V10l2.8 1.8" />
  </>
)

export const LayersGlyph = glyph(
  <path d="m10 2.8 7 3.6-7 3.6-7-3.6ZM3 10l7 3.6L17 10M3 13.6l7 3.6 7-3.6" />
)

export const IssueGlyph = glyph(
  <>
    <circle cx="10" cy="10" r="7" />
    <circle cx="10" cy="10" r="2.2" />
  </>
)

export const PullGlyph = glyph(
  <>
    <circle cx="6" cy="5" r="2" />
    <circle cx="6" cy="15" r="2" />
    <circle cx="14" cy="15" r="2" />
    <path d="M6 7v6M14 13V9.5a3 3 0 0 0-3-3H8.5m1.7-1.7L8.5 6.5l1.7 1.7" />
  </>
)

export const DiffGlyph = glyph(
  <path d="M6.5 3.8v5.4M3.8 6.5h5.4M10.8 13.5h5.4" />
)

export const GearGlyph = glyph(
  <>
    <circle cx="10" cy="10" r="2.6" />
    <path d="M10 2.6h.9l.4 2a5.9 5.9 0 0 1 1.7 1l1.9-.8.7.8.7 1.2-1.5 1.4a6 6 0 0 1 0 2l1.5 1.4-.7 1.2-.7.8-1.9-.8a5.9 5.9 0 0 1-1.7 1l-.4 2H9.1l-.4-2a5.9 5.9 0 0 1-1.7-1l-1.9.8-.7-.8-.7-1.2 1.5-1.4a6 6 0 0 1 0-2L3.7 6.8l.7-1.2.7-.8 1.9.8a5.9 5.9 0 0 1 1.7-1l.4-2Z" />
  </>
)

export const KeyboardGlyph = glyph(
  <>
    <rect x="2" y="5" width="16" height="10" rx="2" />
    <path d="M5.5 8.5h.01M8.5 8.5h.01M11.5 8.5h.01M14.5 8.5h.01M6.5 11.5h7" />
  </>
)
