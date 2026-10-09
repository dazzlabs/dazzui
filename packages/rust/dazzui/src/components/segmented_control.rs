//! A choice between a few segments that always has one.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};

use crate::components::checkbox::kit;
use crate::components::icon::icon_slot;
use crate::foundation::{ControlColors, Size, State, Tint, Variant, pointer_down};
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

type OnValueChange = Rc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>;

/// How the picked-out segment is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ActiveStyle {
    /// A lighter capsule lifted off the track, as a desktop control draws it.
    #[default]
    Raised,
    /// Painted with the tint — louder, for a choice that has to shout.
    Filled,
}

impl ActiveStyle {
    pub fn parse(name: &str) -> Option<ActiveStyle> {
        Some(match name {
            "raised" => ActiveStyle::Raised,
            "filled" => ActiveStyle::Filled,
            _ => return None,
        })
    }
}

/// One segment of a [`SegmentedControl`].
pub struct SegmentedItem {
    pub value: SharedString,
    /// The segment's name. With [`SegmentedControl::icon_only`] it is not
    /// drawn, and GPUI has no accessibility tree to name the glyph with it.
    pub label: AnyElement,
    /// A glyph drawn before the label, or in its place with `icon_only`.
    pub icon: Option<AnyElement>,
    pub disabled: bool,
}

impl SegmentedItem {
    pub fn new(value: impl Into<SharedString>, label: impl IntoElement) -> Self {
        SegmentedItem {
            value: value.into(),
            label: label.into_any_element(),
            icon: None,
            disabled: false,
        }
    }

    /// A glyph before the label, drawn `control.label-glyph.shrink` under
    /// the profile's icon size; the leading end pads
    /// `control.label-glyph.inset` less, as Button's does.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A recessed track with one segment picked out. Pressing the active
/// segment again is ignored rather than clearing it. The track is one tab
/// stop; the arrow keys move focus along it and Space picks.
#[derive(IntoElement)]
pub struct SegmentedControl {
    id: ElementId,
    items: Vec<SegmentedItem>,
    value: SharedString,
    size: Size,
    tint: Tint,
    active_style: ActiveStyle,
    stretch: bool,
    icon_only: bool,
    on_value_change: Option<OnValueChange>,
}

impl SegmentedControl {
    pub fn new(
        id: impl Into<ElementId>,
        items: Vec<SegmentedItem>,
        value: impl Into<SharedString>,
    ) -> Self {
        SegmentedControl {
            id: id.into(),
            items,
            value: value.into(),
            size: Size::Medium,
            tint: Tint::Primary,
            active_style: ActiveStyle::Raised,
            stretch: false,
            icon_only: false,
            on_value_change: None,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn active_style(mut self, active_style: ActiveStyle) -> Self {
        self.active_style = active_style;
        self
    }

    /// Fill the available width, splitting it evenly between the segments.
    pub fn stretch(mut self, stretch: bool) -> Self {
        self.stretch = stretch;
        self
    }

    /// Draw each segment's glyph alone, at the profile's full icon size. A
    /// segment without a glyph still draws its label.
    pub fn icon_only(mut self, icon_only: bool) -> Self {
        self.icon_only = icon_only;
        self
    }

    /// The accessible name. GPUI has no accessibility tree to give it to.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for SegmentedControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let profile = self.size.profile(t);
        // The medium track takes the regular control corner, a step above
        // the button beside it.
        let radius = if self.size == Size::Medium {
            t.radius_medium
        } else {
            profile.radius
        };
        let inset = t.segmented_control_inset;
        let count = self.items.len();
        let handles = kit::focus_handles(&self.id, count, window, cx);
        let enabled: Vec<bool> = self.items.iter().map(|item| !item.disabled).collect();
        let selected = self.items.iter().position(|item| item.value == self.value);
        let tab_stop = kit::focused(&handles, window)
            .or(selected.filter(|index| enabled[*index]))
            .or_else(|| enabled.iter().position(|e| *e));
        let filled = ControlColors::resolve(t, Variant::Filled, self.tint, State::Normal);
        let quiet = t.label_quiet;
        let font_size = profile.content.size;
        let icon_only = self.icon_only;
        let label_glyph = profile.icon_size - t.control_label_glyph_shrink;
        let glyph_padding = px(t.spacing_3 - t.control_label_glyph_inset);

        let segments = self
            .items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let pressed = selected == Some(index);
                let focus = handles[index].clone();
                let visible = crate::foundation::focus_visible(&focus, window, cx);
                // A glyph beside the label is drawn and padded as Button
                // draws one; a glyph alone keeps the slot's full size.
                let leading = item.icon.is_some() && !icon_only;
                let glyph = item.icon.map(|icon| {
                    if icon_only {
                        icon
                    } else {
                        icon_slot(label_glyph, icon).into_any_element()
                    }
                });
                let label = (!icon_only || glyph.is_none()).then_some(item.label);
                let mut segment = div()
                    .id(kit::child_id(&self.id, &format!("segment-{index}")))
                    .relative()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(profile.gap))
                    .h(px(profile.size - 2. * inset))
                    .px(px(t.spacing_3))
                    .when(leading, |this| this.pl(glyph_padding))
                    .rounded(px(radius - inset))
                    .font_family(quiet.face.family())
                    .text_size(px(font_size))
                    .font_weight(quiet.font_weight())
                    .line_height(px(font_size))
                    .text_color(t.color_content_secondary.hsla())
                    .whitespace_nowrap()
                    .when(self.stretch, |this| this.flex_1())
                    .when(pressed, |this| {
                        let this = this
                            .font_weight(t.label_medium.font_weight())
                            .shadow(shadows(t.shadow_2xs));
                        match self.active_style {
                            ActiveStyle::Raised => this
                                .bg(t.color_surface_raised.hsla())
                                .text_color(t.color_content.hsla()),
                            ActiveStyle::Filled => {
                                this.bg(filled.surface).text_color(filled.content)
                            }
                        }
                    })
                    .children(glyph)
                    .children(label)
                    .when(visible, |this| this.child(kit::outline(t, radius - inset)));
                if item.disabled {
                    segment = segment.opacity(0.5).cursor_not_allowed();
                } else {
                    let handler = self.on_value_change.clone();
                    let value = item.value.clone();
                    let content = t.color_content.hsla();
                    segment = segment
                        .cursor_pointer()
                        .when(!pressed, |this| {
                            this.hover(move |style| style.text_color(content))
                        })
                        .track_focus(&focus.tab_stop(tab_stop == Some(index)))
                        .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                        .on_click(move |_, window, cx| {
                            if pressed {
                                return;
                            }
                            if let Some(handler) = &handler {
                                handler(value.clone(), window, cx);
                            }
                        });
                }
                segment
            })
            .collect::<Vec<_>>();

        icon_slot(
            profile.icon_size,
            div()
                .id(self.id)
                .flex()
                .when(self.stretch, |this| this.w_full())
                .p(px(inset))
                .bg(t.color_surface_inset.hsla())
                .rounded(px(radius))
                .on_key_down(move |event, window, cx| {
                    let Some(step) = kit::arrow(&event.keystroke.key, false) else {
                        return;
                    };
                    let Some(current) = kit::focused(&handles, window) else {
                        return;
                    };
                    if let Some(next) = kit::target(current, &enabled, step) {
                        window.focus(&handles[next]);
                    }
                    cx.stop_propagation();
                })
                .children(segments),
        )
    }
}
