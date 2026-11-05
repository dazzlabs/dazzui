use crate::{Size, Theme, Variant};
use nsh_core::{
    element::{KeyOptions, on_key_with},
    keymap::When,
    prelude::*,
};

#[derive(Clone, Copy)]
struct MenubarContext {
    open: Signal<Option<SharedString>>,
    entries: Stored<Vec<(SharedString, nsh_core::element::ElementRef, Prop<bool>)>>,
}
#[derive(Clone, PartialEq, Props)]
pub struct MenubarProps {
    pub accessibility_label: Prop<Option<SharedString>>,
    pub children: Prop<Children>,
}
#[component]
pub fn Menubar(p: MenubarProps) -> impl IntoElement {
    let context = MenubarContext {
        open: signal(None),
        entries: stored(vec![]),
    };
    provide_context(context);
    effect(move || {
        let Some(current) = context.open.get() else {
            return;
        };
        for (key, delta) in [("left", -1), ("right", 1)] {
            let current = current.clone();
            on_key_with(
                key,
                KeyOptions {
                    when: When::Always,
                    repeat: false,
                },
                move |_| {
                    let entries = context.entries.with(|entries| {
                        entries
                            .iter()
                            .filter(|(_, _, disabled)| !disabled.get())
                            .cloned()
                            .collect::<Vec<_>>()
                    });
                    if let Some(i) = entries.iter().position(|(id, _, _)| *id == current) {
                        let next = (i as isize + delta).rem_euclid(entries.len() as isize) as usize;
                        context.open.set(Some(entries[next].0.clone()));
                        entries[next].1.focus();
                    }
                },
            );
        }
    });
    let theme = Theme::current();
    nsh_base::toolbar()
        .accessibility_label(p.accessibility_label)
        .flex_row()
        .gap(move || px(theme.tokens().spacing_0_5))
        .child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct MenubarMenuProps {
    pub value: Prop<SharedString>,
    pub label: Prop<SharedString>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    pub children: Prop<Children>,
}
#[component]
pub fn MenubarMenu(p: MenubarMenuProps) -> impl IntoElement {
    let context = use_context::<MenubarContext>().expect("MenubarMenu needs Menubar");
    let editor = element_ref();
    let id = p.value.get_untracked();
    let dispose = id.clone();
    context
        .entries
        .with(|entries| entries.push((id, editor, p.disabled)));
    cleanup(move || {
        context
            .entries
            .with(|entries| entries.retain(|(id, _, _)| *id != dispose))
    });
    let active = move || context.open.get().as_ref() == Some(&p.value.get());
    crate::menu()
        .open(move || Some(active()))
        .on_open_change(move |event: &nsh_base::MenuEvent| {
            if let nsh_base::MenuEvent::OpenChange(open, _) = event {
                if *open && !p.disabled.get() {
                    context.open.set(Some(p.value.get()));
                } else if !*open && active() {
                    context.open.set(None);
                }
            }
        })
        .trigger(
            crate::button()
                .element_ref(&editor)
                .size(Size::Small)
                .disabled(p.disabled)
                .variant(move || {
                    if active() {
                        Variant::Filled
                    } else {
                        Variant::Plain
                    }
                })
                .style_with(move |s| {
                    let t = Theme::current().tokens();
                    s.px(px(t.spacing_2))
                        .py(px(0.0))
                        .border(px(0.0))
                        .rounded(px(t.radius_small))
                })
                .child(crate::styles::label(
                    move || p.label.get(),
                    move || Theme::current().tokens().control_small_content,
                )),
        )
        .child(p.children)
}
