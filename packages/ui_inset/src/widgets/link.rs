use super::{Callback, Pressable};
use crate::{ControlState, ControlVariant, Theme, Tint};
use inset_embedder::TextDecoration;
use inset_foundation::App;
use inset_widgets::*;
use std::rc::Rc;
#[derive(Debug, Clone, Copy, Default)]
pub enum LinkUnderline {
    #[default]
    Hover,
    Always,
    None,
}
#[derive(Clone)]
pub struct Link {
    pub label: String,
    pub tint: Tint,
    pub underline: LinkUnderline,
    pub external: bool,
    pub on_pressed: Option<Callback>,
}
impl std::fmt::Debug for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Link").field("label", &self.label).finish()
    }
}
impl Link {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            tint: Tint::Primary,
            underline: LinkUnderline::Hover,
            external: false,
            on_pressed: None,
        }
    }
    pub fn on_pressed(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.on_pressed = Some(Rc::new(callback));
        self
    }
}
impl StatelessWidget for Link {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.clone();
        let mut p = Pressable::new(move |app, context, phase| {
            let v = Theme::of(app, context);
            let color = v
                .control_colors(ControlVariant::Plain, w.tint, phase)
                .content;
            let mut style = DefaultTextStyle::of(app, context)
                .style
                .clone()
                .color(color);
            if matches!(w.underline, LinkUnderline::Always)
                || matches!(w.underline, LinkUnderline::Hover) && phase == ControlState::Hovered
            {
                style = style.decoration(TextDecoration::UNDERLINE);
            }
            Text::new(format!("{}{}", w.label, if w.external { " ↗" } else { "" }))
                .style(style)
                .into_widget()
        });
        p.on_pressed = self.on_pressed.clone();
        p.into_widget()
    }
}
