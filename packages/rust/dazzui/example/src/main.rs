#![allow(special_module_name)]
//! The GPUI example: the React example's window, drawn with `dazzui`.

// `lib/` mirrors the React example's `lib/`, which is worth the name clash
// with a library root this binary does not have.
mod conformance;
mod lib;
mod stories;
#[cfg(test)]
mod stories_test;
mod window;

use gpui::{
    App, AppContext, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, point, px,
    size,
};

use crate::window::Shell;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    if let Some(options) = conformance::Options::parse(&args) {
        Application::new()
            .with_assets(lib::assets::Assets)
            .run(move |cx: &mut App| {
                dazzui::init(cx);
                conformance::run(options, cx);
            });
        return;
    }
    let page = arg("--story");
    let theme = arg("--theme").unwrap_or_else(|| "studio-light".into());

    Application::new()
        .with_assets(lib::assets::Assets)
        .run(move |cx: &mut App| {
            dazzui::init(cx);
            let bounds = Bounds::centered(None, size(px(1280.), px(820.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("DazzUI — GPUI".into()),
                        appears_transparent: false,
                        traffic_light_position: Some(point(px(12.), px(12.))),
                    }),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| Shell::new(page.clone(), &theme, cx)),
            )
            .unwrap();
            cx.activate(true);
        });
}
