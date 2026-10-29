//! The floating sheet, and the rows inside it — React's `popup.css`.
//!
//! A Menu's panel, a Select's list and a Popover's panel are one drawing
//! wearing different data, so the parts they share live here: where a sheet
//! goes ([`positioned`], against an [`Anchor`] a trigger was [`measured`]
//! into), what it looks like ([`sheet`]), how it goes away ([`dismissable`]:
//! an outside press or Escape), the row a list draws ([`PopupRow`]), and the
//! state a popup keeps ([`PopupState`]: open, the highlighted row, the arrow
//! keys, type-ahead and the hover delays).
//!
//! Base UI does the positioning on the web; the positioner there is padded
//! by `--spacing-1` on the side facing the trigger, which is the gap here.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    AlignItems, AnyElement, App, Bounds, Context, Deferred, Display, Div, Element, ElementId,
    Entity, FocusHandle, GlobalElementId, InspectorElementId, InteractiveElement, IntoElement,
    KeyDownEvent, LayoutId, ParentElement, Pixels, Point, Position, RenderOnce, SharedString,
    Stateful, Style, Styled, Task, Window, deferred, div, point, prelude::*, px, size,
};

use crate::StyledExt;
use crate::components::icon::{Icon, icon_slot};
use crate::foundation::scope::Provide;
use crate::foundation::{Tint, provide, scoped};
use crate::generated::ThemeTokens;
use crate::glyphs;
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

/// What a trigger is handed to open or close its popup, and what a row does
/// when chosen.
pub type Press = Rc<dyn Fn(&mut Window, &mut App)>;

/// A trigger, built once the popup knows what pressing it does and whether
/// it is open — React's `trigger` element, or the function of the open state
/// a `Menu` also takes.
pub type TriggerFn = Box<dyn FnOnce(Press, bool, &mut Window, &mut App) -> AnyElement>;

/// What reports an open state changing — React's `onOpenChange`.
pub type OnOpenChange = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// Base UI's `collisionPadding`: how close a sheet may come to the window's
/// edge before it is pushed back in.
const COLLISION_PADDING: f32 = 5.0;

/// The tallest a sheet may be and still fit the window with the collision
/// padding kept: what a list long enough to scroll is capped at.
pub fn max_sheet_height(window: &Window) -> f32 {
    f32::from(window.viewport_size().height) - 2. * COLLISION_PADDING
}

/// The sheets open inside a sheet, as they were last drawn — a select's
/// list inside a picker's panel. An outside press is judged against the
/// sheet's own box, so a press in a list that overflows the sheet would read
/// as outside it and close the panel under the reader's choice; a sheet
/// registers its box here, up through every sheet it is nested in, and each
/// counts a press inside it as its own.
#[derive(Clone, Default)]
pub struct Nest {
    bounds: Rc<RefCell<Vec<Bounds<Pixels>>>>,
    parent: Option<Rc<Nest>>,
}

impl Nest {
    /// A nest inside the one in force, if a sheet is being built inside one.
    fn inside(cx: &App) -> Nest {
        Nest {
            bounds: Rc::default(),
            parent: scoped::<Nest>(cx).map(Rc::new),
        }
    }

    fn register(&self, bounds: Bounds<Pixels>) {
        self.bounds.borrow_mut().push(bounds);
        if let Some(parent) = &self.parent {
            parent.register(bounds);
        }
    }

    fn contains(&self, position: Point<Pixels>) -> bool {
        self.bounds.borrow().iter().any(|b| b.contains(&position))
    }
}

/// Which side of its trigger a sheet opens on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Side {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

impl Side {
    pub fn parse(name: &str) -> Option<Side> {
        Some(match name {
            "top" => Side::Top,
            "bottom" => Side::Bottom,
            "left" | "inline-start" => Side::Left,
            "right" | "inline-end" => Side::Right,
            _ => return None,
        })
    }

    fn opposite(self) -> Side {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

/// Where along its side a sheet lines up with the trigger.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Align {
    Start,
    #[default]
    Center,
    End,
}

impl Align {
    pub fn parse(name: &str) -> Option<Align> {
        Some(match name {
            "start" => Align::Start,
            "center" => Align::Center,
            "end" => Align::End,
            _ => return None,
        })
    }
}

/// Where a sheet goes against its anchor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Placement {
    /// Beside the anchor, `gap` away from it, flipping to the other side
    /// when this one has no room.
    Side { side: Side, align: Align, gap: f32 },
    /// Over the anchor, its origin at the anchor's plus this offset — a
    /// Select's list, the chosen row laid over the value that names it.
    Over(Point<Pixels>),
}

impl Placement {
    /// A side, with the gap `popup.css` pads the positioner by.
    pub fn side(side: Side, align: Align, t: &ThemeTokens) -> Placement {
        Placement::Side {
            side,
            align,
            gap: t.spacing_1,
        }
    }
}

/// Where something was last drawn, in window coordinates: the box a sheet is
/// placed against, and the one a press may land in without dismissing it.
#[derive(Clone, Default)]
pub struct Anchor(Rc<Cell<Option<Bounds<Pixels>>>>);

impl Anchor {
    /// An anchor fixed at a box — a point, for a menu opened at the pointer.
    pub fn at(bounds: Bounds<Pixels>) -> Anchor {
        Anchor(Rc::new(Cell::new(Some(bounds))))
    }

    pub fn bounds(&self) -> Option<Bounds<Pixels>> {
        self.0.get()
    }

    pub fn set(&self, bounds: Bounds<Pixels>) {
        self.0.set(Some(bounds));
    }

    pub fn contains(&self, position: Point<Pixels>) -> bool {
        self.bounds()
            .is_some_and(|bounds| bounds.contains(&position))
    }
}

/// `child`, with the box it is drawn at written into `anchor`. It adds no
/// box of its own: the trigger is the element, as Base UI's `render` makes
/// it.
pub fn measured(anchor: &Anchor, child: impl IntoElement) -> Measured {
    Measured {
        anchor: anchor.clone(),
        child: child.into_any_element(),
    }
}

pub struct Measured {
    anchor: Anchor,
    child: AnyElement,
}

impl IntoElement for Measured {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for Measured {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.anchor.set(bounds);
        self.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);
    }
}

/// `sheet`, drawn above everything and placed against `anchor` — Base UI's
/// portal and positioner. The sheet is laid out at its own size, then put on
/// its side, flipped when that side has no room, and kept inside the window.
pub fn positioned(anchor: &Anchor, placement: Placement, sheet: impl IntoElement) -> Deferred {
    deferred(Positioned {
        anchor: anchor.clone(),
        placement,
        child: sheet.into_any_element(),
        nest: None,
    })
    .priority(1)
}

struct Positioned {
    anchor: Anchor,
    placement: Placement,
    child: AnyElement,
    /// The sheet this one opens inside, which hears where it was drawn.
    nest: Option<Nest>,
}

impl IntoElement for Positioned {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for Positioned {
    type RequestLayoutState = LayoutId;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, LayoutId) {
        // Laid out while the sheet it is inside, if any, is laid out — which
        // is when that sheet's nest is in force.
        self.nest = scoped::<Nest>(cx);
        let child = self.child.request_layout(window, cx);
        // A box the size of the window, out of flow, so the sheet is laid out
        // at its own width rather than squeezed into its trigger's.
        let viewport = window.viewport_size();
        let mut style = Style::default();
        style.position = Position::Absolute;
        style.display = Display::Flex;
        style.align_items = Some(AlignItems::FlexStart);
        style.size = size(viewport.width.into(), viewport.height.into());
        (window.request_layout(style, [child], cx), child)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        child: &mut LayoutId,
        window: &mut Window,
        cx: &mut App,
    ) {
        let laid = window.layout_bounds(*child);
        let anchor = self
            .anchor
            .bounds()
            .unwrap_or(Bounds::new(bounds.origin, size(px(0.), px(0.))));
        let viewport = Bounds::new(point(px(0.), px(0.)), window.viewport_size());
        let origin = place(self.placement, anchor, laid.size, viewport);
        let offset = origin - laid.origin;
        let offset = point(offset.x.round(), offset.y.round());
        if let Some(nest) = &self.nest {
            nest.register(Bounds::new(laid.origin + offset, laid.size));
        }
        window.with_element_offset(offset, |window| self.child.prepaint(window, cx));
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut LayoutId,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);
    }
}

/// Where a sheet of `size` goes against `anchor`, inside `viewport`.
fn place(
    placement: Placement,
    anchor: Bounds<Pixels>,
    sheet: gpui::Size<Pixels>,
    viewport: Bounds<Pixels>,
) -> Point<Pixels> {
    let origin = match placement {
        Placement::Over(offset) => anchor.origin + offset,
        Placement::Side { side, align, gap } => {
            let at = |side| beside(side, align, px(gap), anchor, sheet);
            let fits = |origin: Point<Pixels>| {
                let bounds = Bounds::new(origin, sheet);
                match side {
                    Side::Top | Side::Bottom => {
                        bounds.top() >= viewport.top() && bounds.bottom() <= viewport.bottom()
                    }
                    Side::Left | Side::Right => {
                        bounds.left() >= viewport.left() && bounds.right() <= viewport.right()
                    }
                }
            };
            let wanted = at(side);
            let flipped = at(side.opposite());
            if !fits(wanted) && fits(flipped) {
                flipped
            } else {
                wanted
            }
        }
    };
    let pad = px(COLLISION_PADDING);
    let x = origin
        .x
        .min(viewport.right() - pad - sheet.width)
        .max(viewport.left() + pad);
    let y = origin
        .y
        .min(viewport.bottom() - pad - sheet.height)
        .max(viewport.top() + pad);
    point(x, y)
}

fn beside(
    side: Side,
    align: Align,
    gap: Pixels,
    anchor: Bounds<Pixels>,
    sheet: gpui::Size<Pixels>,
) -> Point<Pixels> {
    let along = |start: Pixels, length: Pixels, extent: Pixels| match align {
        Align::Start => start,
        Align::Center => start + (length - extent) / 2.,
        Align::End => start + length - extent,
    };
    match side {
        Side::Bottom => point(
            along(anchor.left(), anchor.size.width, sheet.width),
            anchor.bottom() + gap,
        ),
        Side::Top => point(
            along(anchor.left(), anchor.size.width, sheet.width),
            anchor.top() - gap - sheet.height,
        ),
        Side::Right => point(
            anchor.right() + gap,
            along(anchor.top(), anchor.size.height, sheet.height),
        ),
        Side::Left => point(
            anchor.left() - gap - sheet.width,
            along(anchor.top(), anchor.size.height, sheet.height),
        ),
    }
}

/// `.dz-popup`: the overlay surface, a strong hairline, the large corner,
/// the large shadow, and a single step of padding around the rows.
pub fn sheet(t: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .min_w(px(t.menu_min_width))
        .p(px(t.spacing_1))
        .bg(t.color_surface_overlay.hsla())
        .border(px(t.stroke_hairline))
        .border_color(t.color_border_strong.hsla())
        .rounded(px(t.radius_large))
        .shadow(shadows(t.shadow_lg))
        .text_color(t.color_content.hsla())
}

/// `.dz-popup.dz-popover`: the same sheet padded for prose rather than rows,
/// as wide as what is in it, in the small body face.
pub fn prose_sheet(t: &ThemeTokens) -> Div {
    sheet(t)
        .min_w(px(0.))
        .p(px(t.spacing_4 + t.spacing_0_5))
        .type_style(t.body_small)
}

/// The popup's own state: whether it is open, what it is anchored to, the
/// highlighted row, and what the keyboard and the pointer are doing to it.
/// Keep one per popup with [`PopupState::keyed`].
pub struct PopupState {
    pub open: bool,
    /// The trigger, as it was last drawn.
    pub anchor: Anchor,
    /// Where the pointer opened it, for a menu that opens there.
    pub point: Option<Point<Pixels>>,
    /// The row the arrow keys and the pointer have reached.
    pub highlighted: Option<usize>,
    /// Takes the keyboard while the popup is open.
    pub focus: FocusHandle,
    restore: Option<FocusHandle>,
    typed: String,
    typed_at: Option<Instant>,
    hovered: [bool; 2],
    timer: Option<Task<()>>,
}

impl PopupState {
    pub fn new(cx: &mut Context<Self>) -> PopupState {
        PopupState {
            open: false,
            anchor: Anchor::default(),
            point: None,
            highlighted: None,
            focus: cx.focus_handle(),
            restore: None,
            typed: String::new(),
            typed_at: None,
            hovered: [false; 2],
            timer: None,
        }
    }

    /// The state kept for the popup `id` names, opened at first if `open`.
    pub fn keyed(id: &ElementId, open: bool, window: &mut Window, cx: &mut App) -> Entity<Self> {
        window.use_keyed_state(part_id(id, "popup"), cx, |_, cx| {
            let mut state = PopupState::new(cx);
            state.open = open;
            state
        })
    }

    /// Opens it and hands it the keyboard, remembering who had it.
    pub fn show(
        &mut self,
        highlighted: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.open {
            self.restore = window.focused(cx);
        }
        self.open = true;
        self.highlighted = highlighted;
        self.typed.clear();
        self.timer = None;
        window.focus(&self.focus);
        cx.notify();
    }

    /// Closes it and gives the keyboard back.
    pub fn hide(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            return;
        }
        self.open = false;
        self.highlighted = None;
        self.timer = None;
        self.hovered = [false; 2];
        if let Some(restore) = self.restore.take()
            && self.focus.contains_focused(window, cx)
        {
            window.focus(&restore);
        }
        cx.notify();
    }

    /// Moves the highlight for a navigation key — the arrows, Home and End —
    /// over the rows `enabled` says can take it. Whether the key was one.
    pub fn step(&mut self, key: &str, enabled: &[bool]) -> bool {
        let rows: Vec<usize> = (0..enabled.len()).filter(|&i| enabled[i]).collect();
        let (Some(&first), Some(&last)) = (rows.first(), rows.last()) else {
            return matches!(key, "down" | "up" | "home" | "end");
        };
        self.highlighted = Some(match (key, self.highlighted) {
            ("home", _) | ("down", None) => first,
            ("end", _) | ("up", None) => last,
            ("down", Some(at)) => rows.iter().copied().find(|&i| i > at).unwrap_or(first),
            ("up", Some(at)) => rows.iter().rev().copied().find(|&i| i < at).unwrap_or(last),
            _ => return false,
        });
        true
    }

    /// Type-ahead: a printable key joins what was typed in the last moment,
    /// and the highlight moves to the next row whose label starts with it.
    pub fn type_ahead(&mut self, text: &str, labels: &[SharedString], enabled: &[bool]) -> bool {
        if text.chars().count() != 1 || text.chars().any(char::is_control) {
            return false;
        }
        let now = Instant::now();
        if self
            .typed_at
            .is_none_or(|at| now.duration_since(at) > Duration::from_millis(750))
        {
            self.typed.clear();
        }
        self.typed_at = Some(now);
        self.typed.push_str(&text.to_lowercase());
        let count = labels.len();
        let start = match (self.highlighted, self.typed.chars().count()) {
            // A fresh key looks past the row it is on; a run keeps it.
            (Some(at), 1) => at + 1,
            (Some(at), _) => at,
            (None, _) => 0,
        };
        let found = (0..count).map(|i| (start + i) % count).find(|&i| {
            enabled.get(i).copied().unwrap_or(false)
                && labels[i].to_lowercase().starts_with(&self.typed)
        });
        if let Some(found) = found {
            self.highlighted = Some(found);
        }
        true
    }

    /// The pointer entered or left a part — `0` the trigger, `1` the sheet —
    /// of a popup the pointer opens: it opens `delay` after the pointer
    /// arrives and closes `close_delay` after it has left both.
    pub fn hover(
        &mut self,
        part: usize,
        hovered: bool,
        delay: Duration,
        close_delay: Duration,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.hovered[part] = hovered;
        let wanted = self.hovered.iter().any(|&h| h);
        if wanted == self.open {
            self.timer = None;
            return;
        }
        let wait = if wanted { delay } else { close_delay };
        if wait.is_zero() {
            self.open = wanted;
            self.timer = None;
            cx.notify();
            return;
        }
        self.timer = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(wait).await;
            let _ = this.update(cx, |state, cx| {
                state.open = wanted;
                state.timer = None;
                cx.notify();
            });
        }));
    }
}

/// A sheet that goes away on its own: an outside press closes it — except a
/// press inside `keep`, the trigger, which toggles it itself, or inside a
/// sheet opened from this one — and so does Escape. It takes the popup's
/// focus, so the keyboard reaches it.
pub fn dismissable(
    sheet: Stateful<Div>,
    state: &Entity<PopupState>,
    keep: &Anchor,
    on_open_change: Option<OnOpenChange>,
    cx: &App,
) -> Provide {
    let focus = state.read(cx).focus.clone();
    let outside = state.clone();
    let escape = state.clone();
    let keep = keep.clone();
    let nest = Nest::inside(cx);
    let inner = nest.clone();
    let notify = on_open_change.clone();
    let sheet = sheet
        .track_focus(&focus)
        .occlude()
        .on_mouse_down_out(move |event, window, cx| {
            if keep.contains(event.position) || inner.contains(event.position) {
                return;
            }
            outside.update(cx, |state, cx| state.hide(window, cx));
            if let Some(notify) = &notify {
                notify(false, window, cx);
            }
        })
        .on_key_down(move |event: &KeyDownEvent, window, cx| {
            if event.keystroke.key == "escape" {
                escape.update(cx, |state, cx| state.hide(window, cx));
                if let Some(notify) = &on_open_change {
                    notify(false, window, cx);
                }
                cx.stop_propagation();
            }
        });
    provide(nest, sheet)
}

/// One row of a list sheet — `.dz-popup__item`: a quiet-weight label on a
/// 26px row, an optional check column, icon and shortcut, and the neutral
/// wash while it is highlighted.
#[derive(IntoElement)]
pub struct PopupRow {
    id: ElementId,
    label: SharedString,
    icon: Option<AnyElement>,
    shortcut: Option<AnyElement>,
    check: Option<bool>,
    highlighted: bool,
    disabled: bool,
    tint: Tint,
    on_hover: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    on_press: Option<Press>,
}

impl PopupRow {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        PopupRow {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            check: None,
            highlighted: false,
            disabled: false,
            tint: Tint::Neutral,
            on_hover: None,
            on_press: None,
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn shortcut(mut self, shortcut: impl IntoElement) -> Self {
        self.shortcut = Some(shortcut.into_any_element());
        self
    }

    /// Gives the row a check column, the mark shown when `checked`. A row
    /// without one has no column at all.
    pub fn check(mut self, checked: bool) -> Self {
        self.check = Some(checked);
        self
    }

    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The ramp the highlight wash reads — the sheet's `data-tint`.
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn on_hover(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_hover = Some(Rc::new(handler));
        self
    }

    pub fn on_press(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_press = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for PopupRow {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let ramp = self.tint.ramp(t);
        let content = if self.disabled {
            t.control_color_normal_content_disabled.resolve(ramp)
        } else {
            t.color_content.hsla()
        };
        let quiet = t.label_quiet;
        let check = self.check.map(|checked| {
            icon_slot(
                t.icon_small,
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .w(px(t.icon_small))
                    .when(checked, |this| {
                        this.child(Icon::new(glyphs::CHECKMARK_12_FILLED))
                    }),
            )
        });
        let icon = self.icon.map(|icon| {
            icon_slot(
                t.icon_medium,
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .text_color(if self.disabled {
                        content
                    } else {
                        t.color_content_muted.hsla()
                    })
                    .child(icon),
            )
        });
        let shortcut = self.shortcut.map(|shortcut| {
            div()
                .flex_none()
                .ml_auto()
                .text_color(if self.disabled {
                    content
                } else {
                    t.color_content_faint.hsla()
                })
                .font_family(t.label_strong.face.family())
                .text_size(px(t.label_small.size))
                .font_weight(t.label_small.font_weight())
                .line_height(px(t.label_small.size))
                .child(shortcut)
        });
        let mut row = div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .gap(px(t.spacing_2))
            .w_full()
            .py(px(t.menu_item_padding))
            .px(px(t.spacing_2))
            .rounded(px(t.radius_large - t.spacing_1))
            .text_color(content)
            .whitespace_nowrap()
            .font_family(quiet.face.family())
            .text_size(px(quiet.size))
            .font_weight(quiet.font_weight())
            .line_height(px(quiet.size))
            .when(self.highlighted && !self.disabled, |this| {
                this.bg(t.control_color_normal_surface_hovered.resolve(ramp))
            })
            .when(self.disabled, |this| this.cursor_not_allowed())
            .children(check)
            .children(icon)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(crate::foundation::text_child(self.label)),
            )
            .children(shortcut);
        if !self.disabled {
            if let Some(on_hover) = self.on_hover {
                row = row.on_hover(move |hovered, window, cx| on_hover(*hovered, window, cx));
            }
            if let Some(on_press) = self.on_press {
                row = row.on_click(move |_, window, cx| on_press(window, cx));
            }
        }
        row
    }
}

/// `.dz-popup__group-label`: a heading inside the sheet, in the section
/// label's voice on the row's own inset.
pub fn group_label(t: &ThemeTokens, label: impl IntoElement) -> Div {
    div()
        .pt(px(t.spacing_1_5))
        .pb(px(t.spacing_1))
        .px(px(t.spacing_2))
        .text_color(t.color_content_subtle.hsla())
        .font_family(t.caption_small.face.family())
        .text_size(px(t.caption_small.size))
        .font_weight(t.label_small.font_weight())
        .line_height(px(t.caption_small.size))
        .child(crate::foundation::text_child(label))
}

/// The height of one row: its label set solid, padded by `menu.item-padding`.
pub fn row_height(t: &ThemeTokens) -> f32 {
    t.label_quiet.size + 2. * t.menu_item_padding
}

/// The height of a group heading.
pub fn group_label_height(t: &ThemeTokens) -> f32 {
    t.caption_small.size + t.spacing_1_5 + t.spacing_1
}

/// An id for one part of the component `id` names — the state a popup keeps
/// beside the state its trigger keeps.
pub fn part_id(id: &ElementId, part: &'static str) -> ElementId {
    ElementId::NamedChild(Box::new(id.clone()), part.into())
}

/// Whether a popup is open: `controlled` when the caller holds the state —
/// React's `open` — and the popup's own otherwise. A controlled value is
/// written through, so dismissal sees what is drawn.
pub fn open_state(state: &Entity<PopupState>, controlled: Option<bool>, cx: &mut App) -> bool {
    match controlled {
        Some(open) => {
            if state.read(cx).open != open {
                state.update(cx, |state, _| state.open = open);
            }
            open
        }
        None => state.read(cx).open,
    }
}

/// The press a trigger is handed: it opens the popup — with `highlighted`
/// reached — or closes it, and reports the change.
pub fn toggle_press(
    state: &Entity<PopupState>,
    highlighted: Option<usize>,
    on_open_change: Option<OnOpenChange>,
) -> Press {
    let state = state.clone();
    Rc::new(move |window, cx| {
        let open = !state.read(cx).open;
        state.update(cx, |state, cx| {
            if open {
                state.show(highlighted, window, cx);
            } else {
                state.hide(window, cx);
            }
        });
        if let Some(notify) = &on_open_change {
            notify(open, window, cx);
        }
    })
}

/// A press that does nothing — a trigger drawn outside any popup.
pub fn no_press() -> Press {
    Rc::new(|_, _| {})
}
