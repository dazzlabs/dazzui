//! The GPUI collector: draws every scene in scenes.json, in every theme
//! asked for, in a real window at the display's scale, and writes
//! readings.json, the scene images and a crop per specimen. See
//! specs/conformance.md; `tool/conformance.sh` is its front door.
//!
//! A specimen's box is read from the bounds the element built for its node
//! laid out at (`lib/specimen.rs`). Components also observe their own surface
//! styles and submitted text/SVG runs. Unsupported paint properties stay unread.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use dazzui::{ActiveTheme, Theme, themed};
use gpui::{
    AnyElement, App, AppContext, AsyncApp, Bounds, Context, Entity, IntoElement, ParentElement,
    Pixels, Render, Styled, Window, WindowBounds, WindowHandle, WindowKind, WindowOptions, div,
    point, px, size,
};
use serde_json::{Map, Value, json};

use crate::lib::scene::SceneView;
use crate::lib::specimen;
use crate::lib::stage::page_type;
use crate::lib::story::{Scene, stories};
use crate::lib::themes::THEME_FAMILY_GROUPS;

/// The width every collector draws a scene at: 40rem.
const WIDTH: f32 = 640.0;
/// React's browser viewport surrounds the 40rem scene. Window-anchored
/// controls such as ToastViewport resolve against this, rather than the frame.
const VIEWPORT_WIDTH: f32 = 1024.0;
const VIEWPORT_HEIGHT: f32 = 768.0;
/// The index the frame itself is recorded under, beside the specimens'.
const FRAME: usize = usize::MAX;

pub struct Options {
    out: PathBuf,
    contract: PathBuf,
    themes: String,
    scenes: Option<Vec<String>>,
    images: bool,
    window_capture: bool,
    commit: String,
    motion_time: Option<Duration>,
    reduced_motion: bool,
    modal: Option<String>,
    modal_index: usize,
    caret_focus: Option<usize>,
    caret_input: Option<String>,
    caret_click: Option<f32>,
    caret_click_y: Option<f32>,
    caret_click_count: usize,
    caret_drag: Vec<(f32, f32)>,
    caret_shift_clicks: Vec<(f32, f32)>,
    caret_replacement: Option<String>,
    caret_keys: Vec<String>,
    caret_batch_keys: bool,
    selection_probe: bool,
}

impl Options {
    /// `--conformance --out <dir> --contract <scenes.json> [--themes …]
    /// [--scenes …] [--no-images] [--window-capture] [--positions]
    /// [--caret-focus <index>] [--caret-input <text>]
    /// [--caret-click <x>] [--caret-click-y <y>] [--caret-click-count <1|2|3>]
    /// [--caret-drag <x,y;x,y>] [--caret-shift-clicks <x,y;x,y>]
    /// [--caret-selection-probe] [--caret-batch-keys] [--caret-keys <key,…>] [--caret-replace-selection <text>] [--commit …]`, or `None` without
    /// `--conformance`.
    pub fn parse(args: &[String]) -> Option<Options> {
        if !args.iter().any(|arg| arg == "--conformance") {
            return None;
        }
        let value = |name: &str| {
            args.iter()
                .position(|arg| arg == name)
                .and_then(|i| args.get(i + 1))
                .cloned()
        };
        let points = |name: &str| {
            value(name)
                .map(|path| {
                    path.split(';')
                        .map(|point| {
                            let (x, y) = point
                                .split_once(',')
                                .unwrap_or_else(|| panic!("{name} expects x,y;x,y points"));
                            let x: f32 = x
                                .parse()
                                .unwrap_or_else(|_| panic!("{name} x must be a pixel offset"));
                            let y: f32 = y
                                .parse()
                                .unwrap_or_else(|_| panic!("{name} y must be a pixel offset"));
                            assert!(
                                x.is_finite() && y.is_finite(),
                                "{name} points must be finite"
                            );
                            (x, y)
                        })
                        .collect()
                })
                .unwrap_or_default()
        };
        Some(Options {
            out: PathBuf::from(value("--out").expect("--out is required")),
            contract: PathBuf::from(value("--contract").expect("--contract is required")),
            themes: value("--themes").unwrap_or_else(|| "drawn".into()),
            scenes: value("--scenes")
                .filter(|s| !s.is_empty())
                .map(|s| s.split(',').map(str::to_string).collect()),
            images: !args.iter().any(|arg| arg == "--no-images"),
            window_capture: args.iter().any(|arg| arg == "--window-capture"),
            commit: value("--commit").unwrap_or_default(),
            motion_time: value("--motion-ms").map(|value| {
                let milliseconds: f64 = value.parse().expect("--motion-ms must be a number");
                assert!(
                    milliseconds.is_finite() && milliseconds >= 0.,
                    "--motion-ms must be nonnegative and finite"
                );
                Duration::from_secs_f64(milliseconds / 1000.)
            }),
            reduced_motion: args.iter().any(|arg| arg == "--reduced-motion"),
            modal: value("--modal").map(|mode| {
                assert!(
                    matches!(mode.as_str(), "open" | "close"),
                    "--modal must be open or close"
                );
                mode
            }),
            modal_index: value("--modal-index")
                .map(|value| {
                    value
                        .parse()
                        .expect("--modal-index must be an unsigned integer")
                })
                .unwrap_or(0),
            caret_focus: value("--caret-focus").map(|value| {
                value
                    .parse()
                    .expect("--caret-focus must be an unsigned integer")
            }),
            caret_input: value("--caret-input"),
            caret_click: value("--caret-click").map(|x| {
                let x: f32 = x.parse().expect("--caret-click must be a pixel offset");
                assert!(x.is_finite(), "--caret-click must be finite");
                x
            }),
            caret_click_y: value("--caret-click-y").map(|y| {
                let y: f32 = y.parse().expect("--caret-click-y must be a pixel offset");
                assert!(y.is_finite(), "--caret-click-y must be finite");
                y
            }),
            caret_click_count: value("--caret-click-count")
                .map(|count| {
                    let count = count
                        .parse()
                        .expect("--caret-click-count must be an integer");
                    assert!(
                        (1..=3).contains(&count),
                        "--caret-click-count must be 1, 2 or 3"
                    );
                    count
                })
                .unwrap_or(1),
            caret_drag: points("--caret-drag"),
            caret_shift_clicks: points("--caret-shift-clicks"),
            caret_replacement: value("--caret-replace-selection"),
            caret_batch_keys: args.iter().any(|arg| arg == "--caret-batch-keys"),
            selection_probe: args.iter().any(|arg| arg == "--caret-selection-probe"),
            caret_keys: value("--caret-keys")
                .map(|keys| keys.split(',').map(str::to_owned).collect())
                .unwrap_or_default(),
        })
    }

    fn theme_ids(&self) -> Vec<String> {
        let halves = |groups: &[usize]| {
            groups
                .iter()
                .flat_map(|&group| THEME_FAMILY_GROUPS[group].families)
                .flat_map(|family| {
                    [
                        format!("{}-light", family.value),
                        format!("{}-dark", family.value),
                    ]
                })
                .collect::<Vec<_>>()
        };
        match self.themes.as_str() {
            "drawn" => halves(&[0]),
            "all" => halves(&(0..THEME_FAMILY_GROUPS.len()).collect::<Vec<_>>()),
            list => list.split(',').map(str::to_string).collect(),
        }
    }
}

/// The window's one view: a scene in the frame every collector repeats —
/// 40rem wide, `spacing-6` around it, on the surface, its content centred.
struct Stage {
    theme: &'static dazzui::tokens::ThemeEntry,
    scene: Option<Entity<SceneView>>,
}

impl Render for Stage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        specimen::begin_frame();
        let scene = self.scene.clone();
        themed(self.theme, Framed { scene })
    }
}

#[derive(IntoElement)]
struct Framed {
    scene: Option<Entity<SceneView>>,
}

impl gpui::RenderOnce for Framed {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let frame: AnyElement = page_type(div(), cx)
            .w(px(WIDTH))
            .p(px(t.spacing_6))
            .bg(t.color_surface.hsla())
            .flex()
            .items_center()
            .justify_center()
            .children(self.scene)
            .into_any_element();
        div()
            .size_full()
            .bg(t.color_surface.hsla())
            .child(specimen::wrap(Some(&json!(FRAME)), frame, ""))
    }
}

pub fn run(options: Options, cx: &mut App) {
    if options.caret_focus.is_some() {
        specimen::trace_carets();
    }
    let reduced =
        (options.motion_time.is_none() && options.modal.is_none()) || options.reduced_motion;
    dazzui::Motion::set_preference(
        if reduced {
            dazzui::MotionPreference::Reduced
        } else {
            dazzui::MotionPreference::Full
        },
        cx,
    );
    if options.modal.is_some() {
        dazzui::Motion::seek(Duration::ZERO, cx);
    } else {
        dazzui::Motion::freeze_at(options.motion_time.unwrap_or_default(), cx);
    }
    let bounds = Bounds::new(
        point(px(40.), px(40.)),
        size(px(VIEWPORT_WIDTH), px(VIEWPORT_HEIGHT)),
    );
    let window = cx
        .open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: None,
                focus: false,
                kind: WindowKind::PopUp,
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| Stage {
                    theme: Theme::named("bright-light").unwrap(),
                    scene: None,
                })
            },
        )
        .expect("a window to collect in");
    cx.activate(true);
    cx.spawn(async move |cx| {
        let code = match collect(&options, window, cx).await {
            Ok(()) => 0,
            Err(error) => {
                eprintln!("gpui: {error}");
                1
            }
        };
        let _ = cx.update(|cx| cx.quit());
        std::process::exit(code);
    })
    .detach();
}

async fn frame(cx: &mut AsyncApp) {
    cx.background_executor()
        .timer(Duration::from_millis(60))
        .await;
}

/// Waits until a scene has been drawn and what it records has stopped
/// changing. A fixed wait is not enough: a busy machine can hold a frame
/// back long enough that a scene reads as having drawn nothing.
async fn settle(
    window: WindowHandle<Stage>,
    cx: &mut AsyncApp,
    presentation: bool,
) -> Result<(), String> {
    let mut previous = None;
    for _ in 0..25 {
        if !presentation {
            // A compositor can stop requesting presentation for an occluded
            // window. Readings and renderer capture still need a fresh frame.
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear())
                .map_err(|error| error.to_string())?;
        }
        frame(cx).await;
        let frames = specimen::frames();
        if !frames.contains_key(&FRAME) {
            continue;
        }
        let input = cx
            .update_window(window.into(), |_, window, cx| {
                window
                    .focused_input_snapshot(cx)
                    .map(|(text, selection)| (text, selection.range, selection.reversed))
            })
            .map_err(|error| error.to_string())?;
        let snapshot = (frames, specimen::carets(), input);
        if previous.as_ref() == Some(&snapshot) {
            return Ok(());
        }
        previous = Some(snapshot);
    }
    Err("the window did not draw a stable scene frame".into())
}

async fn collect(
    options: &Options,
    window: WindowHandle<Stage>,
    cx: &mut AsyncApp,
) -> Result<(), String> {
    let scale = window
        .update(cx, |_, window, _| window.scale_factor())
        .map_err(|error| error.to_string())?;
    let text = std::fs::read_to_string(&options.contract)
        .map_err(|e| format!("{}: {e}", options.contract.display()))?;
    let contract: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let drawable: BTreeMap<String, &'static Scene> = stories()
        .iter()
        .flat_map(|story| {
            std::iter::once((story.preview.key.clone(), &story.preview)).chain(
                story
                    .examples
                    .iter()
                    .map(|example| (example.demo.key.clone(), &example.demo)),
            )
        })
        .collect();

    let wanted = |scene: &&Value| {
        options.scenes.as_ref().is_none_or(|only| {
            only.iter().any(|id| {
                scene["id"]
                    .as_str()
                    .is_some_and(|key| key == id || key.starts_with(&format!("{id}/")))
            })
        })
    };
    let scenes: Vec<&Value> = contract["stories"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|story| story["scenes"].as_array().into_iter().flatten())
        .filter(wanted)
        .collect();

    let _ = std::fs::remove_dir_all(&options.out);
    std::fs::create_dir_all(&options.out).map_err(|e| e.to_string())?;
    let mut readings_themes = Map::new();
    let mut readings_scenes = Map::new();
    let mut caret_themes = Map::new();
    let mut failures = Vec::new();

    for theme_id in options.theme_ids() {
        let entry = Theme::named(&theme_id).ok_or(format!("no theme named {theme_id}"))?;
        // Isolate actual selection pixels without changing layout or fonts,
        // just as the browser probe overrides foreground and ::selection ink.
        let entry: &'static dazzui::tokens::ThemeEntry = if options.selection_probe {
            let mut tokens = *entry.tokens;
            tokens.color_content = dazzui::tokens::Color::TRANSPARENT;
            tokens.focus_ring = dazzui::tokens::Paint::Color(dazzui::tokens::Color(0xff00ffff), 1.);
            Box::leak(Box::new(dazzui::tokens::ThemeEntry {
                name: entry.name,
                dark: entry.dark,
                tokens: Box::leak(Box::new(tokens)),
            }))
        } else {
            entry
        };
        let image_dir = options.out.join("images").join(&theme_id);
        if options.images {
            std::fs::create_dir_all(&image_dir).map_err(|e| e.to_string())?;
        }
        let mut readings_of = Map::new();
        let mut scenes_of = Map::new();
        let mut carets_of = Map::new();

        for contracted in &scenes {
            let id = contracted["id"].as_str().unwrap_or_default();
            write(
                &options.out.join("progress.txt"),
                &format!("{theme_id} {id}\n"),
            )?;
            let specimens = contracted["specimens"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            let Some(scene) = drawable.get(id).copied() else {
                failures.push(format!("{id}: the example has no such scene"));
                continue;
            };
            // Codegen addressed each specimen; the scene must carry the same.
            let carried = addressed(&scene.content);
            let want: Vec<String> = specimens
                .iter()
                .map(|s| s["component"].as_str().unwrap_or_default().to_string())
                .collect();
            let got: Vec<String> = (0..want.len())
                .map(|i| {
                    carried
                        .get(&i)
                        .map(|c| kebab(c))
                        .unwrap_or("nothing".into())
                })
                .collect();
            if want != got || carried.len() != want.len() {
                failures.push(format!(
                    "{id}: codegen addressed {}, the scene carries {}",
                    want.join(" "),
                    got.join(" ")
                ));
                continue;
            }

            // Draw the scene, then size the window to it and draw again.
            if options.modal.is_some() {
                cx.update(|cx| dazzui::Motion::seek(Duration::ZERO, cx))
                    .map_err(|e| e.to_string())?;
            }
            specimen::start();
            window
                .update(cx, |stage, window, cx| {
                    if options.caret_focus.is_some() {
                        window.blur();
                    }
                    stage.theme = entry;
                    stage.scene = Some(SceneView::new(scene, cx));
                    cx.notify();
                })
                .map_err(|e| e.to_string())?;
            settle(window, cx, options.images && options.window_capture).await?;
            let height = specimen::frames()
                .get(&FRAME)
                .map(|b| f32::from(b.size.height))
                .unwrap_or(400.0);
            let viewport = size(px(VIEWPORT_WIDTH), px(height.ceil().max(VIEWPORT_HEIGHT)));
            let resized = window
                .update(cx, |_, window, cx| {
                    if window.viewport_size() == viewport {
                        return false;
                    }
                    specimen::start();
                    window.resize(viewport);
                    cx.notify();
                    true
                })
                .map_err(|e| e.to_string())?;
            if resized {
                settle(window, cx, options.images && options.window_capture).await?;
            }
            if let Some(index) = options.caret_focus {
                cx.update_window(window.into(), |_, window, _| {
                    window.activate_window();
                    for _ in 0..=index {
                        window.focus_next();
                    }
                })
                .map_err(|error| error.to_string())?;
                settle(window, cx, options.images && options.window_capture).await?;
                cx.update_window(window.into(), |_, window, cx| {
                    let select = if cfg!(target_os = "macos") {
                        "cmd-a"
                    } else {
                        "ctrl-a"
                    };
                    window.dispatch_keystroke(gpui::Keystroke::parse(select).unwrap(), cx);
                })
                .map_err(|error| error.to_string())?;
                // Keep selection and replacement in distinct native event
                // turns, committing the current input handler between them.
                settle(window, cx, options.images && options.window_capture).await?;
                cx.update_window(window.into(), |_, window, cx| {
                    if let Some(text) = &options.caret_input {
                        window.dispatch_keystroke(
                            gpui::Keystroke {
                                key: "unidentified".into(),
                                key_char: Some(text.clone()),
                                ..Default::default()
                            },
                            cx,
                        );
                    } else {
                        window.dispatch_keystroke(gpui::Keystroke::parse("left").unwrap(), cx);
                    }
                })
                .map_err(|error| error.to_string())?;
                settle(window, cx, options.images && options.window_capture).await?;
                // A noncollapsed selection has no caret quad. Retain the
                // focused control's surface before the first pointer action.
                let pointer_surface =
                    if options.caret_click.is_some() || !options.caret_shift_clicks.is_empty() {
                        specimen::carets()
                            .into_values()
                            .find(|trace| {
                                trace["quads"]
                                    .as_array()
                                    .is_some_and(|quads| !quads.is_empty())
                            })
                            .ok_or("the focused input did not paint a caret")?["surface"]
                            .clone()
                    } else {
                        Value::Null
                    };
                if let Some(x) = options.caret_click {
                    let surface = &pointer_surface;
                    let position = point(
                        px(surface["left"].as_f64().unwrap() as f32 + x),
                        px(surface["top"].as_f64().unwrap() as f32
                            + options
                                .caret_click_y
                                .unwrap_or(surface["height"].as_f64().unwrap() as f32 / 2.)),
                    );
                    cx.update_window(window.into(), |_, window, cx| {
                        window.dispatch_event(
                            gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                                button: gpui::MouseButton::Left,
                                position,
                                click_count: options.caret_click_count,
                                ..Default::default()
                            }),
                            cx,
                        );
                        if options.caret_drag.is_empty() {
                            window.dispatch_event(
                                gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                                    button: gpui::MouseButton::Left,
                                    position,
                                    click_count: options.caret_click_count,
                                    ..Default::default()
                                }),
                                cx,
                            );
                        }
                    })
                    .map_err(|error| error.to_string())?;
                    settle(window, cx, options.images && options.window_capture).await?;
                    if !options.caret_drag.is_empty() {
                        let mut release = position;
                        for &(x, y) in &options.caret_drag {
                            release = point(
                                px(surface["left"].as_f64().unwrap() as f32 + x),
                                px(surface["top"].as_f64().unwrap() as f32 + y),
                            );
                            cx.update_window(window.into(), |_, window, cx| {
                                window.dispatch_event(
                                    gpui::PlatformInput::MouseMove(gpui::MouseMoveEvent {
                                        position: release,
                                        pressed_button: Some(gpui::MouseButton::Left),
                                        modifiers: Default::default(),
                                    }),
                                    cx,
                                );
                            })
                            .map_err(|error| error.to_string())?;
                            settle(window, cx, options.images && options.window_capture).await?;
                        }
                        cx.update_window(window.into(), |_, window, cx| {
                            window.dispatch_event(
                                gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                                    position: release,
                                    button: gpui::MouseButton::Left,
                                    click_count: options.caret_click_count,
                                    ..Default::default()
                                }),
                                cx,
                            );
                        })
                        .map_err(|error| error.to_string())?;
                        settle(window, cx, options.images && options.window_capture).await?;
                    }
                }
                if !options.caret_shift_clicks.is_empty() {
                    let surface = &pointer_surface;
                    for &(x, y) in &options.caret_shift_clicks {
                        let position = point(
                            px(surface["left"].as_f64().unwrap() as f32 + x),
                            px(surface["top"].as_f64().unwrap() as f32 + y),
                        );
                        cx.update_window(window.into(), |_, window, cx| {
                            let modifiers = gpui::Modifiers {
                                shift: true,
                                ..Default::default()
                            };
                            window.dispatch_event(
                                gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                                    position,
                                    button: gpui::MouseButton::Left,
                                    click_count: 1,
                                    modifiers,
                                    ..Default::default()
                                }),
                                cx,
                            );
                            window.dispatch_event(
                                gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                                    position,
                                    button: gpui::MouseButton::Left,
                                    click_count: 1,
                                    modifiers,
                                }),
                                cx,
                            );
                        })
                        .map_err(|error| error.to_string())?;
                        settle(window, cx, options.images && options.window_capture).await?;
                    }
                }
                for key in &options.caret_keys {
                    cx.update_window(window.into(), |_, window, cx| {
                        window.dispatch_keystroke(
                            gpui::Keystroke::parse(key).expect("invalid --caret-keys keystroke"),
                            cx,
                        );
                    })
                    .map_err(|error| error.to_string())?;
                    if !options.caret_batch_keys {
                        settle(window, cx, options.images && options.window_capture).await?;
                    }
                }
                if options.caret_batch_keys {
                    settle(window, cx, options.images && options.window_capture).await?;
                }
                if let Some(text) = &options.caret_replacement {
                    cx.update_window(window.into(), |_, window, cx| {
                        window.dispatch_keystroke(
                            gpui::Keystroke {
                                key: "unidentified".into(),
                                key_char: Some(text.clone()),
                                ..Default::default()
                            },
                            cx,
                        );
                    })
                    .map_err(|error| error.to_string())?;
                    settle(window, cx, options.images && options.window_capture).await?;
                }
                let snapshot = cx
                    .update_window(window.into(), |_, window, cx| {
                        window.focused_input_snapshot(cx)
                    })
                    .map_err(|error| error.to_string())?;
                let traces = specimen::carets();
                for (index, mut trace) in traces {
                    if let Some((value, selection)) = &snapshot {
                        trace["input"] = json!({"value": value, "selection": [selection.range.start, selection.range.end], "reversed": selection.reversed});
                    }
                    if let Some(address) = specimens
                        .get(index)
                        .and_then(|specimen| specimen["address"].as_str())
                    {
                        carets_of.insert(address.into(), trace);
                    }
                }
            }
            if let Some(mode) = &options.modal {
                let index = specimens
                    .iter()
                    .enumerate()
                    .filter(|(_, specimen)| {
                        matches!(specimen["component"].as_str(), Some("dialog" | "drawer"))
                    })
                    .nth(options.modal_index)
                    .map(|(index, _)| index)
                    .ok_or_else(|| format!("{id}: no modal {}", options.modal_index))?;
                let bounds = specimen::frames()
                    .get(&index)
                    .copied()
                    .ok_or_else(|| format!("{id}: modal trigger has no bounds"))?;
                let (number, height) = window
                    .update(cx, |_, window, _| {
                        (
                            window_number(window),
                            f32::from(window.viewport_size().height),
                        )
                    })
                    .map_err(|e| e.to_string())?;
                let number = number.ok_or("the window has no native input number")?;
                modal_click(number, height, bounds.center())?;
                settle(window, cx, options.images && options.window_capture).await?;
                let offset = if mode == "close" {
                    let duration = window
                        .update(cx, |_, _, cx| {
                            Duration::from_secs_f32(cx.theme().motion_duration / 1000.)
                        })
                        .map_err(|e| e.to_string())?;
                    cx.update(|cx| dazzui::Motion::seek(duration, cx))
                        .map_err(|e| e.to_string())?;
                    settle(window, cx, options.images && options.window_capture).await?;
                    modal_escape(number)?;
                    settle(window, cx, options.images && options.window_capture).await?;
                    duration
                } else {
                    Duration::ZERO
                };
                cx.update(|cx| {
                    dazzui::Motion::seek(offset + options.motion_time.unwrap_or_default(), cx)
                })
                .map_err(|e| e.to_string())?;
                settle(window, cx, options.images && options.window_capture).await?;
            }
            let frames = specimen::frames();
            let styles = specimen::styles(&specimens);
            let current_scale = window
                .update(cx, |_, window, _| window.scale_factor())
                .map_err(|error| error.to_string())?;
            if current_scale != scale {
                return Err(format!(
                    "{theme_id} {id}: display scale changed from {scale} to {current_scale}; refusing mixed-scale readings"
                ));
            }

            let file = id.replace('/', "-");
            let mut crops: BTreeMap<usize, String> = BTreeMap::new();
            if options.images {
                let path = image_dir.join(format!("{file}.png"));
                let picture = if options.window_capture {
                    let number = window
                        .update(cx, |_, window, _| window_number(window))
                        .map_err(|e| e.to_string())?
                        .ok_or("the window has no number to capture it by")?;
                    capture(number, &path)?;
                    image::open(&path).map_err(|e| e.to_string())?
                } else {
                    let pixels = window
                        .update(cx, |_, window, _| window.capture_scene())
                        .map_err(|e| e.to_string())?
                        .map_err(|e| e.to_string())?;
                    image::DynamicImage::ImageRgba8(pixels)
                };
                let scale = picture.width() as f32 / VIEWPORT_WIDTH;
                let capture_height = if options.modal.is_some() {
                    window
                        .update(cx, |_, window, _| f32::from(window.viewport_size().height))
                        .map_err(|e| e.to_string())?
                } else {
                    height
                };
                let scene_height = (capture_height * scale).round() as u32;
                let scene_width = ((if options.modal.is_some() {
                    VIEWPORT_WIDTH
                } else {
                    WIDTH
                }) * scale)
                    .round() as u32;
                let picture = picture.crop_imm(
                    0,
                    0,
                    scene_width.min(picture.width()),
                    scene_height.min(picture.height()),
                );
                picture.save(&path).map_err(|e| e.to_string())?;
                scenes_of.insert(id.into(), json!(format!("images/{theme_id}/{file}.png")));
                for (index, bounds) in &frames {
                    if *index == FRAME {
                        continue;
                    }
                    if let Some(crop) = crop(&picture, bounds, scale) {
                        let name = format!("{file}-{index}.png");
                        crop.save(image_dir.join(&name))
                            .map_err(|e| e.to_string())?;
                        crops.insert(*index, name);
                    }
                }
            }

            for (index, specimen) in specimens.iter().enumerate() {
                let (Some(bounds), Some(address)) =
                    (frames.get(&index), specimen["address"].as_str())
                else {
                    continue;
                };
                let (width, height) = (f32::from(bounds.size.width), f32::from(bounds.size.height));
                if width <= 0.0 && height <= 0.0 {
                    continue;
                }
                let mut reading = json!({
                    "box": { "width": round(width), "height": round(height) },
                    "values": styles.get(&index).cloned().unwrap_or_else(|| json!({})),
                });
                {
                    let origin = if options.modal.is_some() {
                        gpui::Point::default()
                    } else {
                        frames
                            .get(&FRAME)
                            .map(|frame| frame.origin)
                            .unwrap_or_default()
                    };
                    reading["values"]["left"] = json!(round(f32::from(bounds.origin.x - origin.x)));
                    reading["values"]["top"] = json!(round(f32::from(bounds.origin.y - origin.y)));
                    if let Some(baseline) = reading["values"]["baseline"].as_f64() {
                        reading["values"]["baseline"] =
                            json!(round(baseline as f32 - f32::from(origin.y)));
                    }
                }
                if let Some(crop) = crops.get(&index) {
                    reading["image"] = json!(format!("images/{theme_id}/{crop}"));
                }
                readings_of.insert(address.into(), reading);
            }
        }
        readings_themes.insert(theme_id.clone(), Value::Object(readings_of));
        readings_scenes.insert(theme_id.clone(), Value::Object(scenes_of));
        if options.caret_focus.is_some() {
            caret_themes.insert(theme_id, Value::Object(carets_of));
        }
    }

    let readings = json!({
        "platform": "gpui",
        "title": "GPUI",
        "role": "implementation",
        "collected": now(),
        "commit": options.commit,
        "scale": scale,
        "capture": options.images.then_some(if options.window_capture { "window" } else { "renderer" }),
        // The platform's own UI face, as React's; no embedded test face.
        "font": null,
        "themes": readings_themes,
        "scenes": readings_scenes,
    });
    write(&options.out.join("readings.json"), &readings.to_string())?;
    if let Some(focus) = options.caret_focus {
        write(
            &options.out.join("carets.json"),
            &json!({"scaleFactor": scale, "focusIndex": focus, "themes": caret_themes}).to_string(),
        )?;
    }
    if !failures.is_empty() {
        write(
            &options.out.join("failures.txt"),
            &(failures.join("\n") + "\n"),
        )?;
    }
    Ok(())
}

/// Every `$at` a scene's JSON carries, and the component at it.
fn addressed(value: &Value) -> BTreeMap<usize, String> {
    let mut found = BTreeMap::new();
    fn visit(value: &Value, found: &mut BTreeMap<usize, String>) {
        match value {
            Value::Array(items) => items.iter().for_each(|item| visit(item, found)),
            Value::Object(object) => {
                if let (Some(at), Some(component)) = (
                    object.get("$at").and_then(Value::as_u64),
                    object.get("$el").and_then(Value::as_str),
                ) {
                    found.insert(at as usize, component.to_string());
                }
                object.values().for_each(|item| visit(item, found));
            }
            _ => {}
        }
    }
    visit(value, &mut found);
    found
}

/// `SegmentedControl` → `segmented-control`, as scenes.json names it.
fn kebab(name: &str) -> String {
    let mut out = String::new();
    let mut previous: Option<char> = None;
    for c in name.chars() {
        if c.is_ascii_uppercase()
            && previous.is_some_and(|p| p.is_ascii_lowercase() || p.is_ascii_digit())
        {
            out.push('-');
        }
        out.push(c.to_ascii_lowercase());
        previous = Some(c);
    }
    out
}

fn round(n: f32) -> f64 {
    ((n as f64) * 100.0).round() / 100.0
}

fn crop(
    picture: &image::DynamicImage,
    bounds: &Bounds<Pixels>,
    scale: f32,
) -> Option<image::DynamicImage> {
    let x = (f32::from(bounds.origin.x) * scale).floor().max(0.0) as u32;
    let y = (f32::from(bounds.origin.y) * scale).floor().max(0.0) as u32;
    let right = ((f32::from(bounds.origin.x + bounds.size.width)) * scale).ceil() as u32;
    let bottom = ((f32::from(bounds.origin.y + bounds.size.height)) * scale).ceil() as u32;
    let right = right.min(picture.width());
    let bottom = bottom.min(picture.height());
    (right > x && bottom > y).then(|| picture.crop_imm(x, y, right - x, bottom - y))
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

// Queue real AppKit events rather than re-entering GPUI while App is borrowed.
#[cfg(target_os = "macos")]
fn modal_click(number: i64, height: f32, position: gpui::Point<Pixels>) -> Result<(), String> {
    use cocoa::{
        appkit::{NSApp, NSApplication, NSEvent, NSEventModifierFlags, NSEventType},
        base::{NO, nil},
        foundation::NSPoint,
    };
    unsafe {
        let location = NSPoint::new(
            f32::from(position.x) as f64,
            (height - f32::from(position.y)) as f64,
        );
        for kind in [
            NSEventType::NSMouseMoved,
            NSEventType::NSLeftMouseDown,
            NSEventType::NSLeftMouseUp,
        ] {
            let event = NSEvent::mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure_(
                nil, kind, location, NSEventModifierFlags::empty(), 0., number, nil, 0, 1, 1.);
            if event == nil {
                return Err("could not create modal pointer event".into());
            }
            NSApp().postEvent_atStart_(event, NO);
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn modal_escape(number: i64) -> Result<(), String> {
    use cocoa::{
        appkit::{NSApp, NSApplication, NSEvent, NSEventModifierFlags, NSEventType},
        base::{NO, nil},
        foundation::{NSPoint, NSString},
    };
    use objc::{msg_send, sel, sel_impl};
    unsafe {
        let characters = NSString::alloc(nil).init_str("\u{1b}");
        for kind in [NSEventType::NSKeyDown, NSEventType::NSKeyUp] {
            let event = NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode_(
                nil, kind, NSPoint::new(0., 0.), NSEventModifierFlags::empty(), 0., number, nil, characters, characters, NO, 53);
            if event == nil {
                return Err("could not create modal keyboard event".into());
            }
            NSApp().postEvent_atStart_(event, NO);
        }
        let _: () = msg_send![characters, release];
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn modal_click(_: i64, _: f32, _: gpui::Point<Pixels>) -> Result<(), String> {
    Err("modal capture currently requires macOS native input".into())
}
#[cfg(not(target_os = "macos"))]
fn modal_escape(_: i64) -> Result<(), String> {
    Err("modal capture currently requires macOS native input".into())
}

fn now() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .expect("UTC timestamps are valid RFC3339")
}

/// The window's number, which `screencapture -l` takes.
#[cfg(target_os = "macos")]
fn window_number(window: &Window) -> Option<i64> {
    use objc::{msg_send, runtime::Object, sel, sel_impl};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    let handle = HasWindowHandle::window_handle(window).ok()?;
    let RawWindowHandle::AppKit(appkit) = handle.as_raw() else {
        return None;
    };
    let view = appkit.ns_view.as_ptr() as *mut Object;
    unsafe {
        let ns_window: *mut Object = msg_send![view, window];
        if ns_window.is_null() {
            return None;
        }
        let number: i64 = msg_send![ns_window, windowNumber];
        Some(number)
    }
}

#[cfg(not(target_os = "macos"))]
fn window_number(_: &Window) -> Option<i64> {
    None
}

fn capture(number: i64, path: &Path) -> Result<(), String> {
    let status = std::process::Command::new("screencapture")
        .args(["-x", "-o", "-l", &number.to_string()])
        .arg(path)
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("screencapture failed for window {number}"));
    }
    // Captures carry the display's ICC profile; image's PNG writer strips it.
    // Convert the pixels before cropping so the saved PNGs remain sRGB.
    let output = std::process::Command::new("sips")
        .args([
            "--matchTo",
            "/System/Library/ColorSync/Profiles/sRGB Profile.icc",
        ])
        .arg(path)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "capture colour conversion: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}
