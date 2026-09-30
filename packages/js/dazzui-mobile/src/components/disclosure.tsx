import { Collapsible as Base } from '@base-ui/react/collapsible'
import {
  createContext,
  type CSSProperties,
  type ReactNode,
  useContext,
} from 'react'

import { cx } from '../lib/cx'
import { ChevronRight } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

/** How many disclosures this one sits inside; each level indents a step. */
const DepthContext = createContext(0)

export interface DisclosureProps extends Omit<
  Base.Root.Props,
  'className' | 'style' | 'render' | 'children' | 'title'
> {
  className?: string
  style?: CSSProperties
  /** The row's primary line. */
  title: ReactNode
  /** The line beneath it. */
  subtitle?: ReactNode
  /** A glyph at the leading edge, as a `ListItem` takes it. */
  icon?: ReactNode
  /** The tile the glyph sits on, on iOS; the glyph's own colour on Android. */
  iconColor?: string
  /** What the row reveals — usually more `ListItem`s. */
  children?: ReactNode
}

/**
 * A row in a list that opens onto rows of its own. On iOS it is SwiftUI's
 * `DisclosureGroup` in an inset grouped list: the title, and a chevron in
 * the tint at the trailing edge that turns from pointing right to
 * pointing down; the rows it reveals join the group a step in, and the
 * hairlines carry on between them. On Android it is Material's expandable
 * list item: a 56dp row with the state layer and an expand arrow that
 * turns over as the content opens beneath it. The open state is Base
 * UI's collapsible, and the panel's height animates from Base UI's
 * measurement.
 */
export function Disclosure({
  title,
  subtitle,
  icon,
  iconColor,
  className,
  style,
  children,
  ...props
}: DisclosureProps) {
  const platform = usePlatform()
  const depth = useContext(DepthContext)
  return (
    <Base.Root
      data-platform={platform}
      className={cx('dzm-disclosure', className)}
      style={{ ...style, '--dzm-disclosure-depth': depth } as never}
      {...props}
    >
      <Base.Trigger
        data-platform={platform}
        data-interactive=""
        data-two-line={subtitle ? '' : undefined}
        className="dzm-list-item dzm-disclosure__row dzm-m3-layer"
      >
        {icon ? (
          <span
            className="dzm-list-item__icon"
            style={
              iconColor
                ? ({ '--dzm-list-icon-fill': iconColor } as never)
                : undefined
            }
          >
            {icon}
          </span>
        ) : null}
        <span className="dzm-list-item__text">
          <span className="dzm-list-item__title">{title}</span>
          {subtitle ? (
            <span className="dzm-list-item__subtitle">{subtitle}</span>
          ) : null}
        </span>
        <span className="dzm-disclosure__chevron">
          <ChevronRight />
        </span>
      </Base.Trigger>
      <Base.Panel data-platform={platform} className="dzm-disclosure__panel">
        <div
          className="dzm-disclosure__content"
          style={{ '--dzm-disclosure-depth': depth + 1 } as never}
        >
          <DepthContext.Provider value={depth + 1}>
            {children}
          </DepthContext.Provider>
        </div>
      </Base.Panel>
    </Base.Root>
  )
}
