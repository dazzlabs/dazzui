//! Submitted selection paint and the native layout's per-row fragments.

#[allow(dead_code)]
mod support;

use dazzui::foundation::text_input::{InputState, TextInput};
use gpui::{
    AppContext, CaretStop, Context, Entity, IntoElement, LineLayout, ObservedPaint, ParentElement,
    Render, ShapedGlyph, ShapedRun, Styled, TestAppContext, Window, WrapBoundary,
    WrappedLineLayout, div, point, px, rgb,
};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use support::paint::PaintProbe;

#[gpui::test]
fn wrapped_selection_rectangles_keep_their_row_origins(cx: &mut TestAppContext) {
    let cx = cx.add_empty_window();
    let (font_id, glyph_id) = cx.update(|window, _| {
        let line = window.text_system().shape_line(
            "a".into(),
            px(12.),
            &[gpui::TextStyle::default().to_run(1)],
            None,
        );
        (line.runs[0].font_id, line.runs[0].glyphs[0].id)
    });
    // Native glyph/caret data for three four-character rows. Soft separators
    // belong to editor paint, so this layout API returns only selected text.
    let line = WrappedLineLayout {
        unwrapped_layout: Arc::new(LineLayout {
            width: px(120.),
            len: 12,
            carets: (0..=12)
                .map(|i| CaretStop::new(i, px(i as f32 * 10.), px(i as f32 * 10.), 0, 0))
                .collect(),
            runs: vec![ShapedRun {
                font_id,
                glyphs: (0..12)
                    .map(|i| ShapedGlyph {
                        id: glyph_id,
                        position: point(px(i as f32 * 10.), px(0.)),
                        index: i,
                        is_emoji: false,
                    })
                    .collect(),
            }],
            ..Default::default()
        }),
        wrap_boundaries: [4, 8]
            .into_iter()
            .map(|glyph_ix| WrapBoundary {
                run_ix: 0,
                glyph_ix,
            })
            .collect(),
        row_layouts: Vec::new(),
        wrap_width: Some(px(45.)),
    };
    let fragments = |range| {
        line.selection_rects(range, px(19.1875))
            .unwrap()
            .into_iter()
            .map(|r| (r.left(), r.top(), r.right(), r.bottom()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        fragments(1..11),
        vec![
            (px(10.), px(0.), px(40.), px(19.1875)),
            (px(0.), px(19.1875), px(40.), px(38.375)),
            (px(0.), px(38.375), px(30.), px(57.5625)),
        ]
    );
    assert_eq!(
        fragments(4..8),
        vec![(px(0.), px(19.1875), px(40.), px(38.375))]
    );
    assert!(fragments(4..4).is_empty());
}

struct Harness {
    input: Entity<InputState>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .ml(px(0.25))
            .mt(px(0.25))
            .w(px(74.))
            .font_family("monospace")
            .text_size(px(12.))
            .line_height(px(19.2))
            .child(PaintProbe {
                child: TextInput::new("input")
                    .state(self.input.clone())
                    .multiline(true)
                    .rows(4)
                    .selection_color(rgb(0xff00ff))
                    .into_any_element(),
                paints: self.paints.clone(),
            })
    }
}

#[gpui::test]
fn newline_selection_is_short_and_submitted_edges_are_css_pixels(cx: &mut TestAppContext) {
    cx.update(dazzui::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        input: cx.new(|cx| InputState::new(window, cx)),
        paints: Rc::default(),
    });
    let input = view.read_with(cx, |view, _| view.input.clone());
    cx.update(|window, cx| {
        input.read(cx).focus(window);
        input.update(cx, |input, cx| {
            input.set_value("one\n\nthree", cx);
            input.select_all(cx);
        });
    });
    cx.run_until_parked();
    let paint = view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone());
    let selection: Vec<_> = paint
        .quads
        .iter()
        .filter(|q| q.background.solid_color() == Some(rgb(0xff00ff).into()))
        .map(|q| q.bounds)
        .collect();
    assert_eq!(selection.len(), 3);
    for rect in &selection {
        assert!(rect.size.width > px(0.) && rect.size.width < px(50.));
        for edge in [rect.left(), rect.right(), rect.top(), rect.bottom()] {
            assert_eq!(edge, edge.round(), "submitted selection edge {edge:?}");
        }
    }
    assert_eq!(selection[0].bottom(), selection[1].top());
    assert_eq!(selection[1].bottom(), selection[2].top());
    assert!(selection[1].size.width < selection[0].size.width);
}

#[test]
fn bidi_rows_use_logical_ranges_for_carets_hits_and_selection() {
    use gpui::{WrappedRowLayout, size};
    let row = || {
        Arc::new(LineLayout {
            len: 6,
            width: px(30.),
            carets: vec![
                CaretStop::new(0, px(30.), px(0.), 0, 1),
                CaretStop::new(2, px(20.), px(20.), 1, 1),
                CaretStop::new(4, px(10.), px(10.), 1, 1),
                CaretStop::new(6, px(30.), px(0.), 1, 0),
            ],
            ..Default::default()
        })
    };
    let line = WrappedLineLayout {
        unwrapped_layout: Arc::new(LineLayout {
            len: 12,
            width: px(60.),
            ..Default::default()
        }),
        // The glyph marker is deliberately unrelated to the logical row
        // range. Bidi geometry must use the row's source range throughout.
        wrap_boundaries: [WrapBoundary {
            run_ix: 0,
            glyph_ix: 0,
        }]
        .into_iter()
        .collect(),
        row_layouts: vec![
            WrappedRowLayout {
                range: 0..6,
                layout: row(),
            },
            WrappedRowLayout {
                range: 6..12,
                layout: row(),
            },
        ],
        wrap_width: Some(px(35.)),
    };
    assert_eq!(line.wrap_source_index(0), 6);
    assert_eq!(line.size(px(18.5)), size(px(35.), px(37.)));
    assert_eq!(
        line.position_for_index(8, px(18.5)),
        Some(point(px(20.), px(18.5)))
    );
    assert_eq!(
        line.closest_index_for_position(point(px(20.1), px(22.)), px(18.5)),
        Ok(8)
    );
    assert_eq!(
        line.closest_index_for_position(point(px(40.), px(22.)), px(18.5)),
        Err(12)
    );
    let rects = line.selection_rects(2..10, px(18.5)).unwrap();
    assert_eq!(rects.len(), 2);
    assert_eq!(
        (rects[0].left(), rects[0].top(), rects[0].right()),
        (px(0.), px(0.), px(20.))
    );
    assert_eq!(
        (rects[1].left(), rects[1].top(), rects[1].right()),
        (px(10.), px(18.5), px(30.))
    );
}

#[test]
fn selection_edges_do_not_replace_native_insertion_positions() {
    let mut end = CaretStop::new(1, px(20.75), px(20.75), 0, 0);
    end.insertion_edges = Some((px(21.), px(21.)));
    let line = WrappedLineLayout {
        unwrapped_layout: Arc::new(LineLayout {
            len: 1,
            width: px(25.),
            carets: vec![CaretStop::new(0, px(0.), px(0.), 0, 0), end],
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(
        line.position_for_index(1, px(19.)),
        Some(point(px(21.), px(0.)))
    );
    assert_eq!(
        line.selection_rects(0..1, px(19.)).unwrap()[0].right(),
        px(20.75)
    );
    assert_eq!(
        line.closest_index_for_position(point(px(21.), px(0.)), px(19.)),
        Ok(1)
    );
}

#[test]
fn bidi_highlight_uses_the_browser_inline_grid_at_half_pixel_edges() {
    // Recorded native offsets and the browser's selected-alef interval.
    // Raw native rounding loses a complete CSS highlight column here.
    let line = WrappedLineLayout {
        unwrapped_layout: Arc::new(LineLayout {
            len: 6,
            width: px(27.48046875),
            carets: vec![
                CaretStop::new(0, px(0.), px(0.), 0, 0),
                CaretStop::new(4, px(27.48046875), px(24.08203125), 0, 1),
                CaretStop::new(6, px(27.48046875), px(24.08203125), 1, 0),
            ],
            ..Default::default()
        }),
        ..Default::default()
    };
    let selected = line.selection_rects(4..6, px(16.)).unwrap();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].left(), px(24.09375));
    assert_eq!(selected[0].right(), px(27.5));
    assert_eq!(line.unwrapped_layout.inline_x_for_index(6), Some(px(27.5)));
    assert_eq!(line.unwrapped_layout.x_for_index(6), px(27.48046875));
}

#[test]
fn wrapped_highlights_preserve_the_rows_allocated_inline_origin() {
    use gpui::WrappedRowLayout;
    // The first fragment's allocated advance can exceed its rounded
    // selection edge by 1/64px. Reallocating from that edge loses a pixel
    // when the following fragment ends at a half-CSS-pixel boundary.
    let row = Arc::new(LineLayout {
        len: 3,
        width: px(10.5),
        carets: vec![
            CaretStop::new(0, px(0.), px(0.), 0, 0),
            CaretStop::new(1, px(10.5), px(5.5), 0, 1),
            CaretStop::new(3, px(10.5), px(5.515625), 1, 0),
        ],
        ..Default::default()
    });
    let line = WrappedLineLayout {
        unwrapped_layout: Arc::new(LineLayout {
            len: 6,
            ..Default::default()
        }),
        row_layouts: vec![
            WrappedRowLayout {
                range: 0..3,
                layout: row.clone(),
            },
            WrappedRowLayout {
                range: 3..6,
                layout: row,
            },
        ],
        ..Default::default()
    };
    let selected = line.selection_rects(4..6, px(16.)).unwrap();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].left(), px(5.515625));
    assert_eq!(selected[0].right(), px(10.5));
    assert_eq!(selected[0].top(), px(16.));
}
