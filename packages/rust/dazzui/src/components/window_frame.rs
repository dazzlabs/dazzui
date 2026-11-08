//! Presentation windows. Chrome metrics follow the platform's stylesheet;
//! surfaces, type, spacing and shadows come from the active design system.
use crate::components::collapsible::part_id;
use crate::{
    ActiveTheme, Length, ThemeTokens,
    foundation::{StyledExt, precise_text, provide, scoped},
    tokens::{TypeStyle, shadows},
};
use gpui::{
    AnyElement, App, BoxShadow, Component, ElementId, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, Styled, Window, div, point, prelude::FluentBuilder, px,
};
use std::{cell::Cell, rc::Rc};

mod chrome;
pub(crate) use chrome::SVGS;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowFramePlatform {
    #[default]
    Macos,
    Macos15,
    Windows,
    Gnome,
    Ubuntu,
    Kde,
    Omarchy,
}
impl WindowFramePlatform {
    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "macos" => Self::Macos,
            "macos15" => Self::Macos15,
            "windows" => Self::Windows,
            "gnome" => Self::Gnome,
            "ubuntu" => Self::Ubuntu,
            "kde" => Self::Kde,
            "omarchy" => Self::Omarchy,
            _ => return None,
        })
    }
    fn mac(self) -> bool {
        matches!(self, Self::Macos | Self::Macos15)
    }
    fn radius(self) -> f32 {
        match self {
            Self::Macos => 16.,
            Self::Macos15 => 10.,
            Self::Windows => 8.,
            Self::Gnome | Self::Ubuntu => 15.,
            Self::Kde => 5.,
            Self::Omarchy => 0.,
        }
    }
    fn height(self) -> f32 {
        match self {
            Self::Macos | Self::Windows => 32.,
            Self::Macos15 => 28.,
            Self::Gnome | Self::Ubuntu => 47.,
            Self::Kde => 30.,
            Self::Omarchy => 0.,
        }
    }
    fn header_inset(self) -> f32 {
        if self == Self::Kde { 6. } else { 12. }
    }
    fn title_style(self, t: &ThemeTokens) -> TypeStyle {
        match self {
            Self::Macos | Self::Macos15 => t.label_large,
            Self::Windows => TypeStyle {
                size: 12.,
                ..t.caption_medium
            },
            Self::Gnome | Self::Ubuntu => TypeStyle {
                weight: 700,
                ..t.title_small
            },
            _ => t.body_medium,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowControlState {
    #[default]
    Enabled,
    Disabled,
    Hidden,
}
impl WindowControlState {
    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "enabled" => Self::Enabled,
            "disabled" => Self::Disabled,
            "hidden" => Self::Hidden,
            _ => return None,
        })
    }
}
impl From<bool> for WindowControlState {
    fn from(value: bool) -> Self {
        if value { Self::Enabled } else { Self::Disabled }
    }
}
/// Missing entries are disabled. GNOME additionally hides unnamed minimize
/// and maximize controls in a partial set.
#[derive(Clone, Copy, Debug, Default)]
pub struct WindowFrameControls {
    pub close: Option<WindowControlState>,
    pub minimize: Option<WindowControlState>,
    pub maximize: Option<WindowControlState>,
}
impl WindowFrameControls {
    pub fn all() -> Self {
        Self {
            close: Some(WindowControlState::Enabled),
            minimize: Some(WindowControlState::Enabled),
            maximize: Some(WindowControlState::Enabled),
        }
    }
    pub fn close(mut self, state: impl Into<WindowControlState>) -> Self {
        self.close = Some(state.into());
        self
    }
    pub fn minimize(mut self, state: impl Into<WindowControlState>) -> Self {
        self.minimize = Some(state.into());
        self
    }
    pub fn maximize(mut self, state: impl Into<WindowControlState>) -> Self {
        self.maximize = Some(state.into());
        self
    }
    fn resolved(mut self, platform: WindowFramePlatform) -> Self {
        if platform == WindowFramePlatform::Gnome {
            self.minimize.get_or_insert(WindowControlState::Hidden);
            self.maximize.get_or_insert(WindowControlState::Hidden);
        }
        self
    }
    fn any(self, p: WindowFramePlatform) -> bool {
        [self.close, self.minimize, self.maximize]
            .into_iter()
            .any(|state| {
                if p.mac() {
                    state != Some(WindowControlState::Hidden)
                } else {
                    state == Some(WindowControlState::Enabled)
                }
            })
    }
}
type Handler = Rc<dyn Fn(&mut Window, &mut App)>;
#[derive(Clone, Default)]
struct Handlers {
    close: Option<Handler>,
    minimize: Option<Handler>,
    maximize: Option<Handler>,
}
#[derive(Default)]
pub struct WindowTitlebarSlots {
    leading: Option<AnyElement>,
    toolbar: Option<Vec<AnyElement>>,
    trailing: Option<AnyElement>,
    accessory: Option<AnyElement>,
}
impl WindowTitlebarSlots {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn leading(mut self, child: impl IntoElement) -> Self {
        self.leading = Some(precise_text(child.into_any_element()));
        self
    }
    pub fn toolbar(mut self, child: impl IntoElement) -> Self {
        self.toolbar
            .get_or_insert_with(Vec::new)
            .push(precise_text(child.into_any_element()));
        self
    }
    pub fn toolbar_elements(mut self, children: impl IntoIterator<Item = AnyElement>) -> Self {
        self.toolbar
            .get_or_insert_with(Vec::new)
            .extend(children.into_iter().map(precise_text));
        self
    }
    pub fn trailing(mut self, child: impl IntoElement) -> Self {
        self.trailing = Some(precise_text(child.into_any_element()));
        self
    }
    pub fn accessory(mut self, child: impl IntoElement) -> Self {
        self.accessory = Some(precise_text(child.into_any_element()));
        self
    }
    fn tall(&self) -> bool {
        self.toolbar.is_some() || self.accessory.is_some()
    }
}
#[derive(Clone)]
struct FrameContext {
    id: ElementId,
    platform: WindowFramePlatform,
    inactive: bool,
    draws: bool,
    handoff: Option<WindowFrameControls>,
    handlers: Handlers,
    main_controls: Rc<Cell<bool>>,
    floating: bool,
}
#[derive(Clone, Copy)]
struct MainBand;
#[derive(Clone, Copy)]
struct TopBand {
    height: f32,
}

#[derive(IntoElement)]
pub struct WindowFrame {
    id: ElementId,
    platform: WindowFramePlatform,
    title: Option<AnyElement>,
    subtitle: Option<AnyElement>,
    title_hidden: bool,
    titlebar: Option<WindowTitlebarSlots>,
    full_size_content: bool,
    controls: Option<Option<WindowFrameControls>>,
    inactive: bool,
    width: Option<Length>,
    height: Option<Length>,
    min_width: Option<Length>,
    max_width: Option<Length>,
    min_height: Option<Length>,
    max_height: Option<Length>,
    handlers: Handlers,
    children: Vec<AnyElement>,
}
impl WindowFrame {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            platform: WindowFramePlatform::Macos,
            title: None,
            subtitle: None,
            title_hidden: false,
            titlebar: Some(WindowTitlebarSlots::new()),
            full_size_content: false,
            controls: None,
            inactive: false,
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            handlers: Handlers::default(),
            children: Vec::new(),
        }
    }
    pub fn platform(mut self, value: WindowFramePlatform) -> Self {
        self.platform = value;
        self
    }
    pub fn title(mut self, value: impl IntoElement) -> Self {
        self.title = Some(precise_text(value.into_any_element()));
        self
    }
    pub fn subtitle(mut self, value: impl IntoElement) -> Self {
        self.subtitle = Some(precise_text(value.into_any_element()));
        self
    }
    pub fn title_hidden(mut self, value: bool) -> Self {
        self.title_hidden = value;
        self
    }
    pub fn titlebar(mut self, value: WindowTitlebarSlots) -> Self {
        self.titlebar = Some(value);
        self
    }
    pub fn without_titlebar(mut self) -> Self {
        self.titlebar = None;
        self
    }
    pub fn full_size_content(mut self, value: bool) -> Self {
        self.full_size_content = value;
        self
    }
    pub fn controls(mut self, value: WindowFrameControls) -> Self {
        self.controls = Some(Some(value));
        self
    }
    pub fn without_controls(mut self) -> Self {
        self.controls = Some(None);
        self
    }
    pub fn inactive(mut self, value: bool) -> Self {
        self.inactive = value;
        self
    }
    pub fn width(mut self, value: Length) -> Self {
        self.width = Some(value);
        self
    }
    pub fn height(mut self, value: Length) -> Self {
        self.height = Some(value);
        self
    }
    pub fn min_width(mut self, value: Length) -> Self {
        self.min_width = Some(value);
        self
    }
    pub fn max_width(mut self, value: Length) -> Self {
        self.max_width = Some(value);
        self
    }
    pub fn min_height(mut self, value: Length) -> Self {
        self.min_height = Some(value);
        self
    }
    pub fn max_height(mut self, value: Length) -> Self {
        self.max_height = Some(value);
        self
    }
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handlers.close = Some(Rc::new(handler));
        self
    }
    pub fn on_minimize(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handlers.minimize = Some(Rc::new(handler));
        self
    }
    pub fn on_maximize(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handlers.maximize = Some(Rc::new(handler));
        self
    }
}
impl ParentElement for WindowFrame {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements.into_iter().map(precise_text));
    }
}
impl RenderOnce for WindowFrame {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let p = self.platform;
        let band = if p == WindowFramePlatform::Omarchy {
            None
        } else {
            self.titlebar
        };
        // The implicit ALL is resolved before controlsOf in React: GNOME's
        // default therefore includes all three; a partial set hides unnamed ones.
        let controls = self
            .controls
            .unwrap_or_else(|| Some(WindowFrameControls::all()))
            .map(|controls| controls.resolved(p));
        let draws = p != WindowFramePlatform::Omarchy && controls.is_some();
        let height =
            if self.full_size_content || band.as_ref().is_some_and(WindowTitlebarSlots::tall) {
                t.frame_titlebar_size
            } else {
                p.height()
            };
        let context = FrameContext {
            id: self.id.clone(),
            platform: p,
            inactive: self.inactive,
            draws,
            handoff: if draws && band.is_none() && !p.mac() {
                controls
            } else {
                None
            },
            handlers: self.handlers.clone(),
            main_controls: Rc::new(Cell::new(false)),
            floating: band.is_none() && draws,
        };
        let mut frame = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_col()
            .overflow_hidden()
            .w(self
                .width
                .unwrap_or(Length::Px(t.frame_window_width))
                .definite())
            .max_w(self.max_width.unwrap_or(Length::Fraction(1.)).definite())
            .rounded(px(p.radius()))
            .bg(t.color_surface.hsla())
            .text_color(t.color_content.hsla())
            .type_style(t.body_medium)
            .when_some(self.height, |this, v| this.h(v.definite()))
            .when_some(self.min_width, |this, v| this.min_w(v.definite()))
            .when_some(self.min_height, |this, v| this.min_h(v.definite()))
            .when_some(self.max_height, |this, v| this.max_h(v.definite()));
        if p == WindowFramePlatform::Omarchy {
            frame = frame.border(px(2.)).border_color(if self.inactive {
                t.color_border_muted.hsla()
            } else {
                t.color_primary.shade(500).hsla()
            });
        } else {
            let mut shadow = vec![BoxShadow {
                color: t.color_border_strong.hsla(),
                offset: point(px(0.), px(0.)),
                blur_radius: px(0.),
                spread_radius: px(t.stroke_hairline),
            }];
            shadow.extend(shadows(if self.inactive {
                t.shadow_md
            } else {
                t.shadow_xl
            }));
            frame = crate::foundation::motion::shadow_transition(
                frame.shadow(shadow),
                part_id(&self.id, "shadow"),
                200.,
                [0.25, 0.1, 0.25, 1.],
                window,
                cx,
            );
        }
        if let Some(slots) = band {
            frame = frame.child(provide(
                TopBand { height },
                WindowTitlebar {
                    id: part_id(&self.id, "titlebar"),
                    title: self.title,
                    subtitle: self.subtitle,
                    title_hidden: self.title_hidden,
                    slots,
                    controls: Some(controls),
                    handlers: self.handlers,
                    overlay: self.full_size_content,
                },
            ));
        }
        let body = self.children.iter_mut().any(is_body);
        frame = frame.child(
            div()
                .flex()
                .flex_1()
                .flex_col()
                .min_w_0()
                .min_h_0()
                .when(p == WindowFramePlatform::Macos && self.inactive, |this| {
                    this.opacity(0.72)
                })
                .when(p == WindowFramePlatform::Omarchy && !body, |this| {
                    this.pt(px(20.))
                })
                .children(self.children),
        );
        if context.floating {
            frame = frame.child(provide(
                TopBand { height },
                chrome::FloatingControls {
                    controls: controls.unwrap(),
                },
            ));
        }
        provide(context, frame)
    }
}
fn is_body(element: &mut AnyElement) -> bool {
    if element.downcast_mut::<AnyElement>().is_some() {
        return is_body(element.downcast_mut::<AnyElement>().unwrap());
    }
    if element.transparent_child_mut().is_some() {
        return is_body(element.transparent_child_mut().unwrap());
    }
    element.downcast_mut::<Component<WindowBody>>().is_some()
}

#[derive(IntoElement)]
pub struct WindowTitlebar {
    id: ElementId,
    title: Option<AnyElement>,
    subtitle: Option<AnyElement>,
    title_hidden: bool,
    slots: WindowTitlebarSlots,
    controls: Option<Option<WindowFrameControls>>,
    handlers: Handlers,
    overlay: bool,
}
impl WindowTitlebar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            subtitle: None,
            title_hidden: false,
            slots: WindowTitlebarSlots::new(),
            controls: None,
            handlers: Handlers::default(),
            overlay: false,
        }
    }
    pub fn title(mut self, value: impl IntoElement) -> Self {
        self.title = Some(precise_text(value.into_any_element()));
        self
    }
    pub fn subtitle(mut self, value: impl IntoElement) -> Self {
        self.subtitle = Some(precise_text(value.into_any_element()));
        self
    }
    pub fn title_hidden(mut self, value: bool) -> Self {
        self.title_hidden = value;
        self
    }
    pub fn leading(mut self, value: impl IntoElement) -> Self {
        self.slots = self.slots.leading(value);
        self
    }
    pub fn toolbar(mut self, value: impl IntoElement) -> Self {
        self.slots = self.slots.toolbar(value);
        self
    }
    pub fn toolbar_elements(mut self, children: impl IntoIterator<Item = AnyElement>) -> Self {
        self.slots = self.slots.toolbar_elements(children);
        self
    }
    pub fn trailing(mut self, value: impl IntoElement) -> Self {
        self.slots = self.slots.trailing(value);
        self
    }
    pub fn accessory(mut self, value: impl IntoElement) -> Self {
        self.slots = self.slots.accessory(value);
        self
    }
    pub fn controls(mut self, value: WindowFrameControls) -> Self {
        self.controls = Some(Some(value));
        self
    }
    pub fn without_controls(mut self) -> Self {
        self.controls = Some(None);
        self
    }
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handlers.close = Some(Rc::new(handler));
        self
    }
    pub fn on_minimize(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handlers.minimize = Some(Rc::new(handler));
        self
    }
    pub fn on_maximize(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handlers.maximize = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for WindowTitlebar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let frame = scoped::<FrameContext>(cx);
        let p = frame
            .as_ref()
            .map_or(WindowFramePlatform::Macos, |v| v.platform);
        let top = scoped::<TopBand>(cx);
        let main = scoped::<MainBand>(cx).is_some() && top.is_none();
        let handed = self.controls.is_none() && frame.as_ref().is_some_and(|f| f.handoff.is_some());
        let carrying = self
            .controls
            .unwrap_or_else(|| {
                frame.as_ref().and_then(|f| f.handoff).or_else(|| {
                    if frame.as_ref().is_some_and(|f| f.draws) || p == WindowFramePlatform::Omarchy
                    {
                        None
                    } else {
                        Some(WindowFrameControls::all())
                    }
                })
            })
            .map(|c| c.resolved(p))
            .filter(|c| c.any(p));
        if main && carrying.is_some() && !p.mac() {
            if let Some(frame) = &frame {
                frame.main_controls.set(true);
            }
        }
        let mut handlers = self.handlers;
        if handed {
            if let Some(frame) = &frame {
                handlers.close = handlers.close.or_else(|| frame.handlers.close.clone());
                handlers.minimize = handlers
                    .minimize
                    .or_else(|| frame.handlers.minimize.clone());
                handlers.maximize = handlers
                    .maximize
                    .or_else(|| frame.handlers.maximize.clone());
            }
        }
        let height = top.map_or(t.frame_titlebar_size, |v| v.height);
        let mut bar = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(t.spacing_2))
            .h(px(height))
            .when(self.overlay, |bar| {
                bar.absolute().top_0().left_0().right_0()
            });
        if p == WindowFramePlatform::Kde {
            bar = bar
                .bg(t.color_surface_chrome.hsla())
                .border_b(px(t.stroke_hairline))
                .border_color(t.color_border.hsla());
        }
        if main {
            let caption = carrying.is_some() && p == WindowFramePlatform::Windows;
            let header = carrying.is_some() && !p.mac() && p != WindowFramePlatform::Windows;
            let lead = if p.mac() && frame.as_ref().is_some_and(|f| f.floating) {
                crate::components::sidebar::window_chrome_inset(cx).map_or(t.spacing_5, |icon| {
                    if icon {
                        t.spacing_5.max(80. - t.frame_sidebar_icon_width)
                    } else {
                        80.
                    }
                })
            } else {
                t.spacing_5
            };
            bar = bar.pl(px(lead)).pr(px(if caption {
                0.
            } else if header {
                p.header_inset()
            } else {
                t.spacing_5
            }));
        } else if p == WindowFramePlatform::Windows && top.is_some() {
            bar = bar.pl(px(t.spacing_4));
        }
        let lead = if main || p == WindowFramePlatform::Windows {
            0.
        } else if p.mac() {
            80.
        } else {
            p.header_inset()
        };
        if let Some(leading) = self.slots.leading {
            bar = bar.child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .min_w_0()
                    .gap(px(t.spacing_2))
                    .ml(px(lead))
                    .child(leading),
            );
        }
        if let Some(title) = self.title {
            let style = p.title_style(t);
            let ink = if frame.as_ref().is_some_and(|f| f.inactive) {
                t.color_content_subtle.hsla()
            } else {
                t.color_content.hsla()
            };
            let mut text = div()
                .flex()
                .items_baseline()
                .gap(px(t.spacing_2))
                .min_w_0()
                .font_family(style.face.family())
                .text_size(px(style.size))
                .font_weight(style.font_weight())
                .text_color(ink)
                .when(self.title_hidden, |mut this| {
                    this.style().visibility = Some(gpui::Visibility::Hidden);
                    this
                });
            text = if main || p != WindowFramePlatform::Windows {
                text.flex_1()
            } else {
                text.flex_shrink()
            };
            if !main && p != WindowFramePlatform::Windows {
                text = text.justify_center();
            }
            if top.is_some() {
                text = text.px(px(if p.mac() {
                    80.
                } else if matches!(
                    p,
                    WindowFramePlatform::Gnome
                        | WindowFramePlatform::Ubuntu
                        | WindowFramePlatform::Kde
                ) {
                    96.
                } else {
                    0.
                }));
            }
            text = text.child(
                div()
                    .flex()
                    .min_w_0()
                    .overflow_hidden()
                    .text_ellipsis()
                    .whitespace_nowrap()
                    .child(title),
            );
            if let Some(subtitle) = self.subtitle {
                text = text.child(
                    div()
                        .flex()
                        .min_w_0()
                        .flex_shrink()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .font_family(t.caption_small.face.family())
                        .text_size(px(t.caption_small.size))
                        .font_weight(t.caption_small.font_weight())
                        .text_color(t.color_content_subtle.hsla())
                        .child(subtitle),
                );
            }
            bar = bar.child(text);
        }
        if let Some(toolbar) = self.slots.toolbar {
            bar = bar.child(
                div()
                    .flex()
                    .items_center()
                    .min_w_0()
                    .gap(px(t.spacing_3))
                    .when(p == WindowFramePlatform::Windows && !main, |this| {
                        this.flex_1()
                    })
                    .children(toolbar),
            );
        }
        if let Some(trailing) = self.slots.trailing {
            bar = bar.child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .min_w_0()
                    .gap(px(t.spacing_2))
                    .ml_auto()
                    .mr(px(
                        if main || p.mac() || p == WindowFramePlatform::Windows {
                            0.
                        } else if p == WindowFramePlatform::Kde {
                            72.
                        } else {
                            108.
                        },
                    ))
                    .child(trailing),
            );
        }
        if let Some(accessory) = self.slots.accessory {
            bar = bar.child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(accessory),
            );
        }
        if let Some(controls) = carrying {
            bar = bar.child(chrome::Controls {
                id: part_id(&self.id, "controls"),
                platform: p,
                controls,
                handlers,
                inactive: frame.as_ref().is_some_and(|f| f.inactive),
                height,
                main,
            });
        }
        bar
    }
}

macro_rules! layout_part {
    ($name:ident,$build:expr) => {
        #[derive(IntoElement, Default)]
        pub struct $name {
            children: Vec<AnyElement>,
        }
        impl $name {
            pub fn new() -> Self {
                Self::default()
            }
        }
        impl ParentElement for $name {
            fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
                self.children.extend(elements.into_iter().map(precise_text));
            }
        }
        impl RenderOnce for $name {
            fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
                ($build)(self.children, cx)
            }
        }
    };
}
layout_part!(WindowBody, |children, cx: &mut App| {
    let _ = cx;
    div()
        .flex()
        .flex_1()
        .min_w_0()
        .min_h_0()
        .overflow_hidden()
        .children(children)
});
layout_part!(WindowMain, |children, cx: &mut App| {
    let _ = cx;
    provide(
        MainBand,
        div()
            .flex()
            .flex_1()
            .flex_col()
            .min_w_0()
            .overflow_hidden()
            .children(children),
    )
});
layout_part!(WindowContent, |children, cx: &mut App| {
    let _ = cx;
    div()
        .flex()
        .flex_1()
        .min_w_0()
        .min_h_0()
        .overflow_hidden()
        .children(children)
});
layout_part!(WindowFooter, |children, cx: &mut App| {
    let t = cx.theme();
    div()
        .flex()
        .flex_none()
        .items_center()
        .gap(px(t.spacing_4))
        .py(px(t.spacing_3))
        .px(px(t.spacing_5))
        .bg(t.color_surface_chrome.hsla())
        .border_t(px(t.stroke_hairline))
        .border_color(t.color_border.hsla())
        .children(children)
});
