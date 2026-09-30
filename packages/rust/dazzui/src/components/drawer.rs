//! A sheet that comes in from an edge of the window. It shares the dialog's
//! materials — the overlay surface, the scrim, the three rows — and differs
//! in where it stands: flush to one edge, rounded only on the two corners
//! still inside the view.

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled,
    Window, div, prelude::FluentBuilder, px,
};

use crate::components::collapsible::{OnOpenChange, part_id};
use crate::components::dialog::{
    DialogClose, modal, modal_layer, sheet_body, sheet_footer, sheet_header, trigger,
};
use crate::foundation::Length;
use crate::theme::ActiveTheme;

/// The edge a drawer comes in from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    Left,
    #[default]
    Right,
    Top,
    Bottom,
}

impl DrawerSide {
    pub fn parse(name: &str) -> Option<DrawerSide> {
        match name {
            "left" => Some(DrawerSide::Left),
            "right" => Some(DrawerSide::Right),
            "top" => Some(DrawerSide::Top),
            "bottom" => Some(DrawerSide::Bottom),
            _ => None,
        }
    }
}

#[derive(IntoElement)]
pub struct Drawer {
    id: ElementId,
    side: DrawerSide,
    size: Option<Length>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<OnOpenChange>,
    title: Option<AnyElement>,
    description: Option<AnyElement>,
    footer: Option<AnyElement>,
    trigger: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl Drawer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Drawer {
            id: id.into(),
            side: DrawerSide::Right,
            size: None,
            open: None,
            default_open: false,
            on_open_change: None,
            title: None,
            description: None,
            footer: None,
            trigger: None,
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    /// Overrides `drawer.size` — the width, or the height on a top or bottom
    /// drawer.
    pub fn size(mut self, size: Length) -> Self {
        self.size = Some(size);
        self
    }

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
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }

    pub fn description(mut self, description: impl IntoElement) -> Self {
        self.description = Some(description.into_any_element());
        self
    }

    /// A row of controls pinned to the far edge, clear of the scrolling body.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }
}

impl ParentElement for Drawer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Drawer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let modal = modal(
            &self.id,
            self.open,
            self.default_open,
            self.on_open_change.clone(),
            window,
            cx,
        );
        let trigger_el = self
            .trigger
            .map(|el| trigger(part_id(&self.id, "trigger"), el, modal.set_open.clone()));

        let sheet = modal.open.then(|| {
            let t = cx.theme();
            let side = self.side;
            let size = self
                .size
                .map(Length::definite)
                .unwrap_or(px(t.drawer_size).into());
            let radius = px(t.frame_window_radius);
            let rule = px(t.stroke_hairline);
            let vertical = matches!(side, DrawerSide::Top | DrawerSide::Bottom);
            let has_header = self.title.is_some() || self.description.is_some();

            let panel = div()
                .id(part_id(&self.id, "sheet"))
                .debug_selector(|| "dz-drawer".into())
                .absolute()
                .flex()
                .flex_col()
                .overflow_hidden()
                .bg(t.color_surface_overlay.hsla())
                .border_color(t.color_border_strong.hsla())
                .occlude()
                .map(|this| match side {
                    DrawerSide::Right => this
                        .top_0()
                        .bottom_0()
                        .right_0()
                        .w(size)
                        .max_w_full()
                        .border_l(rule)
                        .rounded_tl(radius)
                        .rounded_bl(radius),
                    DrawerSide::Left => this
                        .top_0()
                        .bottom_0()
                        .left_0()
                        .w(size)
                        .max_w_full()
                        .border_r(rule)
                        .rounded_tr(radius)
                        .rounded_br(radius),
                    DrawerSide::Top => this
                        .left_0()
                        .right_0()
                        .top_0()
                        .h(size)
                        .max_h_full()
                        .border_b(rule)
                        .rounded_bl(radius)
                        .rounded_br(radius),
                    DrawerSide::Bottom => this
                        .left_0()
                        .right_0()
                        .bottom_0()
                        .h(size)
                        .max_h_full()
                        .border_t(rule)
                        .rounded_tl(radius)
                        .rounded_tr(radius),
                })
                // The grabber, on the two edges a thumb pushes.
                .when(vertical, |this| {
                    this.child(
                        div()
                            .flex_none()
                            .map(|mut this| {
                                this.style().align_self = Some(gpui::AlignSelf::Center);
                                this
                            })
                            .w(px(t.spacing_9))
                            .h(px(t.spacing_1))
                            .my(px(t.spacing_2))
                            .rounded(px(t.radius_full))
                            .bg(t.color_border_strong.hsla()),
                    )
                })
                .when(has_header, |this| {
                    this.child(sheet_header(self.title, self.description, Vec::new(), cx))
                })
                .child(sheet_body(part_id(&self.id, "body"), self.children, cx).flex_1())
                .when_some(self.footer, |this, footer| {
                    this.child(sheet_footer(vec![footer], cx))
                });
            modal_layer(&self.id, &modal, true, panel, window, cx)
        });

        div().children(trigger_el).children(sheet)
    }
}

/// Wraps a control so pressing it closes the drawer it sits in.
pub type DrawerClose = DialogClose;
