use crate::{
    Bounds, CaretStop, DevicePixels, Font, FontFeatures, FontId, FontMetrics, FontRun, FontStyle,
    FontWeight, GlyphId, LineLayout, Pixels, PlatformTextSystem, Point, RenderGlyphParams,
    SUBPIXEL_VARIANTS_X, SUBPIXEL_VARIANTS_Y, ShapedGlyph, ShapedRun, SharedString, Size, point,
    size,
};
use anyhow::{Context as _, Ok, Result};
use collections::HashMap;
use cosmic_text::{
    Attrs, AttrsList, CacheKey, Family, Font as CosmicTextFont, FontFeatures as CosmicFontFeatures,
    FontSystem, ShapeBuffer, ShapeLine, SwashCache,
};

use itertools::Itertools;
use parking_lot::RwLock;
use pathfinder_geometry::{
    rect::{RectF, RectI},
    vector::{Vector2F, Vector2I},
};
use smallvec::SmallVec;
use std::{borrow::Cow, sync::Arc};

pub(crate) struct CosmicTextSystem(RwLock<CosmicTextSystemState>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FontKey {
    family: SharedString,
    features: FontFeatures,
}

impl FontKey {
    fn new(family: SharedString, features: FontFeatures) -> Self {
        Self { family, features }
    }
}

struct CosmicTextSystemState {
    swash_cache: SwashCache,
    synthesis_context: swash::scale::ScaleContext,
    synthesis_images: HashMap<RenderGlyphParams, cosmic_text::SwashImage>,
    font_system: FontSystem,
    scratch: ShapeBuffer,
    /// Contains all already loaded fonts, including all faces. Indexed by `FontId`.
    loaded_fonts: Vec<LoadedFont>,
    /// Caches the `FontId`s associated with a specific family to avoid iterating the font database
    /// for every font face in a family.
    font_ids_by_family_cache: HashMap<FontKey, SmallVec<[FontId; 4]>>,
}

#[derive(Clone)]
struct LoadedFont {
    font: Arc<CosmicTextFont>,
    features: CosmicFontFeatures,
    is_known_emoji_font: bool,
    synthetic_bold: bool,
    synthetic_italic: bool,
}

impl CosmicTextSystem {
    pub(crate) fn new() -> Self {
        // todo(linux) make font loading non-blocking
        let mut font_system = FontSystem::new();

        Self(RwLock::new(CosmicTextSystemState {
            font_system,
            swash_cache: SwashCache::new(),
            synthesis_context: swash::scale::ScaleContext::new(),
            synthesis_images: HashMap::default(),
            scratch: ShapeBuffer::default(),
            loaded_fonts: Vec::new(),
            font_ids_by_family_cache: HashMap::default(),
        }))
    }
}

impl Default for CosmicTextSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformTextSystem for CosmicTextSystem {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        self.0.write().add_fonts(fonts)
    }

    fn all_font_names(&self) -> Vec<String> {
        let mut result = self
            .0
            .read()
            .font_system
            .db()
            .faces()
            .flat_map(|face| {
                face.families
                    .first()
                    .map(|family| family.0.clone())
                    .into_iter()
                    .chain(std::iter::once(face.post_script_name.clone()))
            })
            .collect_vec();
        result.sort();
        result.dedup();
        result
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        // todo(linux): Do we need to use CosmicText's Font APIs? Can we consolidate this to use font_kit?
        let mut state = self.0.write();
        let key = FontKey::new(font.family.clone(), font.features.clone());
        let candidates = if let Some(font_ids) = state.font_ids_by_family_cache.get(&key) {
            font_ids.as_slice()
        } else {
            let font_ids = state.load_family(&font.family, &font.features)?;
            state.font_ids_by_family_cache.insert(key.clone(), font_ids);
            state.font_ids_by_family_cache[&key].as_ref()
        };

        // todo(linux) ideally we would make fontdb's `find_best_match` pub instead of using font-kit here
        let candidate_properties = candidates
            .iter()
            .map(|font_id| {
                let database_id = state.loaded_font(*font_id).font.id();
                let face_info = state.font_system.db().face(database_id).expect("");
                face_info_into_properties(face_info)
            })
            .collect::<SmallVec<[_; 4]>>();

        let ix =
            font_kit::matching::find_best_match(&candidate_properties, &font_into_properties(font))
                .context("requested font family contains no font matching the other parameters")?;

        let selected = candidates[ix];
        let face = state
            .font_system
            .db()
            .face(state.loaded_font(selected).font.id())
            .unwrap();
        let bold = font.weight.0 >= 600. && face.weight.0 < 600;
        let italic =
            font.style != FontStyle::Normal && face.style == cosmic_text::fontdb::Style::Normal;
        if bold || italic {
            let mut loaded = state.loaded_font(selected).clone();
            loaded.synthetic_bold = bold;
            loaded.synthetic_italic = italic;
            let id = FontId(state.loaded_fonts.len());
            state.loaded_fonts.push(loaded);
            Ok(id)
        } else {
            Ok(selected)
        }
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        let metrics = self
            .0
            .read()
            .loaded_font(font_id)
            .font
            .as_swash()
            .metrics(&[]);

        FontMetrics {
            units_per_em: metrics.units_per_em as u32,
            ascent: metrics.ascent,
            descent: -metrics.descent, // todo(linux) confirm this is correct
            line_gap: metrics.leading,
            underline_position: metrics.underline_offset,
            underline_thickness: metrics.stroke_size,
            cap_height: metrics.cap_height,
            x_height: metrics.x_height,
            // todo(linux): Compute this correctly
            bounding_box: Bounds {
                origin: point(0.0, 0.0),
                size: size(metrics.max_width, metrics.ascent + metrics.descent),
            },
        }
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        let lock = self.0.read();
        let glyph_metrics = lock.loaded_font(font_id).font.as_swash().glyph_metrics(&[]);
        let glyph_id = glyph_id.0 as u16;
        // todo(linux): Compute this correctly
        // see https://github.com/servo/font-kit/blob/master/src/loaders/freetype.rs#L614-L620
        Ok(Bounds {
            origin: point(0.0, 0.0),
            size: size(
                glyph_metrics.advance_width(glyph_id),
                glyph_metrics.advance_height(glyph_id),
            ),
        })
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        self.0.read().advance(font_id, glyph_id)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.0.read().glyph_for_char(font_id, ch)
    }

    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        self.0.write().raster_bounds(params)
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        self.0.write().rasterize_glyph(params, raster_bounds)
    }

    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        self.0.write().layout_line(text, font_size, runs)
    }
}

impl CosmicTextSystemState {
    fn loaded_font(&self, font_id: FontId) -> &LoadedFont {
        &self.loaded_fonts[font_id.0]
    }

    #[profiling::function]
    fn add_fonts(&mut self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        let db = self.font_system.db_mut();
        for bytes in fonts {
            match bytes {
                Cow::Borrowed(embedded_font) => {
                    db.load_font_data(embedded_font.to_vec());
                }
                Cow::Owned(bytes) => {
                    db.load_font_data(bytes);
                }
            }
        }
        Ok(())
    }

    #[profiling::function]
    fn load_family(
        &mut self,
        name: &str,
        features: &FontFeatures,
    ) -> Result<SmallVec<[FontId; 4]>> {
        // TODO: Determine the proper system UI font.
        let name = crate::text_system::font_name_with_fallbacks(name, "IBM Plex Sans");

        let families = self
            .font_system
            .db()
            .faces()
            .filter(|face| {
                face.families.iter().any(|family| *name == family.0)
                    || face.post_script_name == name
            })
            .map(|face| (face.id, face.post_script_name.clone()))
            .collect::<SmallVec<[_; 4]>>();

        let mut loaded_font_ids = SmallVec::new();
        for (font_id, postscript_name) in families {
            let font = self
                .font_system
                .get_font(font_id)
                .context("Could not load font")?;

            let font_id = FontId(self.loaded_fonts.len());
            loaded_font_ids.push(font_id);
            self.loaded_fonts.push(LoadedFont {
                font,
                features: features.try_into()?,
                is_known_emoji_font: check_is_known_emoji_font(&postscript_name),
                synthetic_bold: false,
                synthetic_italic: false,
            });
        }

        Ok(loaded_font_ids)
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        let glyph_metrics = self.loaded_font(font_id).font.as_swash().glyph_metrics(&[]);
        Ok(Size {
            width: glyph_metrics.advance_width(glyph_id.0 as u16),
            height: glyph_metrics.advance_height(glyph_id.0 as u16),
        })
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        let glyph_id = self.loaded_font(font_id).font.as_swash().charmap().map(ch);
        if glyph_id == 0 {
            None
        } else {
            Some(GlyphId(glyph_id.into()))
        }
    }

    fn raster_image(&mut self, params: &RenderGlyphParams) -> Result<cosmic_text::SwashImage> {
        let font = self.loaded_fonts[params.font_id.0].clone();
        let subpixel = point(
            params.subpixel_variant.x as f32 / SUBPIXEL_VARIANTS_X as f32 / params.scale_factor,
            params.subpixel_variant.y as f32 / SUBPIXEL_VARIANTS_Y as f32 / params.scale_factor,
        );
        let logical_size = if bitmap_only(&font.font) {
            params.font_size
        } else {
            (params.font_size * 64.).floor() / 64.
        };
        let size = logical_size * params.scale_factor;
        let image = if (font.synthetic_bold || font.synthetic_italic) && !font.is_known_emoji_font {
            if let Some(image) = self.synthesis_images.get(params) {
                return Ok(image.clone());
            }
            use swash::{
                scale::{Render, Source, StrikeWith},
                zeno::{Angle, Format, Transform, Vector},
            };
            let mut scaler = self
                .synthesis_context
                .builder(font.font.as_swash())
                .size(f32::from(size))
                .hint(true)
                .build();
            let image = Render::new(&[
                Source::ColorOutline(0),
                Source::ColorBitmap(StrikeWith::BestFit),
                Source::Outline,
            ])
            .format(Format::Alpha)
            .offset(Vector::new(subpixel.x, subpixel.y.trunc()))
            .embolden(if font.synthetic_bold {
                f32::from(size) / 24.
            } else {
                0.
            })
            .transform(
                font.synthetic_italic
                    .then(|| Transform::skew(Angle::from_degrees(14.), Angle::from_degrees(0.))),
            )
            .render(&mut scaler, params.glyph_id.0 as u16);
            if let Some(image) = &image {
                self.synthesis_images.insert(params.clone(), image.clone());
            }
            image
        } else {
            self.swash_cache
                .get_image(
                    &mut self.font_system,
                    CacheKey::new(
                        font.font.id(),
                        params.glyph_id.0 as u16,
                        size.into(),
                        (subpixel.x, subpixel.y.trunc()),
                        cosmic_text::CacheKeyFlags::empty(),
                    )
                    .0,
                )
                .clone()
        };
        image.with_context(|| format!("no image for {params:?}"))
    }

    fn raster_bounds(&mut self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        let image = self.raster_image(params)?;
        Ok(Bounds {
            origin: point(image.placement.left.into(), (-image.placement.top).into()),
            size: size(image.placement.width.into(), image.placement.height.into()),
        })
    }

    #[profiling::function]
    fn rasterize_glyph(
        &mut self,
        params: &RenderGlyphParams,
        glyph_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        if glyph_bounds.size.width.0 == 0 || glyph_bounds.size.height.0 == 0 {
            anyhow::bail!("glyph bounds are empty");
        } else {
            let bitmap_size = glyph_bounds.size;
            let mut image = self.raster_image(params)?;

            if params.is_emoji {
                // Swash's layered color outlines are premultiplied, while
                // decoded color bitmaps are straight. The atlas API takes
                // straight BGRA in both cases, as it does for images.
                let premultiplied = matches!(image.source, swash::scale::Source::ColorOutline(_));
                for pixel in image.data.chunks_exact_mut(4) {
                    if premultiplied {
                        crate::swap_rgba_pa_to_bgra(pixel);
                    } else {
                        pixel.swap(0, 2);
                    }
                }
            }

            Ok((bitmap_size, image.data))
        }
    }

    /// This is used when cosmic_text has chosen a fallback font instead of using the requested
    /// font, typically to handle some unicode characters. When this happens, `loaded_fonts` may not
    /// yet have an entry for this fallback font, and so one is added.
    ///
    /// Note that callers shouldn't use this `FontId` somewhere that will retrieve the corresponding
    /// `LoadedFont.features`, as it will have an arbitrarily chosen or empty value. The only
    /// current use of this field is for the *input* of `layout_line`, and so it's fine to use
    /// `font_id_for_cosmic_id` when computing the *output* of `layout_line`.
    fn font_id_for_cosmic_id(&mut self, id: cosmic_text::fontdb::ID) -> FontId {
        if let Some(ix) = self
            .loaded_fonts
            .iter()
            .position(|loaded_font| loaded_font.font.id() == id)
        {
            FontId(ix)
        } else {
            let font = self.font_system.get_font(id).unwrap();
            let face = self.font_system.db().face(id).unwrap();

            let font_id = FontId(self.loaded_fonts.len());
            self.loaded_fonts.push(LoadedFont {
                font,
                features: CosmicFontFeatures::new(),
                is_known_emoji_font: check_is_known_emoji_font(&face.post_script_name),
                synthetic_bold: false,
                synthetic_italic: false,
            });

            font_id
        }
    }

    #[profiling::function]
    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        let bidi = unicode_bidi::BidiInfo::new(text, Some(unicode_bidi::Level::ltr()));
        // ShapeLine has no paragraph-direction argument. A zero-width LRM
        // fixes its paragraph base without overriding embedded RTL runs.
        let prefixed = bidi.has_rtl().then(|| format!("\u{200e}{text}"));
        let prefix = if prefixed.is_some() {
            '\u{200e}'.len_utf8()
        } else {
            0
        };
        let shaped_text = prefixed.as_deref().unwrap_or(text);
        let mut attrs_list = AttrsList::new(&Attrs::new());
        let mut offs = 0;
        for run in font_runs {
            let loaded_font = self.loaded_font(run.font_id);
            let font = self.font_system.db().face(loaded_font.font.id()).unwrap();

            let end = offs + run.len + if offs == 0 { prefix } else { 0 };
            attrs_list.add_span(
                offs..end,
                &Attrs::new()
                    .metadata(run.font_id.0)
                    .family(Family::Name(&font.families.first().unwrap().0))
                    .stretch(font.stretch)
                    .style(font.style)
                    .weight(font.weight)
                    .font_features(loaded_font.features.clone()),
            );
            offs = end;
        }

        let line = ShapeLine::new(
            &mut self.font_system,
            shaped_text,
            &attrs_list,
            cosmic_text::Shaping::Advanced,
            4,
        );
        let mut layout_lines = Vec::with_capacity(1);
        line.layout_to_buffer(
            &mut self.scratch,
            ((font_size * 64.).floor() / 64.).0,
            None, // We do our own wrapping
            cosmic_text::Wrap::None,
            None,
            &mut layout_lines,
            None,
        );
        let layout = layout_lines.first_mut().unwrap();
        // Bitmap-only fonts carry their own advances at each strike size.
        // Their hmtx design advances can disagree with those bitmap metrics.
        // Like FreeType/Skia, scale the strike advance by the requested size,
        // keeping all following glyphs and bidi insertion edges on that grid.
        let adjustments = layout
            .glyphs
            .iter()
            .enumerate()
            .filter(|(_, glyph)| glyph.w > 0.)
            .filter_map(|(index, glyph)| {
                let font = self.font_system.get_font(glyph.font_id)?;
                if !bitmap_only(&font) {
                    return None;
                }
                use skrifa::{FontRef, GlyphId, MetadataProvider, instance::Size};
                let face = self.font_system.db().face(glyph.font_id)?;
                let font = FontRef::from_index(font.data(), face.index).ok()?;
                let bitmap = font
                    .bitmap_strikes()
                    .glyph_for_size(Size::new(font_size.0), GlyphId::new(glyph.glyph_id.into()))?;
                let advance = bitmap.advance? * font_size.0 / bitmap.ppem_x;
                (bitmap.ppem_x > 0.).then_some((index, glyph.x + glyph.w, advance - glyph.w))
            })
            .collect_vec();
        if !adjustments.is_empty() {
            let mut widths = vec![0.; layout.glyphs.len()];
            let mut shifts = adjustments
                .iter()
                .map(|&(index, end, delta)| {
                    widths[index] = delta;
                    (end, delta)
                })
                .collect_vec();
            shifts.sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut total = 0.;
            for (_, shift) in &mut shifts {
                total += *shift;
                *shift = total;
            }
            for (index, glyph) in layout.glyphs.iter_mut().enumerate() {
                let after = shifts.partition_point(|(end, _)| *end <= glyph.x);
                if after > 0 {
                    glyph.x += shifts[after - 1].1;
                }
                glyph.w += widths[index];
            }
            layout.w += total;
        }

        let mut runs: Vec<ShapedRun> = Vec::new();
        for glyph in &layout.glyphs {
            if glyph.end <= prefix && prefix > 0 {
                continue;
            }
            let mut font_id = FontId(glyph.metadata);
            let mut loaded_font = self.loaded_font(font_id);
            if loaded_font.font.id() != glyph.font_id {
                font_id = self.font_id_for_cosmic_id(glyph.font_id);
                loaded_font = self.loaded_font(font_id);
            }
            let is_emoji = loaded_font.is_known_emoji_font;

            // HACK: Prevent crash caused by variation selectors.
            if glyph.glyph_id == 3 && is_emoji {
                continue;
            }

            let shaped_glyph = ShapedGlyph {
                id: GlyphId(glyph.glyph_id as u32),
                position: point(
                    (glyph.x + glyph.font_size * glyph.x_offset).into(),
                    (glyph.y - glyph.font_size * glyph.y_offset).into(),
                ),
                index: glyph.start.saturating_sub(prefix),
                is_emoji,
            };

            if let Some(last_run) = runs
                .last_mut()
                .filter(|last_run| last_run.font_id == font_id)
            {
                last_run.glyphs.push(shaped_glyph);
            } else {
                runs.push(ShapedRun {
                    font_id,
                    glyphs: vec![shaped_glyph],
                });
            }
        }

        let mut carets = Vec::new();
        if bidi.has_rtl() {
            let mut clusters = std::collections::BTreeMap::<(usize, usize), (f32, f32, u8)>::new();
            for glyph in &layout.glyphs {
                if glyph.end <= prefix {
                    continue;
                }
                let key = (
                    glyph.start.saturating_sub(prefix),
                    glyph.end.saturating_sub(prefix),
                );
                let entry = clusters.entry(key).or_insert((
                    glyph.x,
                    glyph.x + glyph.w,
                    glyph.level.number(),
                ));
                entry.0 = entry.0.min(glyph.x);
                entry.1 = entry.1.max(glyph.x + glyph.w);
            }
            for at in text.char_indices().map(|(at, _)| at).chain([text.len()]) {
                let edge = |start: usize, end: usize, left: f32, right: f32, level: u8| {
                    let count = text[start..end].chars().count().max(1);
                    let fraction = text[start..at].chars().count() as f32 / count as f32;
                    if level % 2 == 0 {
                        left + (right - left) * fraction
                    } else {
                        right - (right - left) * fraction
                    }
                };
                let (leading, after) = clusters
                    .iter()
                    .find(|&(&(start, end), _)| start <= at && at < end)
                    .map_or((layout.w, 0), |(&(start, end), &(left, right, level))| {
                        (edge(start, end, left, right, level), level)
                    });
                let (trailing, before) = clusters
                    .iter()
                    .find(|&(&(start, end), _)| start < at && at <= end)
                    .map_or((0., 0), |(&(start, end), &(left, right, level))| {
                        (edge(start, end, left, right, level), level)
                    });
                carets.push(CaretStop::new(
                    at,
                    leading.into(),
                    trailing.into(),
                    before,
                    after,
                ));
            }
        }
        LineLayout {
            font_size,
            width: layout.w.into(),
            ascent: layout.max_ascent.into(),
            descent: layout.max_descent.into(),
            runs,
            carets,
            len: text.len(),
        }
    }
}

fn bitmap_only(font: &CosmicTextFont) -> bool {
    let tables = font.rustybuzz().tables();
    tables.glyf.is_none() && tables.cff.is_none() && tables.cff2.is_none()
}

impl TryFrom<&FontFeatures> for CosmicFontFeatures {
    type Error = anyhow::Error;

    fn try_from(features: &FontFeatures) -> Result<Self> {
        let mut result = CosmicFontFeatures::new();
        for feature in features.0.iter() {
            let name_bytes: [u8; 4] = feature
                .0
                .as_bytes()
                .try_into()
                .context("Incorrect feature flag format")?;

            let tag = cosmic_text::FeatureTag::new(&name_bytes);

            result.set(tag, feature.1);
        }
        Ok(result)
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

impl From<FontWeight> for cosmic_text::Weight {
    fn from(value: FontWeight) -> Self {
        cosmic_text::Weight(value.0 as u16)
    }
}

impl From<FontStyle> for cosmic_text::Style {
    fn from(style: FontStyle) -> Self {
        match style {
            FontStyle::Normal => cosmic_text::Style::Normal,
            FontStyle::Italic => cosmic_text::Style::Italic,
            FontStyle::Oblique => cosmic_text::Style::Oblique,
        }
    }
}

fn font_into_properties(font: &crate::Font) -> font_kit::properties::Properties {
    font_kit::properties::Properties {
        style: match font.style {
            crate::FontStyle::Normal => font_kit::properties::Style::Normal,
            crate::FontStyle::Italic => font_kit::properties::Style::Italic,
            crate::FontStyle::Oblique => font_kit::properties::Style::Oblique,
        },
        weight: font_kit::properties::Weight(font.weight.0),
        stretch: Default::default(),
    }
}

fn face_info_into_properties(
    face_info: &cosmic_text::fontdb::FaceInfo,
) -> font_kit::properties::Properties {
    font_kit::properties::Properties {
        style: match face_info.style {
            cosmic_text::Style::Normal => font_kit::properties::Style::Normal,
            cosmic_text::Style::Italic => font_kit::properties::Style::Italic,
            cosmic_text::Style::Oblique => font_kit::properties::Style::Oblique,
        },
        // both libs use the same values for weight
        weight: font_kit::properties::Weight(face_info.weight.0.into()),
        stretch: match face_info.stretch {
            cosmic_text::Stretch::Condensed => font_kit::properties::Stretch::CONDENSED,
            cosmic_text::Stretch::Expanded => font_kit::properties::Stretch::EXPANDED,
            cosmic_text::Stretch::ExtraCondensed => font_kit::properties::Stretch::EXTRA_CONDENSED,
            cosmic_text::Stretch::ExtraExpanded => font_kit::properties::Stretch::EXTRA_EXPANDED,
            cosmic_text::Stretch::Normal => font_kit::properties::Stretch::NORMAL,
            cosmic_text::Stretch::SemiCondensed => font_kit::properties::Stretch::SEMI_CONDENSED,
            cosmic_text::Stretch::SemiExpanded => font_kit::properties::Stretch::SEMI_EXPANDED,
            cosmic_text::Stretch::UltraCondensed => font_kit::properties::Stretch::ULTRA_CONDENSED,
            cosmic_text::Stretch::UltraExpanded => font_kit::properties::Stretch::ULTRA_EXPANDED,
        },
    }
}

fn check_is_known_emoji_font(postscript_name: &str) -> bool {
    // TODO: Include other common emoji fonts
    postscript_name == "NotoColorEmoji"
}

#[cfg(test)]
mod native_font_tests {
    use super::*;
    use crate::{font, px};

    #[test]
    fn translucent_color_outline_glyphs_use_straight_bgra_at_the_atlas_boundary() {
        let system = CosmicTextSystem::new();
        system
            .add_fonts(vec![Cow::Borrowed(include_bytes!(
                "../../../fixtures/colr-alpha.ttf"
            ))])
            .unwrap();
        let id = system.font_id(&font("DazzUIAlphaProbe-Regular")).unwrap();
        let params = RenderGlyphParams {
            font_id: id,
            glyph_id: system.glyph_for_char(id, 'A').unwrap(),
            font_size: px(24.),
            subpixel_variant: point(0, 0),
            scale_factor: 1.,
            is_emoji: true,
        };
        let raw = system.0.write().raster_image(&params).unwrap();
        assert!(matches!(raw.source, swash::scale::Source::ColorOutline(_)));
        let raw_ink = raw.data.chunks_exact(4).max_by_key(|p| p[3]).unwrap();
        assert!((120..=128).contains(&raw_ink[3]));
        assert_eq!(raw_ink[0], raw_ink[3], "Swash supplies premultiplied red");
        let bounds = system.glyph_raster_bounds(&params).unwrap();
        let (_, bytes) = system.rasterize_glyph(&params, bounds).unwrap();
        let ink = bytes.chunks_exact(4).max_by_key(|p| p[3]).unwrap();
        assert_eq!(&ink[..3], &[0, 0, 255]);
        assert_eq!(ink[3], raw_ink[3]);
    }

    #[test]
    #[ignore = "requires installed Noto Color Emoji"]
    fn non_latin_faces_load_without_a_latin_sentinel() {
        let system = CosmicTextSystem::new();
        let id = system.font_id(&font("NotoColorEmoji")).unwrap();
        assert!(system.glyph_for_char(id, '👨').is_some());
        assert!(system.glyph_for_char(id, 'm').is_none());
        let line = system.layout_line(
            "👨",
            px(24.),
            &[FontRun {
                font_id: id,
                len: "👨".len(),
            }],
        );
        assert!(line.width > px(0.));
        assert!(!line.runs.is_empty());
        let pair = system.layout_line(
            "👨👨",
            px(24.),
            &[FontRun {
                font_id: id,
                len: "👨👨".len(),
            }],
        );
        assert!((pair.width - line.width * 2.).abs() < px(0.001));
        let second = pair
            .runs
            .iter()
            .flat_map(|run| &run.glyphs)
            .find(|glyph| glyph.index == "👨".len())
            .unwrap();
        assert!((second.position.x - line.width).abs() < px(0.001));
    }

    #[test]
    #[ignore = "requires installed Noto Naskh Arabic"]
    fn synthetic_bold_keeps_advances_and_adds_ink() {
        let system = CosmicTextSystem::new();
        let regular = font("NotoNaskhArabic-Regular");
        let mut bold = regular.clone();
        bold.weight = FontWeight(700.);
        let regular = system.font_id(&regular).unwrap();
        let bold = system.font_id(&bold).unwrap();
        assert_ne!(regular, bold);
        let widths = [regular, bold].map(|id| {
            system
                .layout_line(
                    "م",
                    px(24.),
                    &[FontRun {
                        font_id: id,
                        len: "م".len(),
                    }],
                )
                .width
        });
        assert_eq!(widths[0], widths[1]);
        let ink: Vec<u64> = [regular, bold]
            .into_iter()
            .map(|id| {
                let params = RenderGlyphParams {
                    font_id: id,
                    glyph_id: system.glyph_for_char(id, 'م').unwrap(),
                    font_size: px(24.),
                    subpixel_variant: point(0, 0),
                    scale_factor: 1.,
                    is_emoji: false,
                };
                let bounds = system.glyph_raster_bounds(&params).unwrap();
                system
                    .rasterize_glyph(&params, bounds)
                    .unwrap()
                    .1
                    .iter()
                    .map(|&alpha| u64::from(alpha))
                    .sum()
            })
            .collect();
        assert!(ink[0] > 0);
        assert!(ink[1] > ink[0]);
    }
}
