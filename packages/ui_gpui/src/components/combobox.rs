use std::rc::Rc;

use gpui::{
    App, ElementId, Entity, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, deferred, div, relative,
};

use crate::assets::icons;
use crate::components::Icon;
use crate::components::menu::{MenuItem, menu_item, menu_panel};
use crate::components::text_field::{
    TextBuffer, TextFieldSize, TextFieldState, editable, field_box,
};
use crate::prelude::*;
use crate::{
    ControlPart, ControlState, ControlVariant, SharedValueListener, Tint, TypeStyle, ValueListener,
    colors, latched, pressable,
};

/// One choosable option: the value a caller gets back, and the label the query
/// is matched against.
pub struct ComboboxOption {
    value: SharedString,
    label: SharedString,
    group: Option<SharedString>,
    disabled: bool,
}

impl ComboboxOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        ComboboxOption {
            value: value.into(),
            label: label.into(),
            group: None,
            disabled: false,
        }
    }

    /// The heading this option sits under. Options carrying one are grouped in
    /// the order the groups first appear; a list where none do is flat.
    pub fn group(mut self, group: impl Into<SharedString>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A choice out of a list too long to scroll: the reader types to narrow it,
/// and picks one of what is left.
///
/// It is a [`Select`](crate::Select) with a query in front of it, and that is
/// the only question worth asking when choosing between them — a list of six
/// is a select, a list of six hundred is this. Both end in one of the options;
/// neither lets a reader enter something that is not on the list, which is
/// what separates them from an [`Autocomplete`].
///
/// The box is the [`TextField`](crate::TextField)'s and the list is the menu
/// panel, so a combobox in a form sits on the same rhythm as everything around
/// it and its list is the same panel a [`Menu`](crate::Menu) opens.
#[derive(IntoElement)]
pub struct Combobox {
    id: ElementId,
    /// The query. The host holds it, the way it holds a text field's.
    query: Entity<TextBuffer>,
    options: Vec<ComboboxOption>,
    value: Option<SharedString>,
    /// What the list says when the query matches nothing.
    empty_label: SharedString,
    size: TextFieldSize,
    state: TextFieldState,
    tint: Tint,
    disabled: bool,
    on_change: Option<ValueListener<SharedString>>,
}

impl Combobox {
    pub fn new(id: impl Into<ElementId>, query: &Entity<TextBuffer>) -> Self {
        Combobox {
            id: id.into(),
            query: query.clone(),
            options: Vec::new(),
            value: None,
            empty_label: "No matches".into(),
            size: TextFieldSize::Medium,
            state: TextFieldState::Default,
            tint: Tint::Primary,
            disabled: false,
            on_change: None,
        }
    }

    pub fn option(mut self, option: ComboboxOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn empty_label(mut self, label: impl Into<SharedString>) -> Self {
        self.empty_label = label.into();
        self
    }

    pub fn size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: TextFieldState) -> Self {
        self.state = state;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(listener));
        self
    }
}

/// The panel a combobox and an autocomplete both drop: the menu's own, with
/// the group headings and the empty line a query needs.
#[allow(clippy::too_many_arguments)]
fn filtered_list(
    rows: Vec<(SharedString, SharedString, Option<SharedString>, bool)>,
    chosen: Option<&SharedString>,
    query: &str,
    empty_label: SharedString,
    tint: Tint,
    dismiss: Rc<dyn Fn(&mut Window)>,
    on_pick: Option<SharedValueListener<SharedString>>,
    vars: &'static crate::ThemeVariables,
) -> gpui::Div {
    let needle = query.trim().to_lowercase();
    let kept: Vec<_> = rows
        .into_iter()
        .filter(|(_, label, _, _)| needle.is_empty() || label.to_lowercase().contains(&needle))
        .collect();

    let mut panel = menu_panel(vars);

    if kept.is_empty() {
        // A line of prose in a panel of rows, so it takes the row's inset and
        // the faint ink rather than looking like a row that cannot be picked.
        return panel.child(
            div()
                .py(vars.menu_item_padding)
                .px(vars.spacing_2)
                .type_style(vars.body_small)
                .text_color(vars.color_content_faint)
                .child(empty_label),
        );
    }

    let mut heading: Option<SharedString> = None;
    for (index, (value, label, group, disabled)) in kept.into_iter().enumerate() {
        // A group the query emptied is not a heading over nothing, which is
        // why the headings are drawn from what survived rather than from the
        // list as it was given.
        if group.is_some() && group != heading {
            heading = group.clone();
            if let Some(name) = group {
                panel = panel.child(
                    div()
                        .pt(vars.spacing_1_5)
                        .pb(vars.spacing_1)
                        .px(vars.spacing_2)
                        .type_style(vars.caption_small)
                        .font_weight(vars.label_small.weight)
                        .line_height(relative(1.0))
                        .text_color(vars.color_content_subtle)
                        .child(name),
                );
            }
        }

        let picked = chosen == Some(&value);
        let on_pick = on_pick.clone();
        panel = panel.child(menu_item(
            index,
            MenuItem::new(label)
                .checked(picked)
                .disabled(disabled)
                .on_select(move |window, cx| {
                    if let Some(listener) = on_pick.as_ref() {
                        listener(&value, window, cx);
                    }
                }),
            dismiss.clone(),
            tint,
            vars,
        ));
    }

    panel
}

/// The clear and the chevron: the field's own furniture rather than buttons on
/// it — no box at rest, the subtle ink, and the neutral wash only under the
/// pointer. A pair of drawn buttons inside a field would read as two more
/// controls in the form.
fn field_action(
    id: &'static str,
    icon: &'static str,
    on_press: impl Fn(&mut Window, &mut App) + 'static,
    vars: &'static crate::ThemeVariables,
) -> impl IntoElement {
    let on_press = Rc::new(on_press);

    pressable(id, true, move |state, _, _| {
        let washed = state.hovered || state.pressed;

        div()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .size(vars.spacing_5)
            .rounded(vars.radius_small)
            .bg(if washed {
                vars.control_color(
                    ControlVariant::Normal,
                    ControlPart::Surface,
                    Tint::Neutral,
                    ControlState::Hovered,
                )
            } else {
                colors::TRANSPARENT
            })
            .child(
                Icon::new(icon)
                    .size(vars.label_small.size)
                    .color(if washed {
                        vars.color_content
                    } else {
                        vars.color_content_subtle
                    }),
            )
            .when(true, |el| {
                let on_press = on_press.clone();
                el.on_click(move |_, window, cx| on_press(window, cx))
            })
            .into_any_element()
    })
}

impl RenderOnce for Combobox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Combobox {
            id,
            query,
            options,
            value,
            empty_label,
            size,
            state,
            tint,
            disabled,
            on_change,
        } = self;

        let metrics = vars.metrics(size.control());
        let tinted = state.tint(tint);
        let accent = vars.ramp(tinted).shade(600);
        let focused = query.read(cx).is_focused(window);
        let text = if disabled {
            vars.control_color(
                ControlVariant::Normal,
                ControlPart::Content,
                tinted,
                ControlState::Disabled,
            )
        } else if state == TextFieldState::Error {
            vars.ramp(tinted).shade(800)
        } else {
            vars.color_content
        };
        let rows: Vec<_> = options
            .into_iter()
            .map(|option| (option.value, option.label, option.group, option.disabled))
            .collect();
        let on_change: Option<SharedValueListener<SharedString>> = on_change.map(Rc::from);
        let needle = query.read(cx).value().to_string();

        latched::<bool>(id.clone(), move |open, _window, cx| {
            let is_open = open.get() && !disabled;

            div()
                .relative()
                .w_full()
                .child(
                    field_box(size, state, tinted, disabled, focused || is_open, false, cx)
                        .id("combobox-field")
                        .pr(vars.spacing_1)
                        .gap(vars.spacing_1)
                        .on_mouse_down(MouseButton::Left, {
                            let query = query.clone();
                            let open = open.clone();
                            move |_, window, cx| {
                                query.read(cx).focus(window);
                                open.set(true);
                                window.refresh();
                            }
                        })
                        .child(editable(
                            &query,
                            TypeStyle {
                                weight: vars.body_small.weight,
                                ..metrics.content
                            },
                            metrics.content.line_height,
                            text,
                            vars.color_content_faint,
                            accent,
                            accent.opacity(vars.focus_glow_alpha),
                            false,
                            cx,
                        ))
                        .when(!disabled, |el| {
                            el.child(field_action(
                                "combobox-chevron",
                                if is_open {
                                    icons::CHEVRON_UP_12_REGULAR
                                } else {
                                    icons::CHEVRON_DOWN_12_REGULAR
                                },
                                {
                                    let open = open.clone();
                                    move |window, _| {
                                        open.set(!open.get());
                                        window.refresh();
                                    }
                                },
                                vars,
                            ))
                        }),
                )
                .when(is_open, |el| {
                    let open = open.clone();
                    el.child(deferred(
                        filtered_list(
                            rows,
                            value.as_ref(),
                            &needle,
                            empty_label,
                            tint,
                            Rc::new({
                                let open = open.clone();
                                move |window: &mut Window| {
                                    open.set(false);
                                    window.refresh();
                                }
                            }),
                            on_change,
                            vars,
                        )
                        .absolute()
                        .top_full()
                        .left_0()
                        // The list is at least as wide as the field it dropped
                        // from, the way a select's is.
                        .w_full()
                        .mt(vars.spacing_1)
                        .occlude()
                        .on_mouse_down_out(move |_, window, _| {
                            open.set(false);
                            window.refresh();
                        }),
                    ))
                })
                .into_any_element()
        })
    }
}

/// A text field that suggests as you type.
///
/// The difference from a [`Combobox`] is what the value is: a combobox ends in
/// one of its options and nothing else; an autocomplete's value is whatever
/// was typed, and the list is a shortcut to a likely answer rather than the
/// set of allowed ones. A branch name, a search query, a tag that may not
/// exist yet.
///
/// So there is no chevron and no check column: nothing here is opened or
/// chosen, and the field is emptied the way any text field is.
#[derive(IntoElement)]
pub struct Autocomplete {
    id: ElementId,
    field: Entity<TextBuffer>,
    items: Vec<SharedString>,
    empty_label: SharedString,
    size: TextFieldSize,
    state: TextFieldState,
    tint: Tint,
    mono: bool,
    disabled: bool,
    on_change: Option<ValueListener<SharedString>>,
}

impl Autocomplete {
    pub fn new(id: impl Into<ElementId>, field: &Entity<TextBuffer>) -> Self {
        Autocomplete {
            id: id.into(),
            field: field.clone(),
            items: Vec::new(),
            empty_label: "No matches".into(),
            size: TextFieldSize::Medium,
            state: TextFieldState::Default,
            tint: Tint::Primary,
            mono: false,
            disabled: false,
            on_change: None,
        }
    }

    pub fn item(mut self, item: impl Into<SharedString>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn empty_label(mut self, label: impl Into<SharedString>) -> Self {
        self.empty_label = label.into();
        self
    }

    pub fn size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: TextFieldState) -> Self {
        self.state = state;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Set the mono face — for a path, a branch name, a key.
    pub fn mono(mut self, mono: bool) -> Self {
        self.mono = mono;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Autocomplete {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Autocomplete {
            id,
            field,
            items,
            empty_label,
            size,
            state,
            tint,
            mono,
            disabled,
            on_change,
        } = self;

        let metrics = vars.metrics(size.control());
        let tinted = state.tint(tint);
        let accent = vars.ramp(tinted).shade(600);
        let focused = field.read(cx).is_focused(window);
        let text = if disabled {
            vars.control_color(
                ControlVariant::Normal,
                ControlPart::Content,
                tinted,
                ControlState::Disabled,
            )
        } else if state == TextFieldState::Error {
            vars.ramp(tinted).shade(800)
        } else {
            vars.color_content
        };
        let face = TypeStyle {
            family: if mono {
                &["ui-monospace"]
            } else {
                metrics.content.family
            },
            weight: vars.body_small.weight,
            ..metrics.content
        };
        // The suggestions carry no group and no value of their own: what is
        // picked *is* the label.
        let rows: Vec<_> = items
            .into_iter()
            .map(|item| (item.clone(), item, None, false))
            .collect();
        let on_change: Option<SharedValueListener<SharedString>> = on_change.map(Rc::from);
        let needle = field.read(cx).value().to_string();

        latched::<bool>(id.clone(), move |open, _window, cx| {
            let is_open = open.get() && !disabled;

            div()
                .relative()
                .w_full()
                .child(
                    field_box(size, state, tinted, disabled, focused, false, cx)
                        .id("autocomplete-field")
                        .on_mouse_down(MouseButton::Left, {
                            let field = field.clone();
                            let open = open.clone();
                            move |_, window, cx| {
                                field.read(cx).focus(window);
                                open.set(true);
                                window.refresh();
                            }
                        })
                        .child(editable(
                            &field,
                            face,
                            metrics.content.line_height,
                            text,
                            vars.color_content_faint,
                            accent,
                            accent.opacity(vars.focus_glow_alpha),
                            false,
                            cx,
                        )),
                )
                .when(is_open, |el| {
                    let open = open.clone();
                    el.child(deferred(
                        filtered_list(
                            rows,
                            None,
                            &needle,
                            empty_label,
                            tint,
                            Rc::new({
                                let open = open.clone();
                                move |window: &mut Window| {
                                    open.set(false);
                                    window.refresh();
                                }
                            }),
                            on_change,
                            vars,
                        )
                        .absolute()
                        .top_full()
                        .left_0()
                        .w_full()
                        .mt(vars.spacing_1)
                        .occlude()
                        .on_mouse_down_out(move |_, window, _| {
                            open.set(false);
                            window.refresh();
                        }),
                    ))
                })
                .into_any_element()
        })
    }
}
