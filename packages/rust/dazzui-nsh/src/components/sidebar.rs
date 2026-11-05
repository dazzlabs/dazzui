use crate::{Size, Theme, Tint, Variant, styles};
use nsh_core::prelude::*;
#[derive(Clone, Copy)]
struct SidebarContext {
    open: Signal<bool>,
    variant: Signal<SidebarVariant>,
}
#[derive(Clone, Copy)]
struct SidebarContentContext {
    collapsed: Memo<bool>,
    rail: Memo<bool>,
}
#[derive(Clone, PartialEq, Props)]
pub struct SidebarProviderProps {
    pub children: Prop<Children>,
}
#[component]
pub fn SidebarProvider(p: SidebarProviderProps) -> impl IntoElement {
    let open = signal(true);
    provide_context(SidebarContext {
        open,
        variant: signal(SidebarVariant::Sidebar),
    });
    nsh_core::element::on_key("mod+b", move |_| open.update(|v| *v = !*v));
    view().flex_row().w_full().h_full().child(p.children)
}
#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum SidebarCollapse {
    None,
    Icon,
    #[default]
    Offcanvas,
}
#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}
#[derive(Clone, PartialEq, Props)]
pub struct SidebarProps {
    #[prop(default)]
    pub collapsible: Prop<SidebarCollapse>,
    #[prop(default)]
    pub variant: Prop<SidebarVariant>,
    pub children: Prop<Children>,
}
#[component]
pub fn Sidebar(p: SidebarProps) -> impl IntoElement {
    let theme = Theme::current();
    let context = use_context::<SidebarContext>();
    let collapsed = computed(move || {
        p.collapsible.get() != SidebarCollapse::None && context.is_some_and(|c| !c.open.get())
    });
    provide_context(SidebarContentContext {
        collapsed,
        rail: computed(move || p.collapsible.get() == SidebarCollapse::None),
    });
    if let Some(context) = context {
        effect(move || context.variant.set(p.variant.get()));
    }
    view()
        .flex_shrink_0()
        .h_full()
        .overflow_hidden()
        .style_with(move |s| {
            let t = theme.tokens();
            let off = collapsed.get() && p.collapsible.get() == SidebarCollapse::Offcanvas;
            let width = if off {
                0.0
            } else if collapsed.get() {
                t.frame_sidebar_icon_width
            } else if p.collapsible.get() == SidebarCollapse::None {
                t.frame_rail_width
            } else {
                t.frame_sidebar_width
            };
            s.w(px(width))
                .style_when(off, |s| s.hidden())
                .style_when(p.variant.get() != SidebarVariant::Sidebar, |s| {
                    s.p(px(t.spacing_2))
                })
        })
        .child(
            view()
                .flex_1()
                .min_h(px(0.0))
                .style_with(move |s| {
                    let t = theme.tokens();
                    s.bg(t.color_surface_column.hsla())
                        .style_when(p.variant.get() == SidebarVariant::Sidebar, |s| {
                            s.border_r(px(t.stroke_hairline))
                        })
                        .border_color(t.color_border.hsla())
                        .style_when(p.variant.get() == SidebarVariant::Floating, |s| {
                            s.rounded(px(t.radius_large))
                                .border(px(t.stroke_hairline))
                                .shadow(crate::tokens::shadows(t.shadow_sm))
                        })
                })
                .child(p.children),
        )
}
pub fn sidebar_header() -> nsh_core::element::View {
    let t = Theme::current();
    view()
        .min_h(move || px(t.tokens().frame_titlebar_size))
        .justify_center()
        .py(move || px(t.tokens().spacing_2))
        .px(move || px(t.tokens().spacing_2_5))
        .gap(move || px(t.tokens().spacing_2))
}
pub fn sidebar_content() -> nsh_core::element::Scroll {
    let t = Theme::current();
    let context = use_context::<SidebarContentContext>();
    nsh_core::element::scroll()
        .flex_1()
        .min_h(px(0.0))
        .py(move || {
            px(if context.is_some_and(|c| c.rail.get()) {
                t.tokens().spacing_2_5
            } else {
                t.tokens().spacing_2
            })
        })
}
pub fn sidebar_group() -> nsh_core::element::View {
    let t = Theme::current();
    let context = use_context::<SidebarContentContext>();
    view()
        .w_full()
        .min_w(px(0.0))
        .py(move || px(t.tokens().spacing_1_5))
        .px(move || px(t.tokens().spacing_2_5))
        .app_class(move || {
            context
                .is_some_and(|c| c.rail.get())
                .then(|| SharedString::from("first:pt-0 last:pb-0"))
        })
}
pub fn sidebar_menu() -> nsh_core::element::View {
    let t = Theme::current();
    view().gap(move || px(t.tokens().frame_nav_gap))
}
#[derive(Clone, Copy)]
struct SidebarItemState(Signal<bool>);
#[derive(Clone, PartialEq, Props)]
pub struct SidebarMenuItemProps {
    pub children: Prop<Children>,
}
#[component]
pub fn SidebarMenuItem(p: SidebarMenuItemProps) -> impl IntoElement {
    provide_context(SidebarItemState(signal(false)));
    view().relative().child(p.children)
}
pub fn sidebar_inset() -> nsh_core::element::View {
    let theme = Theme::current();
    let context = use_context::<SidebarContext>();
    view().flex_1().min_w(px(0.0)).style_with(move |s| {
        let t = theme.tokens();
        s.bg(t.color_surface.hsla()).style_when(
            context.is_some_and(|c| c.variant.get() == SidebarVariant::Inset),
            |s| {
                s.my(px(t.spacing_2))
                    .mr(px(t.spacing_2))
                    .ml(px(if context.is_some_and(|c| !c.open.get()) {
                        t.spacing_2
                    } else {
                        0.0
                    }))
                    .rounded(px(t.radius_large))
                    .shadow(crate::tokens::shadows(t.shadow_sm))
            },
        )
    })
}
#[derive(Clone, PartialEq, Props)]
pub struct SidebarGroupLabelProps {
    pub children: Prop<Children>,
}
#[component]
pub fn SidebarGroupLabel(p: SidebarGroupLabelProps) -> impl IntoElement {
    let context = use_context::<SidebarContentContext>();
    let theme = Theme::current();
    crate::section_label()
        .py(move || px(theme.tokens().spacing_1))
        .px(move || px(theme.tokens().spacing_2_5))
        .style_when(
            move || context.is_some_and(|c| c.collapsed.get()),
            |s| s.hidden(),
        )
        .child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct SidebarMenuButtonProps {
    #[prop(default)]
    pub active: Prop<bool>,
    pub icon: Prop<Option<Slot>>,
    pub tooltip: Prop<Option<SharedString>>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum SidebarMenuButtonEvent {
    Press,
}
#[component]
pub fn SidebarMenuButton(
    p: SidebarMenuButtonProps,
    cx: Scope<SidebarMenuButtonEvent>,
) -> impl IntoElement {
    let context = use_context::<SidebarContentContext>();
    let theme = Theme::current();
    if let Some(item) = use_context::<SidebarItemState>() {
        effect(move || item.0.set(p.active.get()));
    }
    provide_context(super::icon::IconSize(std::rc::Rc::new(move || {
        theme.tokens().icon_medium
    })));
    nsh_base::button()
        .w_full()
        .min_w(px(0.0))
        .flex_row()
        .items_center()
        .accessibility_label(p.tooltip)
        .on_press(move || cx.emit(SidebarMenuButtonEvent::Press))
        .style_with(move |s| {
            let t = theme.tokens();
            let s = styles::typography(s, t.label_quiet)
                .font_weight(if p.active.get() {
                    t.label_medium.font_weight()
                } else {
                    t.label_quiet.font_weight()
                })
                .line_height(px(t.label_quiet.size))
                .justify_start()
                .px(px(t.spacing_2_5))
                .py(px(t.spacing_2))
                .gap(px(t.spacing_1_5))
                .rounded(px(t.radius_small));
            if p.active.get() {
                styles::colors(s, t, Variant::Filled, Tint::Primary, crate::State::Normal)
            } else {
                s.bg(crate::tokens::Color::TRANSPARENT.hsla())
                    .text_color(t.color_content_nav.hsla())
            }
        })
        .hover(move |s| {
            s.style_with(move |s| {
                if p.active.get() {
                    s
                } else {
                    styles::colors(
                        s,
                        theme.tokens(),
                        Variant::Plain,
                        Tint::Primary,
                        crate::State::Hovered,
                    )
                }
            })
        })
        .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
        .child(p.icon.place(move |slot| {
            slot.w(px(theme.tokens().icon_medium))
                .h(px(theme.tokens().label_quiet.size))
                .items_center()
                .justify_center()
        }))
        .child(
            view()
                .style_when(
                    move || context.is_some_and(|c| c.collapsed.get()),
                    |s| s.hidden(),
                )
                .child(p.children),
        )
}
#[derive(Clone, PartialEq, Props)]
pub struct SidebarMenuBadgeProps {
    pub children: Prop<Children>,
}
#[component]
pub fn SidebarMenuBadge(p: SidebarMenuBadgeProps) -> impl IntoElement {
    let context = use_context::<SidebarContentContext>();
    let theme = Theme::current();
    let item = use_context::<SidebarItemState>();
    view()
        .absolute()
        .right(move || px(theme.tokens().spacing_1))
        .top(move || px((theme.tokens().control_medium_size - theme.tokens().spacing_5) / 2.0))
        .h(move || px(theme.tokens().spacing_5))
        .min_w(move || px(theme.tokens().spacing_5))
        .px(move || px(theme.tokens().spacing_1))
        .items_center()
        .justify_center()
        .style_with(move |s| {
            styles::typography(s, theme.tokens().caption_small)
                .font_weight(theme.tokens().label_medium.font_weight())
                .tabular_nums()
                .text_color(if item.is_some_and(|item| item.0.get()) {
                    crate::axes::ControlColors::resolve(
                        theme.tokens(),
                        Variant::Filled,
                        Tint::Primary,
                        crate::State::Normal,
                    )
                    .content
                } else {
                    theme.tokens().color_content_nav.hsla()
                })
        })
        .style_when(
            move || context.is_some_and(|c| c.collapsed.get()),
            |s| s.hidden(),
        )
        .child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct SidebarTriggerProps {}
#[component]
pub fn SidebarTrigger(_: SidebarTriggerProps) -> impl IntoElement {
    let context = use_context::<SidebarContext>();
    crate::button()
        .size(Size::Tiny)
        .icon_only(true)
        .tint(Tint::Neutral)
        .variant(Variant::Plain)
        .accessibility_label("Toggle sidebar")
        .on_press(move || {
            if let Some(c) = context {
                c.open.update(|v| *v = !*v)
            }
        })
        .child(crate::icon("PanelLeft20Regular"))
}
