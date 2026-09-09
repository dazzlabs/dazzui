use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    relative,
};

use crate::prelude::*;
use crate::{ControlPart, ControlState, ControlVariant, Tint};

/// The three avatar sizes.
///
/// Off the 4px grid rather than off the control profile: an avatar is not a
/// control, and these are the steps a row, a header and a card actually ask
/// for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// A person is a circle and a thing is a square — a project, a repository.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarShape {
    #[default]
    Circle,
    Square,
}

/// The picture that stands for a person or a project.
///
/// GPUI has no image decoder in the tests and no network here, so what this
/// draws is the fallback: the initials of the name, on the `tinted` recipe's
/// resting pair. That is the case the React component exists for anyway —
/// the initials are what is drawn until a picture has decoded and instead of
/// it when it never does.
///
/// It is not interactive and stays off [`pressable`](crate::pressable): an
/// avatar that lit up under the pointer would read as something you can press.
#[derive(IntoElement)]
pub struct Avatar {
    /// Who or what this is. It is where the initials come from.
    name: SharedString,
    /// Drawn in place of the initials — a glyph, usually.
    fallback: Option<AnyElement>,
    size: AvatarSize,
    shape: AvatarShape,
    tint: Tint,
}

impl Avatar {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Avatar {
            name: name.into(),
            fallback: None,
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
            tint: Tint::Primary,
        }
    }

    pub fn fallback(mut self, fallback: impl IntoElement) -> Self {
        self.fallback = Some(fallback.into_any_element());
        self
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// The first letter of the first word and of the last, which is the pair a
    /// reader actually recognises. One word gives one letter rather than two
    /// off the same word, since "Ac" for Acme reads as a truncation rather
    /// than as a monogram.
    pub fn initials(name: &str) -> String {
        let words: Vec<&str> = name.split_whitespace().collect();
        let mut initials = String::new();
        if let Some(first) = words.first().and_then(|word| word.chars().next()) {
            initials.push(first);
        }
        if words.len() > 1
            && let Some(last) = words.last().and_then(|word| word.chars().next())
        {
            initials.push(last);
        }
        initials.to_uppercase()
    }

    /// The box, per size. `AvatarGroup` measures its overlap off the same
    /// three steps, and has no other way to know them.
    pub(crate) fn dimension(vars: &crate::ThemeVariables, size: AvatarSize) -> gpui::Pixels {
        match size {
            AvatarSize::Small => vars.spacing_5,
            AvatarSize::Medium => vars.spacing_7,
            AvatarSize::Large => vars.spacing_9,
        }
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let dimension = Avatar::dimension(vars, self.size);

        // The type moves one step of the scale per size — 11, 12, 13 — rather
        // than tracking the box, which grows nearly twice as fast: two letters
        // at a third of a 36px circle would fill it edge to edge.
        let face = match self.size {
            AvatarSize::Small => vars.caption_small,
            AvatarSize::Medium => vars.label_medium,
            AvatarSize::Large => vars.label_large,
        };

        let surface = vars.control_color(
            ControlVariant::Tinted,
            ControlPart::Surface,
            self.tint,
            ControlState::Normal,
        );
        let content = vars.control_color(
            ControlVariant::Tinted,
            ControlPart::Content,
            self.tint,
            ControlState::Normal,
        );

        div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(dimension)
            // The square takes the container corner rather than the control
            // corner: under Bright, where the control corner is a pill, a
            // control-cornered square would go round and stop being the shape
            // that says "project".
            .map(|el| match self.shape {
                AvatarShape::Circle => el.rounded(vars.radius_full),
                AvatarShape::Square => el.rounded(vars.radius_medium),
            })
            .overflow_hidden()
            .bg(surface)
            .text_color(content)
            .type_style(face)
            .font_family(vars.label_strong.font_family())
            .font_weight(vars.label_strong.weight)
            .line_height(relative(1.0))
            .map(|el| match self.fallback {
                Some(fallback) => el.child(fallback),
                None => el.child(SharedString::from(Avatar::initials(&self.name))),
            })
    }
}

/// A run of avatars, overlapped.
///
/// Each one is cut out of the one before it, so the stack reads as a group
/// rather than as a smear. The cut is drawn as a ring the avatar sits inside
/// rather than a border on it, so it costs the run nothing: a border would
/// grow every avatar by three pixels.
#[derive(IntoElement)]
pub struct AvatarGroup {
    avatars: Vec<Avatar>,
    /// How many to draw before the rest become a count.
    max: Option<usize>,
    /// The size the avatars inside are drawn at. The group does not set it on
    /// them — each one carries its own — but the overlap, the ring and the
    /// count chip are measured from it.
    size: AvatarSize,
    shape: AvatarShape,
    /// The colour each avatar is cut out of its neighbour with. Defaults to
    /// the paper a group usually sits on; a group on a card passes the card's
    /// own surface.
    ring: Option<gpui::Hsla>,
}

impl Default for AvatarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl AvatarGroup {
    pub fn new() -> Self {
        AvatarGroup {
            avatars: Vec::new(),
            max: None,
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
            ring: None,
        }
    }

    pub fn avatar(mut self, avatar: Avatar) -> Self {
        self.avatars.push(avatar);
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn ring(mut self, ring: gpui::Hsla) -> Self {
        self.ring = Some(ring);
        self
    }
}

impl RenderOnce for AvatarGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let dimension = Avatar::dimension(vars, self.size);
        let cut = self.ring.unwrap_or(vars.color_surface);

        // A third of the box, on every size — a fixed overlap would swallow a
        // small avatar and barely touch a large one.
        let overlap = match self.size {
            AvatarSize::Small => vars.spacing_1_5,
            AvatarSize::Medium => vars.spacing_2,
            AvatarSize::Large => vars.spacing_3,
        };

        let shown = self
            .max
            .unwrap_or(self.avatars.len())
            .min(self.avatars.len());
        let hidden = self.avatars.len() - shown;
        let mut avatars = self.avatars;
        avatars.truncate(shown);

        let radius = match self.shape {
            AvatarShape::Circle => vars.radius_full,
            AvatarShape::Square => vars.radius_medium,
        };

        // The ring is a box around each one rather than a border on it: the
        // ring is drawn *outside* the avatar's own box, and the negative
        // margin takes back exactly the overlap plus the two ring widths, so
        // the run measures the same as it does in the browser.
        let cut_out = move |child: AnyElement, first: bool| {
            div()
                .flex_none()
                .p(vars.stroke_control)
                .rounded(radius + vars.stroke_control)
                .bg(cut)
                .when(!first, |el| {
                    el.ml(-(overlap + vars.stroke_control + vars.stroke_control))
                })
                .child(child)
        };

        div()
            .flex()
            .items_center()
            .children(
                avatars
                    .into_iter()
                    .enumerate()
                    .map(|(index, avatar)| cut_out(avatar.into_any_element(), index == 0)),
            )
            .when(hidden > 0, |el| {
                el.child(cut_out(
                    // The overflow count wears the same box in the neutral
                    // groove rather than a tint: it is a number, not a person.
                    div()
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .size(dimension)
                        .rounded(radius)
                        .bg(vars.color_surface_sunken)
                        .text_color(vars.color_content_muted)
                        .type_style(match self.size {
                            AvatarSize::Large => vars.label_medium,
                            _ => vars.caption_small,
                        })
                        .font_family(vars.label_strong.font_family())
                        .font_weight(vars.label_strong.weight)
                        .line_height(relative(1.0))
                        .child(SharedString::from(format!("+{hidden}")))
                        .into_any_element(),
                    shown == 0,
                ))
            })
    }
}
