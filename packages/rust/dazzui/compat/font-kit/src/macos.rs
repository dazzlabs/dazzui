use std::any::Any;

use core_foundation::{
    base::TCFType, dictionary::CFDictionary, number::CFNumber, string::CFString,
};
use core_text::{font, font_descriptor::kCTFontVariationAttribute};
use upstream::{
    error::SelectionError, family_handle::FamilyHandle, family_name::FamilyName, font::Font,
    handle::Handle, matching, properties::Properties, properties::Weight, source::Source,
    sources::core_text::CoreTextSource,
};

/// The weight a face draws at, when Core Text carries one on the `wght` axis.
///
/// `Properties::weight` is the face's own trait — the weight its file
/// declares. An instance created on a variable font draws at a weight its
/// file does not name: SF Pro's Medium is 530 by trait on macOS 15 and 500 on
/// macOS 26, and either way a 500 asked of the family is answered by CSS face
/// matching, which prefers a 400 face over a nearer one when nothing says
/// 500. So the axis is what a caller matching weights should read.
pub fn css_weight(font: &Font) -> Option<f64> {
    let native = font.native_font();
    let attributes = native.copy_descriptor().attributes();
    let key = unsafe { CFString::wrap_under_get_rule(kCTFontVariationAttribute) };
    let variations = attributes.find(&key)?.downcast::<CFDictionary>()?;
    let axis = CFNumber::from(u32::from_be_bytes(*b"wght") as i64);
    let value = variations.find(axis.as_CFType().as_CFTypeRef())?;
    let number = unsafe { CFNumber::wrap_under_get_rule(*value as _) };
    number.to_f64()
}

/// A face's properties with its drawn weight in place of its trait.
pub fn css_properties(font: &Font) -> Properties {
    let mut properties = font.properties();
    if let Some(weight) = css_weight(font).or_else(|| static_css_weight(font)) {
        properties.weight = Weight(weight as f32);
    }
    properties
}

fn static_css_weight(font: &Font) -> Option<f64> {
    let native = font.native_font();
    // Named variable instances share OS/2's default weight. Their axis or
    // Core Text trait is authoritative; static faces declare CSS in OS/2.
    if native
        .get_font_table(u32::from_be_bytes(*b"fvar"))
        .is_some()
    {
        return None;
    }
    let table = native.get_font_table(u32::from_be_bytes(*b"OS/2"))?;
    let weight = u16::from_be_bytes(table.bytes().get(4..6)?.try_into().ok()?);
    match weight {
        1..=9 => Some(f64::from(weight * 100)),
        10..=1000 => Some(f64::from(weight)),
        _ => None,
    }
}

pub struct SystemSource(CoreTextSource);

impl Default for SystemSource {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemSource {
    pub fn new() -> Self {
        Self(CoreTextSource::new())
    }
    pub fn all_fonts(&self) -> Result<Vec<Handle>, SelectionError> {
        self.0.all_fonts()
    }
    pub fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        self.0.all_families()
    }
    pub fn select_by_postscript_name(&self, name: &str) -> Result<Handle, SelectionError> {
        self.0.select_by_postscript_name(name)
    }

    pub fn select_best_match(
        &self,
        names: &[FamilyName],
        properties: &Properties,
    ) -> Result<Handle, SelectionError> {
        self.best_match(names, properties)
    }

    /// CSS Fonts 3 § 5.2 over the weights the faces draw at, rather than over
    /// the traits they declare. See [`css_weight`].
    fn best_match(
        &self,
        names: &[FamilyName],
        query: &Properties,
    ) -> Result<Handle, SelectionError> {
        for name in names {
            let Ok(family) = <Self as Source>::select_family_by_generic_name(self, name) else {
                continue;
            };
            let mut handles = Vec::with_capacity(family.fonts().len());
            let mut candidates = Vec::with_capacity(family.fonts().len());
            for handle in family.fonts() {
                let Ok(font) = handle.load() else {
                    continue;
                };
                candidates.push(css_properties(&font));
                handles.push(handle.clone());
            }
            if let Ok(index) = matching::find_best_match(&candidates, query) {
                return Ok(handles[index].clone());
            }
        }
        Err(SelectionError::NotFound)
    }

    pub fn select_family_by_name(&self, name: &str) -> Result<FamilyHandle, SelectionError> {
        let family = self.0.select_family_by_name(name)?;
        if name != ".AppleSystemUIFont" {
            return Ok(family);
        }
        let fonts = family.fonts().iter().map(|handle| {
            let Ok(loaded) = handle.load() else {
                return handle.clone();
            };
            let native = loaded.native_font();
            let Some(axes) = native.get_variation_axes() else {
                return handle.clone();
            };
            // Older macOS faces without variable weight keep their native instances.
            let weight_axis = u32::from_be_bytes(*b"wght") as i64;
            let has_weight = axes.iter().any(|axis| {
                let key = CFString::new("NSCTVariationAxisIdentifier");
                axis.find(&key)
                    .and_then(|value| value.downcast::<CFNumber>())
                    .and_then(|value| value.to_i64())
                    == Some(weight_axis)
            });
            if !has_weight {
                return handle.clone();
            }
            // Core Text's UI Semibold is wght=590, although its CSS property
            // is 600; Chromium sets the actual variable axis to CSS 600.
            let weight = (loaded.properties().weight.0 / 100.).round() * 100.;
            let variations = CFDictionary::from_CFType_pairs(&[(
                CFNumber::from(weight_axis),
                CFNumber::from(weight as f64),
            )]);
            let variation_key = unsafe { CFString::wrap_under_get_rule(kCTFontVariationAttribute) };
            let attrs = CFDictionary::from_CFType_pairs(&[(
                variation_key.as_CFType(),
                variations.as_CFType(),
            )]);
            let Ok(descriptor) = native
                .copy_descriptor()
                .create_copy_with_attributes(attrs.to_untyped())
            else {
                return handle.clone();
            };
            let native = font::new_from_descriptor(&descriptor, native.pt_size());
            let loaded = unsafe { Font::from_core_text_font_no_path(native) };
            Handle::from_native(&loaded)
        });
        Ok(FamilyHandle::from_font_handles(fonts))
    }
}

impl Source for SystemSource {
    fn all_fonts(&self) -> Result<Vec<Handle>, SelectionError> {
        self.all_fonts()
    }
    fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        self.all_families()
    }
    fn select_family_by_name(&self, name: &str) -> Result<FamilyHandle, SelectionError> {
        self.select_family_by_name(name)
    }
    fn select_by_postscript_name(&self, name: &str) -> Result<Handle, SelectionError> {
        self.select_by_postscript_name(name)
    }
    fn select_best_match(
        &self,
        names: &[FamilyName],
        properties: &Properties,
    ) -> Result<Handle, SelectionError> {
        self.best_match(names, properties)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use upstream::{
        family_name::FamilyName,
        properties::{Properties, Weight},
    };

    /// The weight a selected face draws at, whichever way the platform
    /// reports it: Core Text elides a variation equal to a face's native
    /// instance, which is how Regular and Bold are drawn at their own
    /// weights, and the trait still says so.
    fn drawn_weight(font: &Font) -> f64 {
        css_weight(font).unwrap_or(f64::from(font.properties().weight.0))
    }

    #[test]
    fn system_family_matches_and_preserves_css_weight_instances() {
        let source = SystemSource::new();
        for weight in [400., 500., 600., 700.] {
            let handle = source
                .select_best_match(
                    &[FamilyName::Title(".AppleSystemUIFont".into())],
                    &Properties::new().weight(Weight(weight as f32)),
                )
                .unwrap();
            let selected = handle.load().unwrap();
            // The face is chosen by the weight it draws at. Its own trait may
            // say another: SF Pro's Medium is 530 on macOS 15, and CSS face
            // matching given a 500 it cannot find prefers Regular.
            assert!(
                (drawn_weight(&selected) - weight).abs() < 0.1,
                "CSS {weight} must select the face that draws at it, not the adjacent weight"
            );
            let native = selected.native_font();
            if native.get_variation_axes().is_none() {
                continue;
            }
            // Medium and Semibold have no native instance, so the face has to
            // carry the CSS weight on its axis for shaping and rasterization.
            if matches!(weight, 400. | 700.) {
                continue;
            }
            assert_eq!(
                css_weight(&selected),
                Some(weight),
                "native shaping and rasterization must retain CSS {weight}"
            );
        }
    }

    #[test]
    fn static_fallback_faces_use_their_declared_css_weights() {
        let source = SystemSource::new();
        for weight in [400., 500., 600.] {
            let handle = source
                .select_best_match(
                    &[FamilyName::Title("PingFang SC".into())],
                    &Properties::new().weight(Weight(weight as f32)),
                )
                .unwrap();
            let selected = handle.load().unwrap();
            assert_eq!(f64::from(css_properties(&selected).weight.0), weight);
        }
    }
}
