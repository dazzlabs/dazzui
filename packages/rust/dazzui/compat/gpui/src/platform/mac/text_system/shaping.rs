//! OpenType positioning for native runs whose glyph/cluster identities agree.
//! Core Text still resolves fonts and insertion points. A shared identity map
//! lets their positions follow HarfBuzz without guessing which native glyph a
//! positioning adjustment belongs to.

use crate::FontFeatures;
use collections::HashMap;
use core_foundation::{
    base::TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    number::CFNumber,
};
use core_graphics::{
    base::CGGlyph,
    geometry::{CGPoint, CGSize},
};
use core_text::{
    font::{CTFont, CTFontRef},
    font_descriptor::kCTFontOrientationHorizontal,
};
use std::sync::Arc;
use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};

#[link(name = "CoreText", kind = "framework")]
unsafe extern "C" {
    fn CTFontCopyVariation(font: CTFontRef) -> CFDictionaryRef;
}

#[derive(Default)]
pub(super) struct FontCache(HashMap<(String, bool), Option<harfrust::Font>>);

pub(super) struct RunPositions {
    pub positions: Vec<CGPoint>,
    pub advances: Vec<CGSize>,
    pub width: f64,
}

struct NativeMetrics<'a> {
    font: &'a CTFont,
    native_advances: bool,
}

impl harfrust::FontFuncs for NativeMetrics<'_> {
    fn glyph_h_advance(
        &self,
        shaper: &harfrust::ShaperFont<'_, '_>,
        glyph: harfrust::GlyphId,
    ) -> i32 {
        let Ok(glyph) = CGGlyph::try_from(glyph.to_u32()) else {
            return shaper.default_glyph_h_advance(glyph);
        };
        let mut advance = CGSize::new(0., 0.);
        unsafe {
            self.font.get_advances_for_glyphs(
                kCTFontOrientationHorizontal,
                &glyph,
                &mut advance,
                1,
            );
        }
        if self.native_advances {
            (advance.width * 65536.).round() as i32
        } else {
            // Blink's tracked outline fonts use integer hmtx variation
            // advances. Round in design units before the scaled GPOS pass.
            let units = f64::from(self.font.units_per_em());
            let design = (advance.width * units / self.font.pt_size()).round();
            let scale = (self.font.pt_size() * 65536.).round();
            (design * scale / units).round() as i32
        }
    }
}

impl FontCache {
    pub fn shape_fragment(
        &mut self,
        font: &CTFont,
        features: &FontFeatures,
        paragraph: &str,
        range: std::ops::Range<usize>,
        rtl: bool,
        native_advances: bool,
    ) -> Option<harfrust::Buffer> {
        self.shape(
            font,
            features,
            &paragraph[range.clone()],
            rtl,
            &paragraph[..range.start],
            &paragraph[range.end..],
            native_advances,
            true,
        )
    }

    fn shape(
        &mut self,
        font: &CTFont,
        features: &FontFeatures,
        text: &str,
        rtl: bool,
        pre: &str,
        post: &str,
        native_advances: bool,
        tracking: bool,
    ) -> Option<harfrust::Buffer> {
        let source = self
            .0
            .entry((font.postscript_name(), tracking))
            .or_insert_with(|| font_data(font, tracking))
            .as_ref()?;
        let mut face = source.clone();
        // Native instances retain weight, optical size and other explicit
        // axes. Reopening only the family would silently shape its defaults.
        face = face
            .instance_builder()
            .variations(font_variations(font))
            .build();
        // Fresh rows let the shaper apply tracking to its own grapheme
        // groups. Repositioned native runs keep the native tracking map.
        let features = features
            .tag_value_list()
            .iter()
            .filter_map(|(tag, value)| {
                Some(harfrust::Feature::new(
                    harfrust::Tag::new(tag.as_bytes().try_into().ok()?),
                    *value,
                    ..,
                ))
            })
            .collect::<Vec<_>>();
        let mut buffer = harfrust::Buffer::new();
        buffer.push_str(text);
        buffer.set_pre_context(pre);
        buffer.set_post_context(post);
        buffer.set_direction(if rtl {
            harfrust::Direction::RightToLeft
        } else {
            harfrust::Direction::LeftToRight
        });
        buffer.guess_segment_properties();
        // Scale before GPOS, retaining fractional variation deltas in the
        // browser's 16.16 coordinate space instead of rounding in font units.
        let metrics = NativeMetrics {
            font,
            native_advances,
        };
        let shaping_font = harfrust::ShaperFont::new(&face)
            .with_scale((font.pt_size() * 65536.).round() as i32)
            .with_font_funcs(Some(&metrics));
        harfrust::shape(
            &shaping_font,
            &mut buffer,
            harfrust::ShapeOptions::new()
                .features(&features)
                .point_size(Some(font.pt_size() as f32)),
        )
        .ok()?;
        if !buffer.allocation_successful() {
            return None;
        }
        Some(buffer)
    }

    pub fn position_rtl_run(
        &mut self,
        font: &CTFont,
        features: &FontFeatures,
        text: &str,
        glyphs: &[CGGlyph],
        indices: &[isize],
        utf16_start: isize,
        origin: f64,
        native_advances: bool,
    ) -> Option<RunPositions> {
        let buffer = self.shape(font, features, text, true, "", "", native_advances, false)?;
        // Core Text omits invisible format glyphs. Filter them after shaping:
        // asking HarfBuzz to remove them merges their clusters into adjacent
        // letters, losing the source identity needed by the native caret map.
        let shaped_glyphs = buffer
            .glyph_infos()
            .iter()
            .zip(buffer.glyph_positions())
            .filter(|(info, position)| {
                !(position.x_advance == 0
                    && position.y_advance == 0
                    && text
                        .get(info.cluster as usize..)
                        .and_then(|suffix| suffix.chars().next())
                        .is_some_and(|ch| ch.general_category() == GeneralCategory::Format))
            })
            .collect::<Vec<_>>();
        // Different substitutions or cluster partitions need a complete
        // source-map replacement; this positioning seam must not invent one.
        let mut utf16 = utf16_start;
        let source_indices = text
            .char_indices()
            .map(|(byte, ch)| {
                let index = (byte as u32, utf16);
                utf16 += ch.len_utf16() as isize;
                index
            })
            .collect::<Vec<_>>();
        if shaped_glyphs.len() != glyphs.len()
            || indices.len() != glyphs.len()
            || !shaped_glyphs
                .iter()
                .zip(glyphs)
                .zip(indices)
                .all(|(((info, _), glyph), index)| {
                    info.glyph_id == u32::from(*glyph)
                        && source_indices
                            .binary_search_by_key(&info.cluster, |&(byte, _)| byte)
                            .is_ok_and(|at| source_indices[at].1 == *index)
                })
        {
            return None;
        }
        let scaled = |value: i32| f64::from(value) / 65536.;
        let mut x = origin;
        let mut positions = Vec::with_capacity(glyphs.len());
        let mut advances = Vec::with_capacity(glyphs.len());
        for (_, position) in &shaped_glyphs {
            let advance = scaled(position.x_advance);
            positions.push(CGPoint::new(
                x + scaled(position.x_offset),
                scaled(position.y_offset),
            ));
            advances.push(CGSize::new(advance, 0.));
            x += advance;
        }
        Some(RunPositions {
            positions,
            advances,
            width: x - origin,
        })
    }
}

pub(super) fn font_variations(font: &CTFont) -> Vec<harfrust::font::Variation> {
    let variations = unsafe { CTFontCopyVariation(font.as_concrete_TypeRef()) };
    if !variations.is_null() {
        let variations: CFDictionary<CFNumber, CFNumber> =
            unsafe { CFDictionary::wrap_under_create_rule(variations) };
        let (keys, values) = variations.get_keys_and_values();
        let axes = keys
            .into_iter()
            .zip(values)
            .filter_map(|(key, value)| {
                let key = unsafe { CFNumber::wrap_under_get_rule(key.cast()) };
                let value = unsafe { CFNumber::wrap_under_get_rule(value.cast()) };
                Some(harfrust::font::Variation {
                    tag: harfrust::Tag::new(&(key.to_i64()? as u32).to_be_bytes()),
                    value: value.to_f64()? as f32,
                })
            })
            .collect::<Vec<_>>();
        return axes;
    }
    Vec::new()
}

/// Internal system faces often have no font file URL. Public Core Text table
/// access preserves the selected face without relying on private font paths.
fn font_data(font: &CTFont, tracking: bool) -> Option<harfrust::Font> {
    let font = font.clone();
    let tables = harfrust::font::TableFunction::new(Arc::new(move |tag| {
        if !tracking && tag == harfrust::Tag::new(b"trak") {
            return None;
        }
        font.get_font_table(u32::from_be_bytes(tag.to_be_bytes()))
            .map(|data| data.bytes().to_vec().into())
    }));
    harfrust::Font::new(tables, 0)
}
