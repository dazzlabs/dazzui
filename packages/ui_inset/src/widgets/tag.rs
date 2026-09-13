use super::{BadgeVariant, Callback, FluentGlyph, FluentIcon, Pressable, ValueChanged};
use crate::{ColorExt, ControlPart, ControlSize, ControlState, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_painting::{
    Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry,
};
use inset_rendering::{CrossAxisAlignment, MainAxisSize};
use inset_widgets::*;
use std::rc::Rc;

/// The ✕ inside a tag: faint at rest, resolved under the pointer, fainter dead.
const DISMISS_REST: f64 = 0.6;
const DISMISS_DEAD: f64 = 0.4;
/// The pointer wash on the dismiss target, an alpha of the capsule's own ink —
/// the one colour guaranteed to read on every variant in every theme.
const DISMISS_HOVER_ALPHA: f64 = 0.16;

/// A [`Badge`](super::Badge) you can act on: selectable, removable, or both.
#[derive(Clone)]
pub struct Tag {
    pub label: String,
    pub tint: Tint,
    pub size: ControlSize,
    pub variant: BadgeVariant,
    /// `Some` makes the tag pressable; `true` draws it selected.
    pub selected: Option<bool>,
    pub disabled: bool,
    pub on_selected_changed: Option<ValueChanged<bool>>,
    pub on_remove: Option<Callback>,
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tag").field("label", &self.label).finish()
    }
}
impl Tag {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            tint: Tint::Primary,
            size: ControlSize::Medium,
            variant: BadgeVariant::Tinted,
            selected: None,
            disabled: false,
            on_selected_changed: None,
            on_remove: None,
        }
    }
    pub fn on_remove(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.on_remove = Some(Rc::new(callback));
        self
    }
}
impl StatelessWidget for Tag {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.clone();
        let pressable = w.selected.is_some();
        let mut p = Pressable::new(move |app, context, state| {
            let v = Theme::of(app, context);
            // A static tag is a label, and a label has no pointer states.
            let phase = if pressable {
                state
            } else {
                ControlState::Normal
            };
            let drawn = if w.selected == Some(true) {
                BadgeVariant::Tinted
            } else {
                w.variant
            };
            let recipe = |part| {
                let variant = match drawn {
                    BadgeVariant::Filled => ControlVariant::Filled,
                    BadgeVariant::Tinted => ControlVariant::Tinted,
                    BadgeVariant::Outlined => ControlVariant::Outlined,
                    _ => return None,
                };
                Some(v.control_color(variant, part, w.tint, phase))
            };
            let transparent = crate::generated::colors::TRANSPARENT;
            let (surface, content) = match drawn {
                BadgeVariant::Raised => (v.color_surface, v.color_content_muted),
                _ => (
                    recipe(ControlPart::Surface).unwrap_or(transparent),
                    recipe(ControlPart::Content).unwrap_or(v.color_content),
                ),
            };
            // Selected is drawn twice over: the tinted fill and the tint's edge.
            let border = if w.selected == Some(true) {
                v.control_color(
                    ControlVariant::Outlined,
                    ControlPart::Border,
                    w.tint,
                    ControlState::Normal,
                )
            } else if drawn == BadgeVariant::Outlined {
                recipe(ControlPart::Border).unwrap_or(transparent)
            } else {
                transparent
            };
            let large = w.size == ControlSize::Large;
            let x = if matches!(w.size, ControlSize::Tiny | ControlSize::Small) {
                v.spacing_1_5
            } else {
                v.spacing_2
            };
            let y = if large { v.spacing_1 } else { v.spacing_0_5 };
            let mut face = v.label_strong;
            face.size = if large {
                v.label_medium.size
            } else {
                v.label_small.size
            };
            face.weight = v.label_medium.weight;
            let mut children = vec![
                Text::new(&w.label)
                    .style(face.text_style().height(1.4).color(content))
                    .into_widget(),
            ];
            if let Some(remove) = w.on_remove.clone() {
                let disabled = w.disabled;
                let mut dismiss = Pressable::new(move |app, context, state| {
                    let v = Theme::of(app, context);
                    let hovered = state == ControlState::Hovered && !disabled;
                    let opacity = if disabled {
                        DISMISS_DEAD
                    } else if hovered {
                        1.0
                    } else {
                        DISMISS_REST
                    };
                    Container::new()
                        .margin(EdgeInsetsGeometry::only(v.spacing_1, 0.0, 0.0, 0.0))
                        .decoration(
                            BoxDecoration::new()
                                .color(if hovered {
                                    ColorExt::opacity(content, DISMISS_HOVER_ALPHA)
                                } else {
                                    transparent
                                })
                                .border_radius(BorderRadiusGeometry::circular(v.radius_full)),
                        )
                        .child(IconTheme::merge(
                            None,
                            IconThemeData::new()
                                .size(v.spacing_3)
                                .color(ColorExt::opacity(content, opacity)),
                            FluentIcon(FluentGlyph::Dismiss12Regular),
                        ))
                        .into_widget()
                });
                dismiss.radius = v.radius_full;
                if !disabled {
                    dismiss.on_pressed = Some(Rc::new(move |app| remove(app)));
                }
                children.push(dismiss.into_widget());
            }
            Container::new()
                .padding(EdgeInsetsGeometry::symmetric(
                    y + v.spacing_px,
                    x + v.spacing_px,
                ))
                .decoration(
                    BoxDecoration::new()
                        .color(surface)
                        .border_radius(BorderRadiusGeometry::circular(v.radius_tiny)),
                )
                .foreground_decoration(
                    BoxDecoration::new()
                        .border(Border::all(border, v.spacing_px, BorderStyle::Solid, -1.0))
                        .border_radius(BorderRadiusGeometry::circular(v.radius_tiny)),
                )
                .child(
                    Row::new()
                        .main_axis_size(MainAxisSize::Min)
                        .cross_axis_alignment(CrossAxisAlignment::Center)
                        .children(children),
                )
                .into_widget()
        });
        if let (Some(selected), false) = (self.selected, self.disabled) {
            if let Some(callback) = self.on_selected_changed.clone() {
                p.on_pressed = Some(Rc::new(move |app| callback(app, !selected)));
            }
        }
        p.into_widget()
    }
}
