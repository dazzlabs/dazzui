//! Editing/IME/selection use Inset's Flutter text-field implementation;
//! all field geometry and decoration are supplied by DazzUI.
use super::ValueChanged;
use crate::{ControlSize, Theme, Tint};
use inset_cupertino::CupertinoTextField;
use inset_foundation::{App, Handle};
use inset_painting::{
    Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry,
};
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone, Default)]
pub struct TextField {
    pub controller: Option<Handle<TextEditingController>>,
    pub placeholder: String,
    pub size: ControlSize,
    pub invalid: bool,
    pub mono: bool,
    pub tint: Tint,
    pub disabled: bool,
    pub read_only: bool,
    pub obscure_text: bool,
    pub multiline: bool,
    pub on_changed: Option<ValueChanged<String>>,
    pub on_submitted: Option<ValueChanged<String>>,
    pub prefix: Option<WidgetRef>,
    pub suffix: Option<WidgetRef>,
}
impl std::fmt::Debug for TextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextField")
            .field("placeholder", &self.placeholder)
            .finish()
    }
}
impl TextField {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn controller(mut self, controller: Handle<TextEditingController>) -> Self {
        self.controller = Some(controller);
        self
    }
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
    pub fn mono(mut self, mono: bool) -> Self {
        self.mono = mono;
        self
    }
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }
    pub fn obscure_text(mut self, obscure: bool) -> Self {
        self.obscure_text = obscure;
        self
    }
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, String) + 'static) -> Self {
        self.on_changed = Some(Rc::new(callback));
        self
    }
    pub fn on_submitted(mut self, callback: impl Fn(&mut App, String) + 'static) -> Self {
        self.on_submitted = Some(Rc::new(callback));
        self
    }
}
impl StatefulWidget for TextField {
    type State = TextFieldState;
    fn create_state(&self) -> Self::State {
        TextFieldState {
            state: StateData::new(),
            focused: false,
        }
    }
}
pub struct TextFieldState {
    state: StateData<TextField>,
    focused: bool,
}
impl State for TextFieldState {
    type Widget = TextField;
    inset_widgets::state_accessors!();
    fn build(self: Handle<Self>, app: &mut App, context: BuildContext) -> WidgetRef {
        let w = self.widget(app).clone();
        let v = Theme::of(app, context);
        let m = v.metrics(w.size);
        let focused = app.get(self).focused;
        let tint = if w.invalid { Tint::Danger } else { w.tint };
        let accent = v.ramp(tint).shade(v.focus_ring_shade);
        let surface = if w.disabled {
            v.color_surface_sunken
        } else if focused {
            v.color_surface
        } else {
            v.color_surface_muted
        };
        let border = if focused || w.invalid {
            accent
        } else {
            v.color_border_strong
        };
        let radius = if w.multiline {
            v.radius_large
        } else {
            v.radius_medium
        };
        let face = if w.size == ControlSize::Tiny {
            v.label_small
        } else {
            v.label_medium
        };
        let mut style =
            DefaultTextStyle::of(app, context)
                .style
                .merge(Some(&face.text_style().color(if w.disabled {
                    v.color_content_faint
                } else {
                    v.color_content
                })));
        if w.mono {
            style = style.font_family("monospace");
        }
        let placeholder_style = if w.mono {
            face.text_style().font_family("monospace")
        } else {
            face.text_style()
        };
        let mut field = CupertinoTextField::new()
            .decoration(None)
            .padding(EdgeInsetsGeometry::symmetric(
                m.padding_block,
                v.spacing_2_5,
            ))
            .style(style)
            .placeholder(&w.placeholder)
            .placeholder_style(Some(placeholder_style.color(v.color_content_faint)))
            .cursor_color(accent)
            .enabled(!w.disabled)
            .read_only(w.read_only)
            .obscure_text(w.obscure_text)
            .max_lines(if w.multiline { None } else { Some(1) });
        if let Some(controller) = w.controller {
            field = field.controller(controller);
        }
        if let Some(callback) = w.on_changed {
            field = field.on_changed(move |app, value| callback(app, value));
        }
        if let Some(callback) = w.on_submitted {
            field = field.on_submitted(move |app, value| callback(app, value));
        }
        if let Some(prefix) = w.prefix {
            field = field.prefix(prefix);
        }
        if let Some(suffix) = w.suffix {
            field = field.suffix(suffix);
        }
        let mut box_ = Container::new()
            .decoration(
                BoxDecoration::new()
                    .color(surface)
                    .border(Border::all(
                        border,
                        v.stroke_hairline,
                        BorderStyle::Solid,
                        -1.0,
                    ))
                    .border_radius(BorderRadiusGeometry::circular(radius)),
            )
            .child(field);
        if !w.multiline {
            box_ = box_.height(m.size);
        }
        Focus::new(box_)
            .can_request_focus(false)
            .on_focus_change(move |app, value| self.set_state(app, |s| s.focused = value))
            .into_widget()
    }
}
/// Inline search for a list or a table. A hint sits in the trailing slot while
/// the field is empty and gives way to the clear button once there is something
/// to clear; Escape clears first and dismisses only once nothing is left.
#[derive(Clone)]
pub struct SearchField {
    pub field: TextField,
    /// The current query, when the host holds it; it decides hint or clear.
    pub value: Option<String>,
    pub hint: Option<WidgetRef>,
    pub on_dismiss: Option<super::Callback>,
}
impl std::fmt::Debug for SearchField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchField")
            .field("value", &self.value)
            .finish()
    }
}
impl SearchField {
    pub fn new(controller: Handle<TextEditingController>) -> Self {
        let mut field = TextField::new()
            .controller(controller)
            .placeholder("Search");
        field.prefix = Some(Text::new("⌕").into_widget());
        Self {
            field,
            value: None,
            hint: None,
            on_dismiss: None,
        }
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, String) + 'static) -> Self {
        self.field = self.field.on_changed(callback);
        self
    }
}
impl StatelessWidget for SearchField {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut field = self.field.clone();
        field.prefix = field.prefix.map(|prefix| {
            Padding::new(EdgeInsetsGeometry::only(v.spacing_2_5, 0.0, 0.0, 0.0))
                .child(prefix)
                .into_widget()
        });
        let empty = self.value.as_ref().is_some_and(String::is_empty);
        let Some(controller) = field.controller else {
            return field.into_widget();
        };
        let callback = field.on_changed.clone();
        let clear = Rc::new(move |app: &mut App| {
            controller.set_text(app, "");
            if let Some(callback) = &callback {
                callback(app, String::new());
            }
        });
        field.suffix = if empty {
            self.hint.clone().map(|hint| {
                Padding::new(EdgeInsetsGeometry::only(0.0, 0.0, v.spacing_2, 0.0))
                    .child(hint)
                    .into_widget()
            })
        } else {
            let clear = clear.clone();
            Some(
                super::Button::new("×")
                    .variant(crate::ControlVariant::Plain)
                    .size(ControlSize::Tiny)
                    .on_pressed(move |app| clear(app))
                    .into_widget(),
            )
        };
        let dismiss = self.on_dismiss.clone();
        Focus::new(field)
            .can_request_focus(false)
            .on_key_event(Rc::new(move |app, _, event| {
                if matches!(event, inset_services::KeyEvent::Up(_))
                    || event.logical_key() != inset_services::LogicalKeyboardKey::ESCAPE
                {
                    return KeyEventResult::Ignored;
                }
                if empty {
                    if let Some(dismiss) = &dismiss {
                        dismiss(app);
                    }
                } else {
                    clear(app);
                }
                KeyEventResult::Handled
            }))
            .into_widget()
    }
}
