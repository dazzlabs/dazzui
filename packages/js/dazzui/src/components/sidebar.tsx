import { useRender } from '@base-ui/react/use-render'
import { PanelLeft20Regular } from '@fluentui/react-icons'
import {
  type ComponentPropsWithoutRef,
  createContext,
  type ReactElement,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { Divider, type DividerProps } from './divider'
import { Icon } from './icon'
import { IconButton, type IconButtonProps } from './icon-button'
import { SectionLabel } from './section-label'
import { TextField, type TextFieldProps } from './text-field'
import { Tooltip, TooltipProvider } from './tooltip'

/*
 * The sidebar, after shadcn's: one column and the parts it is built from,
 * rather than a component per place a column can stand. A second navigation
 * column is a second `Sidebar` with a narrower `--sidebar-width`; the
 * information column on the far edge is a `Sidebar` with `side="right"`.
 * What differs between them is what is put inside, not the column.
 *
 * Two of shadcn's parts are not here. The mobile `Sheet` is a phone's
 * concern and this is a desktop kit; and `SidebarMenuSkeleton` needs a
 * skeleton primitive the system does not have yet.
 */

/** ⌘B, as every editor with a sidebar has it. */
const KEYBOARD_SHORTCUT = 'b'

export type SidebarSide = 'left' | 'right'

export type SidebarVariant = 'sidebar' | 'floating' | 'inset'

export type SidebarCollapsible = 'offcanvas' | 'icon' | 'none'

interface SidebarContextValue {
  state: 'expanded' | 'collapsed'
  open: boolean
  setOpen: (open: boolean) => void
  toggleSidebar: () => void
}

const SidebarContext = createContext<SidebarContextValue | null>(null)

export function useSidebar(): SidebarContextValue {
  const context = useContext(SidebarContext)
  if (!context) {
    throw new Error('useSidebar must be used within a SidebarProvider.')
  }
  return context
}

export interface SidebarProviderProps extends ComponentPropsWithoutRef<'div'> {
  defaultOpen?: boolean
  /** Controlled: whether the collapsible sidebar inside is open. */
  open?: boolean
  onOpenChange?: (open: boolean) => void
}

/**
 * The window. It holds whether the sidebar is open, binds ⌘B to it, and is
 * the flex row every column and the inset sit in — which is also where the
 * column widths are declared, so a nested sidebar can re-point one.
 */
export function SidebarProvider({
  defaultOpen = true,
  open: openProp,
  onOpenChange,
  className,
  children,
  ...props
}: SidebarProviderProps) {
  const [ownOpen, setOwnOpen] = useState(defaultOpen)
  const open = openProp ?? ownOpen

  const setOpen = useCallback(
    (next: boolean) => {
      if (openProp === undefined) setOwnOpen(next)
      onOpenChange?.(next)
    },
    [openProp, onOpenChange]
  )

  const toggleSidebar = useCallback(() => setOpen(!open), [open, setOpen])

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== KEYBOARD_SHORTCUT) return
      if (!event.metaKey && !event.ctrlKey) return
      event.preventDefault()
      toggleSidebar()
    }
    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
  }, [toggleSidebar])

  const value = useMemo<SidebarContextValue>(
    () => ({
      state: open ? 'expanded' : 'collapsed',
      open,
      setOpen,
      toggleSidebar,
    }),
    [open, setOpen, toggleSidebar]
  )

  return (
    <SidebarContext.Provider value={value}>
      {/* One delay for every tooltip in the window, so moving down a column
          of icons opens each one at once after the first. */}
      <TooltipProvider delay={0}>
        <div className={cx('dz-sidebar-wrapper', className)} {...props}>
          {children}
        </div>
      </TooltipProvider>
    </SidebarContext.Provider>
  )
}

export interface SidebarProps extends ComponentPropsWithoutRef<'div'> {
  side?: SidebarSide
  /**
   * `sidebar` runs the window's full height with a hairline on its inner
   * edge; `floating` is a card set in from the edge; `inset` is the ground
   * the content sits raised on.
   */
  variant?: SidebarVariant
  /**
   * How it leaves. `offcanvas` slides out; `icon` narrows to a column of
   * glyphs, with the labels in tooltips; `none` stays — the second column
   * and the information column are always `none`.
   */
  collapsible?: SidebarCollapsible
}

/** The column. */
export function Sidebar({
  side = 'left',
  variant = 'sidebar',
  collapsible = 'offcanvas',
  className,
  children,
  ...props
}: SidebarProps) {
  const { state } = useSidebar()

  if (collapsible === 'none') {
    return (
      <div
        data-side={side}
        data-variant={variant}
        data-collapsible="none"
        className={cx('dz-sidebar', className)}
        {...props}
      >
        {children}
      </div>
    )
  }

  // The root is the space the column takes in the row; the container is the
  // column itself, positioned over it so it can slide without reflowing
  // what it holds. Collapsing narrows the root and moves the container.
  return (
    <div
      data-state={state}
      data-collapsible={state === 'collapsed' ? collapsible : ''}
      data-variant={variant}
      data-side={side}
      className="dz-sidebar"
    >
      <div className={cx('dz-sidebar__container', className)} {...props}>
        <div className="dz-sidebar__inner">{children}</div>
      </div>
    </div>
  )
}

export interface SidebarTriggerProps extends Omit<
  IconButtonProps,
  'label' | 'children'
> {
  label?: string
  /** The glyph; a panel by default. */
  children?: ReactNode
}

/** The toolbar button that opens and closes the sidebar. */
export function SidebarTrigger({
  label = 'Toggle sidebar',
  className,
  children,
  onClick,
  ...props
}: SidebarTriggerProps) {
  const { open, toggleSidebar } = useSidebar()

  return (
    <IconButton
      label={label}
      /* The trigger already knows, so no caller has to tell it. It is
         `aria-expanded` rather than a pressed state: this button discloses the
         column beside it, and a reader is told whether that column is showing
         rather than whether the button is held down. */
      aria-expanded={open}
      className={cx('dz-sidebar__trigger', className)}
      onClick={event => {
        onClick?.(event)
        toggleSidebar()
      }}
      {...props}
    >
      {children ?? <Icon icon={PanelLeft20Regular} />}
    </IconButton>
  )
}

export interface SidebarRailProps extends ComponentPropsWithoutRef<'button'> {
  label?: string
}

/**
 * The strip along the sidebar's edge that toggles it on click — what a
 * split view's divider does on double-click. Out of the tab order, since the
 * trigger already reaches the keyboard.
 */
export function SidebarRail({
  label = 'Toggle sidebar',
  className,
  ...props
}: SidebarRailProps) {
  const { toggleSidebar } = useSidebar()

  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      tabIndex={-1}
      className={cx('dz-sidebar__rail', className)}
      onClick={toggleSidebar}
      {...props}
    />
  )
}

/** The content beside the sidebar. With `variant="inset"` it is the raised sheet. */
export function SidebarInset({
  className,
  ...props
}: ComponentPropsWithoutRef<'main'>) {
  return <main className={cx('dz-sidebar-inset', className)} {...props} />
}

/** A field at the sidebar's own scale. */
export function SidebarInput({ className, ...props }: TextFieldProps) {
  return (
    <TextField
      size="small"
      {...(props as TextFieldProps)}
      className={cx('dz-sidebar__input', className)}
    />
  )
}

/**
 * The strip above the rows, held at the titlebar's height so it lines up
 * with the toolbar in the pane beside it — and, in a window with its
 * titlebar hidden, is where the system's own controls land.
 */
export function SidebarHeader({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-sidebar__header', className)} {...props} />
}

/** Pinned to the foot, whatever the content above it does. */
export function SidebarFooter({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-sidebar__footer', className)} {...props} />
}

export function SidebarSeparator({ className, ...props }: DividerProps) {
  return (
    <Divider className={cx('dz-sidebar__separator', className)} {...props} />
  )
}

/** The part that scrolls. */
export function SidebarContent({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-sidebar__content', className)} {...props} />
}

/**
 * A run of rows, and the label that names it. The first run usually goes
 * unlabelled — it is what the column is already about — and the gap between
 * groups does most of the work.
 */
export function SidebarGroup({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-sidebar__group', className)} {...props} />
}

export function SidebarGroupLabel({
  className,
  ...props
}: ComponentPropsWithoutRef<'span'>) {
  return (
    <SectionLabel
      faint
      className={cx('dz-sidebar__group-label', className)}
      {...props}
    />
  )
}

/** The small button on a group's label — "add", "collapse". */
export function SidebarGroupAction({ className, ...props }: IconButtonProps) {
  return (
    <IconButton
      className={cx('dz-sidebar__group-action', className)}
      {...props}
    />
  )
}

export function SidebarGroupContent({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return (
    <div className={cx('dz-sidebar__group-content', className)} {...props} />
  )
}

export function SidebarMenu({
  className,
  ...props
}: ComponentPropsWithoutRef<'ul'>) {
  return <ul className={cx('dz-sidebar__menu', className)} {...props} />
}

export function SidebarMenuItem({
  className,
  ...props
}: ComponentPropsWithoutRef<'li'>) {
  return <li className={cx('dz-sidebar__menu-item', className)} {...props} />
}

export type SidebarMenuButtonSize = 'small' | 'medium' | 'large'

/**
 * The `control.color.*` recipes, named as Button and Badge name them — but a
 * row spends them differently, because a row idles transparent rather than
 * as a chip. What the variant decides is how the row marks itself as the
 * **current page**, and its hover follows from that: a row that selects with
 * a fill can afford an accent hover, since there is a long way between an 8%
 * wash and a solid one; a row that selects with a wash cannot, and hovers
 * neutral instead.
 *
 * `filled` is the destination row a window navigates by. `tinted` marks the
 * page more quietly, for a column whose selection is one of many. `outlined`
 * rings it instead of washing it, where the rows carry colour of their own.
 * `recessed` selects in grey, for a list whose selection is a cursor rather
 * than a destination — a file, a branch, a suite. `normal` is the raised row
 * a workspace switcher or an account takes, and is never a page. `plain` is
 * printed in the ramp rather than filled with it, for the row that adds
 * rather than selects — "New", "Add a provider".
 */
export type SidebarMenuButtonVariant =
  | 'filled'
  | 'tinted'
  | 'outlined'
  | 'recessed'
  | 'normal'
  | 'plain'

export interface SidebarMenuButtonProps extends ComponentPropsWithoutRef<'button'> {
  /** Marks this row as the page or pane currently shown. */
  active?: boolean
  /** Leading glyph. It follows the row's colour, so it tints with selection. */
  icon?: ReactNode
  /** `medium` in a sidebar, `small` in a narrower second column, `large` for a two-line row. */
  size?: SidebarMenuButtonSize
  variant?: SidebarMenuButtonVariant
  tint?: Tint
  /**
   * What the row says once the sidebar has collapsed to its icons. Shown
   * only then: an expanded row has its label.
   */
  tooltip?: ReactNode
  /** Render as another element — an `<a>` where the row is a link. */
  render?: ReactElement
}

/** One row of the column. */
export function SidebarMenuButton({
  active = false,
  icon,
  size = 'medium',
  variant = 'filled',
  tint = 'primary',
  tooltip,
  render,
  className,
  children,
  type = 'button',
  ...props
}: SidebarMenuButtonProps) {
  // Read directly rather than through `useSidebar`, so a row can be drawn on
  // its own — in a gallery, or a list that is not a sidebar.
  const sidebar = useContext(SidebarContext)

  const button = useRender({
    render,
    defaultTagName: 'button',
    props: {
      type: render ? undefined : type,
      'aria-current': active ? 'page' : undefined,
      'data-size': size,
      'data-tint': tint,
      'data-variant': variant,
      className: cx('dz-sidebar__menu-button', className),
      children: (
        <>
          {icon ? (
            <span aria-hidden className="dz-sidebar__menu-icon">
              {icon}
            </span>
          ) : null}
          {children}
        </>
      ),
      ...props,
    },
  })

  if (tooltip === undefined) return button

  return (
    <Tooltip
      label={tooltip}
      side="right"
      disabled={sidebar?.state !== 'collapsed'}
    >
      {button as ReactElement<Record<string, unknown>>}
    </Tooltip>
  )
}

export interface SidebarMenuActionProps extends IconButtonProps {
  /** Keep it out of sight until the row is hovered or focused. */
  showOnHover?: boolean
}

/** The small button on a row's trailing edge — "more", "close". */
export function SidebarMenuAction({
  showOnHover = false,
  className,
  ...props
}: SidebarMenuActionProps) {
  return (
    <IconButton
      data-show-on-hover={showOnHover || undefined}
      className={cx('dz-sidebar__menu-action', className)}
      {...props}
    />
  )
}

export interface SidebarMenuBadgeProps extends ComponentPropsWithoutRef<'span'> {
  /**
   * Which ramp the selected row's ink comes off. It is stated here as well
   * as on the row because the count is the row's *sibling* — the ramp a
   * `data-tint` binds reaches a row's children, not the box beside it — so
   * a row given a tint of its own passes the same one here.
   */
  tint?: Tint
}

/** A count on a row's trailing edge, in the row's own colour. */
export function SidebarMenuBadge({
  tint = 'primary',
  className,
  ...props
}: SidebarMenuBadgeProps) {
  return (
    <span
      data-tint={tint}
      className={cx('dz-sidebar__menu-badge', className)}
      {...props}
    />
  )
}

/** The rows under a row, set in by a hairline. */
export function SidebarMenuSub({
  className,
  ...props
}: ComponentPropsWithoutRef<'ul'>) {
  return <ul className={cx('dz-sidebar__menu-sub', className)} {...props} />
}

export function SidebarMenuSubItem({
  className,
  ...props
}: ComponentPropsWithoutRef<'li'>) {
  return (
    <li className={cx('dz-sidebar__menu-sub-item', className)} {...props} />
  )
}

export interface SidebarMenuSubButtonProps extends Omit<
  SidebarMenuButtonProps,
  'size' | 'tooltip'
> {
  size?: 'small' | 'medium'
}

/**
 * A sub-row is the row at the small size and nothing else — shadcn renders
 * an `<a>` here, but a desktop kit has no router, and a row does something
 * rather than going somewhere. Pass `render` for the ones that are links.
 */
export function SidebarMenuSubButton({
  size = 'small',
  className,
  ...props
}: SidebarMenuSubButtonProps) {
  return (
    <SidebarMenuButton
      size={size}
      className={cx('dz-sidebar__menu-sub-button', className)}
      {...props}
    />
  )
}
