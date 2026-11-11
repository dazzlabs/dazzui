//! Sidebar actions position the existing icon button on the original hit box.
use super::*;
use crate::IconButton;
use crate::foundation::motion::{Motion, Presence};
use gpui::SharedString;
use std::cell::RefCell;

struct ActionButton {
    id: ElementId,
    button: IconButton,
    size: Size,
}

/// An icon button on the trailing edge of a group's heading — handed to
/// [`SidebarGroupLabel::action`], which gives it a column of its own.
#[derive(IntoElement)]
pub struct SidebarGroupAction {
    action: ActionButton,
}

/// An icon button centred on a menu item's trailing edge. Transparent
/// actions remain keyboard reachable, as CSS opacity does on React.
#[derive(IntoElement)]
pub struct SidebarMenuAction {
    action: ActionButton,
    show_on_hover: bool,
    popup_open: bool,
}

macro_rules! action_api {
    ($name:ident $(, $field:ident: $value:expr)*) => {
        impl $name {
            pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
                let id = id.into();
                Self { action: ActionButton {button: IconButton::new(id.clone(), label), id, size: Size::Tiny}, $($field: $value,)* }
            }
            pub fn size(mut self, size: Size) -> Self {self.action.button = self.action.button.size(size);self.action.size = size;self}
            pub fn variant(mut self, variant: Variant) -> Self {self.action.button = self.action.button.variant(variant);self}
            pub fn tint(mut self, tint: Tint) -> Self {self.action.button = self.action.button.tint(tint);self}
            pub fn disabled(mut self, disabled: bool) -> Self {self.action.button = self.action.button.disabled(disabled);self}
            pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
                self.action.button = self.action.button.on_click(handler);self
            }
        }
        impl ParentElement for $name {
            fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {self.action.button.extend(elements);}
        }
    };
}
action_api!(SidebarGroupAction);
action_api!(SidebarMenuAction, show_on_hover: false, popup_open: false);

impl SidebarMenuAction {
    pub fn show_on_hover(mut self, show: bool) -> Self {
        self.show_on_hover = show;
        self
    }
    /// A menu trigger can retain its visible action while its popup is open.
    pub fn popup_open(mut self, open: bool) -> Self {
        self.popup_open = open;
        self
    }
    pub(super) fn identity(&self) -> ElementId {
        self.action.id.clone()
    }
}
impl RenderOnce for SidebarGroupAction {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        if icon_column(cx) {
            return div().into_any_element();
        }
        self.action.button.into_any_element()
    }
}
impl RenderOnce for SidebarMenuAction {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if icon_column(cx) {
            return div().into_any_element();
        }
        let right = cx.theme().spacing_1;
        let half_height = self.action.size.profile(cx.theme()).size / 2.;
        let focus = kept_focus(part_id(&self.action.id, "focus"), window, cx);
        let owner = scoped::<ItemSlot>(cx);
        let show = self.show_on_hover;
        let popup = self.popup_open;
        let id = self.action.id;
        self.action
            .button
            .focus_handle(focus.clone())
            .decorate(move |button, window, cx| {
                let button = button
                    .absolute()
                    .top(gpui::relative(0.5))
                    .mt(px(-half_height))
                    .right(px(right));
                if !show {
                    return button;
                }
                let animation = window.use_keyed_state(part_id(&id, "reveal"), cx, |_, _| {
                    Rc::new(RefCell::new(None))
                });
                let animation = animation.read(cx).clone();
                button
                    .opacity(0.)
                    .group_hover(ITEM_GROUP, |style| style.opacity(1.))
                    .map_paint_style(move |style, window, cx| {
                        let target = popup
                            || style.opacity.unwrap_or(1.) > 0.
                            || focus.is_focused(window)
                            || owner
                                .as_ref()
                                .is_some_and(|owner| owner.revealed(window, cx));
                        let frame = Motion::frame(cx);
                        let t = cx.theme();
                        let mut animation = animation.borrow_mut();
                        let animation =
                            animation.get_or_insert_with(|| Presence::new(target, frame.elapsed));
                        let sample =
                            animation.sample(target, frame, t.motion_duration, t.motion_easing);
                        style.opacity = Some(sample.amount);
                        if sample.animating {
                            window.request_animation_frame();
                        }
                    })
            })
            .into_any_element()
    }
}
