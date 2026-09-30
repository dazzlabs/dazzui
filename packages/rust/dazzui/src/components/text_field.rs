//! A line of text, or several, that a person writes — and the box every text
//! control wears.

use gpui::{
    App, BoxShadow, ElementId, Hsla, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, Styled, Window, div, point, prelude::FluentBuilder, px, relative,
};

use crate::components::form_field::FormFieldScope;
use crate::foundation::text_input::{OnEvent, OnText, TextInput, input_state};
use crate::foundation::{ControlColors, Size, State, Tint, Variant, ring_color};
use crate::generated::ThemeTokens;
use crate::theme::ActiveTheme;
use crate::tokens::Face;

/// Whether a text control is reporting a problem: React's `TextFieldState`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FieldState {
    #[default]
    Default,
    Error,
}

impl FieldState {
    pub fn parse(name: &str) -> Option<FieldState> {
        Some(match name {
            "default" => FieldState::Default,
            "error" => FieldState::Error,
            _ => return None,
        })
    }

    /// React's `tintFor`: an erroring control is re-tinted rather than given
    /// a second palette, so its border, wash and focus glow move together.
    pub fn tint(self, tint: Tint) -> Tint {
        match self {
            FieldState::Error => Tint::Danger,
            FieldState::Default => tint,
        }
    }
}

/// One state of the text controls' box, resolved.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldColors {
    pub surface: Hsla,
    pub border: Hsla,
    pub content: Hsla,
    /// The accent wash a focused field lifts behind: `0 0 0 --focus-width
    /// --focus-glow`.
    pub glow: Option<BoxShadow>,
}

/// The box the text controls share, and share deliberately — `text-field.css`
/// names five wearers of one rule: the field, the read-only value, the select
/// trigger, the number field's group and the combobox's. A select is a field
/// with a chevron, so the five drifting apart would be the bug rather than
/// the feature; this is that rule, written once.
///
/// It is a recessed card one step below the paper, lifting to the paper on
/// focus behind an accent border and a soft accent wash. That card→paper
/// move is the focus signal. An erroring box is re-tinted to `danger` rather
/// than given a palette of its own.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldBox {
    pub size: Size,
    pub tint: Tint,
    pub state: FieldState,
    /// Focused, or — for a select or a combobox — its list is open.
    pub focused: bool,
    pub disabled: bool,
    pub mono: bool,
    /// Grows with its text, rather than sitting at the control height.
    pub multiline: bool,
}

impl FieldBox {
    pub fn new(size: Size) -> Self {
        FieldBox {
            size,
            tint: Tint::Primary,
            state: FieldState::Default,
            focused: false,
            disabled: false,
            mono: false,
            multiline: false,
        }
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn state(mut self, state: FieldState) -> Self {
        self.state = state;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn mono(mut self, mono: bool) -> Self {
        self.mono = mono;
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// The colours for this state. The rules stack as the stylesheet orders
    /// them: focus lifts the box, an error repaints it over the lift (the
    /// glow stays), and disabled greys the surface and the ink over both.
    pub fn colors(&self, t: &ThemeTokens) -> FieldColors {
        let tint = self.state.tint(self.tint);
        let ramp = tint.ramp(t);
        let mut colors = FieldColors {
            surface: t.color_surface_muted.hsla(),
            border: t.color_border_strong.hsla(),
            content: t.color_content.hsla(),
            glow: None,
        };
        if self.focused && !self.disabled {
            colors.border = ramp.shade(600).hsla();
            colors.surface = t.color_surface.hsla();
            colors.glow = Some(glow(t, tint));
        }
        if self.state == FieldState::Error {
            colors.border = ramp.shade(600).hsla();
            colors.surface = ramp.shade(50).hsla();
            colors.content = ramp.shade(800).hsla();
        }
        if self.disabled {
            let disabled = ControlColors::resolve(t, Variant::Normal, tint, State::Disabled);
            colors.surface = disabled.surface;
            colors.content = disabled.content;
        }
        colors
    }

    /// The colours alone, on a box that brings its own geometry — the
    /// search field's, which is a different shape of the same card.
    pub fn paint<E: Styled>(&self, element: E, t: &ThemeTokens) -> E {
        let colors = self.colors(t);
        element
            .border(px(t.stroke_hairline))
            .border_color(colors.border)
            .bg(colors.surface)
            .text_color(colors.content)
            .shadow(colors.glow.into_iter().collect())
    }

    /// The whole box: geometry off the size profile, with two deliberate
    /// overrides — entered text is body weight, not label weight, and the
    /// box takes the field corner, a step above the buttons'.
    pub fn style<E: Styled>(&self, element: E, t: &ThemeTokens) -> E {
        let profile = self.size.profile(t);
        let content = if self.mono {
            profile.content.in_face(Face::Mono)
        } else {
            profile.content
        };
        let element = self
            .paint(element, t)
            .w_full()
            .flex()
            .px(px(t.spacing_3))
            .font_family(content.face.family())
            .text_size(px(content.size))
            .font_weight(t.body_small.font_weight());
        if self.multiline {
            // A box that grows takes the container corner — a theme that
            // makes controls pills would turn it into a lozenge — a floor
            // rather than a height, and real leading, since its text wraps.
            element
                .items_start()
                .min_h(px(t.spacing_20))
                .py(px(t.spacing_2_5))
                .rounded(px(t.radius_large))
                .line_height(relative(1.6))
        } else {
            element
                .items_center()
                .h(px(profile.size))
                .py(px(profile.padding_block))
                .rounded(px(t.control_field_radius))
                .line_height(px(content.line_height))
        }
    }
}

/// `--focus-glow` against a tint: the wash a focused field lifts behind.
pub fn glow(t: &ThemeTokens, tint: Tint) -> BoxShadow {
    BoxShadow {
        color: t.focus_glow.resolve(tint.ramp(t)),
        offset: point(px(0.), px(0.)),
        blur_radius: px(0.),
        spread_radius: px(t.focus_width),
    }
}

/// An id for a part of a component, under the component's own.
pub(crate) fn part_id(id: &ElementId, part: &'static str) -> ElementId {
    ElementId::NamedChild(Box::new(id.clone()), part.into())
}

/// What a text control's placeholder is set in: `::placeholder`.
pub fn placeholder_color(t: &ThemeTokens) -> Hsla {
    t.color_content_faint.hsla()
}

/// The selection's wash. The browser paints the system's highlight, which no
/// token names; the system accent at the focus ring's strength is the
/// nearest thing the theme holds.
pub fn selection_color(t: &ThemeTokens) -> Hsla {
    ring_color(t)
}

/// A text field. `multiline` is the axis rather than a second component: the
/// box, the states and the tinting are one design either way, and only the
/// corner, the leading and how the height is arrived at differ.
///
/// Controlled with [`value`](Self::value) and [`on_change`](Self::on_change),
/// or left to keep its own text from [`default_value`](Self::default_value).
#[derive(IntoElement)]
pub struct TextField {
    id: ElementId,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    placeholder: SharedString,
    password: bool,
    size: Size,
    state: FieldState,
    tint: Tint,
    mono: bool,
    read_only: bool,
    disabled: bool,
    multiline: bool,
    rows: usize,
    on_change: Option<OnText>,
    on_submit: Option<OnText>,
    on_blur: Option<OnEvent>,
}

impl TextField {
    pub fn new(id: impl Into<ElementId>) -> Self {
        TextField {
            id: id.into(),
            value: None,
            default_value: None,
            placeholder: SharedString::default(),
            password: false,
            size: Size::Medium,
            state: FieldState::Default,
            tint: Tint::Primary,
            mono: false,
            read_only: false,
            disabled: false,
            multiline: false,
            // A `<textarea>`'s own default.
            rows: 2,
            on_change: None,
            on_submit: None,
            on_blur: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// React's `type`. Only `password` changes what is drawn — each
    /// character a bullet — so it is the one kept; `email`, `url` and the
    /// rest only hint a keyboard GPUI does not have.
    pub fn input_type(mut self, input_type: &str) -> Self {
        self.password = input_type == "password";
        self
    }

    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    /// Accepted for React's shape. A required field draws nothing of its own
    /// there either; the constraint is the form's.
    pub fn required(self, _required: bool) -> Self {
        self
    }

    /// Accepted for React's shape; GPUI 0.2 has no accessibility tree to
    /// name the field in.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    /// `small`, `medium` or `large`; `medium` by default.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: FieldState) -> Self {
        self.state = state;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Sets the mono face — for a key, an ID, or a URL.
    pub fn mono(mut self, mono: bool) -> Self {
        self.mono = mono;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// The rows a multiline field is tall at least.
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn on_submit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn on_blur(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_blur = Some(std::rc::Rc::new(handler));
        self
    }
}

impl RenderOnce for TextField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        // A field inside an erroring `FormField` is marked invalid by it, as
        // Base UI's `Field` marks its control.
        let state = if crate::foundation::scoped::<FormFieldScope>(cx).is_some_and(|f| f.invalid) {
            FieldState::Error
        } else {
            self.state
        };
        let input = input_state(self.id.clone(), window, cx);
        let focused = input.read(cx).is_focused(window);
        let chrome = FieldBox::new(self.size)
            .tint(self.tint)
            .state(state)
            .focused(focused)
            .disabled(self.disabled)
            .mono(self.mono)
            .multiline(self.multiline);

        let mut text = TextInput::new(self.id)
            .state(input.clone())
            .placeholder(self.placeholder)
            .placeholder_color(placeholder_color(t))
            .selection_color(selection_color(t))
            .masked(self.password)
            .read_only(self.read_only)
            .disabled(self.disabled)
            .multiline(self.multiline)
            .rows(self.rows);
        if let Some(value) = self.value {
            text = text.value(value);
        }
        if let Some(value) = self.default_value {
            text = text.default_value(value);
        }
        if let Some(on_change) = self.on_change {
            text = text.on_change(move |value, window, cx| on_change(value, window, cx));
        }
        if let Some(on_submit) = self.on_submit {
            text = text.on_submit(move |value, window, cx| on_submit(value, window, cx));
        }
        if let Some(on_blur) = self.on_blur {
            text = text.on_blur(move |window, cx| on_blur(window, cx));
        }

        let disabled = self.disabled;
        chrome
            .style(div(), t)
            .map(|this| {
                if disabled {
                    this.cursor_not_allowed()
                } else {
                    this.cursor_text()
                }
            })
            // A press in the padding is a press in the field.
            .when(!disabled, |this| {
                let input = input.downgrade();
                this.on_mouse_down(MouseButton::Left, move |event, window, cx| {
                    input
                        .update(cx, |input, cx| input.mouse_down(event, window, cx))
                        .ok();
                })
            })
            .child(text)
    }
}
