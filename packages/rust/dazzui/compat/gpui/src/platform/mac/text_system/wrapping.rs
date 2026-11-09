//! Re-shape soft rows with paragraph context, rather than slicing a ligature.

use super::*;
use std::{collections::BTreeMap, ops::Range};
use unicode_properties::GeneralCategory;
use unicode_script::{Script, UnicodeScript};

fn bidi_control(ch: char) -> bool {
    matches!(ch, '\u{061c}' | '\u{200e}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

impl MacTextSystemState {
    pub(super) fn layout_fragment(
        &mut self,
        bidi: &unicode_bidi::ParagraphBidiInfo<'_>,
        range: Range<usize>,
        font_size: Pixels,
        font_runs: &[FontRun],
    ) -> Option<LineLayout> {
        let text = &bidi.text[range.clone()];
        let mut at = 0;
        let styles = font_runs
            .iter()
            .filter_map(|run| {
                let source = at..at + run.len;
                at = source.end;
                let overlap = source.start.max(range.start)..source.end.min(range.end);
                (overlap.start < overlap.end).then_some((overlap, run.font_id))
            })
            .collect::<Vec<_>>();
        let sliced = styles
            .iter()
            .map(|(range, font_id)| FontRun {
                len: range.len(),
                font_id: *font_id,
            })
            .collect::<Vec<_>>();
        let template = self.layout_line(text, font_size, &sliced);
        // Use the actual native fallback faces, including their requested
        // weight. A physical glyph shared by different font runs is not a
        // sufficient source map for replacing those runs.
        let mut fonts = BTreeMap::new();
        for run in &template.runs {
            for glyph in &run.glyphs {
                if let Some(previous) = fonts.insert(glyph.index, run.font_id)
                    && previous != run.font_id
                {
                    return None;
                }
            }
        }
        let initial_font = fonts
            .first_key_value()
            .map(|(_, id)| *id)
            .or_else(|| sliced.first().map(|run| run.font_id))?;
        fonts.entry(0).or_insert(initial_font);
        let chars = text.char_indices().collect::<Vec<_>>();
        let strong =
            |script| !matches!(script, Script::Common | Script::Inherited | Script::Unknown);
        let mut script = chars
            .iter()
            .map(|(_, ch)| ch.script())
            .find(|script| strong(*script))
            .unwrap_or(Script::Common);
        let mut spans = Vec::<(Range<usize>, FontId, FontId, Script, unicode_bidi::Level)>::new();
        for &(at, ch) in &chars {
            if strong(ch.script()) {
                script = ch.script();
            }
            let font = *fonts.range(..=at).next_back()?.1;
            let source = range.start + at;
            let requested = styles.iter().find(|(style, _)| style.contains(&source))?.1;
            let level = bidi.levels[source];
            if let Some((span, previous, style, previous_script, previous_level)) = spans.last_mut()
                && (*previous, *style, *previous_script, *previous_level)
                    == (font, requested, script, level)
            {
                span.end = at + ch.len_utf8();
            } else {
                spans.push((at..at + ch.len_utf8(), font, requested, script, level));
            }
        }
        let mut carets = Vec::new();
        let mut before = 0;
        for index in text.char_indices().map(|(at, _)| at).chain([text.len()]) {
            let after = bidi
                .levels
                .get(range.start + index)
                .filter(|_| index < text.len())
                .map_or(0, |level| level.number());
            carets.push(CaretStop::new(index, px(0.), px(0.), before, after));
            before = after;
        }
        let mut runs = Vec::new();
        let mut x = 0.;
        let levels = spans.iter().map(|span| span.4).collect::<Vec<_>>();
        for at in unicode_bidi::ParagraphBidiInfo::reorder_visual(&levels) {
            let (span, font_id, requested, _, level) = spans[at].clone();
            let rtl = level.is_rtl();
            self.tracking_advance(font_id, font_size);
            let font = self.sized_font(font_id, font_size);
            let font = if self.suppressed_tracking.contains(&font_id) {
                without_tracking(&font)?
            } else {
                font
            };
            let features = self
                .font_queries
                .get(&requested)
                .map(|font| font.features.clone())
                .unwrap_or_default();
            let native_advances = self
                .integer_advances
                .get(&font_id)
                .is_none_or(Option::is_none);
            let buffer = self.shaping_fonts.shape_fragment(
                &font,
                &features,
                bidi.text,
                range.start + span.start..range.start + span.end,
                rtl,
                native_advances,
            )?;
            let mut clusters = BTreeMap::<usize, (f32, f32)>::new();
            let mut glyphs = Vec::new();
            for (info, position) in buffer.glyph_infos().iter().zip(buffer.glyph_positions()) {
                let index = span.start + info.cluster as usize;
                if position.x_advance == 0
                    && text[index..]
                        .chars()
                        .next()
                        .is_some_and(|ch| ch.general_category() == GeneralCategory::Format)
                {
                    continue;
                }
                let advance = position.x_advance as f32 / 65536.;
                glyphs.push(ShapedGlyph {
                    id: GlyphId(info.glyph_id),
                    index,
                    position: point(
                        px(x + position.x_offset as f32 / 65536.),
                        px(-position.y_offset as f32 / 65536.),
                    ),
                    is_emoji: self.is_emoji(font_id),
                });
                clusters
                    .entry(index)
                    .and_modify(|edge| {
                        edge.0 = edge.0.min(x);
                        edge.1 = edge.1.max(x + advance);
                    })
                    .or_insert((x, x + advance));
                x += advance;
            }
            let clusters = clusters.into_iter().collect::<Vec<_>>();
            for (cluster, &(start, (left, right))) in clusters.iter().enumerate() {
                let mut end = clusters.get(cluster + 1).map_or(span.end, |(at, _)| *at);
                if let Some((at, _)) = text[start..end]
                    .char_indices()
                    .find(|(_, ch)| bidi_control(*ch))
                {
                    end = start + at;
                }
                let graphemes = text[start..end].grapheme_indices(true).collect::<Vec<_>>();
                for (step, &(offset, grapheme)) in graphemes.iter().enumerate() {
                    let a = step as f32 / graphemes.len() as f32;
                    let b = (step + 1) as f32 / graphemes.len() as f32;
                    let edge = |fraction| {
                        px(if rtl {
                            right - (right - left) * fraction
                        } else {
                            left + (right - left) * fraction
                        })
                    };
                    let first = carets
                        .binary_search_by_key(&(start + offset), |stop| stop.index)
                        .ok()?;
                    let last = carets
                        .binary_search_by_key(&(start + offset + grapheme.len()), |stop| stop.index)
                        .ok()?;
                    carets[first].leading = edge(a);
                    carets[last].trailing = edge(b);
                    for stop in &mut carets[first + 1..last] {
                        stop.leading = edge(a);
                        stop.trailing = edge(b);
                        stop.insertion_edges = Some((edge(b), edge(b)));
                    }
                }
            }
            runs.push(ShapedRun { font_id, glyphs });
        }
        let fresh = LineLayout {
            font_size,
            width: px(x),
            ascent: template.ascent,
            descent: template.descent,
            runs,
            carets,
            len: text.len(),
        };
        let local_bidi = unicode_bidi::ParagraphBidiInfo {
            text,
            original_classes: bidi.original_classes[range.clone()].to_vec(),
            levels: bidi.levels[range].to_vec(),
            paragraph_level: bidi.paragraph_level,
            is_pure_ltr: bidi.is_pure_ltr,
        };
        let mut glyphs = fresh
            .runs
            .iter()
            .flat_map(|run| run.glyphs.iter().map(|glyph| (run.font_id, glyph)))
            .collect::<Vec<_>>();
        glyphs.sort_by_key(|(_, glyph)| glyph.index);
        Some(fresh.reordered_row(&local_bidi, 0..text.len(), &glyphs))
    }
}
