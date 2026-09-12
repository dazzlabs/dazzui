use super::{Card, CardVariant};
use crate::{ControlSize, ControlState, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_painting::{BorderRadiusGeometry, BoxDecoration, EdgeInsetsGeometry};
use inset_rendering::{CrossAxisAlignment, MainAxisSize};
use inset_widgets::*;

#[derive(Debug, Clone, Default)]
pub struct Divider {
    pub vertical: bool,
    pub muted: bool,
}
impl Divider {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn vertical() -> Self {
        Self {
            vertical: true,
            muted: false,
        }
    }
}
impl StatelessWidget for Divider {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let c = Container::new().color(if self.muted {
            v.color_border_strong
        } else {
            v.color_border
        });
        if self.vertical {
            c.width(v.stroke_hairline).into_widget()
        } else {
            c.height(v.stroke_hairline).into_widget()
        }
    }
}
#[derive(Debug, Clone)]
pub struct SectionLabel {
    pub label: String,
    pub faint: bool,
    pub tint: Option<Tint>,
}
impl SectionLabel {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            faint: false,
            tint: None,
        }
    }
}
impl StatelessWidget for SectionLabel {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let color = self
            .tint
            .map(|t| {
                v.control_colors(ControlVariant::Plain, t, ControlState::Normal)
                    .content
            })
            .unwrap_or(if self.faint {
                v.color_content_faint
            } else {
                v.color_content_subtle
            });
        Text::new(&self.label)
            .style(v.label_small.text_style().color(color))
            .into_widget()
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub enum KeyCapVariant {
    Hint,
    Strong,
    #[default]
    Key,
}
#[derive(Debug, Clone)]
pub struct KeyCap {
    pub label: String,
    pub variant: KeyCapVariant,
    pub size: ControlSize,
}
impl KeyCap {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: KeyCapVariant::Key,
            size: ControlSize::Small,
        }
    }
}
impl StatelessWidget for KeyCap {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut face = v.label_strong;
        face.size = if self.size == ControlSize::Large {
            v.label_medium.size
        } else {
            v.label_small.size
        };
        face.weight = v.label_medium.weight;
        let text = Text::new(&self.label).style(face.text_style().color(match self.variant {
            KeyCapVariant::Hint => v.color_content_faint,
            KeyCapVariant::Strong => v.color_content_subtle,
            KeyCapVariant::Key => v.color_content,
        }));
        if matches!(self.variant, KeyCapVariant::Key) {
            Container::new()
                .height(v.spacing_5 + v.spacing_0_5)
                .padding(EdgeInsetsGeometry::symmetric(0.0, v.spacing_2_5))
                .decoration(
                    BoxDecoration::new()
                        .color(v.color_surface_inset)
                        .border_radius(BorderRadiusGeometry::circular(v.radius_tiny)),
                )
                .child(Center::new().width_factor(1.0).child(text))
                .into_widget()
        } else {
            text.into_widget()
        }
    }
}
#[derive(Debug, Clone)]
pub struct Avatar {
    pub name: String,
    pub size: ControlSize,
    pub tint: Tint,
    pub square: bool,
    pub image: Option<WidgetRef>,
}
impl Avatar {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size: ControlSize::Medium,
            tint: Tint::Primary,
            square: false,
            image: None,
        }
    }
    pub fn initials(&self) -> String {
        let words: Vec<_> = self.name.split_whitespace().collect();
        let mut initials = String::new();
        if let Some(c) = words.first().and_then(|s| s.chars().next()) {
            initials.extend(c.to_uppercase());
        }
        if words.len() > 1
            && let Some(c) = words.last().and_then(|s| s.chars().next())
        {
            initials.extend(c.to_uppercase());
        }
        initials
    }
}
impl StatelessWidget for Avatar {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let c = v.control_colors(ControlVariant::Tinted, self.tint, ControlState::Normal);
        let (size, face) = match self.size {
            ControlSize::Large => (v.spacing_9, v.label_large),
            ControlSize::Medium => (v.spacing_7, v.label_medium),
            _ => (v.spacing_5, v.caption_small),
        };
        let radius = BorderRadiusGeometry::circular(if self.square {
            v.radius_medium
        } else {
            v.radius_full
        });
        let child = self.image.clone().unwrap_or_else(|| {
            Center::new()
                .child(Text::new(self.initials()).style(face.text_style().color(c.content)))
                .into_widget()
        });
        ClipRRect::new()
            .border_radius(radius)
            .child(
                Container::new()
                    .width(size)
                    .height(size)
                    .color(c.surface)
                    .child(child),
            )
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Callout {
    pub title: String,
    pub message: String,
    pub tint: Tint,
    pub actions: Vec<WidgetRef>,
}
impl Callout {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            tint: Tint::Info,
            actions: vec![],
        }
    }
}
impl StatelessWidget for Callout {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let title_color = v
            .control_colors(ControlVariant::Tinted, self.tint, ControlState::Hovered)
            .content;
        let content = v
            .control_colors(ControlVariant::Tinted, self.tint, ControlState::Normal)
            .content;
        let mut children = vec![
            Text::new(&self.title)
                .style(v.label_medium.text_style().color(title_color))
                .into_widget(),
        ];
        if !self.message.is_empty() {
            children.push(
                Text::new(&self.message)
                    .style(v.body_small.text_style().color(content))
                    .into_widget(),
            );
        }
        if !self.actions.is_empty() {
            children.push(
                Row::new()
                    .spacing(v.spacing_2)
                    .children(self.actions.clone())
                    .into_widget(),
            );
        }
        Card::new(
            Column::new()
                .main_axis_size(MainAxisSize::Min)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .spacing(v.spacing_1)
                .children(children),
        )
        .variant(CardVariant::Tinted)
        .tint(self.tint)
        .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct EmptyState {
    pub title: String,
    pub description: String,
    pub action: Option<WidgetRef>,
}
impl EmptyState {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            action: None,
        }
    }
}
impl StatelessWidget for EmptyState {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut children = vec![
            Text::new(&self.title)
                .style(v.title_medium.text_style())
                .into_widget(),
            Text::new(&self.description)
                .style(v.body_small.text_style().color(v.color_content_muted))
                .into_widget(),
        ];
        if let Some(action) = &self.action {
            children.push(action.clone());
        }
        Padding::new(EdgeInsetsGeometry::all(v.spacing_8))
            .child(
                Column::new()
                    .main_axis_size(MainAxisSize::Min)
                    .spacing(v.spacing_2)
                    .children(children),
            )
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Code {
    pub text: String,
    pub block: bool,
}
impl Code {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            block: false,
        }
    }
    pub fn block(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            block: true,
        }
    }
}
impl StatelessWidget for Code {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let text = Text::new(&self.text).style(
            v.body_small
                .text_style()
                .font_family("monospace")
                .color(v.color_content),
        );
        if self.block {
            Card::new(SingleChildScrollView::new().child(text))
                .variant(CardVariant::Sunken)
                .into_widget()
        } else {
            Container::new()
                .padding(EdgeInsetsGeometry::symmetric(v.spacing_0_5, v.spacing_1))
                .decoration(
                    BoxDecoration::new()
                        .color(v.color_surface_sunken)
                        .border_radius(BorderRadiusGeometry::circular(v.radius_tiny)),
                )
                .child(text)
                .into_widget()
        }
    }
}
