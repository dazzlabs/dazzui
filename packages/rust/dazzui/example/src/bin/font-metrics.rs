//! Inspect the platform text system without TestAppContext's synthetic fonts.
use dazzui::tokens::Face;
use gpui::{Application, FontWeight, TextStyle, WindowTextSystem, px};
use serde_json::{Value, json};

fn main() {
    let file = std::env::args()
        .nth(1)
        .expect("font-metrics <requests.json>");
    let requests: Vec<Value> = serde_json::from_slice(&std::fs::read(file).unwrap()).unwrap();
    // A headless Application retains the host's real platform text system.
    // No AppKit event loop or desktop login is needed for font measurements.
    let app = Application::headless();
    let system = WindowTextSystem::new(app.text_system());
    let mut readings = Vec::with_capacity(requests.len());
    for request in requests {
        let text = request["text"].as_str().unwrap();
        let face = if request["face"] == "mono" {
            Face::Mono
        } else {
            Face::Text
        };
        let size = request["size"].as_f64().unwrap() as f32;
        let style = TextStyle {
            font_family: face.family().into(),
            font_weight: FontWeight(request["weight"].as_f64().unwrap_or(400.) as f32),
            ..Default::default()
        };
        let runs = dazzui::foundation::text_runs_with_system(text, &style, px(size), &system);
        let base_font = system.resolve_font(&style.font());
        let line = system.shape_line(text.to_owned().into(), px(size), &runs, None);
        let glyphs = line
                        .runs
                        .iter()
                        .flat_map(|run| run.glyphs.iter())
                        .map(
                            |glyph| json!({"glyph": format!("{:?}", glyph.id), "index": glyph.index, "x": f32::from(glyph.position.x), "y": f32::from(glyph.position.y)}),
                        )
                        .collect::<Vec<_>>();
        let rows = request["wrap"].as_f64().map(|width| {
            let lines = system
                .shape_text(
                    text.to_owned().into(),
                    px(size),
                    &runs,
                    Some(px(width as f32)),
                    None,
                )
                .unwrap();
            lines
                .iter()
                .flat_map(|line| {
                    if line.row_layouts.is_empty() {
                        vec![row_value(0..line.len(), &line.unwrapped_layout)]
                    } else {
                        line.row_layouts
                            .iter()
                            .map(|row| row_value(row.range.clone(), &row.layout))
                            .collect()
                    }
                })
                .collect::<Vec<_>>()
        });
        readings.push(json!({
            "request": request,
            "family": face.family(),
            "families": runs.iter().map(|run| run.font.family.as_ref()).collect::<Vec<_>>(),
            "width": f32::from(line.width),
            "resolvedMetrics": {
                "ascent": f32::from(system.ascent(base_font, px(size))).abs(),
                "descent": f32::from(system.descent(base_font, px(size))).abs(),
                "lineGap": f32::from(system.line_gap(base_font, px(size))),
            },
            "runMetrics": line.runs.iter().map(|run| json!({
                "ascent": f32::from(system.ascent(run.font_id, px(size))),
                "descent": f32::from(system.descent(run.font_id, px(size))).abs(),
                "lineGap": f32::from(system.line_gap(run.font_id, px(size))),
            })).collect::<Vec<_>>(),
            "carets": text.char_indices().map(|(at, _)| at).chain([text.len()]).map(|at| json!({
                "index": at,
                "x": f32::from(line.x_for_index(at)),
                "secondary": f32::from(line.secondary_x_for_index(at)),
            })).collect::<Vec<_>>(),
            "glyphs": glyphs,
            "rows": rows,
        }));
    }
    println!(
        "{}",
        json!({"platform": std::env::consts::OS, "readings": readings})
    );
}

fn row_value(range: std::ops::Range<usize>, layout: &gpui::LineLayout) -> Value {
    json!({
        "source": [range.start, range.end],
        "width": f32::from(layout.width),
        "carets": layout.carets.iter().map(|stop| json!({
            "index": stop.index,
            "leading": f32::from(stop.leading),
            "trailing": f32::from(stop.trailing),
            "primary": f32::from(stop.primary()),
            "embeddingLevel": stop.embedding_level,
            "preferLeading": stop.prefer_leading,
            "insertionEdges": stop.insertion_edges.map(|(a,b)| [f32::from(a),f32::from(b)]),
        })).collect::<Vec<_>>(),
        "glyphs": layout.runs.iter().flat_map(|run| &run.glyphs).map(|glyph| json!({
            "glyph": format!("{:?}", glyph.id),
            "index": glyph.index,
            "x": f32::from(glyph.position.x),
            "y": f32::from(glyph.position.y),
        })).collect::<Vec<_>>(),
    })
}
