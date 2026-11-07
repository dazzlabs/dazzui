use std::any::Any;

use core_foundation::{
    base::TCFType, dictionary::CFDictionary, number::CFNumber, string::CFString,
};
use core_text::{font, font_descriptor::kCTFontVariationAttribute};
use upstream::{
    error::SelectionError, family_handle::FamilyHandle, family_name::FamilyName, font::Font,
    handle::Handle, properties::Properties, source::Source, sources::core_text::CoreTextSource,
};

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
        <Self as Source>::select_best_match(self, names, properties)
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

    #[test]
    fn system_family_matches_and_preserves_css_weight_instances() {
        let source = SystemSource::new();
        for weight in [400., 500., 600., 700.] {
            let handle = source
                .select_best_match(
                    &[FamilyName::Title(".AppleSystemUIFont".into())],
                    &Properties::new().weight(Weight(weight)),
                )
                .unwrap();
            let loaded = handle.load().unwrap();
            assert!(
                (loaded.properties().weight.0 - weight).abs() < 0.1,
                "CSS {weight} must select its own face, not the adjacent weight"
            );
            let native = loaded.native_font();
            if native.get_variation_axes().is_none() {
                continue;
            }
            let key = unsafe { CFString::wrap_under_get_rule(kCTFontVariationAttribute) };
            let attributes = native.copy_descriptor().attributes();
            let Some(variations) = attributes.find(&key) else {
                // Core Text elides variations equal to a face's native default.
                assert!(
                    matches!(weight, 400. | 700.),
                    "Medium and Semibold require explicit CSS weight instances"
                );
                continue;
            };
            let variations = variations.downcast::<CFDictionary>().unwrap();
            let axis = CFNumber::from(u32::from_be_bytes(*b"wght") as i64);
            let value = variations.find(axis.as_CFType().as_CFTypeRef()).unwrap();
            let actual = unsafe { CFNumber::wrap_under_get_rule(*value as _) };
            assert_eq!(
                actual.to_f64(),
                Some(weight as f64),
                "native shaping and rasterization must retain CSS {weight}"
            );
        }
    }
}
