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

#[cfg(target_os = "macos")]
fn system_fallbacks(
    _: &str,
    _: &gpui::Font,
    _: gpui::Pixels,
    _: crate::tokens::Face,
) -> Vec<String> {
    Vec::new()
}

#[cfg(target_os = "windows")]
fn locale() -> &'static str {
    static LOCALE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    LOCALE.get_or_init(|| {
        let mut name = [0u16; 85];
        let count = unsafe { windows::Win32::Globalization::GetUserDefaultLocaleName(&mut name) };
        String::from_utf16_lossy(&name[..count.saturating_sub(1) as usize]).to_ascii_lowercase()
    })
}

#[cfg(target_os = "windows")]
fn generic_families(monospace: bool) -> &'static [&'static str] {
    let locale = locale();
    // Chromium's Windows generic defaults are localized. These are platform
    // font mappings, independent of DazzUI's typography values.
    if locale == "zh-cn" || locale == "zh-sg" || locale.starts_with("zh-hans") {
        if monospace {
            &["NSimSun"]
        } else {
            &["Noto Sans SC", "Noto Sans CJK SC", "Microsoft YaHei"]
        }
    } else if locale.starts_with("ja") {
        if monospace {
            &["BIZ UDGothic", "MS Gothic"]
        } else {
            &["Noto Sans JP", "Noto Sans CJK JP", "Meiryo", "Yu Gothic"]
        }
    } else if locale.starts_with("ko") {
        if monospace {
            &["Gulimche"]
        } else {
            &["Noto Sans KR", "Noto Sans CJK KR", "Malgun Gothic"]
        }
    } else if monospace {
        &["Consolas", "Courier New"]
    } else {
        &["Times New Roman"]
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn family(monospace: bool) -> &'static str {
    static UI: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
    static CODE: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
    if monospace {
        return CODE.get_or_init(|| {
            let candidates = crate::tokens::Face::Mono
                .families()
                .iter()
                .copied()
                .filter(|name| !matches!(*name, "ui-monospace" | "monospace"))
                .chain(generic_families(true).iter().copied());
            installed_family(candidates).unwrap_or("Consolas")
        });
    }
    UI.get_or_init(|| {
        // The icon-title font can be a locale-specific face. Segoe supplies
        // the UI weights, and shaping resolves scripts it does not cover.
        installed_family(["Segoe UI Variable Text", "Segoe UI"]).unwrap_or(".SystemUIFont")
    })
}

#[cfg(target_os = "windows")]
fn installed_family(candidates: impl IntoIterator<Item = &'static str>) -> Option<&'static str> {
    use windows::{
        Win32::Graphics::DirectWrite::*,
        core::{BOOL, HSTRING},
    };
    unsafe {
        let factory: IDWriteFactory5 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).ok()?;
        let mut collection = None;
        factory
            .GetSystemFontCollection(false, &mut collection, true)
            .ok()?;
        let collection = collection?;
        candidates.into_iter().find(|name| {
            let mut index = 0;
            let mut exists = BOOL(0);
            collection
                .FindFamilyName(&HSTRING::from(*name), &mut index, &mut exists)
                .is_ok()
                && exists.as_bool()
        })
    }
}

#[cfg(target_os = "windows")]
fn system_fallbacks(
    grapheme: &str,
    _: &gpui::Font,
    _: gpui::Pixels,
    face: crate::tokens::Face,
) -> Vec<String> {
    use unicode_script::{Script, UnicodeScript};
    let script = grapheme
        .chars()
        .map(|ch| ch.script())
        .find(|script| !matches!(script, Script::Common | Script::Inherited | Script::Unknown));
    // Browser script fallback follows its generic family after the authored
    // stack is exhausted. Whitespace keeps the primary face. See Chromium's
    // platform/fonts/win/font_fallback_win.cc for the native protocol order.
    let families: &[&str] = match script {
        Some(Script::Arabic | Script::Hebrew) if matches!(face, crate::tokens::Face::Mono) => {
            &["Courier New"]
        }
        Some(Script::Arabic) => &["Tahoma", "Segoe UI"],
        Some(Script::Hebrew) => &["David", "Segoe UI"],
        _ => generic_families(false),
    };
    families.iter().map(|name| (*name).to_owned()).collect()
}

#[cfg(target_os = "linux")]
fn system_fallbacks(
    grapheme: &str,
    _: &gpui::Font,
    _: gpui::Pixels,
    _: crate::tokens::Face,
) -> Vec<String> {
    use std::{collections::HashMap, sync::Mutex};
    static CACHE: OnceLock<Mutex<HashMap<String, Option<String>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(Default::default);
    if let Some(cached) = cache.lock().unwrap().get(grapheme) {
        return standard_fallback()
            .into_iter()
            .chain(cached.iter().cloned())
            .collect();
    }
    let matched = match_grapheme(grapheme);
    let mut cache = cache.lock().unwrap();
    if cache.len() >= 2048 {
        cache.clear();
    }
    cache.insert(grapheme.to_owned(), matched.clone());
    standard_fallback().into_iter().chain(matched).collect()
}

#[cfg(target_os = "linux")]
fn standard_fallback() -> Option<String> {
    static FAMILY: OnceLock<Option<String>> = OnceLock::new();
    FAMILY.get_or_init(|| resolve(c"Times New Roman")).clone()
}

#[cfg(target_os = "linux")]
fn match_grapheme(grapheme: &str) -> Option<String> {
    use fontconfig_sys::{FcMatchPattern, FcResultMatch, constants::*};
    let fc = fontconfig_sys::statics::LIB_RESULT.as_ref().ok()?;
    // The browser's system fallback is independent of the exhausted primary
    // family. Sort first, then test coverage: adding a charset to FcFontMatch
    // changes its priorities and can select a different face than Chromium.
    unsafe {
        let pattern = (fc.FcPatternCreate)();
        if pattern.is_null() {
            return None;
        }
        (fc.FcPatternAddBool)(pattern, FC_SCALABLE.as_ptr(), 1);
        (fc.FcConfigSubstitute)(std::ptr::null_mut(), pattern, FcMatchPattern);
        (fc.FcDefaultSubstitute)(pattern);
        let mut result = FcResultMatch;
        let fonts = (fc.FcFontSort)(
            std::ptr::null_mut(),
            pattern,
            0,
            std::ptr::null_mut(),
            &mut result,
        );
        (fc.FcPatternDestroy)(pattern);
        if fonts.is_null() {
            return None;
        }
        let family = (0..(*fonts).nfont).find_map(|index| {
            let candidate = *(*fonts).fonts.add(index as usize);
            let mut charset = std::ptr::null_mut();
            if (fc.FcPatternGetCharSet)(candidate, FC_CHARSET.as_ptr(), 0, &mut charset)
                != FcResultMatch
                || charset.is_null()
                || !grapheme
                    .chars()
                    .filter(|ch| requires_glyph(*ch))
                    .all(|ch| (fc.FcCharSetHasChar)(charset, ch as u32) != 0)
            {
                return None;
            }
            let mut format = std::ptr::null_mut();
            if (fc.FcPatternGetString)(candidate, FC_FONTFORMAT.as_ptr(), 0, &mut format)
                != FcResultMatch
                || format.is_null()
                || !matches!(
                    CStr::from_ptr(format.cast()).to_bytes(),
                    b"TrueType" | b"CFF"
                )
            {
                return None;
            }
            let mut name = std::ptr::null_mut();
            if (fc.FcPatternGetString)(candidate, FC_POSTSCRIPT_NAME.as_ptr(), 0, &mut name)
                == FcResultMatch
                && !name.is_null()
            {
                CStr::from_ptr(name.cast()).to_str().ok().map(str::to_owned)
            } else {
                None
            }
        });
        (fc.FcFontSetDestroy)(fonts);
        family
    }
}

fn authored_family(name: &str) -> Option<String> {
    #[cfg(target_os = "linux")]
    match name {
        // Chromium's CSS generic defaults go through these aliases, which
        // need not match Fontconfig's bare sans-serif and serif defaults.
        "sans-serif" => return resolve(c"Arial"),
        "serif" => return resolve(c"Times New Roman"),
        "monospace" => return resolve(c"monospace"),
        _ => {}
    }
    Some(name.to_owned())
}

fn requires_glyph(ch: char) -> bool {
    // Joiners and variation selectors have no standalone glyph.
    !ch.is_control()
        && !matches!(ch, '\u{200c}'..='\u{200d}' | '\u{fe00}'..='\u{fe0f}' | '\u{e0100}'..='\u{e01ef}')
}

/// Native automatic fallback does not see the remaining families in the ds
/// stack. Supply those runs explicitly, keeping each grapheme intact.
pub(crate) fn runs(
    text: &str,
    style: &gpui::TextStyle,
    size: gpui::Pixels,
    system: &gpui::TextSystem,
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
    let primary = system.resolve_font(&base.font);
    let mut candidates = None;
    let mut available = None;
    let mut runs: Vec<gpui::TextRun> = Vec::new();
    for grapheme in text.graphemes(true) {
        let supported = |font| {
            grapheme
                .chars()
                .all(|ch| !requires_glyph(ch) || system.advance(font, size, ch).is_ok())
        };
        let mut run = base.clone();
        run.len = grapheme.len();
        if !supported(primary) {
            let candidates = candidates.get_or_insert_with(|| {
                let names = available.get_or_insert_with(|| system.all_font_names());
                face.families()
                    .iter()
                    .filter_map(|family| authored_family(family))
                    .filter(|name| names.contains(name))
                    .map(|family| {
                        let mut font = base.font.clone();
                        font.family = family.into();
                        let id = system.resolve_font(&font);
                        (font, id)
                    })
                    .collect::<Vec<_>>()
            });
            if let Some((font, _)) = candidates.iter().find(|(_, id)| supported(*id)) {
                run.font = font.clone();
            } else {
                for family in system_fallbacks(grapheme, &base.font, size, face) {
                    if !available
                        .as_ref()
                        .unwrap()
                        .iter()
                        .any(|name| name == &family)
                    {
                        continue;
                    }
                    let mut font = base.font.clone();
                    font.family = family.into();
                    if supported(system.resolve_font(&font)) {
                        run.font = font;
                        break;
                    }
                }
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
            let runs = super::runs("A⌘", &style, gpui::px(11.), system);
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
