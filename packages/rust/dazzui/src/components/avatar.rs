//! The picture that stands for a person or a project.

use gpui::{
    AnyElement, App, BoxShadow, Hsla, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    StyledImage, Window, div, img, point, prelude::FluentBuilder, px,
};

use crate::foundation::{Size, Tint, provide, scoped};
use crate::theme::ActiveTheme;

/// A person is a circle and a thing is a square — a project, a repository.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AvatarShape {
    #[default]
    Circle,
    Square,
}

impl AvatarShape {
    pub fn parse(name: &str) -> Option<AvatarShape> {
        Some(match name {
            "circle" => AvatarShape::Circle,
            "square" => AvatarShape::Square,
            _ => return None,
        })
    }
}

/// The ring an [`AvatarGroup`] cuts each avatar out of its neighbour with —
/// React's `--avatar-ring`, which a group on a card re-points.
#[derive(Clone, Copy, Debug, PartialEq)]
struct AvatarRing(Hsla);

/// The first letter of the first word and of the last. One word gives one
/// letter: "Ac" for Acme reads as a truncation rather than a monogram.
fn initials_of(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();
    let first = words.first().and_then(|word| word.chars().next());
    let last = (words.len() > 1)
        .then(|| words.last().and_then(|word| word.chars().next()))
        .flatten();
    first
        .into_iter()
        .chain(last)
        .flat_map(char::to_uppercase)
        .collect()
}

fn edge(size: Size, t: &crate::ThemeTokens) -> f32 {
    match size {
        Size::Tiny | Size::Small => t.spacing_5,
        Size::Medium => t.spacing_7,
        Size::Large => t.spacing_9,
    }
}

fn ring(color: Hsla, t: &crate::ThemeTokens) -> BoxShadow {
    BoxShadow {
        color,
        offset: point(px(0.), px(0.)),
        blur_radius: px(0.),
        spread_radius: px(t.stroke_control),
    }
}

/// An avatar: the picture, or — until it arrives, and instead of it when
/// it never does — the initials on the tint's resting chip colours. Not a
/// control, so nothing here answers the pointer.
#[derive(IntoElement)]
pub struct Avatar {
    src: Option<SharedString>,
    name: Option<SharedString>,
    fallback: Option<AnyElement>,
    size: Size,
    shape: AvatarShape,
    tint: Tint,
}

impl Avatar {
    pub fn new() -> Self {
        Avatar {
            src: None,
            name: None,
            fallback: None,
            size: Size::Medium,
            shape: AvatarShape::Circle,
            tint: Tint::Primary,
        }
    }

    /// The picture: a path or a URL, as GPUI's `img` reads one.
    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(src.into());
        self
    }

    /// Who or what this is; where the initials come from.
    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Drawn in place of the picture. Defaults to the initials of `name`.
    pub fn fallback(mut self, fallback: impl IntoElement) -> Self {
        self.fallback = Some(fallback.into_any_element());
        self
    }

    pub fn size(mut self, size: Size) -> Self {
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
}

impl Default for Avatar {
    fn default() -> Self {
        Avatar::new()
    }
}

impl RenderOnce for Avatar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let ramp = self.tint.ramp(t);
        let edge = edge(self.size, t);
        let radius = match self.shape {
            AvatarShape::Circle => t.radius_full,
            AvatarShape::Square => t.radius_medium,
        };
        // Two letters read at a glance: one step of the type scale per size,
        // which grows far slower than the box.
        let font_size = match self.size {
            Size::Tiny | Size::Small => t.caption_small.size,
            Size::Medium => t.label_medium.size,
            Size::Large => t.label_large.size,
        };
        let fallback = self.fallback.unwrap_or_else(|| {
            let initials = self
                .name
                .as_ref()
                .map(|name| initials_of(name))
                .unwrap_or_default();
            SharedString::from(initials).into_any_element()
        });
        let group_ring = scoped::<AvatarRing>(cx);

        div()
            .relative()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(edge))
            .rounded(px(radius))
            .overflow_hidden()
            .when_some(group_ring, |this, AvatarRing(color)| {
                this.bg(color).shadow(vec![ring(color, t)])
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_full()
                    .rounded(px(radius))
                    .bg(t.control_color_tinted_surface_normal.resolve(ramp))
                    .text_color(t.control_color_tinted_content_normal.resolve(ramp))
                    .font_family(t.label_strong.face.family())
                    .font_weight(t.label_strong.font_weight())
                    .text_size(px(font_size))
                    .line_height(px(font_size))
                    .child(fallback),
            )
            .when_some(self.src, |this, src| {
                this.child(
                    img(src)
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .rounded(px(radius))
                        .object_fit(gpui::ObjectFit::Cover),
                )
            })
    }
}

/// A run of avatars, overlapped by a third of the box and each cut out of
/// the one before by a ring in the ground colour.
#[derive(IntoElement)]
pub struct AvatarGroup {
    max: Option<usize>,
    size: Size,
    ring: Option<Hsla>,
    children: Vec<AnyElement>,
}

impl AvatarGroup {
    pub fn new() -> Self {
        AvatarGroup {
            max: None,
            size: Size::Medium,
            ring: None,
            children: Vec::new(),
        }
    }

    /// How many to draw before the rest become a count.
    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    /// The size the avatars inside are drawn at. Each carries its own; the
    /// overlap and the count chip are measured from this.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// The ground the group sits on — `color.surface` unless it sits on a
    /// card of another colour.
    pub fn ring(mut self, color: impl Into<Hsla>) -> Self {
        self.ring = Some(color.into());
        self
    }
}

impl Default for AvatarGroup {
    fn default() -> Self {
        AvatarGroup::new()
    }
}

impl ParentElement for AvatarGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for AvatarGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let ring_color = self.ring.unwrap_or(t.color_surface.hsla());
        let overlap = match self.size {
            Size::Tiny | Size::Small => t.spacing_1_5,
            Size::Medium => t.spacing_2,
            Size::Large => t.spacing_3,
        };
        let mut children = self.children;
        let hidden = match self.max {
            Some(max) if children.len() > max => {
                let hidden = children.len() - max;
                children.truncate(max);
                hidden
            }
            _ => 0,
        };
        let rest = (hidden > 0).then(|| {
            let edge = edge(self.size, t);
            let font_size = match self.size {
                Size::Large => t.label_medium.size,
                _ => t.caption_small.size,
            };
            div()
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .size(px(edge))
                .rounded(px(t.radius_full))
                .bg(t.color_surface_sunken.hsla())
                .text_color(t.color_content_muted.hsla())
                .shadow(vec![ring(ring_color, t)])
                .font_family(t.label_strong.face.family())
                .font_weight(t.label_strong.font_weight())
                .text_size(px(font_size))
                .line_height(px(font_size))
                .child(SharedString::from(format!("+{hidden}")))
                .into_any_element()
        });
        let items = children
            .into_iter()
            .chain(rest)
            .enumerate()
            .map(move |(index, child)| {
                // Shrinkable, though it never shrinks: taffy 0.9 sizes a row
                // whose items pull back with a negative margin to nothing
                // unless their shrink factor is 1.
                div()
                    .flex()
                    .flex_none()
                    .when(index > 0, |this| this.flex_shrink().ml(px(-overlap)))
                    .child(child)
            });
        provide(
            AvatarRing(ring_color),
            div().flex().flex_none().items_center().children(items),
        )
    }
}
