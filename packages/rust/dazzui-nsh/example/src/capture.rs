//! Native layout collection. Each stage owns and measures its actual views.
use crate::runtime::scene::{Scene, Story};
use dazzui_nsh::prelude::*;
use serde_json::json;
use std::{cell::RefCell, collections::BTreeMap, io::Write, rc::Rc};

#[derive(Clone, Default)]
pub(crate) struct CaptureEnabled(Rc<RefCell<BTreeMap<String, ElementRef>>>);
impl CaptureEnabled {
    pub(crate) fn register(&self, address: String, reference: ElementRef) {
        self.0.borrow_mut().insert(address, reference);
    }
}
#[derive(Clone, Copy)]
struct CaptureStatus(Signal<SharedString>);

#[derive(Clone, PartialEq, Props)]
struct CaptureStageProps {
    scene: Prop<SharedString>,
    theme: Prop<SharedString>,
}
#[component]
fn CaptureStage(p: CaptureStageProps) -> impl IntoElement {
    let theme = Theme::new(&p.theme.get_untracked());
    provide_context(theme);
    let captured = CaptureEnabled::default();
    provide_context(captured.clone());
    let status = use_context::<CaptureStatus>().expect("capture status").0;
    let scene: Scene = serde_json::from_str(&p.scene.get_untracked()).expect("capture scene");
    let id = scene.id.clone();
    let stage = element_ref();
    // Issue all measurements in one commit after the native layout settles.
    // No headless measurer or token-derived geometry enters the report.
    set_timeout(std::time::Duration::from_millis(50), move || {
        let pending: Vec<_> = captured
            .0
            .borrow()
            .iter()
            .map(|(address, reference)| (address.clone(), reference.measure()))
            .collect();
        let stage_pending = stage.measure();
        spawn(async move {
            let Some(stage) = stage_pending.await else {
                return;
            };
            let mut frames = BTreeMap::new();
            for (address, pending) in pending {
                if let Some(m) = pending.await {
                    frames.insert(address, json!({
                        "x": f32::from(m.page.x), "y": f32::from(m.page.y),
                        "width": f32::from(m.frame.size.width), "height": f32::from(m.frame.size.height)
                    }));
                }
            }
            let height = f32::from(stage.frame.size.height);
            let report = json!({
                "theme": theme.name(), "scene": id, "frames": frames,
                "scale": nsh_core::element::pixel_ratio(),
                "stage": { "x": f32::from(stage.page.x), "y": f32::from(stage.page.y),
                    "width": f32::from(stage.frame.size.width), "height": height }
            });
            let path = std::env::var_os("DAZZUI_NSH_CAPTURE").expect("capture path");
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect("capture file");
            writeln!(file, "{report}").expect("write capture");
            status.set(format!("Ready {} {} {height}", theme.name(), id).into());
        });
    });
    view()
        .element_ref(&stage)
        .w(px(640.0))
        .flex_shrink_0()
        .flex_row()
        .p(px(theme.tokens().spacing_6))
        .style_with(move |s| dazzui_nsh::styles::typography(s, theme.tokens().body_medium))
        .bg(theme.tokens().color_surface.hsla())
        .text_color(theme.tokens().color_content.hsla())
        .items_center()
        .justify_center()
        .child(scene.render())
}

#[component]
pub(crate) fn Capture() -> impl IntoElement {
    nsh_core::element::desktop::set_frame(nsh_core::desktop::WindowFrame {
        x: Some(px(80.0)),
        y: Some(px(80.0)),
        ..Default::default()
    });
    let names: Vec<String> =
        serde_json::from_str(&std::env::var("DAZZUI_NSH_THEMES").expect("capture themes"))
            .expect("theme list");
    let selected: Vec<String> =
        serde_json::from_str(&std::env::var("DAZZUI_NSH_SCENES").expect("capture scenes"))
            .expect("scene list");
    let scenes: Vec<Scene> = crate::stories::STORIES
        .iter()
        .map(|s| serde_json::from_str::<Story>(s).expect("story"))
        .flat_map(|s| std::iter::once(s.preview).chain(s.examples))
        .filter(|s| selected.contains(&s.id))
        .collect();
    assert!(!names.is_empty() && scenes.len() == selected.len());
    assert!(
        scenes.iter().all(|s| s.gaps.is_empty()),
        "unmapped capture scene"
    );
    let count = scenes.len();
    let total = count * names.len();
    provide_context(Theme::new(&names[0]));
    let current = signal(0usize);
    let status = signal(SharedString::from("Preparing capture"));
    provide_context(CaptureStatus(status));
    if std::env::var_os("DAZZUI_NSH_AUTO_CAPTURE").is_some() {
        effect(move || {
            if status.get().starts_with("Ready ") {
                set_timeout(std::time::Duration::ZERO, move || {
                    if current.get_untracked() + 1 == total {
                        status.set("Complete capture".into());
                    } else {
                        status.set("Preparing capture".into());
                        current.update(|at| *at += 1);
                    }
                });
            }
        });
    }

    nsh_core::element::on_key_with(
        "ctrl+alt+n",
        nsh_core::element::KeyOptions {
            when: nsh_core::keymap::When::Always,
            repeat: false,
        },
        move |_| {
            if current.get_untracked() + 1 < total {
                status.set("Preparing capture".into());
                current.update(|at| *at += 1);
            }
        },
    );
    view()
        .size_full()
        .items_start()
        .child(
            view()
                .h(px(48.0))
                .flex_shrink_0()
                .flex_row()
                .items_center()
                .child(
                    button()
                        .accessibility_label("Next capture")
                        .disabled(move || current.get() + 1 >= total)
                        .on_press(move || {
                            status.set("Preparing capture".into());
                            current.update(|at| *at += 1);
                        })
                        .child(text("Next capture")),
                )
                .child(text(move || status.get())),
        )
        .child(dynamic(move || {
            let at = current.get();
            capture_stage()
                .key(format!("{}/{}", names[at / count], scenes[at % count].id))
                .theme(names[at / count].clone())
                .scene(serde_json::to_string(&scenes[at % count]).expect("capture scene"))
        }))
}
