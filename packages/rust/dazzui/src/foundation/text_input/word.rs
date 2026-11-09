//! Word editing uses ICU dictionaries, with the browser's punctuation tailoring.
//! These are logical text offsets; shaping and password masking do not define
//! words. Adjacent punctuation and emoji form a navigation run, while apostrophes
//! inside words and separators inside numbers stay with their word.

use std::borrow::Cow;
use std::ops::Range;

use icu_properties::{
    CodePointMapData,
    props::{BidiClass, GeneralCategory, GeneralCategoryGroup},
};
use icu_segmenter::WordSegmenter;

fn boundaries(text: &str) -> Vec<usize> {
    // Chromium's ICU rules move the two full stops from MidNumLet to MidNum,
    // and remove the three colons from MidLetter. Equivalent-property scalars
    // preserve both UTF-8 and UTF-16 offsets without changing the edited value.
    // See chromium/deps/icu source/data/brkitr/rules/word.txt.
    let tailored = if text.contains(['.', '\u{ff0e}', ':', '\u{fe55}', '\u{ff1a}']) {
        Cow::Owned(
            text.chars()
                .map(|ch| match ch {
                    '.' => ',',
                    '\u{ff0e}' => '\u{ff0c}',
                    ':' => '/',
                    '\u{fe55}' | '\u{ff1a}' => '\u{ff0f}',
                    _ => ch,
                })
                .collect::<String>(),
        )
    } else {
        Cow::Borrowed(text)
    };
    WordSegmenter::new_dictionary(Default::default())
        .segment_str(&tailored)
        .collect()
}

fn gap(ch: char) -> bool {
    matches!(ch, ' ' | '\t' | '\n' | '\u{a0}')
}

fn line_break(ch: char) -> bool {
    matches!(ch, '\n' | '\r')
}

fn category(ch: char) -> GeneralCategory {
    CodePointMapData::<GeneralCategory>::new().get(ch)
}

fn punctuation(ch: char) -> bool {
    // The browser tests UTF-16 units here, so even supplementary letters are
    // part of a surrogate run rather than BMP alphanumeric text.
    ch.len_utf16() == 2
        || GeneralCategoryGroup::Punctuation.contains(category(ch))
        || GeneralCategoryGroup::Symbol.contains(category(ch))
}

fn alphanumeric(ch: char) -> bool {
    ch.len_utf16() == 1
        && (GeneralCategoryGroup::Letter.contains(category(ch))
            || category(ch) == GeneralCategory::DecimalNumber)
}

fn significant(ch: char) -> bool {
    ch.len_utf16() == 2
        || line_break(ch)
        || (!gap(ch) && !GeneralCategoryGroup::Other.contains(category(ch)))
}

fn before(text: &str, offset: usize) -> Option<char> {
    text[..offset].chars().next_back()
}

fn after(text: &str, offset: usize) -> Option<char> {
    text[offset..].chars().next()
}

pub(super) fn range(text: &str, offset: usize) -> Range<usize> {
    let points = boundaries(text);
    // Word selection asks for the break before the next UTF-16 unit. ICU's
    // preceding() rounds an interior surrogate back to its leading unit, so
    // at a supplementary character this may include the preceding segment.
    let start_query = if offset == text.len() {
        offset
    } else {
        let ch = after(text, offset).unwrap();
        offset
            + if ch.len_utf16() == 1 {
                ch.len_utf8()
            } else {
                0
            }
    };
    let start = points
        .iter()
        .copied()
        .take_while(|&at| at < start_query)
        .last()
        .unwrap_or(0);
    let end = points
        .into_iter()
        .find(|&at| at > offset)
        .unwrap_or(text.len());
    start..end
}

pub(super) fn previous(text: &str, offset: usize) -> usize {
    let points = boundaries(text);
    let mut punctuation_start = None;
    for at in points.into_iter().rev().filter(|&at| at < offset) {
        let ch = after(text, at).unwrap();
        if punctuation(ch) {
            if before(text, at).is_some_and(alphanumeric) {
                return at;
            }
            punctuation_start = Some(at);
        } else if let Some(start) = punctuation_start {
            return start;
        } else if significant(ch) {
            return at;
        }
    }
    0
}

pub(super) fn next(text: &str, offset: usize, skip_spaces: bool) -> usize {
    let points = boundaries(text);
    let finish = |at| {
        if skip_spaces {
            for (index, ch) in text[at..].char_indices() {
                let white = gap(ch)
                    || CodePointMapData::<BidiClass>::new().get(ch) == BidiClass::WhiteSpace;
                if !white || line_break(ch) {
                    return at + index;
                }
            }
        }
        at
    };
    let mut punctuation_after_gap = None;
    for at in points.into_iter().filter(|&at| at > offset) {
        let Some(ch) = after(text, at) else {
            return finish(punctuation_after_gap.unwrap_or(at));
        };
        let prev = before(text, at).unwrap();
        if line_break(ch) {
            return finish(punctuation_after_gap.unwrap_or(at));
        }
        if punctuation(ch) {
            if alphanumeric(prev) {
                return finish(at);
            }
            if skip_spaces && punctuation_after_gap.is_none() && gap(prev) {
                punctuation_after_gap = Some(at);
            }
            continue;
        }
        if let Some(start) = punctuation_after_gap {
            return finish(start);
        }
        // A new paragraph beginning with indentation has no word at its
        // first position. Continue to its first word rather than stopping
        // between the line break and that indentation.
        if line_break(prev) && !significant(ch) {
            continue;
        }
        if significant(prev) || (skip_spaces && gap(prev) && significant(ch)) {
            return finish(at);
        }
    }
    text.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_selection_matches_recorded_browser_insertion_points() {
        let fixtures: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../js/dazzui/example/scripts/text-field-word-selection.cases.json"
        ))
        .unwrap();
        for fixture in fixtures["cases"].as_array().unwrap() {
            let text = fixture["text"].as_str().unwrap();
            for check in fixture["checks"].as_array().unwrap() {
                let offset_utf16 = check[0].as_u64().unwrap() as usize;
                let offset = text
                    .char_indices()
                    .map(|(at, _)| at)
                    .chain(std::iter::once(text.len()))
                    .find(|&at| text[..at].encode_utf16().count() == offset_utf16)
                    .unwrap();
                let selected = range(text, offset);
                assert_eq!(
                    (
                        text[..selected.start].encode_utf16().count(),
                        text[..selected.end].encode_utf16().count()
                    ),
                    (
                        check[1].as_u64().unwrap() as usize,
                        check[2].as_u64().unwrap() as usize
                    ),
                    "{}/{offset_utf16}",
                    fixture["id"],
                );
            }
        }
    }

    #[test]
    fn rightward_space_skipping_preserves_punctuation_and_line_stops() {
        assert_eq!(next("one two", 0, false), 3);
        assert_eq!(next("one two", 0, true), 4);
        assert_eq!(next("word ...", 4, true), 5);
        assert_eq!(next("word ...", 5, true), 8);
        assert_eq!(next("word ...\n", 4, true), 5);
        assert_eq!(next("word.more", 0, true), 4);
        assert_eq!(next("one  \ntwo", 0, true), 5);
        assert_eq!(next("one\u{2003}two", 0, true), "one\u{2003}".len());
    }
}
