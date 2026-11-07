//! Text that goes somewhere.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::components::icon::{Icon, icon_slot};
use crate::foundation::{Tint, focusable};
use crate::glyphs;
use crate::theme::ActiveTheme;

/// When the rule under the label is drawn. `hover` is the default because a
/// page of prose with an underline on every link reads as struck through.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LinkUnderline {
    #[default]
    Hover,
    Always,
    None,
}

impl LinkUnderline {
    pub fn parse(name: &str) -> Option<LinkUnderline> {
        Some(match name {
            "hover" => LinkUnderline::Hover,
            "always" => LinkUnderline::Always,
            "none" => LinkUnderline::None,
            _ => return None,
        })
    }
}

type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A link: `plain`'s content recipe for its colour, a grade darker on hover,
/// and none of a button's box. The recipe's surface is deliberately not
/// painted — a wash behind a run of text would ragged the line.
#[derive(IntoElement)]
pub struct Link {
    id: ElementId,
    href: Option<SharedString>,
    tint: Tint,
    underline: LinkUnderline,
    external: bool,
    children: Vec<AnyElement>,
    on_click: Option<OnClick>,
}

impl Link {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Link {
            id: id.into(),
            href: None,
            tint: Tint::Primary,
            underline: LinkUnderline::Hover,
            external: false,
            children: Vec::new(),
            on_click: None,
        }
    }

    /// Where it goes. A link with no `on_click` opens an absolute URL in the
    /// system's browser; anything else is the app's to route.
    pub fn href(mut self, href: impl Into<SharedString>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn underline(mut self, underline: LinkUnderline) -> Self {
        self.underline = underline;
        self
    }

    /// Opens away from this window, and draws the arrow that says so.
    pub fn external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Link {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for Link {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let ramp = self.tint.ramp(t);
        let focus = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let hovered = t.control_color_plain_content_hovered.resolve(ramp);
        let pressed = t.control_color_plain_content_pressed.resolve(ramp);
        let underline = self.underline;
        let text_style = window.text_style();
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let font = window.text_system().resolve_font(&text_style.font());
        let height = window.text_system().ascent(font, font_size).abs().round()
            + window.text_system().descent(font, font_size).abs().round();

        let label = div()
            .flex()
            .when(underline == LinkUnderline::Always, |this| this.underline());
        let mut link = div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .when(
                crate::foundation::scoped::<crate::foundation::style::InlineContext>(cx).is_some(),
                |this| this.h(height),
            )
            .cursor_pointer()
            .rounded(px(t.radius_tiny))
            .text_color(t.control_color_plain_content_normal.resolve(ramp))
            .hover(move |style| {
                let style = style.text_color(hovered);
                if underline == LinkUnderline::Hover {
                    style.underline()
                } else {
                    style
                }
            })
            .active(move |style| style.text_color(pressed));
        link = focusable(link, &focus, false, window, cx);

        let href = self.href;
        if let Some(on_click) = self.on_click {
            link = link.on_click(on_click);
        } else if let Some(href) = href.filter(|href| href.contains("://")) {
            link = link.on_click(move |_, _, cx| cx.open_url(&href));
        }

        // The arrow rides with the text and is never underlined — it is a
        // glyph, not a run.
        let arrow = self.external.then(|| {
            icon_slot(
                t.icon_small,
                div()
                    .flex()
                    .ml(px(t.spacing_0_5))
                    .child(Icon::new(glyphs::ARROW_UP_RIGHT_12_REGULAR)),
            )
        });
        // The underline on hover is set on the link's own text style, which
        // the label inherits.
        link.child(label.children(self.children)).children(arrow)
    }
}
