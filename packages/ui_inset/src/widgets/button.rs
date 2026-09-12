use super::{Callback, Pressable};
use crate::{ControlSize, ControlState, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_painting::{
    Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry,
};
use inset_rendering::{MainAxisAlignment, MainAxisSize};
use inset_widgets::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct Button {
    pub child: WidgetRef,
    pub tint: Tint,
    pub variant: ControlVariant,
    pub size: ControlSize,
    pub on_pressed: Option<Callback>,
    pub expand: bool,
    pub square: bool,
    pub autofocus: bool,
}
impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("variant", &self.variant)
            .field("size", &self.size)
            .finish()
    }
}
impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self::child(Text::new(label))
    }
    pub fn child<K>(child: impl IntoWidget<K>) -> Self {
        Self {
            child: child.into_widget(),
            tint: Tint::Primary,
            variant: ControlVariant::Recessed,
            size: ControlSize::Small,
            on_pressed: None,
            expand: false,
            square: false,
            autofocus: false,
        }
    }
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
    pub fn variant(mut self, variant: ControlVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }
    pub fn on_pressed(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.on_pressed = Some(Rc::new(callback));
        self
    }
}
impl StatelessWidget for Button {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let radius = Theme::of(app, context).metrics(self.size).radius;
        let widget = self.clone();
        let mut pressable = Pressable::new(move |app, context, phase| {
            let v = Theme::of(app, context);
            let metrics = v.metrics(widget.size);
            let colors = v.control_colors(widget.variant, widget.tint, phase);
            let mut face = if widget.size == ControlSize::Tiny {
                v.label_small
            } else {
                v.label_medium
            };
            if widget.variant == ControlVariant::Filled && phase != ControlState::Disabled {
                face.weight = v.label_strong.weight;
            }
            let style = DefaultTextStyle::of(app, context)
                .style
                .merge(Some(&face.text_style().color(colors.content)));
            let content = DefaultTextStyle::new(
                style,
                Row::new()
                    .main_axis_size(if widget.expand {
                        MainAxisSize::Max
                    } else {
                        MainAxisSize::Min
                    })
                    .main_axis_alignment(MainAxisAlignment::Center)
                    .children([widget.child.clone()]),
            );
            let content = IconTheme::merge(
                None,
                IconThemeData::new().size(v.spacing_4).color(colors.content),
                content,
            );
            let mut box_ = Container::new()
                .height(metrics.size)
                .padding(EdgeInsetsGeometry::symmetric(
                    0.0,
                    if widget.square {
                        0.0
                    } else {
                        metrics.padding_inline
                    },
                ))
                .decoration(
                    BoxDecoration::new()
                        .color(colors.surface)
                        .box_shadow(
                            if widget.variant == ControlVariant::Filled
                                && phase != ControlState::Disabled
                            {
                                crate::shadows(v.shadow_2xs)
                            } else {
                                vec![]
                            },
                        )
                        .border_radius(BorderRadiusGeometry::circular(metrics.radius)),
                )
                .foreground_decoration(
                    BoxDecoration::new()
                        .border(Border::all(
                            colors.border,
                            v.stroke_hairline,
                            BorderStyle::Solid,
                            -1.0,
                        ))
                        .border_radius(BorderRadiusGeometry::circular(metrics.radius)),
                )
                .child(content);
            if widget.square {
                box_ = box_.width(metrics.size);
            }
            if widget.expand {
                box_ = box_.width(f64::INFINITY);
            }
            box_.into_widget()
        });
        pressable.on_pressed = self.on_pressed.clone();
        pressable.radius = radius;
        pressable.autofocus = self.autofocus;
        pressable.into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct IconButton(pub Button);
impl IconButton {
    pub fn new<K>(icon: impl IntoWidget<K>) -> Self {
        let mut b = Button::child(icon);
        b.square = true;
        Self(b)
    }
    pub fn on_pressed(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.0 = self.0.on_pressed(callback);
        self
    }
    pub fn tint(mut self, tint: Tint) -> Self {
        self.0.tint = tint;
        self
    }
    pub fn variant(mut self, variant: ControlVariant) -> Self {
        self.0.variant = variant;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.0.size = size;
        self
    }
}
impl StatelessWidget for IconButton {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        self.0.clone().into_widget()
    }
}
