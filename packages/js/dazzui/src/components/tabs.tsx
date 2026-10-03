import { DismissRegular, MoreHorizontalRegular } from '@fluentui/react-icons'
import {
  useLayoutEffect,
  useRef,
  useState,
  type KeyboardEvent,
  type ReactNode,
} from 'react'

import { cx } from '../lib/cx'
import { Icon, type IconGlyph } from './icon'
import { IconButton } from './icon-button'
import { Menu } from './menu'

/** What a tab's dot says: nothing is drawn without it. */
export type TabStatus = 'neutral' | 'primary' | 'success' | 'warning' | 'danger'

export interface TabItem<T extends string = string> {
  value: T
  /** The tab's name. Plain text: it is also the tooltip when the tab is drawn as its icon alone. */
  label: string
  icon?: IconGlyph
  /** Trailing count, in the label's own line. */
  count?: number
  /** A dot before the label: a state the reader cannot see while another tab is showing. */
  status?: TabStatus
  /** Whether the dot pulses — something is running. */
  busy?: boolean
  /** A quiet second word after the label, e.g. the machine a terminal is on. */
  detail?: string
  /** Whether the tab can be closed; `onClose` on the tabs is what closes it. */
  closable?: boolean
  /** The tooltip; the label by default. */
  title?: string
  disabled?: boolean
}

export interface TabsProps<T extends string = string> {
  items: readonly TabItem<T>[]
  value: T | null
  onValueChange: (value: T) => void
  /** Closes a `closable` tab. */
  onClose?: (value: T) => void
  /** What follows the last tab and stays put while they give way: the control that opens another. */
  trailing?: ReactNode
  'aria-label'?: string
  className?: string
}

/** What the more-button takes of the row; reserved once anything has to go behind it. */
const MORE_WIDTH = 28

/**
 * Tabs: the navigation of a view or a toolbar. Each is its name in quiet ink,
 * and the one showing is in full ink over a rule along the row's foot — no
 * fill, so a row of them sits in a head without outweighing its title. Where
 * `PillTabs` are filters standing on a page, these are where you are.
 *
 * A tab can lead with an icon, carry a count, a status dot and a close
 * button, so fixed sections (Pull Requests, Issues) and open documents
 * (editors, terminals) are the same control.
 *
 * The row never scrolls and never wraps. When the tabs do not fit, the ones
 * not showing give up their names and keep their icons; when they still do
 * not fit, the last ones go behind a more-button. The tab that is showing is
 * always whole.
 */
export function Tabs<T extends string = string>({
  items,
  value,
  onValueChange,
  onClose,
  trailing,
  'aria-label': ariaLabel,
  className,
}: TabsProps<T>) {
  const root = useRef<HTMLDivElement>(null)
  const ruler = useRef<HTMLDivElement>(null)
  const tail = useRef<HTMLSpanElement>(null)
  const [fit, setFit] = useState<{
    compact: boolean
    shown: number
    basis: number
    floor: number
  }>({
    compact: false,
    shown: items.length,
    basis: 0,
    floor: 0,
  })

  /*
   * The ruler is the same tabs drawn out of sight, each once whole and once
   * as its icon, so what fits is arithmetic on widths that do not depend on
   * what is currently drawn — measuring the visible row would chase itself.
   */
  const signature = items
    .map(
      item =>
        `${item.value}|${item.label}|${item.count ?? ''}|${item.detail ?? ''}|${item.closable ? 1 : 0}|${item.icon ? 1 : 0}|${item.status ?? ''}`
    )
    .join('\n')
  useLayoutEffect(() => {
    const element = root.current
    const measure = ruler.current
    if (!element || !measure) return
    const solve = () => {
      const whole = [
        ...measure.querySelectorAll<HTMLElement>('[data-ruler="whole"]'),
      ].map(node => node.offsetWidth)
      const small = [
        ...measure.querySelectorAll<HTMLElement>('[data-ruler="compact"]'),
      ].map(node => node.offsetWidth)
      const trailingWidth = tail.current?.offsetWidth ?? 0
      const basis = whole.reduce((sum, width) => sum + width, 0) + trailingWidth
      const room = element.clientWidth - trailingWidth
      const active = items.findIndex(item => item.value === value)
      // The least the row is ever given: the tab that is showing, whole, the way to the rest and what trails.
      const floor = Math.min(
        basis,
        (whole[Math.max(active, 0)] ?? 0) +
          (items.length > 1 ? MORE_WIDTH : 0) +
          trailingWidth
      )
      const sum = (widths: readonly number[], count: number) =>
        widths.slice(0, count).reduce((total, width) => total + width, 0)

      let next = { compact: false, shown: items.length, basis, floor }
      if (sum(whole, items.length) > room) {
        // The tab that is showing keeps its name; the others are their icons.
        const mixed = small.map((width, index) =>
          index === active ? (whole[index] ?? width) : width
        )
        let shown = items.length
        while (
          shown > 1 &&
          sum(mixed, shown) + (shown < items.length ? MORE_WIDTH : 0) > room
        )
          shown -= 1
        next = { compact: true, shown, basis, floor }
      }
      setFit(current =>
        current.compact === next.compact &&
        current.shown === next.shown &&
        current.basis === next.basis &&
        current.floor === next.floor
          ? current
          : next
      )
    }
    solve()
    const observer = new ResizeObserver(solve)
    observer.observe(element)
    return () => observer.disconnect()
    // `signature` stands for `items`: the widths change only when what the tabs say does.
  }, [signature, value])

  // The tab that is showing is never the one behind the more-button: it takes the last place in the row.
  const active = items.findIndex(item => item.value === value)
  const order = items.map((_, index) => index)
  if (active >= fit.shown && fit.shown > 0) {
    order.splice(active, 1)
    order.splice(fit.shown - 1, 0, active)
  }
  const shown = order.slice(0, fit.shown).map(index => items[index]!)
  const hidden = order.slice(fit.shown).map(index => items[index]!)

  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    const keys = ['ArrowLeft', 'ArrowRight', 'Home', 'End']
    if (!keys.includes(event.key)) return
    const enabled = shown.filter(item => !item.disabled)
    const at = enabled.findIndex(item => item.value === value)
    const to =
      event.key === 'Home'
        ? 0
        : event.key === 'End'
          ? enabled.length - 1
          : (at + (event.key === 'ArrowRight' ? 1 : -1) + enabled.length) %
            enabled.length
    const target = enabled[to]
    if (!target) return
    event.preventDefault()
    onValueChange(target.value)
    root.current
      ?.querySelector<HTMLElement>(`[data-tab="${CSS.escape(target.value)}"]`)
      ?.focus()
  }

  const body = (item: TabItem<T>, compact: boolean) => (
    <>
      {item.status ? (
        <span
          aria-hidden
          className="dz-tabs__status"
          data-busy={item.busy || undefined}
          data-status={item.status}
        />
      ) : null}
      {item.icon ? <Icon icon={item.icon} size={14} /> : null}
      {compact && item.icon ? null : (
        <span className="dz-tabs__label">{item.label}</span>
      )}
      {compact && item.icon ? null : item.detail ? (
        <span className="dz-tabs__detail">{item.detail}</span>
      ) : null}
      {item.count === undefined ? null : (
        <span className="dz-tabs__count">{item.count}</span>
      )}
    </>
  )

  return (
    <div
      className={cx('dz-tabs', className)}
      ref={root}
      // What the row would take whole: it is that wide when there is room and gives way when there is not.
      style={{
        flexBasis: fit.basis ? `${fit.basis}px` : undefined,
        minWidth: fit.floor ? `${fit.floor}px` : undefined,
      }}
    >
      <div
        aria-label={ariaLabel}
        className="dz-tabs__list"
        role="tablist"
        onKeyDown={onKeyDown}
      >
        {shown.map(item => {
          const selected = item.value === value
          const compact = fit.compact && !selected
          return (
            <span
              className="dz-tabs__item"
              data-active={selected || undefined}
              key={item.value}
            >
              <button
                type="button"
                role="tab"
                aria-label={compact && item.icon ? item.label : undefined}
                aria-selected={selected}
                className="dz-tabs__tab"
                data-tab={item.value}
                disabled={item.disabled}
                tabIndex={selected ? 0 : -1}
                title={item.title ?? (compact ? item.label : undefined)}
                onClick={() => onValueChange(item.value)}
              >
                {body(item, compact)}
              </button>
              {item.closable && onClose && !compact ? (
                <IconButton
                  className="dz-tabs__close"
                  label={`Close ${item.label}`}
                  size="tiny"
                  tint="neutral"
                  variant="plain"
                  onClick={() => onClose(item.value)}
                >
                  <Icon icon={DismissRegular} size={12} />
                </IconButton>
              ) : null}
            </span>
          )
        })}
      </div>

      {hidden.length > 0 ? (
        <Menu
          align="end"
          trigger={
            <IconButton
              className="dz-tabs__more"
              label={`${hidden.length} more`}
              size="tiny"
              tint="neutral"
              variant="plain"
            >
              <Icon icon={MoreHorizontalRegular} size={14} />
            </IconButton>
          }
          items={hidden.map(item => ({
            label:
              item.count === undefined
                ? item.label
                : `${item.label} · ${item.count}`,
            icon: item.icon ? <Icon icon={item.icon} size={14} /> : undefined,
            disabled: item.disabled,
            onSelect: () => onValueChange(item.value),
          }))}
        />
      ) : null}

      {trailing ? (
        <span className="dz-tabs__trailing" ref={tail}>
          {trailing}
        </span>
      ) : null}

      {/* The ruler: every tab whole, then every tab as its icon. Out of sight and out of the tree a reader walks. */}
      <div aria-hidden className="dz-tabs__ruler" ref={ruler}>
        {items.map(item => (
          <span
            className="dz-tabs__item"
            data-ruler="whole"
            key={`whole-${item.value}`}
          >
            <span className="dz-tabs__tab">{body(item, false)}</span>
            {item.closable && onClose ? (
              <span className="dz-tabs__close dz-tabs__close--ruler" />
            ) : null}
          </span>
        ))}
        {items.map(item => (
          <span
            className="dz-tabs__item"
            data-ruler="compact"
            key={`compact-${item.value}`}
          >
            <span className="dz-tabs__tab">{body(item, true)}</span>
          </span>
        ))}
      </div>
    </div>
  )
}
