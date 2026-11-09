use crate::{
    Bounds, CaretStop, DevicePixels, Font, FontFallbacks, FontFeatures, FontId, FontMetrics,
    FontRun, FontStyle, FontWeight, GlyphId, LineLayout, Pixels, PlatformTextSystem, Point,
    RenderGlyphParams, Result, SUBPIXEL_VARIANTS_X, ShapedGlyph, ShapedRun, SharedString, Size,
    point, px, size, swap_rgba_pa_to_bgra,
};
use anyhow::anyhow;
use cocoa::appkit::CGFloat;
use collections::{HashMap, HashSet};
use core_foundation::{
    attributed_string::{CFAttributedStringGetString, CFMutableAttributedString},
    base::{CFRange, CFType, CFTypeRef, TCFType},
    dictionary::CFDictionary,
    number::CFNumber,
    string::{CFString, CFStringGetCharacters},
};
use core_graphics::{
    base::{CGGlyph, kCGImageAlphaPremultipliedLast},
    color_space::CGColorSpace,
    context::{CGContext, CGTextDrawingMode},
    display::CGPoint,
    geometry::CGSize,
};
use core_text::{
    font::CTFont,
    font_descriptor::{
        TraitAccessors, kCTFontBoldTrait, kCTFontCondensedTrait, kCTFontExpandedTrait,
        kCTFontFamilyNameAttribute, kCTFontFeatureSettingsAttribute, kCTFontItalicTrait,
        kCTFontOrientationHorizontal, kCTFontSlantTrait, kCTFontSymbolicTrait,
        kCTFontTraitsAttribute, kCTFontWeightTrait, kCTFontWidthTrait,
    },
    line::{CTLine, CTLineRef},
    run::CTRunRef,
    string_attributes::{kCTFontAttributeName, kCTParagraphStyleAttributeName},
};
use font_kit::{
    font::Font as FontKitFont,
    handle::Handle,
    hinting::HintingOptions,
    metrics::Metrics,
    properties::{Style as FontkitStyle, Weight as FontkitWeight},
    source::SystemSource,
    sources::mem::MemSource,
    tracking::Tracking,
};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use pathfinder_geometry::{
    rect::{RectF, RectI},
    transform2d::Transform2F,
    vector::{Vector2F, Vector2I},
};
use smallvec::SmallVec;
use std::{borrow::Cow, char, convert::TryFrom, sync::Arc};
use unicode_properties::{GeneralCategoryGroup, UnicodeGeneralCategory};
use unicode_segmentation::UnicodeSegmentation;

use super::open_type::apply_features_and_fallbacks;

mod shaping;
mod wrapping;

#[allow(non_upper_case_globals)]
const kCGImageAlphaOnly: u32 = 7;

#[link(name = "CoreText", kind = "framework")]
unsafe extern "C" {
    fn CTLineGetOffsetForStringIndex(
        line: CTLineRef,
        index: isize,
        secondary: *mut CGFloat,
    ) -> CGFloat;
    fn CTParagraphStyleCreate(settings: *const ParagraphSetting, count: usize) -> CFTypeRef;
    fn CTRunGetStringRange(run: CTRunRef) -> CFRange;
    fn CTRunGetAdvances(run: CTRunRef, range: CFRange, advances: *mut CGSize);
    fn CTRunGetStatus(run: CTRunRef) -> u32;
}

#[repr(C)]
struct ParagraphSetting {
    spec: u32,
    size: usize,
    value: *const std::ffi::c_void,
}

pub(crate) struct MacTextSystem(RwLock<MacTextSystemState>);

#[derive(Clone, PartialEq, Eq, Hash)]
struct FontKey {
    font_family: SharedString,
    font_features: FontFeatures,
    font_fallbacks: Option<FontFallbacks>,
}

#[derive(PartialEq, Eq, Hash)]
struct NativeFontKey {
    name: String,
    scale: u32,
    axes: Vec<(u32, u32)>,
}

struct MacTextSystemState {
    memory_source: MemSource,
    system_source: SystemSource,
    fonts: Vec<FontKitFont>,
    font_selections: HashMap<Font, FontId>,
    font_queries: HashMap<FontId, Font>,
    font_ids_by_postscript_name: HashMap<String, FontId>,
    native_font_ids: HashMap<NativeFontKey, FontId>,
    font_scales: HashMap<FontId, f32>,
    font_ids_by_font_key: HashMap<FontKey, SmallVec<[FontId; 4]>>,
    postscript_names_by_font_id: HashMap<FontId, String>,
    /// UTF-16 indices of ZWNJS
    zwnjs_scratch_space: Vec<usize>,
    tracking: HashMap<FontId, Option<(Tracking, u32)>>,
    suppressed_tracking: HashSet<FontId>,
    fallback_fonts: HashMap<(FontId, String), Option<CTFont>>,
    integer_advances: HashMap<FontId, Option<Vec<u16>>>,
    shaping_fonts: shaping::FontCache,
}

impl MacTextSystem {
    pub(crate) fn new() -> Self {
        Self(RwLock::new(MacTextSystemState {
            memory_source: MemSource::empty(),
            system_source: SystemSource::new(),
            fonts: Vec::new(),
            font_selections: HashMap::default(),
            font_queries: HashMap::default(),
            font_ids_by_postscript_name: HashMap::default(),
            native_font_ids: HashMap::default(),
            font_scales: HashMap::default(),
            font_ids_by_font_key: HashMap::default(),
            postscript_names_by_font_id: HashMap::default(),
            zwnjs_scratch_space: Vec::new(),
            tracking: HashMap::default(),
            suppressed_tracking: HashSet::default(),
            fallback_fonts: HashMap::default(),
            integer_advances: HashMap::default(),
            shaping_fonts: shaping::FontCache::default(),
        }))
    }
}

impl Default for MacTextSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformTextSystem for MacTextSystem {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        self.0.write().add_fonts(fonts)
    }

    fn all_font_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        let collection = core_text::font_collection::create_for_all_families();
        let Some(descriptors) = collection.get_descriptors() else {
            return names;
        };
        for descriptor in descriptors.into_iter() {
            names.extend(lenient_font_attributes::family_name(&descriptor));
        }
        if let Ok(fonts_in_memory) = self.0.read().memory_source.all_families() {
            names.extend(fonts_in_memory);
        }
        names
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        let lock = self.0.upgradable_read();
        if let Some(font_id) = lock.font_selections.get(font) {
            Ok(*font_id)
        } else {
            let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
            let font_key = FontKey {
                font_family: font.family.clone(),
                font_features: font.features.clone(),
                font_fallbacks: font.fallbacks.clone(),
            };
            let candidates = if let Some(font_ids) = lock.font_ids_by_font_key.get(&font_key) {
                font_ids.as_slice()
            } else {
                let font_ids =
                    lock.load_family(&font.family, &font.features, font.fallbacks.as_ref())?;
                lock.font_ids_by_font_key.insert(font_key.clone(), font_ids);
                lock.font_ids_by_font_key[&font_key].as_ref()
            };

            // Match on the weight each face draws at rather than the one its
            // file declares: a variable instance's trait still names the
            // face's own weight, and CSS face matching given a 500 it cannot
            // find in the traits prefers Regular over a nearer Medium.
            let candidate_properties = candidates
                .iter()
                .map(|font_id| font_kit::css_properties(&lock.fonts[font_id.0]))
                .collect::<SmallVec<[_; 4]>>();

            let ix = font_kit::matching::find_best_match(
                &candidate_properties,
                &font_kit::properties::Properties {
                    style: font.style.into(),
                    weight: font.weight.into(),
                    stretch: Default::default(),
                },
            )?;

            let mut font_id = candidates[ix];
            if lock
                .font_queries
                .get(&font_id)
                .is_some_and(|query| query != font)
            {
                // Different CSS requests can select the same primary face,
                // yet require different fallback faces (Menlo 500 and 600).
                let instance = lock.fonts[font_id.0].clone();
                let name = instance.postscript_name().unwrap();
                font_id = FontId(lock.fonts.len());
                lock.fonts.push(instance);
                lock.postscript_names_by_font_id.insert(font_id, name);
            }
            lock.font_queries.insert(font_id, font.clone());
            lock.font_selections.insert(font.clone(), font_id);
            Ok(font_id)
        }
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        let state = self.0.read();
        let scale = state.font_scale(font_id);
        let mut metrics: FontMetrics = state.fonts[font_id.0].metrics().into();
        metrics.ascent *= scale;
        metrics.descent *= scale;
        metrics.line_gap *= scale;
        metrics.cap_height *= scale;
        metrics.x_height *= scale;
        metrics.underline_position *= scale;
        metrics.underline_thickness *= scale;
        metrics.bounding_box.origin = metrics.bounding_box.origin.map(|value| value * scale);
        metrics.bounding_box.size = metrics.bounding_box.size.map(|value| value * scale);
        metrics
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        let state = self.0.read();
        let bounds: Bounds<f32> = state.fonts[font_id.0]
            .typographic_bounds(glyph_id.0)?
            .into();
        Ok(bounds.map(|value| value * state.font_scale(font_id)))
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        self.0.read().advance(font_id, glyph_id)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.0.read().glyph_for_char(font_id, ch)
    }

    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        self.0.read().raster_bounds(params)
    }

    fn rasterize_glyph(
        &self,
        glyph_id: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        self.0.read().rasterize_glyph(glyph_id, raster_bounds)
    }

    fn layout_line(&self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        self.0.write().layout_line(text, font_size, font_runs)
    }

    fn layout_fragment(
        &self,
        bidi: &unicode_bidi::ParagraphBidiInfo<'_>,
        range: std::ops::Range<usize>,
        font_size: Pixels,
        runs: &[FontRun],
    ) -> Option<LineLayout> {
        self.0.write().layout_fragment(bidi, range, font_size, runs)
    }
}

impl MacTextSystemState {
    fn add_fonts(&mut self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        let fonts = fonts
            .into_iter()
            .map(|bytes| match bytes {
                Cow::Borrowed(embedded_font) => {
                    let data_provider = unsafe {
                        core_graphics::data_provider::CGDataProvider::from_slice(embedded_font)
                    };
                    let font = core_graphics::font::CGFont::from_data_provider(data_provider)
                        .map_err(|()| anyhow!("Could not load an embedded font."))?;
                    let font = font_kit::loaders::core_text::Font::from_core_graphics_font(font);
                    Ok(Handle::from_native(&font))
                }
                Cow::Owned(bytes) => Ok(Handle::from_memory(Arc::new(bytes), 0)),
            })
            .collect::<Result<Vec<_>>>()?;
        self.memory_source.add_fonts(fonts.into_iter())?;
        Ok(())
    }

    fn load_family(
        &mut self,
        name: &str,
        features: &FontFeatures,
        fallbacks: Option<&FontFallbacks>,
    ) -> Result<SmallVec<[FontId; 4]>> {
        let name = crate::text_system::font_name_with_fallbacks(name, ".AppleSystemUIFont");

        let mut font_ids = SmallVec::new();
        let family = self
            .memory_source
            .select_family_by_name(name)
            .or_else(|_| self.system_source.select_family_by_name(name))?;
        for font in family.fonts() {
            let mut font = font.load()?;

            apply_features_and_fallbacks(&mut font, features, fallbacks)?;
            // A valid face may contain only symbols or non-Latin glyphs.
            // Missing-character metric queries already return a Result.

            // We've seen a number of panics in production caused by calling font.properties()
            // which unwraps a downcast to CFNumber. This is an attempt to avoid the panic,
            // and to try and identify the incalcitrant font.
            let traits = font.native_font().all_traits();
            if unsafe {
                !(traits
                    .get(kCTFontSymbolicTrait)
                    .downcast::<CFNumber>()
                    .is_some()
                    && traits
                        .get(kCTFontWidthTrait)
                        .downcast::<CFNumber>()
                        .is_some()
                    && traits
                        .get(kCTFontWeightTrait)
                        .downcast::<CFNumber>()
                        .is_some()
                    && traits
                        .get(kCTFontSlantTrait)
                        .downcast::<CFNumber>()
                        .is_some())
            } {
                log::error!(
                    "Failed to read traits for font {:?}",
                    font.postscript_name().unwrap()
                );
                continue;
            }

            let font_id = FontId(self.fonts.len());
            font_ids.push(font_id);
            let postscript_name = font.postscript_name().unwrap();
            self.font_ids_by_postscript_name
                .insert(postscript_name.clone(), font_id);
            self.postscript_names_by_font_id
                .insert(font_id, postscript_name);
            self.fonts.push(font);
        }
        Ok(font_ids)
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        let advance: Size<f32> = self.fonts[font_id.0].advance(glyph_id.0)?.into();
        Ok(advance.map(|value| value * self.font_scale(font_id)))
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.fonts[font_id.0].glyph_for_char(ch).map(GlyphId)
    }

    fn font_scale(&self, font_id: FontId) -> f32 {
        self.font_scales.get(&font_id).copied().unwrap_or(1.)
    }

    fn sized_font(&self, font_id: FontId, size: Pixels) -> CTFont {
        self.fonts[font_id.0]
            .native_font()
            .clone_with_font_size(f64::from(size.0 * self.font_scale(font_id)))
    }

    fn id_for_native_font(&mut self, requested_font: CTFont, nominal_size: Pixels) -> FontId {
        let postscript_name = requested_font.postscript_name();
        let scale = if nominal_size.0 > 0. {
            (requested_font.pt_size() / f64::from(nominal_size.0)) as f32
        } else {
            1.
        };
        let mut axes = shaping::font_variations(&requested_font)
            .into_iter()
            .map(|axis| {
                (
                    u32::from_be_bytes(axis.tag.to_be_bytes()),
                    axis.value.to_bits(),
                )
            })
            .collect::<Vec<_>>();
        axes.sort_unstable();
        let key = NativeFontKey {
            name: postscript_name.clone(),
            scale: scale.to_bits(),
            axes,
        };
        if let Some(font_id) = self.native_font_ids.get(&key) {
            *font_id
        } else {
            let font_id = FontId(self.fonts.len());
            self.native_font_ids.insert(key, font_id);
            self.font_scales.insert(font_id, scale);
            self.postscript_names_by_font_id
                .insert(font_id, postscript_name);
            self.fonts
                .push(unsafe { FontKitFont::from_native_font(&requested_font) });
            font_id
        }
    }

    fn is_emoji(&self, font_id: FontId) -> bool {
        self.postscript_names_by_font_id
            .get(&font_id)
            .is_some_and(|postscript_name| {
                postscript_name == "AppleColorEmoji" || postscript_name == ".AppleColorEmojiUI"
            })
    }

    fn raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        let font = &self.fonts[params.font_id.0];
        let font_size = params.font_size * self.font_scale(params.font_id);
        if params.is_emoji {
            // Bitmap strikes are selected at device size, not by scaling a
            // logical-size glyph. Their ink bounds need not scale linearly.
            let native = font
                .native_font()
                .clone_with_font_size((f32::from(font_size) * params.scale_factor) as CGFloat);
            let rect = native.get_bounding_rects_for_glyphs(
                core_text::font_descriptor::kCTFontDefaultOrientation,
                &[params.glyph_id.0 as CGGlyph],
            );
            return Ok(RectF::new(
                Vector2F::new(
                    rect.origin.x as f32,
                    (-rect.origin.y - rect.size.height) as f32,
                ),
                Vector2F::new(rect.size.width as f32, rect.size.height as f32),
            )
            .round_out()
            .to_i32()
            .into());
        }
        let scale = Transform2F::from_scale(params.scale_factor);
        Ok(font
            .raster_bounds(
                params.glyph_id.0,
                font_size.into(),
                scale,
                HintingOptions::None,
                font_kit::canvas::RasterizationOptions::GrayscaleAa,
            )?
            .into())
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        glyph_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        if glyph_bounds.size.width.0 == 0 || glyph_bounds.size.height.0 == 0 {
            anyhow::bail!("glyph bounds are empty");
        } else {
            // Add an extra pixel when the subpixel variant isn't zero to make room for anti-aliasing.
            let mut bitmap_size = glyph_bounds.size;
            if params.subpixel_variant.x > 0 {
                bitmap_size.width += DevicePixels(1);
            }
            if params.subpixel_variant.y > 0 {
                bitmap_size.height += DevicePixels(1);
            }
            let bitmap_size = bitmap_size;

            let mut bytes;
            let cx;
            if params.is_emoji {
                bytes = vec![0; bitmap_size.width.0 as usize * 4 * bitmap_size.height.0 as usize];
                cx = CGContext::create_bitmap_context(
                    Some(bytes.as_mut_ptr() as *mut _),
                    bitmap_size.width.0 as usize,
                    bitmap_size.height.0 as usize,
                    8,
                    bitmap_size.width.0 as usize * 4,
                    &CGColorSpace::create_device_rgb(),
                    kCGImageAlphaPremultipliedLast,
                );
            } else {
                bytes = vec![0; bitmap_size.width.0 as usize * bitmap_size.height.0 as usize];
                cx = CGContext::create_bitmap_context(
                    Some(bytes.as_mut_ptr() as *mut _),
                    bitmap_size.width.0 as usize,
                    bitmap_size.height.0 as usize,
                    8,
                    bitmap_size.width.0 as usize,
                    &CGColorSpace::create_device_gray(),
                    kCGImageAlphaOnly,
                );
            }

            // Move the origin to bottom left and account for scaling, this
            // makes drawing text consistent with the font-kit's raster_bounds.
            cx.translate(
                -glyph_bounds.origin.x.0 as CGFloat,
                (glyph_bounds.origin.y.0 + glyph_bounds.size.height.0) as CGFloat,
            );
            let render_scale = if params.is_emoji {
                1.
            } else {
                params.scale_factor
            };
            cx.scale(render_scale as CGFloat, render_scale as CGFloat);

            let subpixel_shift = params
                .subpixel_variant
                .map(|v| v as f32 / SUBPIXEL_VARIANTS_X as f32);
            cx.set_text_drawing_mode(CGTextDrawingMode::CGTextFill);
            cx.set_gray_fill_color(0.0, 1.0);
            cx.set_allows_antialiasing(true);
            cx.set_should_antialias(true);
            // Match browser grayscale text. Core Graphics font smoothing
            // emboldens even alpha-only masks independently of antialiasing.
            cx.set_should_smooth_fonts(false);
            cx.set_allows_font_subpixel_positioning(true);
            cx.set_should_subpixel_position_fonts(true);
            cx.set_allows_font_subpixel_quantization(false);
            cx.set_should_subpixel_quantize_fonts(false);
            self.fonts[params.font_id.0]
                .native_font()
                .clone_with_font_size(
                    (f32::from(params.font_size)
                        * self.font_scale(params.font_id)
                        * if params.is_emoji {
                            params.scale_factor
                        } else {
                            1.
                        }) as CGFloat,
                )
                .draw_glyphs(
                    &[params.glyph_id.0 as CGGlyph],
                    &[CGPoint::new(
                        (subpixel_shift.x / render_scale) as CGFloat,
                        (subpixel_shift.y / render_scale * if params.is_emoji { -1. } else { 1. })
                            as CGFloat,
                    )],
                    cx,
                );

            if params.is_emoji {
                // Convert from RGBA with premultiplied alpha to BGRA with straight alpha.
                for pixel in bytes.chunks_exact_mut(4) {
                    swap_rgba_pa_to_bgra(pixel);
                }
            }

            Ok((bitmap_size, bytes))
        }
    }

    fn tracking_advance(&mut self, font_id: FontId, font_size: Pixels) -> f32 {
        let effective_size = font_size.0 * self.font_scale(font_id);
        self.tracking
            .entry(font_id)
            .or_insert_with(|| {
                let font = self.fonts[font_id.0].native_font();
                let tracking = font
                    .get_font_table(u32::from_be_bytes(*b"trak"))
                    .and_then(|data| Tracking::from_bytes(data.bytes()))?;
                // Apply tracking ourselves: Core Text and HarfBuzz may use
                // different cluster boundaries for complex scripts.
                self.suppressed_tracking.insert(font_id);
                // HarfBuzz gates AAT tracking on STAT, unlike Core Text.
                if font.get_font_table(u32::from_be_bytes(*b"STAT")).is_none() {
                    return None;
                }
                Some((tracking, self.fonts[font_id.0].metrics().units_per_em))
            })
            .as_ref()
            .map_or(0., |(track, units)| track.advance(effective_size, *units))
    }

    fn matching_fallback(&mut self, source_id: FontId, font: &CTFont) -> Option<CTFont> {
        let source = self.fonts[source_id.0].native_font();
        let family = font.family_name();
        if source.family_name() == family
            || matches!(
                font.postscript_name().as_str(),
                "AppleColorEmoji" | ".AppleColorEmojiUI"
            )
        {
            return None;
        }
        self.fallback_fonts
            .entry((source_id, family.clone()))
            .or_insert_with(|| {
                // A variable UI font can pass its axes to an unrelated
                // fallback face. Match that family independently instead.
                let traits = source.all_traits();
                let mask = kCTFontItalicTrait
                    | kCTFontBoldTrait
                    | kCTFontExpandedTrait
                    | kCTFontCondensedTrait;
                let query = self.font_queries.get(&source_id);
                let mut symbolic = source.symbolic_traits() & mask;
                if let Some(query) = query {
                    symbolic &= !(kCTFontItalicTrait | kCTFontBoldTrait);
                    if query.style != FontStyle::Normal {
                        symbolic |= kCTFontItalicTrait;
                    }
                    if query.weight.0 >= 600. {
                        symbolic |= kCTFontBoldTrait;
                    }
                }
                let weight = query.map_or_else(
                    || traits.normalized_weight(),
                    |query| core_text_weight(query.weight),
                );
                let desired = unsafe {
                    CFDictionary::from_CFType_pairs(&[
                        (
                            CFString::wrap_under_get_rule(kCTFontSymbolicTrait),
                            CFNumber::from(i64::from(symbolic)),
                        ),
                        (
                            CFString::wrap_under_get_rule(kCTFontWeightTrait),
                            CFNumber::from(weight),
                        ),
                        (
                            CFString::wrap_under_get_rule(kCTFontWidthTrait),
                            CFNumber::from(traits.normalized_width()),
                        ),
                        (
                            CFString::wrap_under_get_rule(kCTFontSlantTrait),
                            CFNumber::from(traits.normalized_slant()),
                        ),
                    ])
                };
                let mut attributes = unsafe {
                    vec![
                        (
                            CFString::wrap_under_get_rule(kCTFontFamilyNameAttribute),
                            CFString::new(&family).as_CFType(),
                        ),
                        (
                            CFString::wrap_under_get_rule(kCTFontTraitsAttribute),
                            desired.as_CFType(),
                        ),
                    ]
                };
                let features_key =
                    unsafe { CFString::wrap_under_get_rule(kCTFontFeatureSettingsAttribute) };
                if let Some(features) = source.copy_descriptor().attributes().find(&features_key) {
                    attributes.push((features_key, (*features).clone()));
                }
                let descriptor = core_text::font_descriptor::new_from_attributes(
                    &CFDictionary::from_CFType_pairs(&attributes),
                );
                let matched = core_text::font::new_from_descriptor(&descriptor, source.pt_size());
                (matched.family_name() == family).then_some(matched)
            })
            .as_ref()
            .map(|matched| matched.clone_with_font_size(font.pt_size()))
    }

    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        const ZWNJ: char = '\u{200C}';
        const ZWNJ_STR: &str = "\u{200C}";
        const ZWNJ_SIZE_16: usize = ZWNJ.len_utf16();

        let source_text = text;
        let ordered = canonical_mark_order(text, font_runs);
        let text = ordered
            .as_ref()
            .map_or(text, |ordered| ordered.text.as_str());

        self.zwnjs_scratch_space.clear();
        // Construct the attributed string, converting UTF8 ranges to UTF16 ranges.
        let mut string = CFMutableAttributedString::new();
        let mut max_ascent = 0.0f32;
        let mut max_descent = 0.0f32;
        let mut source_runs = Vec::with_capacity(font_runs.len());

        {
            let mut ix_converter = StringIndexConverter::new(&text);
            let mut last_font_run = None;
            for run in font_runs {
                let text = &text[ix_converter.utf8_ix..][..run.len];
                // if the fonts are the same, we need to disconnect the text with a ZWNJ
                // to prevent core text from forming ligatures between them
                let needs_zwnj = last_font_run.replace(run.font_id) == Some(run.font_id);

                let utf16_start = string.char_len(); // insert at end of string
                ix_converter.advance_to_utf8_ix(ix_converter.utf8_ix + run.len);

                // note: replace_str may silently ignore codepoints it dislikes (e.g., BOM at start of string)
                string.replace_str(&CFString::new(text), CFRange::init(utf16_start, 0));
                if needs_zwnj {
                    let zwnjs_pos = string.char_len();
                    self.zwnjs_scratch_space.push(zwnjs_pos as usize);
                    string.replace_str(
                        &CFString::from_static_string(ZWNJ_STR),
                        CFRange::init(zwnjs_pos, 0),
                    );
                }
                let utf16_end = string.char_len();

                let cf_range = CFRange::init(utf16_start, utf16_end - utf16_start);
                source_runs.push((cf_range, run.font_id));
                let font = &self.fonts[run.font_id.0];

                let font_metrics = font.metrics();
                let font_scale = font_size.0 / font_metrics.units_per_em as f32;
                max_ascent = max_ascent.max(font_metrics.ascent * font_scale);
                max_descent = max_descent.max(-font_metrics.descent * font_scale);

                unsafe {
                    string.set_attribute(
                        cf_range,
                        kCTFontAttributeName,
                        &self.sized_font(run.font_id, font_size),
                    );
                    // Keep logical style boundaries even when Core Text
                    // resolves two requested weights to the same physical face.
                    string.set_attribute(
                        cf_range,
                        CFString::from_static_string("GPUIRequestedFontID").as_concrete_TypeRef(),
                        &CFNumber::from(run.font_id.0 as i64),
                    );
                }
            }
        }
        let bidi = unicode_bidi::BidiInfo::new(source_text, Some(unicode_bidi::Level::ltr()));
        if bidi.has_rtl() {
            // GPUI has no paragraph direction property; its default is LTR,
            // like the browser controls. Core Text otherwise guesses from text.
            let direction: i8 = 0;
            let setting = ParagraphSetting {
                spec: 13,
                size: std::mem::size_of::<i8>(),
                value: (&direction as *const i8).cast(),
            };
            unsafe {
                let style = CFType::wrap_under_create_rule(CTParagraphStyleCreate(&setting, 1));
                string.set_attribute(
                    CFRange::init(0, string.char_len()),
                    kCTParagraphStyleAttributeName,
                    &style,
                );
            }
        }
        // Retrieve the glyphs from the shaped line, converting UTF16 offsets to UTF8 offsets.
        let mut line = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());
        let mut changed = false;
        // Fallback fonts are known only after shaping. Resolve their requested
        // style and suppress native tracking before applying browser tracking.
        for run in line.glyph_runs().into_iter() {
            let attributes = run.attributes().unwrap();
            let mut font = unsafe {
                attributes
                    .get(kCTFontAttributeName)
                    .downcast::<CTFont>()
                    .unwrap()
            };
            let range = unsafe { CTRunGetStringRange(run.as_concrete_TypeRef()) };
            let mut replace = false;
            if let Some((_, source)) = source_runs.iter().find(|(source, _)| {
                source.location <= range.location
                    && range.location < source.location + source.length
            }) && let Some(matched) = self.matching_fallback(*source, &font)
            {
                let mut characters = vec![0; range.length as usize];
                unsafe {
                    CFStringGetCharacters(
                        CFAttributedStringGetString(string.as_concrete_TypeRef()),
                        range,
                        characters.as_mut_ptr(),
                    );
                }
                let loaded = unsafe { FontKitFont::from_native_font(&matched) };
                if String::from_utf16(&characters).is_ok_and(|text| text.chars().all(|ch| {
                    ch.is_control()
                        || matches!(ch, '\u{200c}'..='\u{200d}' | '\u{fe00}'..='\u{fe0f}' | '\u{e0100}'..='\u{e01ef}')
                        || loaded.glyph_for_char(ch).is_some()
                })) {
                    font = matched;
                    replace = true;
                }
            }
            // Browser emoji fallback uses the public face, whose metrics
            // differ from the system UI cascade's scaled emoji face.
            if font.postscript_name() == ".AppleColorEmojiUI"
                && let Ok(emoji) =
                    core_text::font::new_from_name("Apple Color Emoji", font.pt_size())
            {
                font = emoji;
                replace = true;
            }
            let font_id = self.id_for_native_font(font.clone(), font_size);
            self.tracking_advance(font_id, font_size);
            if self.suppressed_tracking.contains(&font_id) {
                let Some(untracked) = without_tracking(&font) else {
                    continue;
                };
                font = untracked;
                replace = true;
            }
            if replace {
                unsafe {
                    string.set_attribute(range, kCTFontAttributeName, &font);
                }
                changed = true;
            }
        }
        if changed {
            line = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());
        }
        let glyph_runs = line.glyph_runs();
        let mut runs = <Vec<ShapedRun>>::with_capacity(glyph_runs.len() as usize);
        let mut ix_converter = StringIndexConverter::new(text);
        // Browser tracking follows grapheme boundaries independently of
        // Core Text's glyph clusters. Resolve boundaries only when needed.
        let mut continuations: Option<Vec<(usize, bool)>> = None;
        let mut advance_shift = 0.;
        let mut caret_coordinates = Vec::new();
        let mut tracked_cluster: Option<(usize, f32)> = None;
        let has_controls = source_text.chars().any(|ch| matches!(ch,
            '\u{061c}' | '\u{200e}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'));
        let mut cluster_edges = std::collections::BTreeMap::<usize, (Pixels, Pixels)>::new();
        let mut run_offset = 0.;
        for run in glyph_runs.into_iter() {
            let run_origin = run_offset;
            let native_width = run.get_typographic_bounds().width;
            run_offset += native_width;
            let attributes = run.attributes().unwrap();
            let font = unsafe {
                attributes
                    .get(kCTFontAttributeName)
                    .downcast::<CTFont>()
                    .unwrap()
            };
            let range = unsafe { CTRunGetStringRange(run.as_concrete_TypeRef()) };
            let source = source_runs
                .iter()
                .find(|(source, _)| {
                    source.location <= range.location
                        && range.location < source.location + source.length
                })
                .map(|(_, source)| *source);
            let font_id = match source {
                Some(source)
                    if self.fonts[source.0]
                        .native_font()
                        .clone_with_font_size(font.pt_size())
                        .postscript_name()
                        == font.postscript_name() =>
                {
                    source
                }
                _ => self.id_for_native_font(font.clone(), font_size),
            };
            let integer_advances = self.integer_advances.entry(font_id).or_insert_with(|| {
                // Blink uses HarfBuzz advances for tracked outline fonts;
                // other fonts retain Skia/Core Text's fractional metrics.
                if font.get_font_table(u32::from_be_bytes(*b"fvar")).is_none()
                    || font.get_font_table(u32::from_be_bytes(*b"trak")).is_none()
                    || font.get_font_table(u32::from_be_bytes(*b"sbix")).is_some()
                {
                    return None;
                }
                let hhea = font.get_font_table(u32::from_be_bytes(*b"hhea"))?;
                let count = u16::from_be_bytes(hhea.bytes().get(34..36)?.try_into().ok()?);
                let hmtx = font.get_font_table(u32::from_be_bytes(*b"hmtx"))?;
                let metrics = hmtx.bytes().get(..usize::from(count) * 4)?;
                Some(
                    metrics
                        .chunks_exact(4)
                        .map(|record| u16::from_be_bytes([record[0], record[1]]))
                        .collect(),
                )
            });
            let glyph_count = run.glyph_count() as usize;
            let mut control_advances = Vec::new();
            if has_controls {
                control_advances.resize(glyph_count, CGSize::new(0., 0.));
                unsafe {
                    CTRunGetAdvances(
                        run.as_concrete_TypeRef(),
                        CFRange::init(0, 0),
                        control_advances.as_mut_ptr(),
                    );
                }
            }
            let mut corrections = SmallVec::<[f32; 32]>::new();
            if let Some(metrics) = integer_advances
                .as_ref()
                .filter(|metrics| !metrics.is_empty())
            {
                corrections.resize(glyph_count, 0.);
                let mut advances = vec![CGSize::new(0., 0.); glyph_count];
                let mut shaped = advances.clone();
                unsafe {
                    font.get_advances_for_glyphs(
                        kCTFontOrientationHorizontal,
                        run.glyphs().as_ptr(),
                        advances.as_mut_ptr(),
                        glyph_count as _,
                    );
                    CTRunGetAdvances(
                        run.as_concrete_TypeRef(),
                        CFRange::init(0, 0),
                        shaped.as_mut_ptr(),
                    );
                }
                for (((correction, advance), shaped), glyph) in corrections
                    .iter_mut()
                    .zip(advances)
                    .zip(shaped)
                    .zip(run.glyphs().iter())
                {
                    // GPOS can zero a mark's advance after hmtx lookup.
                    if shaped.width != 0. {
                        // The final hmtx advance repeats for remaining glyphs.
                        let base = metrics[usize::from(*glyph).min(metrics.len() - 1)];
                        *correction = variable_advance_correction(
                            advance.width,
                            font.pt_size(),
                            font.units_per_em(),
                            base,
                        );
                    }
                }
            }
            let native_positions = run.positions();
            let positioned = if unsafe { CTRunGetStatus(run.as_concrete_TypeRef()) } & 1 != 0 {
                let mut characters = vec![0; range.length as usize];
                unsafe {
                    CFStringGetCharacters(
                        CFAttributedStringGetString(string.as_concrete_TypeRef()),
                        range,
                        characters.as_mut_ptr(),
                    );
                }
                let features = source
                    .and_then(|id| self.font_queries.get(&id))
                    .map(|font| font.features.clone())
                    .unwrap_or_default();
                String::from_utf16(&characters).ok().and_then(|text| {
                    self.shaping_fonts.position_rtl_run(
                        &font,
                        &features,
                        &text,
                        &run.glyphs(),
                        &run.string_indices(),
                        range.location,
                        run_origin,
                        integer_advances.is_none(),
                    )
                })
            } else {
                None
            };
            if let Some(positioned) = &positioned {
                corrections.clear();
                if has_controls {
                    control_advances.clone_from(&positioned.advances);
                }
            }
            let positions = positioned
                .as_ref()
                .map_or(native_positions.as_ref(), |run| &run.positions);
            let adjustment = self.tracking_advance(font_id, font_size);
            if adjustment != 0. && continuations.is_none() {
                continuations = Some(tracking_continuations(text));
            }
            let mut first_glyph = true;

            let mut glyphs = match runs.last_mut() {
                Some(run) if run.font_id == font_id => &mut run.glyphs,
                _ => {
                    runs.push(ShapedRun {
                        font_id,
                        glyphs: Vec::with_capacity(run.glyph_count().try_into().unwrap_or(0)),
                    });
                    &mut runs.last_mut().unwrap().glyphs
                }
            };
            for (glyph_at, (((&glyph_id, position), &glyph_utf16_ix), correction)) in run
                .glyphs()
                .iter()
                .zip(positions.iter())
                .zip(run.string_indices().iter())
                .zip(corrections.into_iter().chain(std::iter::repeat(0.)))
                .enumerate()
            {
                let mut glyph_utf16_ix = usize::try_from(glyph_utf16_ix).unwrap();
                let r = self
                    .zwnjs_scratch_space
                    .binary_search_by(|&it| it.cmp(&glyph_utf16_ix));
                match r {
                    // this glyph is a ZWNJ, skip it
                    Ok(_) => continue,
                    // adjust the index to account for the ZWNJs we've inserted
                    Err(idx) => glyph_utf16_ix -= idx * ZWNJ_SIZE_16,
                }
                if ix_converter.utf16_ix > glyph_utf16_ix {
                    // We cannot reuse current index converter, as it can only seek forward. Restart the search.
                    ix_converter = StringIndexConverter::new(text);
                }
                ix_converter.advance_to_utf16_ix(glyph_utf16_ix);
                if let Some(continuations) = &continuations {
                    let at = ix_converter.utf8_ix;
                    let continued = continuations
                        .binary_search_by_key(&at, |&(at, _)| at)
                        .ok()
                        .is_some_and(|index| continuations[index].1);
                    if first_glyph
                        || tracked_cluster.is_none_or(|(previous, _)| previous != at && !continued)
                    {
                        if let Some((_, adjustment)) = tracked_cluster {
                            advance_shift += adjustment;
                        }
                        tracked_cluster = Some((at, adjustment));
                    }
                }
                first_glyph = false;
                caret_coordinates.push((
                    native_positions[glyph_at].x as f32,
                    position.x as f32 + advance_shift,
                ));
                let source_index = ordered.as_ref().map_or(ix_converter.utf8_ix, |ordered| {
                    ordered.source_index(ix_converter.utf8_ix)
                });
                if let Some(advance) = control_advances
                    .get(glyph_at)
                    .filter(|advance| advance.width > 0.)
                    .filter(|_| {
                        source_text[source_index..]
                            .chars()
                            .next()
                            .unwrap()
                            .general_category_group()
                            != GeneralCategoryGroup::Mark
                    })
                {
                    let left = px(position.x as f32 + advance_shift);
                    let right = left + px(advance.width as f32 + correction);
                    cluster_edges
                        .entry(source_index)
                        .and_modify(|edges| {
                            edges.0 = edges.0.min(left);
                            edges.1 = edges.1.max(right);
                        })
                        .or_insert((left, right));
                }
                glyphs.push(ShapedGlyph {
                    id: GlyphId(glyph_id as u32),
                    position: point(position.x as f32 + advance_shift, -position.y as f32).map(px),
                    index: source_index,
                    is_emoji: self.is_emoji(font_id),
                });
                advance_shift += correction;
            }
            if let Some(positioned) = positioned {
                advance_shift += (positioned.width - native_width) as f32;
            }
        }
        let typographic_bounds = line.get_typographic_bounds();
        if let Some((_, adjustment)) = tracked_cluster {
            advance_shift += adjustment;
        }
        let width = typographic_bounds.width as f32 + advance_shift;
        let mut carets = Vec::new();
        caret_coordinates.push((typographic_bounds.width as f32, width));
        caret_coordinates.sort_by(|a, b| a.0.total_cmp(&b.0));
        caret_coordinates.dedup_by(|a, b| a.0 == b.0);
        let corrected = |value: f64| {
            let x = value as f32;
            let at = caret_coordinates.partition_point(|(raw, _)| *raw < x);
            if at == 0 {
                return px(caret_coordinates[0].1 + x - caret_coordinates[0].0);
            }
            if at == caret_coordinates.len() {
                return px(width + x - typographic_bounds.width as f32);
            }
            let (a, left) = caret_coordinates[at - 1];
            let (b, right) = caret_coordinates[at];
            px(left + (right - left) * (x - a) / (b - a))
        };
        let mut line_levels = bidi.levels.clone();
        for paragraph in &bidi.paragraphs {
            let resolved = bidi.reordered_levels(paragraph, paragraph.range.clone());
            line_levels[paragraph.range.clone()]
                .copy_from_slice(&resolved[paragraph.range.clone()]);
        }
        let mut utf16 = 0usize;
        let mut previous_level = 0;
        for (index, ch) in source_text
            .char_indices()
            .map(|(i, ch)| (i, Some(ch)))
            .chain([(text.len(), None)])
        {
            let mut native_index = ordered
                .as_ref()
                .map_or(utf16, |ordered| ordered.native_index(index));
            for &inserted in &self.zwnjs_scratch_space {
                if inserted <= native_index {
                    native_index += 1;
                }
            }
            let mut secondary = 0.;
            let primary = unsafe {
                CTLineGetOffsetForStringIndex(
                    line.as_concrete_TypeRef(),
                    native_index as _,
                    &mut secondary,
                )
            };
            let after = line_levels.get(index).map_or(0, |level| level.number());
            let (low, high) = (
                corrected(primary.min(secondary)),
                corrected(primary.max(secondary)),
            );
            let (trailing, leading) = if previous_level.min(after) % 2 == 0 {
                (low, high)
            } else {
                (high, low)
            };
            let trailing = if index == 0 { px(0.) } else { trailing };
            let leading = if index == text.len() {
                px(width)
            } else {
                leading
            };
            carets.push(CaretStop::new(
                index,
                leading,
                trailing,
                previous_level,
                after,
            ));
            previous_level = after;
            if let Some(ch) = ch {
                utf16 += ch.len_utf16();
            }
        }
        if bidi.has_rtl() {
            for stop in &mut carets {
                stop.insertion_edges = Some((stop.leading, stop.trailing));
            }
            // Selection boxes stay within the shaped advance, while native
            // insertion stops can extrapolate beyond it under RTL kerning.
            for stop in &mut carets {
                stop.leading = stop.leading.clamp(px(0.), px(width));
                stop.trailing = stop.trailing.clamp(px(0.), px(width));
            }
            let is_control = |ch| {
                matches!(ch, '\u{061c}' | '\u{200e}'..='\u{200f}'
                | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
            };
            // CT can attach a formatting control to a distant visual edge.
            // Recover the preceding cluster's end from its advancing glyphs.
            // Zero-advance marks must not enlarge that cluster's selection.
            for index in 1..carets.len() - 1 {
                let at = carets[index].index;
                if is_control(source_text[at..].chars().next().unwrap())
                    && !is_control(
                        source_text[carets[index - 1].index..]
                            .chars()
                            .next()
                            .unwrap(),
                    )
                    && let Some((_, &(left, right))) = cluster_edges.range(..at).next_back()
                {
                    carets[index].trailing = if carets[index - 1].embedding_level % 2 == 1 {
                        left
                    } else {
                        right
                    };
                }
            }
            // CT can omit internal offsets in a ligature next to an isolate.
            // Preserve genuine native stops; when every reported side lies
            // on an outer edge, distribute its graphemes across the native
            // advancing cluster instead of assigning the whole glyph twice.
            let clusters: Vec<_> = cluster_edges.iter().collect();
            for (cluster, (&start, &(left, right))) in clusters.iter().copied().enumerate() {
                let mut end = clusters
                    .get(cluster + 1)
                    .map_or(source_text.len(), |(at, _)| **at);
                if let Some((at, _)) = source_text[start..end]
                    .char_indices()
                    .find(|(_, ch)| is_control(*ch))
                {
                    end = start + at;
                }
                let graphemes: Vec<_> = source_text[start..end].grapheme_indices(true).collect();
                if graphemes.len() < 2 {
                    continue;
                }
                for (step, (offset, _)) in graphemes.iter().enumerate().skip(1) {
                    let at = start + offset;
                    let Ok(index) = carets.binary_search_by_key(&at, |stop| stop.index) else {
                        continue;
                    };
                    let inside = |edge| left < edge && edge < right;
                    if inside(carets[index].leading) || inside(carets[index].trailing) {
                        continue;
                    }
                    let fraction = step as f32 / graphemes.len() as f32;
                    let edge = if carets[index].embedding_level % 2 == 1 {
                        right - (right - left) * fraction
                    } else {
                        left + (right - left) * fraction
                    };
                    carets[index].leading = edge;
                    carets[index].trailing = edge;
                    carets[index].insertion_edges = Some((edge, edge));
                }
            }
            // Native offsets can attach an invisible control to the remote
            // edge of the adjacent bidi run. Its two source boundaries share
            // the ordinary insertion edge on the weaker surrounding level.
            let mut index = 0;
            while index + 1 < carets.len() {
                if !is_control(source_text[carets[index].index..].chars().next().unwrap()) {
                    index += 1;
                    continue;
                }
                let start = index;
                while index + 1 < carets.len()
                    && is_control(source_text[carets[index].index..].chars().next().unwrap())
                {
                    index += 1;
                }
                let before = start
                    .checked_sub(1)
                    .map_or(0, |at| carets[at].embedding_level);
                let after = carets[index].embedding_level;
                let edge = if before <= after {
                    carets[start].trailing
                } else {
                    carets[index].leading
                };
                for stop in &mut carets[start..=index] {
                    stop.insertion_edges = Some((edge, edge));
                }
            }
            // Explicit formatting characters have no selection advance. Keep
            // insertion edges independent of these zero-ink selection boxes.
            for index in (0..carets.len() - 1).rev() {
                if is_control(source_text[carets[index].index..].chars().next().unwrap()) {
                    let edge = carets[index + 1].leading;
                    carets[index].leading = edge;
                    carets[index + 1].trailing = edge;
                }
            }
            // Core Text can attach a combining mark's scalar offsets to a
            // remote bidi boundary. Selection of any part of its grapheme
            // uses the complete cluster; native insertion edges stay intact.
            for (start, cluster) in source_text.grapheme_indices(true) {
                let first = carets.partition_point(|stop| stop.index < start);
                let last = carets.partition_point(|stop| stop.index < start + cluster.len());
                let (leading, trailing) = (carets[first].leading, carets[last].trailing);
                for stop in &mut carets[first + 1..last] {
                    stop.leading = leading;
                    stop.trailing = trailing;
                }
            }
        }
        if !bidi.has_rtl() {
            // Native insertion offsets can differ from painted cluster
            // origins. The browser uses the following
            // cluster's painted origin there. Keep native interpolation only
            // for grapheme boundaries inside a ligature with no own glyph.
            let boundaries = source_text
                .grapheme_indices(true)
                .map(|(at, _)| at)
                .chain([source_text.len()])
                .collect::<Vec<_>>();
            let mut positions = vec![None::<Pixels>; boundaries.len()];
            for glyph in runs.iter().flat_map(|run| &run.glyphs) {
                let at = boundaries
                    .partition_point(|index| *index <= glyph.index)
                    .saturating_sub(1);
                positions[at] =
                    Some(positions[at].map_or(glyph.position.x, |x| x.min(glyph.position.x)));
            }
            *positions.last_mut().unwrap() = Some(px(width));
            carets = boundaries
                .into_iter()
                .zip(positions)
                .map(|(index, position)| {
                    let x = position.unwrap_or_else(|| {
                        carets[carets
                            .binary_search_by_key(&index, |stop| stop.index)
                            .unwrap()]
                        .primary()
                    });
                    CaretStop::new(index, x, x, 0, 0)
                })
                .collect();
        }
        LineLayout {
            runs,
            font_size,
            width: width.into(),
            carets,
            ascent: max_ascent.into(),
            descent: max_descent.into(),
            len: text.len(),
        }
    }
}

fn without_tracking(font: &CTFont) -> Option<CTFont> {
    let attributes = CFDictionary::from_CFType_pairs(&[(
        CFString::from_static_string("NSCTFontUnscaledTrackingAttribute"),
        CFNumber::from(0),
    )]);
    let descriptor = font
        .copy_descriptor()
        .create_copy_with_attributes(attributes.into_untyped())
        .ok()?;
    Some(core_text::font::new_from_descriptor(
        &descriptor,
        font.pt_size(),
    ))
}

struct OrderedText {
    text: String,
    // Native UTF-8 byte, original UTF-8 byte, native UTF-16 position.
    indices: Vec<(usize, usize, usize)>,
}

impl OrderedText {
    fn source_index(&self, native: usize) -> usize {
        self.indices
            .binary_search_by_key(&native, |&(byte, _, _)| byte)
            .map(|at| self.indices[at].1)
            .unwrap_or(self.text.len())
    }

    fn native_index(&self, source: usize) -> usize {
        self.indices
            .iter()
            .find(|&&(_, original, _)| original == source)
            .map_or_else(|| self.text.encode_utf16().count(), |&(_, _, utf16)| utf16)
    }
}

// Core Text can place a noncanonically ordered accent an entire em below
// the baseline. Reorder only combining marks, retaining font boundaries and
// a source map: editable text and platform input indices stay untouched.
fn canonical_mark_order(text: &str, runs: &[FontRun]) -> Option<OrderedText> {
    use unicode_ccc::get_canonical_combining_class;

    let mut start = 0;
    let mut needs_ordering = false;
    for run in runs {
        let mut previous = 0;
        for ch in text[start..start + run.len].chars() {
            let class = get_canonical_combining_class(ch) as u8;
            needs_ordering |= class != 0 && previous > class;
            previous = class;
        }
        start += run.len;
    }
    if !needs_ordering {
        return None;
    }
    let mut ordered = OrderedText {
        text: String::with_capacity(text.len()),
        indices: Vec::with_capacity(text.chars().count()),
    };
    let mut utf16 = 0;
    start = 0;
    for run in runs {
        let mut chars: Vec<_> = text[start..start + run.len]
            .char_indices()
            .map(|(byte, ch)| (start + byte, ch, get_canonical_combining_class(ch) as u8))
            .collect();
        let mut marks = 0;
        for at in 0..=chars.len() {
            if at == chars.len() || chars[at].2 == 0 {
                chars[marks..at].sort_by_key(|&(_, _, class)| class);
                marks = at + 1;
            }
        }
        for (source, ch, _) in chars {
            ordered.indices.push((ordered.text.len(), source, utf16));
            ordered.text.push(ch);
            utf16 += ch.len_utf16();
        }
        start += run.len;
    }
    Some(ordered)
}

/// HarfBuzz keeps continuation flags on glyphs through reordering. Full
/// Unicode grapheme boundaries cannot replace these: a pre-base Indic mark
/// can precede the base that starts its tracking group in visual order.
fn tracking_continuations(text: &str) -> Vec<(usize, bool)> {
    let graphemes: Vec<_> = text.grapheme_indices(true).map(|(at, _)| at).collect();
    let mut previous = None;
    let mut previous_continued = false;
    text.char_indices().map(|(at, ch)| {
        let regional = |ch: char| matches!(ch, '\u{1f1e6}'..='\u{1f1ff}');
        let continued = ch.general_category_group() == GeneralCategoryGroup::Mark
            || matches!(ch, '\u{200d}' | '\u{1f3fb}'..='\u{1f3ff}' | '\u{ff9e}'..='\u{ff9f}' | '\u{e0020}'..='\u{e007f}')
            || (previous == Some('\u{200d}') && graphemes.binary_search(&at).is_err())
            || (regional(ch) && previous.is_some_and(regional) && !previous_continued);
        previous = Some(ch);
        previous_continued = continued;
        (at, continued)
    }).collect()
}

/// HarfBuzz rounds variable hmtx deltas in design units before scaling;
/// Core Text returns fractional design-unit advances. Keep positioning and
/// kerning from the shaped run, correcting only that unpositioned advance.
fn variable_advance_correction(advance: f64, point_size: f64, units_per_em: u32, base: u16) -> f32 {
    if !advance.is_finite() || !point_size.is_finite() || point_size <= 0. || units_per_em == 0 {
        return 0.;
    }
    let units = f64::from(units_per_em);
    let base = f64::from(base);
    let design = base + (advance * units / point_size - base).round();
    let scale = (point_size * 65536.).round();
    let browser = (design * scale / units).round() / 65536.;
    (browser - advance) as f32
}

/// CSS weights map to Core Text traits independently of the primary face
/// a particular family happened to supply. This is a platform protocol,
/// not a design-system weight scale.
fn core_text_weight(weight: FontWeight) -> f64 {
    let weight = weight.0;
    if !weight.is_finite() || weight <= 50. || weight >= 950. {
        return 0.;
    }
    const WEIGHTS: [f64; 9] = [-0.8, -0.6, -0.4, 0., 0.23, 0.3, 0.4, 0.56, 0.62];
    WEIGHTS[(weight as usize - 50) / 100]
}

#[derive(Debug, Clone)]
struct StringIndexConverter<'a> {
    text: &'a str,
    /// Index in UTF-8 bytes
    utf8_ix: usize,
    /// Index in UTF-16 code units
    utf16_ix: usize,
}

impl<'a> StringIndexConverter<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            utf8_ix: 0,
            utf16_ix: 0,
        }
    }

    fn advance_to_utf8_ix(&mut self, utf8_target: usize) {
        for (ix, c) in self.text[self.utf8_ix..].char_indices() {
            if self.utf8_ix + ix >= utf8_target {
                self.utf8_ix += ix;
                return;
            }
            self.utf16_ix += c.len_utf16();
        }
        self.utf8_ix = self.text.len();
    }

    fn advance_to_utf16_ix(&mut self, utf16_target: usize) {
        for (ix, c) in self.text[self.utf8_ix..].char_indices() {
            if self.utf16_ix >= utf16_target {
                self.utf8_ix += ix;
                return;
            }
            self.utf16_ix += c.len_utf16();
        }
        self.utf8_ix = self.text.len();
    }
}

impl From<Metrics> for FontMetrics {
    fn from(metrics: Metrics) -> Self {
        FontMetrics {
            units_per_em: metrics.units_per_em,
            ascent: metrics.ascent,
            descent: metrics.descent,
            line_gap: metrics.line_gap,
            underline_position: metrics.underline_position,
            underline_thickness: metrics.underline_thickness,
            cap_height: metrics.cap_height,
            x_height: metrics.x_height,
            bounding_box: metrics.bounding_box.into(),
        }
    }
}

impl From<RectF> for Bounds<f32> {
    fn from(rect: RectF) -> Self {
        Bounds {
            origin: point(rect.origin_x(), rect.origin_y()),
            size: size(rect.width(), rect.height()),
        }
    }
}

impl From<RectI> for Bounds<DevicePixels> {
    fn from(rect: RectI) -> Self {
        Bounds {
            origin: point(DevicePixels(rect.origin_x()), DevicePixels(rect.origin_y())),
            size: size(DevicePixels(rect.width()), DevicePixels(rect.height())),
        }
    }
}

impl From<Vector2I> for Size<DevicePixels> {
    fn from(value: Vector2I) -> Self {
        size(value.x().into(), value.y().into())
    }
}

impl From<RectI> for Bounds<i32> {
    fn from(rect: RectI) -> Self {
        Bounds {
            origin: point(rect.origin_x(), rect.origin_y()),
            size: size(rect.width(), rect.height()),
        }
    }
}

impl From<Point<u32>> for Vector2I {
    fn from(size: Point<u32>) -> Self {
        Vector2I::new(size.x as i32, size.y as i32)
    }
}

impl From<Vector2F> for Size<f32> {
    fn from(vec: Vector2F) -> Self {
        size(vec.x(), vec.y())
    }
}

impl From<FontWeight> for FontkitWeight {
    fn from(value: FontWeight) -> Self {
        FontkitWeight(value.0)
    }
}

impl From<FontStyle> for FontkitStyle {
    fn from(style: FontStyle) -> Self {
        match style {
            FontStyle::Normal => FontkitStyle::Normal,
            FontStyle::Italic => FontkitStyle::Italic,
            FontStyle::Oblique => FontkitStyle::Oblique,
        }
    }
}

// Some fonts may have no attributes despite `core_text` requiring them (and panicking).
// This is the same version as `core_text` has without `expect` calls.
mod lenient_font_attributes {
    use core_foundation::{
        base::{CFRetain, CFType, TCFType},
        string::{CFString, CFStringRef},
    };
    use core_text::font_descriptor::{
        CTFontDescriptor, CTFontDescriptorCopyAttribute, kCTFontFamilyNameAttribute,
    };

    pub fn family_name(descriptor: &CTFontDescriptor) -> Option<String> {
        unsafe { get_string_attribute(descriptor, kCTFontFamilyNameAttribute) }
    }

    fn get_string_attribute(
        descriptor: &CTFontDescriptor,
        attribute: CFStringRef,
    ) -> Option<String> {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(descriptor.as_concrete_TypeRef(), attribute);
            if value.is_null() {
                return None;
            }

            let value = CFType::wrap_under_create_rule(value);
            assert!(value.instance_of::<CFString>());
            let s = wrap_under_get_rule(value.as_CFTypeRef() as CFStringRef);
            Some(s.to_string())
        }
    }

    unsafe fn wrap_under_get_rule(reference: CFStringRef) -> CFString {
        unsafe {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");
            let reference = CFRetain(reference as *const ::std::os::raw::c_void) as CFStringRef;
            TCFType::wrap_under_create_rule(reference)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FontRun, FontWeight, GlyphId, MacTextSystem, PlatformTextSystem, font, px};

    #[test]
    fn non_latin_faces_load_without_an_m_sentinel() {
        let system = MacTextSystem::new();
        system
            .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
                "../../../fixtures/colr-alpha.ttf"
            ))])
            .unwrap();
        assert!(
            system
                .all_font_names()
                .iter()
                .any(|name| name == "DazzUI Alpha Probe")
        );
        let id = system.font_id(&font("DazzUI Alpha Probe")).unwrap();
        assert!(system.glyph_for_char(id, 'A').is_some());
        assert!(system.glyph_for_char(id, 'm').is_none());
        let line = system.layout_line(
            "AA",
            px(24.),
            &[FontRun {
                font_id: id,
                len: 2,
            }],
        );
        assert_eq!(line.width, px(48.));
    }

    #[test]
    fn left_to_right_cluster_carets_follow_painted_and_native_edges() {
        let system = MacTextSystem::new();
        let id = system.font_id(&crate::font(".SystemUIFont")).unwrap();
        let text = "Xक्‍षिY";
        let line = system.layout_line(
            text,
            px(12.),
            &[FontRun {
                font_id: id,
                len: text.len(),
            }],
        );
        let start = line.x_for_index("X".len());
        let inside = line.x_for_index("Xक्‍".len());
        let end = line.x_for_index("Xक्‍षि".len());
        assert!(end > start);
        assert_eq!(inside, end);
        assert!(line.carets.iter().any(|stop| stop.index == "Xक्‍षि".len()));
        let text = "Xa\u{301}Y";
        let line = system.layout_line(
            text,
            px(12.),
            &[FontRun {
                font_id: id,
                len: text.len(),
            }],
        );
        let after = "Xa\u{301}".len();
        let painted = line
            .runs
            .iter()
            .flat_map(|run| &run.glyphs)
            .find(|glyph| glyph.index == after)
            .unwrap()
            .position
            .x;
        assert_eq!(line.x_for_index(after), painted);
        assert_eq!(line.x_for_index("Xa".len()), painted);
        assert_eq!(line.closest_index_for_x(painted), after);
    }

    #[test]
    fn combining_order_preserves_original_source_positions_and_font_boundaries() {
        let text = "😀a\u{301}\u{323}";
        let runs = [FontRun {
            font_id: crate::FontId(0),
            len: text.len(),
        }];
        let ordered = super::canonical_mark_order(text, &runs).unwrap();
        assert_eq!(ordered.text, "😀a\u{323}\u{301}");
        assert_eq!(ordered.source_index(5), 7);
        assert_eq!(ordered.source_index(7), 5);
        assert_eq!(ordered.native_index(7), 3);
        assert_eq!(ordered.native_index(5), 4);
        let mixed_lengths = "a\u{301}\u{1dce}";
        let ordered = super::canonical_mark_order(
            mixed_lengths,
            &[FontRun {
                font_id: crate::FontId(0),
                len: mixed_lengths.len(),
            }],
        )
        .unwrap();
        assert_eq!(ordered.text, "a\u{1dce}\u{301}");
        assert_eq!(ordered.source_index(1), 3);
        assert_eq!(ordered.source_index(4), 1);
        assert_eq!(ordered.native_index(3), 1);
        assert_eq!(ordered.native_index(1), 2);
        let runs = [
            FontRun {
                font_id: crate::FontId(0),
                len: 7,
            },
            FontRun {
                font_id: crate::FontId(1),
                len: 2,
            },
        ];
        assert!(super::canonical_mark_order(text, &runs).is_none());
    }

    #[test]
    fn canonically_equivalent_marks_keep_the_same_native_ink_positions() {
        let fonts = MacTextSystem::new();
        let font_id = fonts.font_id(&font(".SystemUIFont")).unwrap();
        let canonical = "a\u{323}\u{301} a\u{323}\u{301}";
        let permuted = "a\u{323}\u{301} a\u{301}\u{323}";
        let runs = [FontRun {
            font_id,
            len: canonical.len(),
        }];
        let expected = fonts.layout_line(canonical, px(12.), &runs);
        let actual = fonts.layout_line(permuted, px(12.), &runs);
        assert_eq!(actual.width, expected.width);
        let glyphs = |layout: &crate::LineLayout| {
            layout
                .runs
                .iter()
                .flat_map(|run| run.glyphs.iter().map(|glyph| (glyph.id, glyph.position)))
                .collect::<Vec<_>>()
        };
        assert_eq!(glyphs(&actual), glyphs(&expected));
        assert!(actual.runs.iter().flat_map(|run| &run.glyphs).all(|glyph| {
            permuted.is_char_boundary(glyph.index) && glyph.position.y.abs() < px(2.)
        }));
    }

    #[test]
    fn variable_metric_ties_round_the_delta_in_both_directions() {
        // hmtx's base advance is integral. HVAR rounds its signed delta,
        // so a negative half unit differs from rounding the final width.
        assert_eq!(
            super::variable_advance_correction(1002.5, 1024., 1024, 1000),
            0.5
        );
        assert_eq!(
            super::variable_advance_correction(997.5, 1024., 1024, 1000),
            -0.5
        );
        assert_eq!(
            super::variable_advance_correction(f64::NAN, 12., 2048, 1000),
            0.
        );
    }

    #[test]
    fn fallback_weights_survive_primary_face_aliases_in_one_line() {
        let fonts = MacTextSystem::new();
        for (left, right) in [(400., 500.), (600., 700.)] {
            let mut requested = font("Menlo");
            requested.weight = FontWeight(left);
            let left_id = fonts.font_id(&requested).unwrap();
            requested.weight = FontWeight(right);
            let right_id = fonts.font_id(&requested).unwrap();
            assert_ne!(left_id, right_id);
            {
                let state = fonts.0.read();
                assert_eq!(
                    state.fonts[left_id.0].postscript_name(),
                    state.fonts[right_id.0].postscript_name(),
                );
            }
            let left = "नमस्ते ";
            let right = "दुनिया";
            let styles = [
                FontRun {
                    font_id: left_id,
                    len: left.len(),
                },
                FontRun {
                    font_id: right_id,
                    len: right.len(),
                },
            ];
            let size = px(13.33);
            let expected = fonts.layout_line(left, size, &styles[..1]).width
                + fonts.layout_line(right, size, &styles[1..]).width;
            let actual = fonts
                .layout_line(&format!("{left}{right}"), size, &styles)
                .width;
            assert!(
                (actual - expected).abs() < px(0.001),
                "{actual:?} != {expected:?}"
            );
        }
    }

    #[test]
    fn test_layout_line_bom_char() {
        let fonts = MacTextSystem::new();
        let font_id = fonts.font_id(&font("Helvetica")).unwrap();
        let line = "\u{feff}";
        let mut style = FontRun {
            font_id,
            len: line.len(),
        };

        let layout = fonts.layout_line(line, px(16.), &[style]);
        assert_eq!(layout.len, line.len());
        assert!(layout.runs.is_empty());

        let line = "a\u{feff}b";
        style.len = line.len();
        let layout = fonts.layout_line(line, px(16.), &[style]);
        assert_eq!(layout.len, line.len());
        assert_eq!(layout.runs.len(), 1);
        assert_eq!(layout.runs[0].glyphs.len(), 2);
        assert_eq!(layout.runs[0].glyphs[0].id, GlyphId(68u32)); // a
        // There's no glyph for \u{feff}
        assert_eq!(layout.runs[0].glyphs[1].id, GlyphId(69u32)); // b

        let line = "\u{feff}ab";
        let font_runs = &[
            FontRun {
                len: "\u{feff}".len(),
                font_id,
            },
            FontRun {
                len: "ab".len(),
                font_id,
            },
        ];
        let layout = fonts.layout_line(line, px(16.), font_runs);
        assert_eq!(layout.len, line.len());
        assert_eq!(layout.runs.len(), 1);
        assert_eq!(layout.runs[0].glyphs.len(), 2);
        // There's no glyph for \u{feff}
        assert_eq!(layout.runs[0].glyphs[0].id, GlyphId(68u32)); // a
        assert_eq!(layout.runs[0].glyphs[1].id, GlyphId(69u32)); // b
    }

    #[test]
    fn test_layout_line_zwnj_insertion() {
        let fonts = MacTextSystem::new();
        let font_id = fonts.font_id(&font("Helvetica")).unwrap();

        let text = "hello world";
        let font_runs = &[
            FontRun { font_id, len: 5 }, // "hello"
            FontRun { font_id, len: 6 }, // " world"
        ];

        let layout = fonts.layout_line(text, px(16.), font_runs);
        assert_eq!(layout.len, text.len());

        for run in &layout.runs {
            for glyph in &run.glyphs {
                assert!(
                    glyph.index < text.len(),
                    "Glyph index {} is out of bounds for text length {}",
                    glyph.index,
                    text.len()
                );
            }
        }

        // Test with different font runs - should not insert ZWNJ
        let font_id2 = fonts.font_id(&font("Times")).unwrap_or(font_id);
        let font_runs_different = &[
            FontRun { font_id, len: 5 }, // "hello"
            // " world"
            FontRun {
                font_id: font_id2,
                len: 6,
            },
        ];

        let layout2 = fonts.layout_line(text, px(16.), font_runs_different);
        assert_eq!(layout2.len, text.len());

        for run in &layout2.runs {
            for glyph in &run.glyphs {
                assert!(
                    glyph.index < text.len(),
                    "Glyph index {} is out of bounds for text length {}",
                    glyph.index,
                    text.len()
                );
            }
        }
    }

    #[test]
    fn test_layout_line_zwnj_edge_cases() {
        let fonts = MacTextSystem::new();
        let font_id = fonts.font_id(&font("Helvetica")).unwrap();

        let text = "hello";
        let font_runs = &[FontRun { font_id, len: 5 }];
        let layout = fonts.layout_line(text, px(16.), font_runs);
        assert_eq!(layout.len, text.len());

        let text = "abc";
        let font_runs = &[
            FontRun { font_id, len: 1 }, // "a"
            FontRun { font_id, len: 1 }, // "b"
            FontRun { font_id, len: 1 }, // "c"
        ];
        let layout = fonts.layout_line(text, px(16.), font_runs);
        assert_eq!(layout.len, text.len());

        for run in &layout.runs {
            for glyph in &run.glyphs {
                assert!(
                    glyph.index < text.len(),
                    "Glyph index {} is out of bounds for text length {}",
                    glyph.index,
                    text.len()
                );
            }
        }

        // Test with empty text
        let text = "";
        let font_runs = &[];
        let layout = fonts.layout_line(text, px(16.), font_runs);
        assert_eq!(layout.len, 0);
        assert!(layout.runs.is_empty());
    }
}
