//! The GPUI collector: draws every scene in scenes.json, in every theme
//! asked for, in a real window at the display's scale, and writes
//! readings.json, the scene images and a crop per specimen. See
//! specs/conformance.md; `tool/conformance.sh` is its front door.
//!
//! A specimen's box is read from the bounds the element built for its node
//! laid out at (`lib/specimen.rs`). What it painted — fill, stroke, type —
//! goes through GPUI's scene, which an app cannot read back, so every other
//! property is written unread and the pictures carry the rest.

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
    commit: String,
}

impl Options {
    /// `--conformance --out <dir> --contract <scenes.json> [--themes …]
    /// [--scenes …] [--no-images] [--commit …]`, or `None` without
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
        Some(Options {
            out: PathBuf::from(value("--out").expect("--out is required")),
            contract: PathBuf::from(value("--contract").expect("--contract is required")),
            themes: value("--themes").unwrap_or_else(|| "drawn".into()),
            scenes: value("--scenes")
                .filter(|s| !s.is_empty())
                .map(|s| s.split(',').map(str::to_string).collect()),
            images: !args.iter().any(|arg| arg == "--no-images"),
            commit: value("--commit").unwrap_or_default(),
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
                    theme: Theme::named("studio-light").unwrap(),
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
    images: bool,
) -> Result<(), String> {
    let mut previous = None;
    for _ in 0..25 {
        if !images {
            // A compositor can stop requesting presentation for an occluded
            // window. Box collection still needs a newly laid-out native frame.
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear())
                .map_err(|error| error.to_string())?;
        }
        frame(cx).await;
        let frames = specimen::frames();
        if !frames.contains_key(&FRAME) {
            continue;
        }
        if previous.as_ref() == Some(&frames) {
            return Ok(());
        }
        previous = Some(frames);
    }
    Err("the window did not draw a stable scene frame".into())
}

async fn collect(
    options: &Options,
    window: WindowHandle<Stage>,
    cx: &mut AsyncApp,
) -> Result<(), String> {
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
    let mut failures = Vec::new();

    for theme_id in options.theme_ids() {
        let entry = Theme::named(&theme_id).ok_or(format!("no theme named {theme_id}"))?;
        let image_dir = options.out.join("images").join(&theme_id);
        if options.images {
            std::fs::create_dir_all(&image_dir).map_err(|e| e.to_string())?;
        }
        let mut readings_of = Map::new();
        let mut scenes_of = Map::new();

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
            specimen::start();
            window
                .update(cx, |stage, _, cx| {
                    stage.theme = entry;
                    stage.scene = Some(SceneView::new(scene, cx));
                    cx.notify();
                })
                .map_err(|e| e.to_string())?;
            settle(window, cx, options.images).await?;
            let height = specimen::frames()
                .get(&FRAME)
                .map(|b| f32::from(b.size.height))
                .unwrap_or(400.0);
            specimen::start();
            window
                .update(cx, |_, window, cx| {
                    window.resize(size(
                        px(VIEWPORT_WIDTH),
                        px(height.ceil().max(VIEWPORT_HEIGHT)),
                    ));
                    cx.notify();
                })
                .map_err(|e| e.to_string())?;
            settle(window, cx, options.images).await?;
            let frames = specimen::frames();

            let file = id.replace('/', "-");
            let mut crops: BTreeMap<usize, String> = BTreeMap::new();
            if options.images {
                let path = image_dir.join(format!("{file}.png"));
                let number = window
                    .update(cx, |_, window, _| window_number(window))
                    .map_err(|e| e.to_string())?
                    .ok_or("the window has no number to capture it by")?;
                capture(number, &path)?;
                let picture = image::open(&path).map_err(|e| e.to_string())?;
                let scale = picture.width() as f32 / VIEWPORT_WIDTH;
                let scene_height = (height * scale).round() as u32;
                let scene_width = (WIDTH * scale).round() as u32;
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
                    // What GPUI painted cannot be read back from it.
                    "values": {},
                });
                if let Some(crop) = crops.get(&index) {
                    reading["image"] = json!(format!("images/{theme_id}/{crop}"));
                }
                readings_of.insert(address.into(), reading);
            }
        }
        readings_themes.insert(theme_id.clone(), Value::Object(readings_of));
        readings_scenes.insert(theme_id.clone(), Value::Object(scenes_of));
    }

    let scale = window
        .update(cx, |_, window, _| window.scale_factor())
        .map_err(|error| error.to_string())?;
    let readings = json!({
        "platform": "gpui",
        "title": "GPUI",
        "role": "implementation",
        "collected": now(),
        "commit": options.commit,
        "scale": scale,
        // The platform's own UI face, as React's; no embedded test face.
        "font": null,
        "themes": readings_themes,
        "scenes": readings_scenes,
    });
    write(&options.out.join("readings.json"), &readings.to_string())?;
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
