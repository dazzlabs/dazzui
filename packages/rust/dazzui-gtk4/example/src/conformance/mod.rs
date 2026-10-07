use crate::{stories, support::scene};
use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::{Value, json};
use std::{
    error::Error,
    path::Path,
    time::{Duration, Instant},
};

pub fn settle() {
    let until = Instant::now() + Duration::from_millis(50);
    while Instant::now() < until {
        while gtk::glib::MainContext::default().pending() {
            gtk::glib::MainContext::default().iteration(false);
        }
        std::thread::sleep(Duration::from_millis(2));
    }
}
pub fn screenshot(
    widget: &gtk::Widget,
    frame: &gtk::Widget,
    window: &gtk::Window,
    path: &Path,
    scale: f32,
) -> Result<(), Box<dyn Error>> {
    let bounds = widget.compute_bounds(frame).ok_or("No widget bounds")?;
    if bounds.width() <= 0.0 || bounds.height() <= 0.0 {
        return Err("Empty snapshot bounds".into());
    }
    let snapshot = gtk::Snapshot::new();
    snapshot.scale(scale, scale);
    let paintable = gtk::WidgetPaintable::new(Some(frame));
    let frame_bounds = frame.compute_bounds(frame).ok_or("No frame bounds")?;
    paintable.snapshot(
        &snapshot,
        frame_bounds.width() as f64,
        frame_bounds.height() as f64,
    );
    let node = snapshot.to_node().ok_or("No render node")?;
    let viewport = gtk::graphene::Rect::new(
        (bounds.x() - frame_bounds.x()) * scale,
        (bounds.y() - frame_bounds.y()) * scale,
        bounds.width() * scale,
        bounds.height() * scale,
    );
    let renderer = window.renderer().ok_or("No GTK renderer")?;
    renderer
        .render_texture(&node, Some(&viewport))
        .save_to_png(path)?;
    Ok(())
}
fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}
pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    let out = flag(&args, "--out").ok_or("--conformance requires --out <directory>")?;
    let requested = flag(&args, "--themes").unwrap_or("drawn".into());
    let story_filter = flag(&args, "--scenes");
    let theme_names: Vec<_> = THEMES
        .iter()
        .copied()
        .filter(|name| {
            if requested == "all" {
                true
            } else if requested == "drawn" {
                [
                    "studio-",
                    "bright-",
                    "frost-",
                    "graphite-",
                    "ember-",
                    "nocturne-",
                ]
                .iter()
                .any(|p| name.starts_with(p))
            } else {
                requested.split(',').any(|s| s == *name)
            }
        })
        .collect();
    if theme_names.is_empty() {
        return Err("No matching themes".into());
    }
    let images = !args.iter().any(|a| a == "--no-images");
    gtk::init()?;
    let settings = gtk::Settings::default().ok_or("No GTK settings")?;
    let animations = settings.is_gtk_enable_animations();
    settings.set_gtk_enable_animations(false);
    let window = gtk::Window::builder().default_width(640).build();

    std::fs::create_dir_all(&out)?;
    let mut themes = serde_json::Map::new();
    let mut scene_images = serde_json::Map::new();
    let mut gaps = serde_json::Map::new();
    for name in theme_names {
        Theme::new(name)?.apply(&window);
        let mut readings = serde_json::Map::new();
        let mut rendered = serde_json::Map::new();
        let image_dir = Path::new(&out).join("images").join(name);
        if images {
            std::fs::create_dir_all(&image_dir)?;
        }
        for story in stories::all() {
            if let Some(filter) = &story_filter
                && !filter.split(',').any(|s| s == story.id)
            {
                continue;
            }
            for scene in story.scenes() {
                let id = scene.id.as_str();
                if !scene.is_supported() {
                    gaps.insert(id.into(), json!(scene.gaps));
                    continue;
                }
                let mut specimens = vec![];
                let content = scene::render(&scene.content, &mut specimens)?;
                let frame = box_(gtk::Orientation::Vertical, "0");
                frame.add_css_class("dz-gallery");
                content.set_halign(gtk::Align::Center);
                frame.append(&content);
                window.set_child(Some(&frame));
                // Reset the previous scene's taller allocation.
                window.set_default_size(640, 1);
                window.present();
                settle();
                if images {
                    let file = format!("{}.png", id.replace('/', "-"));
                    screenshot(
                        frame.upcast_ref(),
                        frame.upcast_ref(),
                        &window,
                        &image_dir.join(&file),
                        2.,
                    )?;
                    rendered.insert(id.into(), json!(format!("images/{name}/{file}")));
                }
                for (at, widget) in specimens {
                    if widget.width() == 0 || widget.height() == 0 || !widget.is_mapped() {
                        continue;
                    }
                    let color = widget.color();
                    let ink = format!(
                        "#{:02x}{:02x}{:02x}{:02x}",
                        (color.red() * 255.).round() as u8,
                        (color.green() * 255.).round() as u8,
                        (color.blue() * 255.).round() as u8,
                        (color.alpha() * 255.).round() as u8
                    );
                    let font = widget.pango_context().font_description();
                    let mut reading = json!({"box":{"width":widget.compute_bounds(&frame).ok_or("No widget bounds")?.width(),"height":widget.compute_bounds(&frame).ok_or("No widget bounds")?.height()},"values":{
                        "ink":ink, "fontSize":font.as_ref().map(|f|f.size() as f64 / gtk::pango::SCALE as f64),
                        "fontWeight":null,"lineHeight":null,"corner":null,"stroke":null,"fill":null,"strokeColor":null,"paddingX":null,"paddingY":null,"gap":null,"shadow":null
                    }});
                    if images {
                        let file = format!("{}-{at}.png", id.replace('/', "-"));
                        screenshot(
                            &widget,
                            frame.upcast_ref(),
                            &window,
                            &image_dir.join(&file),
                            2.,
                        )?;
                        reading["image"] = json!(format!("images/{name}/{file}"));
                    }
                    readings.insert(format!("{id}/{at}"), reading);
                }
            }
        }
        themes.insert(name.into(), Value::Object(readings));
        scene_images.insert(name.into(), Value::Object(rendered));
    }
    let commit = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_default();
    let result = json!({"platform":"gtk4","title":"GTK4","role":"implementation","collected":gtk::glib::DateTime::now_utc()?.format_iso8601()?.as_str(),"commit":commit,"scale":2,"font":null,"themes":themes,"scenes":scene_images});
    std::fs::write(
        Path::new(&out).join("readings.json"),
        serde_json::to_string_pretty(&result)? + "\n",
    )?;
    std::fs::write(
        Path::new(&out).join("gaps.json"),
        serde_json::to_string_pretty(&gaps)? + "\n",
    )?;
    window.destroy();
    settings.set_gtk_enable_animations(animations);
    Ok(())
}
