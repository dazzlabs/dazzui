//! Base UI's default contains filter compares query-sized UTF-16 windows
//! with an ICU search collator. Normalizing an entire string first would
//! change matches for composed accents, ligatures and punctuation.
use icu_collator::{
    Collator, CollatorBorrowed, CollatorPreferences,
    options::{AlternateHandling, CollatorOptions, MaxVariable, Strength},
    preferences::CollationType,
};
use std::{cmp::Ordering, sync::OnceLock};
struct SearchFilter {
    collator: CollatorBorrowed<'static>,
}
impl SearchFilter {
    fn new(locale: &str) -> Self {
        let mut prefs = locale
            .parse()
            .ok()
            .and_then(|locale| CollatorPreferences::from_locale_strict(&locale).ok())
            .unwrap_or_default();
        prefs.collation_type = Some(CollationType::Search);
        let mut options = CollatorOptions::default();
        options.strength = Some(Strength::Primary);
        options.alternate_handling = Some(AlternateHandling::Shifted);
        options.max_variable = Some(MaxVariable::Punctuation);
        Self {
            collator: Collator::try_new(prefs, options).expect("Compiled ICU search data"),
        }
    }
    fn contains(&self, item: &str, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let item: Vec<u16> = item.encode_utf16().collect();
        let query: Vec<u16> = query.encode_utf16().collect();
        item.windows(query.len()).any(|part| {
            // Rust strings cannot contain lone surrogates. A substring that
            // splits a pair cannot equal this valid query; ICU replaces lone
            // surrogates, so skip it rather than matching U+FFFD spuriously.
            char::decode_utf16(part.iter().copied()).all(|c| c.is_ok())
                && self.collator.compare_utf16(part, &query) == Ordering::Equal
        })
    }
}
pub(super) fn trim_query(query: &str) -> &str {
    query.trim_matches(|c| {
        matches!(c,
        '\u{9}'..='\u{d}' | '\u{20}' | '\u{a0}' | '\u{1680}' |
        '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' |
        '\u{205f}' | '\u{3000}' | '\u{feff}')
    })
}
pub(super) fn contains(item: &str, query: &str) -> bool {
    static FILTER: OnceLock<SearchFilter> = OnceLock::new();
    FILTER
        .get_or_init(|| {
            let languages = gtk::glib::language_names();
            let locale = languages.first().map(|s| s.as_str()).unwrap_or("und");
            let locale = locale
                .split(['.', '@'])
                .next()
                .unwrap_or("und")
                .replace('_', "-");
            SearchFilter::new(&locale)
        })
        .contains(item, query)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_base_ui_collator_windows() {
        let filter = SearchFilter::new("en-US");
        for (item, query, expected) in [
            ("CAFÉ", "cafe", true),
            ("cafe\u{301}", "fé", true),
            ("é", "e\u{301}", false),
            ("ß", "ss", false),
            ("Straße", "strasse", false),
            ("foo-bar", "foobar", false),
            ("foo-bar", ".", true),
            ("Japan", ".", false),
            ("a b", "ab ", true),
            ("🦀β", "🦀", true),
            ("🦀β", "�", false),
            ("anything", "", true),
            ("Istanbul", "ı", false),
        ] {
            assert_eq!(
                filter.contains(item, query),
                expected,
                "{item:?} contains {query:?}"
            );
        }
        assert!(SearchFilter::new("tr").contains("Istanbul", "ı"));
    }
}
