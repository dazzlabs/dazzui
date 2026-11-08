//! Inspect the platform text system without TestAppContext's synthetic fonts.
use dazzui::tokens::Face;
use gpui::{AppContext, Application, Empty, FontWeight, TextRun, WindowOptions, black, font, px};
use serde_json::{Value, json};

fn main() {
    let file = std::env::args()
        .nth(1)
        .expect("font-metrics <requests.json>");
    let requests: Vec<Value> = serde_json::from_slice(&std::fs::read(file).unwrap()).unwrap();
    Application::new().run(move |cx| {
        cx.open_window(
            WindowOptions {
                show: false,
                focus: false,
                ..Default::default()
            },
            move |window, cx| {
                let system = window.text_system();
                let mut readings = Vec::with_capacity(requests.len());
                for request in requests {
                    let text = request["text"].as_str().unwrap();
                    let face = if request["face"] == "mono" {
                        Face::Mono
                    } else {
                        Face::Text
                    };
                    let size = request["size"].as_f64().unwrap() as f32;
                    let mut font = font(face.family());
                    font.weight = FontWeight(request["weight"].as_f64().unwrap_or(400.) as f32);
                    let line = system.shape_line(
                        text.to_owned().into(),
                        px(size),
                        &[TextRun {
                            len: text.len(),
                            font,
                            color: black(),
                            background_color: None,
                            underline: None,
                            strikethrough: None,
                        }],
                        None,
                    );
                    let glyphs = line
                        .runs
                        .iter()
                        .flat_map(|run| run.glyphs.iter())
                        .map(
                            |glyph| json!({"index": glyph.index, "x": f32::from(glyph.position.x)}),
                        )
                        .collect::<Vec<_>>();
                    readings.push(json!({
                        "request": request,
                        "family": face.family(),
                        "width": f32::from(line.width),
                        "glyphs": glyphs,
                    }));
                }
                println!(
                    "{}",
                    json!({"platform": std::env::consts::OS, "readings": readings})
                );
                cx.new(|_| Empty)
            },
        )
        .unwrap();
        cx.quit();
    });
}
