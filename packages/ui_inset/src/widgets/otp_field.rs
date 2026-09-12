use super::ValueChanged;
use crate::{ControlSize, Theme};
use inset_cupertino::CupertinoTextField;
use inset_foundation::{App, Handle};
use inset_painting::{
    Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry, TextStyle,
};
use inset_rendering::MainAxisSize;
use inset_services::{
    FilteringTextInputFormatter, LengthLimitingTextInputFormatter, TextInputType,
};
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct OtpField {
    pub length: usize,
    pub value: String,
    pub mask: bool,
    pub size: ControlSize,
    pub on_changed: Option<ValueChanged<String>>,
    pub on_completed: Option<ValueChanged<String>>,
}
impl std::fmt::Debug for OtpField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OtpField")
            .field("length", &self.length)
            .finish()
    }
}
impl OtpField {
    pub fn new(length: usize, value: impl Into<String>) -> Self {
        assert!(length > 0 && length <= i32::MAX as usize);
        Self {
            length,
            value: value.into(),
            mask: false,
            size: ControlSize::Medium,
            on_changed: None,
            on_completed: None,
        }
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, String) + 'static) -> Self {
        self.on_changed = Some(Rc::new(callback));
        self
    }
    pub fn on_completed(mut self, callback: impl Fn(&mut App, String) + 'static) -> Self {
        self.on_completed = Some(Rc::new(callback));
        self
    }
}
impl StatefulWidget for OtpField {
    type State = OtpState;
    fn create_state(&self) -> Self::State {
        OtpState {
            state: StateData::new(),
            controller: None,
            focused: false,
        }
    }
}
pub struct OtpState {
    state: StateData<OtpField>,
    controller: Option<Handle<TextEditingController>>,
    focused: bool,
}
impl State for OtpState {
    type Widget = OtpField;
    inset_widgets::state_accessors!();
    fn init_state(self: Handle<Self>, app: &mut App) {
        let text = self.widget(app).value.clone();
        let c = TextEditingController::new(app).text(app, text);
        app.get_mut(self).controller = Some(c);
    }
    fn did_update_widget(self: Handle<Self>, app: &mut App, old: &OtpField) {
        if old.value != self.widget(app).value {
            let value = self.widget(app).value.clone();
            app.get(self).controller.unwrap().set_text(app, value);
        }
    }
    fn dispose(self: Handle<Self>, app: &mut App) {
        let c = app.get_mut(self).controller.take().unwrap();
        app.get_mut(c).dispose();
        app.destroy(c);
    }
    fn build(self: Handle<Self>, app: &mut App, context: BuildContext) -> WidgetRef {
        let w = self.widget(app).clone();
        let v = Theme::of(app, context);
        let m = v.metrics(w.size);
        let chars: Vec<_> = w.value.chars().collect();
        let focused = app.get(self).focused;
        let clear = crate::generated::colors::TRANSPARENT;
        let cells: Vec<_> = (0..w.length)
            .map(|i| {
                let filled = i < chars.len();
                let here = focused && i == chars.len().min(w.length - 1);
                Container::new()
                    .width(m.size)
                    .height(m.size)
                    .decoration(
                        BoxDecoration::new()
                            .color(if filled || here {
                                v.color_surface
                            } else {
                                v.color_surface_muted
                            })
                            .border(Border::all(
                                if here {
                                    v.color_primary.shade(v.focus_ring_shade)
                                } else {
                                    v.color_border_strong
                                },
                                v.stroke_hairline,
                                BorderStyle::Solid,
                                -1.0,
                            ))
                            .border_radius(BorderRadiusGeometry::circular(v.radius_medium)),
                    )
                    .child(
                        Center::new().child(
                            Text::new(if filled {
                                if w.mask {
                                    "•".into()
                                } else {
                                    chars[i].to_string()
                                }
                            } else {
                                String::new()
                            })
                            .style(
                                v.label_medium
                                    .text_style()
                                    .font_family("monospace")
                                    .color(v.color_content),
                            ),
                        ),
                    )
                    .into_widget()
            })
            .collect();
        let field = CupertinoTextField::new()
            .controller(app.get(self).controller.unwrap())
            .enabled(w.on_changed.is_some())
            .decoration(None)
            .padding(EdgeInsetsGeometry::ZERO)
            .style(TextStyle::new().color(clear))
            .cursor_color(clear)
            .keyboard_type(TextInputType::NUMBER)
            .max_length(w.length as i32)
            .autofill_hints(vec!["oneTimeCode".into()])
            .input_formatters(vec![
                FilteringTextInputFormatter::digits_only(),
                inset_services::TextInputFormatterRef(Rc::new(
                    LengthLimitingTextInputFormatter::new(Some(w.length as i32)),
                )),
            ])
            .on_changed(move |app, value| {
                if let Some(callback) = &w.on_changed {
                    callback(app, value.clone());
                }
                if value.chars().count() == w.length
                    && let Some(callback) = &w.on_completed
                {
                    callback(app, value);
                }
            });
        Focus::new(
            Stack::new().children([
                Row::new()
                    .main_axis_size(MainAxisSize::Min)
                    .spacing(v.spacing_2)
                    .children(cells)
                    .into_widget(),
                Positioned::fill(field).into_widget(),
            ]),
        )
        .can_request_focus(false)
        .on_focus_change(move |app, value| self.set_state(app, |s| s.focused = value))
        .into_widget()
    }
}
