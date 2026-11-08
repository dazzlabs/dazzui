//! The sidebar, after shadcn's: one column and the parts it is built from.
//!
//! [`SidebarProvider`] is the window row: it holds whether the collapsible
//! column is open and hands that to every part inside it, the way React's
//! context does — through [`provide`], so a [`SidebarTrigger`] anywhere in
//! the row toggles it and a [`Sidebar`] reads it. The column itself provides
//! whether it has collapsed to its icons, which is what the rows, labels and
//! counts inside it change for.

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, Hsla, InteractiveElement, IntoElement, KeyDownEvent,
    MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window, div, prelude::FluentBuilder, px,
};

use crate::components::collapsible::{OnOpenChange, kept_focus, part_id};
use crate::components::icon::{Icon, icon_slot};
use crate::foundation::{
    Size, StyledExt, Tint, Variant, focus_visible, pointer_down, provide, ring, scoped,
};
use crate::glyphs;
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

impl SidebarSide {
    pub fn parse(name: &str) -> Option<SidebarSide> {
        match name {
            "left" => Some(SidebarSide::Left),
            "right" => Some(SidebarSide::Right),
            _ => None,
        }
    }
}

/// `sidebar` runs the window's height with a hairline on its inner edge;
/// `floating` is a card set in from the edge; `inset` is the ground the
/// content sits raised on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl SidebarVariant {
    pub fn parse(name: &str) -> Option<SidebarVariant> {
        match name {
            "sidebar" => Some(SidebarVariant::Sidebar),
            "floating" => Some(SidebarVariant::Floating),
            "inset" => Some(SidebarVariant::Inset),
            _ => None,
        }
    }
}

/// How the column leaves: `offcanvas` goes entirely, `icon` narrows to a
/// column of glyphs, `none` stays.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

impl SidebarCollapsible {
    pub fn parse(name: &str) -> Option<SidebarCollapsible> {
        match name {
            "offcanvas" => Some(SidebarCollapsible::Offcanvas),
            "icon" => Some(SidebarCollapsible::Icon),
            "none" => Some(SidebarCollapsible::None),
            _ => None,
        }
    }
}

/// What a provider hands the parts inside it — React's `useSidebar()`.
#[derive(Clone)]
pub struct SidebarContext {
    pub open: bool,
    set_open: OnOpenChange,
    /// Whether any preceding inset sidebar is collapsed. The provider also
    /// reads this after layout to paint the ground beneath all its children.
    inset: Rc<Cell<Option<bool>>>,
}

impl SidebarContext {
    /// The context in force where this is read, if inside a provider.
    pub fn current(cx: &App) -> Option<SidebarContext> {
        scoped::<SidebarContext>(cx)
    }

    pub fn set_open(&self, open: bool, window: &mut Window, cx: &mut App) {
        (self.set_open)(open, window, cx);
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut App) {
        (self.set_open)(!self.open, window, cx);
    }
}

/// Whether the column a part is drawn in has collapsed to its icons.
#[derive(Clone, Copy)]
struct IconColumn(bool);

#[derive(Clone, Copy)]
struct FixedColumn(bool);

#[derive(Clone, Copy)]
struct ContentEdge {
    first: bool,
    last: bool,
}

fn icon_column(cx: &App) -> bool {
    scoped::<IconColumn>(cx).is_some_and(|column| column.0)
}

// The window ------------------------------------------------------------------

/// The window row. It holds whether the sidebar is open, binds ⌘B to it
/// while focus is inside, and is the flex row every column and the inset sit
/// in.
#[derive(IntoElement)]
pub struct SidebarProvider {
    id: ElementId,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<OnOpenChange>,
    children: Vec<AnyElement>,
}

impl SidebarProvider {
    pub fn new(id: impl Into<ElementId>) -> Self {
        SidebarProvider {
            id: id.into(),
            open: None,
            default_open: true,
            on_open_change: None,
            children: Vec::new(),
        }
    }

    /// Controlled: whether the collapsible sidebar inside is open.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for SidebarProvider {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarProvider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default_open = self.default_open;
        let own = window.use_keyed_state(part_id(&self.id, "open"), cx, move |_, _| default_open);
        let open = self.open.unwrap_or(*own.read(cx));
        let kept = self.open.is_none();
        let on_change = self.on_open_change.clone();
        let set_open: OnOpenChange = Rc::new(move |next, window, cx| {
            if kept {
                own.update(cx, |open, cx| {
                    *open = next;
                    cx.notify();
                });
            }
            if let Some(on_change) = &on_change {
                on_change(next, window, cx);
            }
            window.refresh();
        });
        let context = SidebarContext {
            open,
            set_open: set_open.clone(),
            inset: Rc::new(Cell::new(None)),
        };
        let inset = context.inset.clone();
        let ground = cx.theme().color_surface_column.hsla();

        provide(
            context,
            div()
                .id(self.id)
                .flex()
                .flex_1()
                .w_full()
                .min_w_0()
                .min_h_0()
                .map_paint_style(move |style, _, _| {
                    if inset.get().is_some() {
                        style.background = Some(ground.into());
                    }
                })
                .on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let stroke = &event.keystroke;
                    if stroke.key == "b" && (stroke.modifiers.platform || stroke.modifiers.control)
                    {
                        set_open(!open, window, cx);
                        cx.stop_propagation();
                    }
                })
                .children(self.children),
        )
    }
}

// The column ------------------------------------------------------------------

#[derive(IntoElement)]
pub struct Sidebar {
    side: SidebarSide,
    variant: SidebarVariant,
    collapsible: SidebarCollapsible,
    width: Option<f32>,
    children: Vec<AnyElement>,
}

impl Sidebar {
    pub fn new() -> Self {
        Sidebar {
            side: SidebarSide::Left,
            variant: SidebarVariant::Sidebar,
            collapsible: SidebarCollapsible::Offcanvas,
            width: None,
            children: Vec::new(),
        }
    }

    /// The open column's width, over the frame's — React's
    /// `--sidebar-width`, which an app sets to let a person resize it.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn side(mut self, side: SidebarSide) -> Self {
        self.side = side;
        self
    }

    pub fn variant(mut self, variant: SidebarVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn collapsible(mut self, collapsible: SidebarCollapsible) -> Self {
        self.collapsible = collapsible;
        self
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Sidebar::new()
    }
}

impl ParentElement for Sidebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let context = SidebarContext::current(cx);
        let open = context.as_ref().is_none_or(|context| context.open);
        let left = self.side == SidebarSide::Left;
        let rule = px(t.stroke_hairline);
        let border = t.color_border.hsla();
        let collapsed = !open && self.collapsible != SidebarCollapsible::None;
        if self.variant == SidebarVariant::Inset {
            if let Some(context) = &context {
                context
                    .inset
                    .set(Some(context.inset.get().unwrap_or(false) || collapsed));
            }
        }

        // The column that cannot collapse is the one nearer the content, and
        // takes the frame's width for where it stands.
        if self.collapsible == SidebarCollapsible::None {
            let (width, surface) = if left {
                (t.frame_rail_width, t.color_surface.hsla())
            } else {
                (t.frame_aside_width, t.color_surface_column.hsla())
            };
            let width = self.width.unwrap_or(width);
            return provide(
                IconColumn(false),
                div()
                    .flex()
                    .flex_col()
                    .flex_none()
                    .h_full()
                    .w(px(width))
                    .bg(surface)
                    .border_color(border)
                    .map(|this| {
                        if left {
                            this.border_r(rule)
                        } else {
                            this.border_l(rule)
                        }
                    })
                    .children(
                        self.children
                            .into_iter()
                            .map(|child| provide(FixedColumn(true), child)),
                    ),
            )
            .into_any_element();
        }

        if collapsed && self.collapsible == SidebarCollapsible::Offcanvas {
            return div().flex_none().w(px(0.)).into_any_element();
        }
        let icon = collapsed && self.collapsible == SidebarCollapsible::Icon;
        let full = self.width.unwrap_or(if left {
            t.frame_sidebar_width
        } else {
            t.frame_aside_width
        });
        let inset_by = !matches!(self.variant, SidebarVariant::Sidebar);
        let width = match (icon, inset_by) {
            (false, _) => full,
            (true, false) => t.frame_sidebar_icon_width,
            (true, true) => t.frame_sidebar_icon_width + t.spacing_4,
        };

        let inner = div()
            .flex()
            .flex_col()
            .size_full()
            .bg(t.color_surface_column.hsla())
            .when(icon, |this| this.overflow_hidden())
            .when(self.variant == SidebarVariant::Floating, |this| {
                this.border(rule)
                    .border_color(border)
                    .rounded(px(t.radius_large))
                    .shadow(shadows(t.shadow_sm))
            })
            .children(
                self.children
                    .into_iter()
                    .map(|child| provide(FixedColumn(false), child)),
            );

        let container_width = width
            + if icon && inset_by {
                2. * t.stroke_hairline
            } else {
                0.
            };
        let container = div()
            .flex()
            .flex_none()
            .h_full()
            .w(px(container_width))
            .when(inset_by, |this| this.p(px(t.spacing_2)))
            .when(self.variant == SidebarVariant::Sidebar, |this| {
                this.border_color(border).map(|this| {
                    if left {
                        this.border_r(rule)
                    } else {
                        this.border_l(rule)
                    }
                })
            })
            .child(inner);
        provide(
            IconColumn(icon),
            div()
                .flex()
                .flex_none()
                .h_full()
                .w(px(width))
                .when(!left, |this| this.justify_end())
                .child(container),
        )
        .into_any_element()
    }
}

/// The content beside the sidebar. Beside an `inset` column it is the raised
/// sheet, set in by the column's own step.
#[derive(IntoElement, Default)]
pub struct SidebarInset {
    children: Vec<AnyElement>,
}

impl SidebarInset {
    pub fn new() -> Self {
        SidebarInset::default()
    }
}

impl ParentElement for SidebarInset {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarInset {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let inset = SidebarContext::current(cx).and_then(|context| context.inset.get());
        let main = div()
            .relative()
            .flex()
            .flex_1()
            .flex_col()
            .min_w_0()
            .min_h_0();
        match inset {
            Some(collapsed) => {
                let step = t.spacing_2;
                let lead = if collapsed { step } else { 0.0 };
                main.m(px(step))
                    .ml(px(lead))
                    .rounded(px(t.radius_large))
                    .shadow(shadows(t.shadow_sm))
                    .bg(t.color_surface.hsla())
                    .children(self.children)
            }
            _ => main.bg(t.color_surface.hsla()).children(self.children),
        }
    }
}

// Header, content, footer -------------------------------------------------------

fn edge_strip(cx: &App) -> gpui::Div {
    let t = cx.theme();
    div()
        .flex()
        .flex_col()
        .flex_none()
        .gap(px(t.spacing_2))
        .py(px(t.spacing_2))
        .px(px(t.spacing_2_5))
}

/// The strip above the rows, held at the titlebar's height so it lines up
/// with the toolbar in the pane beside it.
#[derive(IntoElement, Default)]
pub struct SidebarHeader {
    children: Vec<AnyElement>,
}

impl SidebarHeader {
    pub fn new() -> Self {
        SidebarHeader::default()
    }
}

impl ParentElement for SidebarHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        edge_strip(cx)
            .justify_center()
            .min_h(px(t.frame_titlebar_size))
            .children(self.children)
    }
}

/// Pinned to the foot, whatever the content above it does.
#[derive(IntoElement, Default)]
pub struct SidebarFooter {
    children: Vec<AnyElement>,
}

impl SidebarFooter {
    pub fn new() -> Self {
        SidebarFooter::default()
    }
}

impl ParentElement for SidebarFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarFooter {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        edge_strip(cx).children(self.children)
    }
}

/// The part that scrolls, so the header and the footer stay put.
#[derive(IntoElement, Default)]
pub struct SidebarContent {
    children: Vec<AnyElement>,
    scroll_handle: Option<gpui::ScrollHandle>,
}

impl SidebarContent {
    pub fn new() -> Self {
        SidebarContent::default()
    }

    /// Track or control this column's own scroll position.
    pub fn scroll_handle(mut self, handle: gpui::ScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
        self
    }
}

impl ParentElement for SidebarContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarContent {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let icon = icon_column(cx);
        let fixed = scoped::<FixedColumn>(cx).is_some_and(|column| column.0);
        let count = self.children.len();
        div()
            .id("dz-sidebar-content")
            .when_some(self.scroll_handle, |this, handle| {
                this.track_scroll(&handle)
            })
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .py(px(if fixed { t.spacing_2_5 } else { t.spacing_2 }))
            .map(|this| {
                if icon {
                    this.overflow_hidden()
                } else {
                    this.overflow_y_scroll()
                }
            })
            .children(self.children.into_iter().enumerate().map(|(index, child)| {
                provide(
                    ContentEdge {
                        first: fixed && index == 0,
                        last: fixed && index + 1 == count,
                    },
                    child,
                )
            }))
    }
}

/// A hairline between runs, set in to the rows' edge.
#[derive(IntoElement, Default)]
pub struct SidebarSeparator;

impl SidebarSeparator {
    pub fn new() -> Self {
        SidebarSeparator
    }
}

impl RenderOnce for SidebarSeparator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        div()
            .flex_none()
            .h(px(t.stroke_hairline))
            .mx(px(t.spacing_2_5))
            .bg(t.color_border.hsla())
    }
}

// Groups ----------------------------------------------------------------------------

/// A run of rows, and the label that names it.
#[derive(IntoElement, Default)]
pub struct SidebarGroup {
    children: Vec<AnyElement>,
}

impl SidebarGroup {
    pub fn new() -> Self {
        SidebarGroup::default()
    }
}

impl ParentElement for SidebarGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let edge = scoped::<ContentEdge>(cx);
        div()
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .py(px(t.spacing_1_5))
            .when(edge.is_some_and(|edge| edge.first), |this| this.pt_0())
            .when(edge.is_some_and(|edge| edge.last), |this| this.pb_0())
            .px(px(t.spacing_2_5))
            .children(self.children.into_iter().map(|child| {
                provide(
                    ContentEdge {
                        first: false,
                        last: false,
                    },
                    child,
                )
            }))
    }
}

/// A group's heading: a faint `SectionLabel` on the rows' own inset. Gone
/// once only the glyphs show.
#[derive(IntoElement, Default)]
pub struct SidebarGroupLabel {
    children: Vec<AnyElement>,
}

impl SidebarGroupLabel {
    pub fn new() -> Self {
        SidebarGroupLabel::default()
    }
}

impl ParentElement for SidebarGroupLabel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarGroupLabel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        if icon_column(cx) {
            return div().into_any_element();
        }
        div()
            .flex()
            .flex_none()
            .items_center()
            .py(px(t.spacing_1))
            .px(px(t.spacing_2_5))
            .type_style(t.label_small)
            .text_color(t.color_content_faint.hsla())
            .children(self.children)
            .into_any_element()
    }
}

#[derive(IntoElement, Default)]
pub struct SidebarGroupContent {
    children: Vec<AnyElement>,
}

impl SidebarGroupContent {
    pub fn new() -> Self {
        SidebarGroupContent::default()
    }
}

impl ParentElement for SidebarGroupContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarGroupContent {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().w_full().children(self.children)
    }
}

// Menus -----------------------------------------------------------------------------

#[derive(IntoElement, Default)]
pub struct SidebarMenu {
    children: Vec<AnyElement>,
}

impl SidebarMenu {
    pub fn new() -> Self {
        SidebarMenu::default()
    }
}

impl ParentElement for SidebarMenu {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarMenu {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        div()
            .flex()
            .flex_col()
            .gap(px(t.frame_nav_gap))
            .w_full()
            .min_w_0()
            .children(self.children)
    }
}

/// What a row tells the count beside it: which recipe it marks itself with,
/// and whether it is the current page. The count is the row's sibling, so
/// it is written into the item as the row lays out and read as the count
/// lays out after it.
#[derive(Clone, Copy)]
struct RowMark {
    variant: Variant,
    tint: Tint,
    active: bool,
}

#[derive(Clone)]
struct ItemSlot {
    mark: Rc<Cell<Option<RowMark>>>,
    has_badge: bool,
}

const ITEM_GROUP: &str = "dz-sidebar-menu-item";

/// One entry of a menu: a row, and anything on its trailing edge.
#[derive(IntoElement, Default)]
pub struct SidebarMenuItem {
    badge: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl SidebarMenuItem {
    pub fn new() -> Self {
        SidebarMenuItem::default()
    }

    /// A count on the row's trailing edge. The row keeps its label clear of
    /// it, and the count takes the row's ink when the row is selected. The
    /// item places it; a [`SidebarMenuBadge`] draws only the count's box.
    pub fn badge(mut self, badge: impl IntoElement) -> Self {
        self.badge = Some(badge.into_any_element());
        self
    }
}

impl ParentElement for SidebarMenuItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarMenuItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let slot = ItemSlot {
            mark: Rc::new(Cell::new(None)),
            has_badge: self.badge.is_some(),
        };
        provide(
            slot,
            div()
                .relative()
                .group(ITEM_GROUP)
                .children(self.children)
                .children(self.badge.map(|badge| {
                    div()
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .right(px(t.spacing_1))
                        .flex()
                        .items_center()
                        .child(badge)
                })),
        )
    }
}

/// `medium` in a sidebar, `small` in a narrower second column, `large` for
/// a two-line row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarMenuButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SidebarMenuButtonSize {
    pub fn parse(name: &str) -> Option<SidebarMenuButtonSize> {
        match name {
            "small" => Some(SidebarMenuButtonSize::Small),
            "medium" => Some(SidebarMenuButtonSize::Medium),
            "large" => Some(SidebarMenuButtonSize::Large),
            _ => None,
        }
    }
}

type OnClick = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// One row of the column. The variant decides how the row marks itself as
/// the current page — `filled` by default — and its hover follows from that.
#[derive(IntoElement)]
pub struct SidebarMenuButton {
    id: ElementId,
    active: bool,
    icon: Option<AnyElement>,
    size: SidebarMenuButtonSize,
    variant: Variant,
    tint: Tint,
    disabled: bool,
    on_click: Option<OnClick>,
    children: Vec<AnyElement>,
}

impl SidebarMenuButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        SidebarMenuButton {
            id: id.into(),
            active: false,
            icon: None,
            size: SidebarMenuButtonSize::Medium,
            variant: Variant::Filled,
            tint: Tint::Primary,
            disabled: false,
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Marks this row as the page or pane currently shown.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Leading glyph. It follows the row's colour.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn size(mut self, size: SidebarMenuButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// What the row says once the column has collapsed to its icons. The
    /// tooltip is not drawn yet: this is accepted so callers can write it
    /// now, and will be wired once the kit's `Tooltip` lands.
    pub fn tooltip(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for SidebarMenuButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

/// A row's resting, hovered and selected colours: (surface, content, ring).
struct RowColors {
    rest: (Option<Hsla>, Hsla, Option<Hsla>),
    hover: Option<(Option<Hsla>, Hsla, Option<Hsla>)>,
}

fn row_colors(variant: Variant, tint: Tint, active: bool, cx: &App) -> RowColors {
    let t = cx.theme();
    let ramp = tint.ramp(t);
    let nav = t.color_content_nav.hsla();
    let ink = t.color_content.hsla();
    let grey_hover = Some((
        Some(t.control_color_recessed_surface_normal.resolve(ramp)),
        ink,
        None,
    ));
    match variant {
        Variant::Filled if active => RowColors {
            rest: (
                Some(t.control_color_filled_surface_normal.resolve(ramp)),
                t.control_color_filled_content_normal.resolve(ramp),
                None,
            ),
            hover: None,
        },
        Variant::Filled => RowColors {
            rest: (None, nav, None),
            hover: Some((
                Some(t.control_color_plain_surface_hovered.resolve(ramp)),
                ink,
                None,
            )),
        },
        Variant::Tinted if active => RowColors {
            rest: (
                Some(t.control_color_tinted_surface_normal.resolve(ramp)),
                t.control_color_tinted_content_normal.resolve(ramp),
                None,
            ),
            hover: None,
        },
        Variant::Outlined if active => RowColors {
            rest: (
                None,
                t.control_color_outlined_content_normal.resolve(ramp),
                Some(t.control_color_outlined_border_normal.resolve(ramp)),
            ),
            hover: None,
        },
        Variant::Recessed if active => RowColors {
            rest: (
                Some(t.control_color_recessed_surface_hovered.resolve(ramp)),
                t.control_color_recessed_content_normal.resolve(ramp),
                None,
            ),
            hover: None,
        },
        Variant::Tinted | Variant::Outlined | Variant::Recessed => RowColors {
            rest: (None, nav, None),
            hover: grey_hover,
        },
        Variant::Normal if active => RowColors {
            rest: (
                Some(t.control_color_normal_surface_hovered.resolve(ramp)),
                t.control_color_normal_content_normal.resolve(ramp),
                Some(t.control_color_normal_border_normal.resolve(ramp)),
            ),
            hover: None,
        },
        Variant::Normal => {
            let surface = Some(t.control_color_normal_surface_normal.resolve(ramp));
            RowColors {
                rest: (surface, nav, Some(t.color_border.hsla())),
                hover: Some((
                    surface,
                    nav,
                    Some(t.control_color_outlined_border_hovered.resolve(ramp)),
                )),
            }
        }
        Variant::Plain => RowColors {
            rest: (
                None,
                t.control_color_plain_content_normal.resolve(ramp),
                None,
            ),
            hover: Some((
                Some(t.control_color_plain_surface_hovered.resolve(ramp)),
                t.control_color_plain_content_hovered.resolve(ramp),
                None,
            )),
        },
    }
}

fn hairline_ring(color: Hsla, width: f32) -> gpui::BoxShadow {
    gpui::BoxShadow {
        color,
        offset: gpui::point(px(0.), px(0.)),
        blur_radius: px(0.),
        spread_radius: px(width),
    }
}

impl RenderOnce for SidebarMenuButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let icon_mode = icon_column(cx);
        let slot = scoped::<ItemSlot>(cx);
        if let Some(slot) = &slot {
            slot.mark.set(Some(RowMark {
                variant: self.variant,
                tint: self.tint,
                active: self.active,
            }));
        }
        let has_badge = slot.is_some_and(|slot| slot.has_badge);
        let focus = kept_focus(part_id(&self.id, "focus"), window, cx);
        let visible = !self.disabled && focus_visible(&focus, window, cx);
        let hairline = t.stroke_hairline;

        let mut row = div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .w_full()
            .gap(px(t.spacing_1_5))
            .rounded(px(t.radius_small))
            .type_style(t.label_quiet)
            .line_height(px(t.label_quiet.size));

        row = match self.size {
            SidebarMenuButtonSize::Medium => row.py(px(t.spacing_2)).px(px(t.spacing_2_5)),
            SidebarMenuButtonSize::Small => row.py(px(t.menu_item_padding)).px(px(t.spacing_2_5)),
            SidebarMenuButtonSize::Large => row
                .min_h(px(t.spacing_12))
                .py(px(t.spacing_2))
                .px(px(t.spacing_2_5))
                .rounded(px(t.radius_large)),
        };
        if has_badge {
            row = row.pr(px(t.spacing_8));
        }
        if icon_mode {
            // Every row the same square, the glyph centred and the label cut
            // off past it.
            row = row
                .w(px(t.control_medium_size))
                .h(px(t.control_medium_size))
                .py(px(0.))
                .px(px((t.control_medium_size - t.icon_medium) / 2.0))
                .overflow_hidden();
        }
        if self.variant == Variant::Plain {
            row = row.font_weight(t.label_medium.font_weight());
        }

        let mut shadow = Vec::new();
        if self.disabled {
            row = row
                .text_color(
                    t.control_color_normal_content_disabled
                        .resolve(self.tint.ramp(t)),
                )
                .cursor_not_allowed();
        } else {
            let colors = row_colors(self.variant, self.tint, self.active, cx);
            let (surface, content, ring_color) = colors.rest;
            row = row
                .text_color(content)
                .when_some(surface, |this, surface| this.bg(surface))
                .cursor_pointer();
            if let Some(ring_color) = ring_color {
                shadow.push(hairline_ring(ring_color, hairline));
            }
            if let Some((surface, content, ring_color)) = colors.hover {
                let mut hover_shadow = Vec::new();
                if let Some(ring_color) = ring_color {
                    hover_shadow.push(hairline_ring(ring_color, hairline));
                }
                if visible {
                    hover_shadow.push(ring(t));
                }
                row = row.hover(move |style| {
                    let style = style.text_color(content);
                    let style = match surface {
                        Some(surface) => style.bg(surface),
                        None => style,
                    };
                    if hover_shadow.is_empty() {
                        style
                    } else {
                        style.shadow(hover_shadow.clone())
                    }
                });
            }
            row = row
                .track_focus(&focus.clone().tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx));
            if let Some(on_click) = self.on_click {
                row = row.on_click(move |event, window, cx| on_click(event, window, cx));
            }
        }
        if visible {
            shadow.push(ring(t));
        }
        if !shadow.is_empty() {
            row = row.shadow(shadow);
        }

        let label = div()
            .min_w_0()
            .overflow_hidden()
            .whitespace_nowrap()
            .text_ellipsis()
            .children(self.children);

        row.when_some(self.icon, |this, icon| {
            // Boxed to the row's line so the taller glyph overflows rather
            // than stretching the row.
            this.child(icon_slot(
                t.icon_medium,
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .w(px(t.icon_medium))
                    .h(px(t.label_quiet.size))
                    .child(icon),
            ))
        })
        .child(crate::foundation::text_child(label))
    }
}

/// A count on a row's trailing edge, in the row's own colour.
#[derive(IntoElement, Default)]
pub struct SidebarMenuBadge {
    children: Vec<AnyElement>,
}

impl SidebarMenuBadge {
    pub fn new() -> Self {
        SidebarMenuBadge::default()
    }
}

impl ParentElement for SidebarMenuBadge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SidebarMenuBadge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        if icon_column(cx) {
            return div().into_any_element();
        }
        let mark = scoped::<ItemSlot>(cx).and_then(|slot| slot.mark.get());
        let selected_ink = mark.filter(|mark| mark.active).map(|mark| {
            let ramp = mark.tint.ramp(t);
            match mark.variant {
                Variant::Filled => t.control_color_filled_content_normal.resolve(ramp),
                Variant::Tinted => t.control_color_tinted_content_normal.resolve(ramp),
                Variant::Outlined => t.control_color_outlined_content_normal.resolve(ramp),
                _ => t.color_content.hsla(),
            }
        });
        let ink = t.color_content.hsla();
        div()
            .flex()
            .items_center()
            .justify_center()
            .h(px(t.spacing_5))
            .min_w(px(t.spacing_5))
            .px(px(t.spacing_1))
            .rounded(px(t.radius_small))
            .font_family(t.caption_small.face.family())
            .text_size(px(t.caption_small.size))
            .font_weight(t.label_medium.font_weight())
            .tabular_nums()
            .map(|this| match selected_ink {
                Some(color) => this.text_color(color),
                None => this
                    .text_color(t.color_content_nav.hsla())
                    .group_hover(ITEM_GROUP, move |style| style.text_color(ink)),
            })
            .children(self.children)
            .into_any_element()
    }
}

/// The toolbar button that opens and closes the sidebar: a tiny plain
/// `IconButton`, a panel glyph by default.
#[derive(IntoElement)]
pub struct SidebarTrigger {
    id: ElementId,
    icon: Option<AnyElement>,
    on_click: Option<OnClick>,
}

impl SidebarTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        SidebarTrigger {
            id: id.into(),
            icon: None,
            on_click: None,
        }
    }

    /// The glyph; a panel by default.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    /// Heard before the sidebar toggles.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for SidebarTrigger {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let profile = Size::Tiny.profile(t);
        let context = SidebarContext::current(cx);
        let focus = kept_focus(part_id(&self.id, "focus"), window, cx);
        let visible = focus_visible(&focus, window, cx);
        let wash = t.color_surface_subtle.hsla();
        let ink = t.color_content.hsla();
        let on_click = self.on_click;
        icon_slot(
            profile.icon_size,
            div()
                .id(self.id)
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .size(px(profile.size))
                .rounded(px(profile.radius))
                .border(px(t.spacing_px))
                .border_color(gpui::transparent_black())
                .text_color(t.color_content_subtle.hsla())
                .cursor_pointer()
                .hover(move |style| style.bg(wash).text_color(ink))
                .track_focus(&focus.clone().tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .when(visible, |this| this.shadow(vec![ring(t)]))
                .on_click(move |event, window, cx| {
                    if let Some(on_click) = &on_click {
                        on_click(event, window, cx);
                    }
                    if let Some(context) = &context {
                        context.toggle(window, cx);
                    }
                })
                .child(self.icon.unwrap_or_else(|| {
                    Icon::new(glyphs::PANEL_LEFT_20_REGULAR).into_any_element()
                })),
        )
    }
}
