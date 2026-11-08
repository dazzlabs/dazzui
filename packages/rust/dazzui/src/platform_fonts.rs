//! Resolve authored font stacks before falling back to the platform shaper.
//!
//! GPUI 0.2 maps `.SystemUIFont` to IBM Plex Sans on Linux. If that face is
//! absent it searches its own fallback list, which can pick a different face
//! than the browser. `monospace` also needs a concrete family: GPUI's Linux
//! loader matches family names literally rather than resolving aliases.

#[cfg(target_os = "linux")]
use std::{ffi::CStr, sync::OnceLock};

#[cfg(target_os = "linux")]
pub(crate) fn family(monospace: bool) -> &'static str {
    static UI: OnceLock<Option<String>> = OnceLock::new();
    static CODE: OnceLock<Option<String>> = OnceLock::new();
    let (cache, generic, fallback) = if monospace {
        (&CODE, c"monospace", "monospace")
    } else {
        (&UI, c"system-ui", ".SystemUIFont")
    };
    cache
        .get_or_init(|| resolve(generic))
        .as_deref()
        .unwrap_or(fallback)
}

#[cfg(target_os = "linux")]
fn resolve(generic: &CStr) -> Option<String> {
    use fontconfig_sys::{FcMatchPattern, FcResultMatch, constants::FC_FAMILY};
    let fc = fontconfig_sys::statics::LIB_RESULT.as_ref().ok()?;
    // Fontconfig owns both patterns and the returned family string. Copy the
    // string before releasing the patterns, and never shut down its global
    // configuration, which GPUI's font loader also uses.
    unsafe {
        let pattern = (fc.FcNameParse)(generic.as_ptr().cast());
        if pattern.is_null() {
            return None;
        }
        (fc.FcConfigSubstitute)(std::ptr::null_mut(), pattern, FcMatchPattern);
        (fc.FcDefaultSubstitute)(pattern);
        let mut result = FcResultMatch;
        let matched = (fc.FcFontMatch)(std::ptr::null_mut(), pattern, &mut result);
        (fc.FcPatternDestroy)(pattern);
        if matched.is_null() {
            return None;
        }
        let mut name = std::ptr::null_mut();
        let family = if (fc.FcPatternGetString)(matched, FC_FAMILY.as_ptr(), 0, &mut name)
            == FcResultMatch
            && !name.is_null()
        {
            CStr::from_ptr(name.cast())
                .to_str()
                .ok()
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
        } else {
            None
        };
        (fc.FcPatternDestroy)(matched);
        family
    }
}

/// Native automatic fallback does not see the remaining families in the ds
/// stack. Supply those runs explicitly, keeping each grapheme intact.
pub(crate) fn runs(
    text: &str,
    style: &gpui::TextStyle,
    size: gpui::Pixels,
    window: &gpui::Window,
) -> Vec<gpui::TextRun> {
    use crate::tokens::Face;
    use unicode_segmentation::UnicodeSegmentation;

    let base = style.to_run(0);
    let Some(face) = [Face::Text, Face::Display, Face::Mono]
        .into_iter()
        .find(|face| style.font_family.as_ref() == face.family())
    else {
        return vec![style.to_run(text.len())];
    };
    let system = window.text_system();
    let primary = system.resolve_font(&base.font);
    let mut candidates = None;
    let mut runs: Vec<gpui::TextRun> = Vec::new();
    for grapheme in text.graphemes(true) {
        let supported = |font| {
            grapheme.chars().all(|ch| {
                // Joiners and variation selectors have no standalone glyph.
                ch.is_control()
                    || matches!(ch, '\u{200c}'..='\u{200d}' | '\u{fe00}'..='\u{fe0f}' | '\u{e0100}'..='\u{e01ef}')
                    || system.advance(font, size, ch).is_ok()
            })
        };
        let mut run = base.clone();
        run.len = grapheme.len();
        if !supported(primary) {
            let candidates = candidates.get_or_insert_with(|| {
                let names = system.all_font_names();
                face.families()
                    .iter()
                    .filter(|name| names.iter().any(|available| available == **name))
                    .map(|family| {
                        let mut font = base.font.clone();
                        font.family = (*family).into();
                        let id = system.resolve_font(&font);
                        (font, id)
                    })
                    .collect::<Vec<_>>()
            });
            if let Some((font, _)) = candidates.iter().find(|(_, id)| supported(*id)) {
                run.font = font.clone();
            }
        }
        match runs.last_mut() {
            Some(last) if last.font == run.font => last.len += run.len,
            _ => runs.push(run),
        }
    }
    runs
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    struct Probe;

    impl gpui::Render for Probe {
        fn render(
            &mut self,
            _: &mut gpui::Window,
            _: &mut gpui::Context<Self>,
        ) -> impl gpui::IntoElement {
            gpui::div()
        }
    }

    #[gpui::test]
    fn authored_fallback_precedes_cosmic_fallback(cx: &mut gpui::TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| Probe);
        cx.update(|window, _| {
            let style = gpui::TextStyle {
                font_family: super::family(false).into(),
                ..Default::default()
            };
            let system = window.text_system();
            if !system
                .all_font_names()
                .iter()
                .any(|name| name == "Noto Sans CJK SC")
            {
                return;
            }
            let primary = system.resolve_font(&style.font());
            if system.advance(primary, gpui::px(11.), '⌘').is_ok() {
                return;
            }
            let runs = super::runs("A⌘", &style, gpui::px(11.), window);
            assert_eq!(runs.iter().map(|run| run.len).sum::<usize>(), "A⌘".len());
            assert_eq!(runs[0].font.family, style.font_family);
            assert_eq!(runs[1].font.family.as_ref(), "Noto Sans CJK SC");
        });
    }

    #[test]
    fn generic_families_resolve_to_installed_faces() {
        for generic in [c"system-ui", c"monospace"] {
            let name = super::resolve(generic).expect("Fontconfig resolves its generic family");
            assert!(!name.is_empty());
            assert_ne!(name, generic.to_str().unwrap());
        }
    }
}
