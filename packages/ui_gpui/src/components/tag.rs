use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::assets::icons;
use crate::components::{BadgeSize, Icon};
use crate::prelude::*;
use crate::{ActionListener, ControlPart, ControlState, ControlVariant, Tint, colors, pressable};

/// The recipes a tag can wear — the [`Badge`](crate::Badge)'s set, since it is
/// the badge's capsule.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TagVariant {
    Filled,
    #[default]
    Tinted,
    Outlined,
    /// The tag that sits on a grey card: paper fill, quiet ink.
    Raised,
}

impl TagVariant {
    pub const ALL: [TagVariant; 4] = [
        TagVariant::Filled,
        TagVariant::Tinted,
        TagVariant::Outlined,
        TagVariant::Raised,
    ];

    fn recipe(self) -> Option<ControlVariant> {
        match self {
            TagVariant::Filled => Some(ControlVariant::Filled),
            TagVariant::Tinted => Some(ControlVariant::Tinted),
            TagVariant::Outlined => Some(ControlVariant::Outlined),
            TagVariant::Raised => None,
        }
    }
}

/// A [`Badge`](crate::Badge) you can act on.
///
/// The two are one drawing and two components rather than one with an
/// `interactive` flag, because the difference is not cosmetic: a badge reads
/// its recipes at rest and deliberately stays out of the state machine, since
/// a label that moves under the pointer reads as something you can press. A
/// tag *is* something you can press, and taking the machine is the whole
/// reason it exists.
///
/// The dismiss button is its own target rather than a second job for the
/// label: a chip that removes itself when clicked has no way to be selected,
/// and one that does both from the same target removes things by accident.
///
/// The capsule's pads and type mirror `badge.rs` — the same box at the same
/// three densities. React shares them in one stylesheet; here they are written
/// twice, and the parity reference test is what holds the two together.
#[derive(IntoElement)]
pub struct Tag {
    id: ElementId,
    label: SharedString,
    size: BadgeSize,
    variant: TagVariant,
    tint: Tint,
    /// Latched. Setting it at all is what makes the label pressable, and a
    /// selected tag is drawn with the `tinted` recipe and the tint's edge
    /// whatever the variant says.
    selected: Option<bool>,
    disabled: bool,
    on_select: Option<ActionListener>,
    on_remove: Option<ActionListener>,
}

impl Tag {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Tag {
            id: id.into(),
            label: label.into(),
            size: BadgeSize::Medium,
            variant: TagVariant::Tinted,
            tint: Tint::Primary,
            selected: None,
            disabled: false,
            on_select: None,
            on_remove: None,
        }
    }

    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: TagVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(listener));
        self
    }

    /// Draws the dismiss button, and is what it does.
    pub fn on_remove(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_remove = Some(Box::new(listener));
        self
    }
}

/// The ✕ inside a tag, at rest and under the pointer.
///
/// Faint at rest and only resolved under the pointer: an ✕ at full strength on
/// every chip turns a list of tags into a list of buttons.
const DISMISS_REST: f32 = 0.6;
const DISMISS_DEAD: f32 = 0.4;
/// The pointer wash on the dismiss target, over whatever the capsule is. It is
/// an alpha of the capsule's own ink rather than a recipe, because it sits on
/// four variants across twelve themes and the only colour guaranteed to read
/// on all of them is the one the label is already set in.
const DISMISS_HOVER_ALPHA: f32 = 0.16;

impl RenderOnce for Tag {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Tag {
            id,
            label,
            size,
            variant,
            tint,
            selected,
            disabled,
            on_select,
            on_remove,
        } = self;
        let pressable_tag = selected.is_some();
        let drawn = if selected == Some(true) {
            TagVariant::Tinted
        } else {
            variant
        };

        // Two capsule densities on the chip corner; the type moves one step at
        // the top. Badge's, because this is Badge's capsule.
        let padding_block = match size {
            BadgeSize::Large => vars.spacing_1,
            _ => vars.spacing_0_5,
        };
        let padding_inline = match size {
            BadgeSize::Small => vars.spacing_1_5,
            _ => vars.spacing_2,
        };
        let font_size = match size {
            BadgeSize::Large => vars.label_medium.size,
            _ => vars.label_small.size,
        };

        let remove_id = ElementId::from(SharedString::from(format!("{id:?}-remove")));
        let on_remove: Option<crate::SharedActionListener> = on_remove.map(std::rc::Rc::from);
        let on_select: Option<crate::SharedActionListener> = on_select.map(std::rc::Rc::from);

        pressable(
            id.clone(),
            pressable_tag && !disabled,
            move |state, _, _| {
                // Only a pressable tag reads the state machine. One that merely
                // carries a dismiss button is still a label, and a wash that
                // answered the pointer anywhere on the capsule would promise a
                // target that is not there.
                // A tag that cannot be pressed is a label, and a label has no
                // disabled state — only the dismiss button inside it does, and it
                // flattens itself. So a static tag rests whatever `disabled` says,
                // which is how React draws it: it never joins the state machine.
                let phase = if pressable_tag {
                    state.control_state()
                } else {
                    ControlState::Normal
                };
                let recipe = |part: ControlPart| {
                    drawn
                        .recipe()
                        .map(|variant| vars.control_color(variant, part, tint, phase))
                };

                let surface = match drawn {
                    TagVariant::Raised => vars.color_surface,
                    TagVariant::Outlined => colors::TRANSPARENT,
                    _ => recipe(ControlPart::Surface).unwrap_or(colors::TRANSPARENT),
                };
                let content = match drawn {
                    TagVariant::Raised => vars.color_content_muted,
                    _ => recipe(ControlPart::Content).unwrap_or(vars.color_content),
                };
                // Selected is drawn twice over: the tinted fill the variant swaps
                // to, and this edge. One alone is not enough — a tinted tag that
                // was already tinted would look untouched, and an edge alone is a
                // pixel to spot in a row of forty.
                let border = if selected == Some(true) {
                    vars.control_color(
                        ControlVariant::Outlined,
                        ControlPart::Border,
                        tint,
                        ControlState::Normal,
                    )
                } else {
                    match drawn {
                        TagVariant::Outlined => {
                            recipe(ControlPart::Border).unwrap_or(colors::TRANSPARENT)
                        }
                        _ => colors::TRANSPARENT,
                    }
                };

                let dismiss = on_remove.clone();
                div()
                    .id(id)
                    .flex()
                    .flex_none()
                    .items_center()
                    .py(padding_block)
                    .px(padding_inline)
                    .rounded(vars.radius_tiny)
                    // A border on every variant rather than only on `outlined`, so
                    // that one does not gain a pixel per side and fall off the
                    // heights the others share.
                    .border(vars.spacing_px)
                    .bg(surface)
                    .text_color(content)
                    .border_color(border)
                    .type_style(
                        vars.label_strong
                            .size(font_size)
                            .weight(vars.label_medium.weight),
                    )
                    .line_height(relative(1.4))
                    .map(|el| {
                        if disabled {
                            el.cursor_not_allowed()
                        } else if pressable_tag {
                            el.cursor_pointer()
                        } else {
                            el
                        }
                    })
                    .when_some(on_select.as_ref().filter(|_| !disabled), |el, _| {
                        el.on_click({
                            let on_select = on_select.clone();
                            move |_, window, cx| {
                                if let Some(listener) = on_select.as_ref() {
                                    listener(window, cx);
                                }
                            }
                        })
                    })
                    .child(label)
                    .when_some(dismiss, |el, listener| {
                        el.child(pressable(remove_id.clone(), !disabled, {
                            let remove_id = remove_id.clone();
                            move |dismiss_state, _, _| {
                                let opacity = if disabled {
                                    DISMISS_DEAD
                                } else if dismiss_state.hovered {
                                    1.
                                } else {
                                    DISMISS_REST
                                };
                                div()
                                    .id(remove_id)
                                    .flex()
                                    .flex_none()
                                    .items_center()
                                    .justify_center()
                                    .ml(vars.spacing_1)
                                    .rounded(vars.radius_full)
                                    .when(dismiss_state.hovered && !disabled, |el| {
                                        el.bg(gpui::Hsla {
                                            a: DISMISS_HOVER_ALPHA,
                                            ..content
                                        })
                                    })
                                    .child(
                                        Icon::new(icons::DISMISS_12_REGULAR)
                                            .size(vars.spacing_3)
                                            .color(gpui::Hsla {
                                                a: content.a * opacity,
                                                ..content
                                            }),
                                    )
                                    .on_click({
                                        let listener = listener.clone();
                                        move |_, window, cx| listener(window, cx)
                                    })
                                    .into_any_element()
                            }
                        }))
                    })
                    .into_any_element()
            },
        )
    }
}
