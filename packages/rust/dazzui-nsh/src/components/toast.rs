use crate::{Size, Theme, Tint, Variant, styles};
use nsh_base::{
    Anchor, ToastManager, ToastMotion, ToastOptions, ToastStackState, ToastTransitionStatus,
};
use nsh_core::{prelude::*, time::now};
use std::{rc::Rc, time::Duration};

#[derive(Clone)]
pub struct ToastNotice {
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub tint: Tint,
    pub action: Option<(SharedString, Rc<dyn Fn()>)>,
}
impl ToastNotice {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            tint: Tint::Primary,
            action: None,
        }
    }
}
#[derive(Clone, Copy)]
pub struct ToastHandle {
    manager: Stored<ToastManager<u64, ToastNotice>>,
    shown: Signal<Vec<(u64, ToastTransitionStatus)>>,
    next: Signal<u64>,
    theme: Theme,
}
impl ToastHandle {
    pub fn current() -> Option<Self> {
        use_context::<Self>()
    }
    pub fn push(self, notice: ToastNotice) -> u64 {
        let id = self.next.get_untracked();
        self.next.set(id + 1);
        let t = self.theme.tokens();
        let timeout = if notice.action.is_some() {
            t.toast_action_duration
        } else {
            t.toast_notice_duration
        };
        self.manager.with(|m| {
            m.push(
                id,
                notice,
                ToastOptions::timeout(Some(Duration::from_secs_f32(timeout / 1000.0))),
                now(),
            )
        });
        self.sync();
        id
    }
    pub fn dismiss(self, id: u64) {
        self.manager.with(|m| m.dismiss(&id, now()));
        self.sync();
    }
    fn sync(self) {
        self.shown.set(
            self.manager
                .with(|m| m.visible(3).map(|(id, _, status)| (*id, status)).collect()),
        );
    }
}
fn motion(theme: Theme) -> ToastMotion {
    let t = theme.tokens();
    ToastMotion {
        duration: Duration::from_secs_f32(t.toast_transition_duration / 1000.0),
        exit_duration: Duration::from_secs_f32(t.toast_transition_duration / 1000.0),
        collapsed_peek: px(t.spacing_2),
        expanded_gap: px(t.spacing_2),
        collapsed_scale_step: 1.0 - t.toast_transition_scale,
        collapsed_visible: 3,
    }
}
#[derive(Clone, PartialEq, Props)]
pub struct ToastProviderProps {
    pub children: Prop<Children>,
}
#[component]
pub fn ToastProvider(p: ToastProviderProps) -> impl IntoElement {
    let theme = Theme::current();
    let handle = ToastHandle {
        manager: stored(ToastManager::new(motion(theme))),
        shown: signal(vec![]),
        next: signal(0),
        theme,
    };
    provide_context(handle);
    view().child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct ToastViewportProps {
    #[prop(default)]
    pub top: Prop<bool>,
}
#[component]
pub fn ToastViewport(p: ToastViewportProps) -> impl IntoElement {
    let handle = ToastHandle::current().expect("ToastViewport needs ToastProvider");
    let theme = Theme::current();
    let state = ToastStackState::new();
    set_interval(Duration::from_millis(50), move || {
        if handle
            .manager
            .with(|m| m.advance(now(), state.is_expanded()).changed)
        {
            handle.sync();
        }
    });
    // Keep the real viewport mounted even while empty. Returning a conditional
    // fragment would make a component reference measure its enclosing layout.
    nsh_core::element::overlay()
        .absolute()
        .left(px(0.0))
        .right(px(0.0))
        .p(move || px(theme.tokens().spacing_4))
        .items_center()
        .focusable(false)
        .pointer_events(nsh_core::PointerEvents::BoxNone)
        .style_with(move |s| {
            if p.top.get() {
                s.top(px(0.0))
            } else {
                s.bottom(px(0.0))
            }
        })
        .child(when(
            move || !handle.shown.get().is_empty(),
            move || {
                nsh_base::toast_stack()
                    .state(state)
                    .motion(move || motion(theme))
                    .placement(move || {
                        if p.top.get() {
                            Anchor::TopRight
                        } else {
                            Anchor::BottomRight
                        }
                    })
                    .w(move || px(theme.tokens().toast_max_width))
                    .on_cancel(move || {
                        handle.manager.with(|m| m.dismiss_all(now()));
                        handle.sync();
                    })
                    .child(each(
                        handle.shown,
                        |(id, _)| *id,
                        move |item, _| {
                            let id = item.get_untracked().0;
                            let notice = handle
                                .manager
                                .with(|m| m.get(&id).cloned())
                                .expect("visible toast");
                            let title = notice.title.clone();
                            let action = notice.action.clone();
                            nsh_base::toast()
                                .transition_status(move || item.get().1)
                                .accessibility_label(Some(title.clone()))
                                .on_dismiss_request(move || handle.dismiss(id))
                                .style_with(move |s| {
                                    let t = theme.tokens();
                                    styles::popup(s, t)
                                        .p(px(t.spacing_3))
                                        .gap(px(t.spacing_2))
                                        .border_color(
                                            notice.tint.ramp(t).shade(600).alpha(t.wash_edge),
                                        )
                                })
                                .child(
                                    view()
                                        .flex_row()
                                        .items_center()
                                        .gap(move || px(theme.tokens().spacing_2))
                                        .child(text(title).flex_1())
                                        .child(
                                            crate::button()
                                                .size(Size::Tiny)
                                                .variant(Variant::Plain)
                                                .accessibility_label("Dismiss notification")
                                                .on_press(move || handle.dismiss(id))
                                                .child(crate::icon("Dismiss12Regular")),
                                        ),
                                )
                                .child(notice.description.map(text))
                                .child(action.map(|(label, action)| {
                                    crate::button()
                                        .size(Size::Small)
                                        .variant(Variant::Outlined)
                                        .child(text(label))
                                        .on_press(move || {
                                            action();
                                            handle.dismiss(id);
                                        })
                                }))
                        },
                    ))
            },
        ))
}
