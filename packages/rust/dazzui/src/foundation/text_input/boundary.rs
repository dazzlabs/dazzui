// Portions derived from Chromium's editing state machines.
// Copyright 2016 The Chromium Authors. See LICENSE-CHROMIUM in this directory.

//! Editing boundaries follow Blink's state_machines rules, independently of
//! shaping clusters. Navigation and forward deletion keep graphemes intact;
//! Backspace removes ordinary marks separately, but keeps emoji sequences.
//! See Chromium's core/editing/state_machines/{state_machine_util,
//! backward_grapheme_boundary_state_machine,backspace_state_machine}.cc.

use icu_properties::{
    CodePointMapData, CodePointSetData,
    props::{
        CanonicalCombiningClass, Emoji, EmojiModifier, EmojiModifierBase, ExtendedPictographic,
        GeneralCategory, GraphemeClusterBreak as Gcb, VariationSelector,
    },
};

fn regional(ch: char) -> bool {
    matches!(ch, '\u{1f1e6}'..='\u{1f1ff}')
}

fn pictographic(ch: char) -> bool {
    CodePointSetData::new::<ExtendedPictographic>().contains(ch)
        || CodePointSetData::new::<Emoji>().contains(ch)
}

fn variation(ch: char) -> bool {
    CodePointSetData::new::<VariationSelector>().contains(ch)
}

fn breaks(before: char, after: char) -> bool {
    let property = CodePointMapData::<Gcb>::new();
    let a = property.get(before);
    let b = property.get(after);
    if a == Gcb::CR && b == Gcb::LF {
        return false;
    }
    if matches!(a, Gcb::Control | Gcb::CR | Gcb::LF)
        || matches!(b, Gcb::Control | Gcb::CR | Gcb::LF)
    {
        return true;
    }
    if (a == Gcb::L && matches!(b, Gcb::L | Gcb::V | Gcb::LV | Gcb::LVT))
        || (matches!(a, Gcb::LV | Gcb::V) && matches!(b, Gcb::V | Gcb::T))
        || (matches!(a, Gcb::LVT | Gcb::T) && b == Gcb::T)
        || matches!(b, Gcb::Extend | Gcb::ZWJ | Gcb::SpacingMark)
        || a == Gcb::Prepend
    {
        return false;
    }
    // Blink joins an adjacent virama and letter, except in Tamil. It does
    // not apply the newer GB9c rule through an intervening joiner or mark.
    let virama = matches!(
        before as u32,
        0x094d
            | 0x09cd
            | 0x0a4d
            | 0x0acd
            | 0x0b4d
            | 0x0c4d
            | 0x0ccd
            | 0x0d4d
            | 0x0dca
            | 0x1b44
            | 0xa8c4
            | 0xa9c0
            | 0x11046
            | 0x110b9
            | 0x111c0
            | 0x11235
            | 0x1134d
            | 0x11442
            | 0x114c2
            | 0x115bf
            | 0x1163f
            | 0x116b6
            | 0x11c3f
    );
    !(virama
        && CodePointMapData::<GeneralCategory>::new().get(after) == GeneralCategory::OtherLetter
        || before == '\u{200d}' && pictographic(after))
}

pub(super) fn previous(text: &str, offset: usize) -> usize {
    let mut chars = text[..offset].char_indices().rev();
    let Some((mut start, mut after)) = chars.next() else {
        return 0;
    };
    while let Some((index, before)) = chars.next() {
        if regional(before) && regional(after) {
            let preceding = 1 + chars.take_while(|(_, ch)| regional(*ch)).count();
            return if preceding % 2 == 1 { index } else { start };
        }
        if breaks(before, after) {
            break;
        }
        start = index;
        after = before;
    }
    start
}

pub(super) fn next(text: &str, offset: usize) -> usize {
    let mut chars = text[offset..].char_indices();
    let Some((_, mut before)) = chars.next() else {
        return text.len();
    };
    let mut indicators = if regional(before) {
        1 + text[..offset]
            .chars()
            .rev()
            .take_while(|ch| regional(*ch))
            .count()
    } else {
        0
    };
    for (index, after) in chars {
        let boundary = if regional(before) && regional(after) {
            indicators % 2 == 0
        } else {
            breaks(before, after)
        };
        if boundary {
            return offset + index;
        }
        indicators = if regional(after) { indicators + 1 } else { 0 };
        before = after;
    }
    text.len()
}

pub(super) fn ceil(text: &str, offset: usize) -> usize {
    if offset == 0 {
        0
    } else {
        next(text, previous(text, offset))
    }
}

enum DeleteState {
    Start,
    LineFeed,
    Keycap,
    VariationKeycap,
    Modifier,
    VariationModifier,
    Variation,
    Emoji,
    Joiner,
    VariationJoiner,
    TagEnd,
    Tags,
}

pub(super) fn backspace(text: &str, offset: usize) -> usize {
    let mut state = DeleteState::Start;
    let mut start = offset;
    let mut tag_base = false;
    for (index, ch) in text[..offset].char_indices().rev() {
        use DeleteState::*;
        state = match state {
            Start => {
                start = index;
                if regional(ch) {
                    let preceding = text[..index]
                        .chars()
                        .rev()
                        .take_while(|ch| regional(*ch))
                        .count();
                    return if preceding % 2 == 1 {
                        index - ch.len_utf8()
                    } else {
                        index
                    };
                }
                match ch {
                    '\n' => LineFeed,
                    '\u{20e3}' => Keycap,
                    '\u{e007f}' => TagEnd,
                    _ if variation(ch) => Variation,
                    _ if CodePointSetData::new::<EmojiModifier>().contains(ch) => Modifier,
                    _ if pictographic(ch) => Emoji,
                    _ => return start,
                }
            }
            LineFeed => {
                if ch == '\r' {
                    start = index;
                }
                return start;
            }
            Keycap if variation(ch) => VariationKeycap,
            Keycap | VariationKeycap => {
                if ch.is_ascii_digit() || matches!(ch, '#' | '*') {
                    start = index;
                }
                return start;
            }
            Modifier if variation(ch) => VariationModifier,
            Modifier | VariationModifier => {
                if !CodePointSetData::new::<EmojiModifierBase>().contains(ch) {
                    return start;
                }
                start = index;
                if tag_base || matches!(state, VariationModifier) {
                    return start;
                }
                Emoji
            }
            Variation => {
                if pictographic(ch) {
                    start = index;
                    if tag_base {
                        return start;
                    }
                    Emoji
                } else {
                    if !variation(ch)
                        && CodePointMapData::<CanonicalCombiningClass>::new().get(ch)
                            == CanonicalCombiningClass::NotReordered
                    {
                        start = index;
                    }
                    return start;
                }
            }
            Emoji if ch == '\u{200d}' => Joiner,
            Emoji => return start,
            Joiner if variation(ch) => VariationJoiner,
            Joiner | VariationJoiner => {
                if !pictographic(ch) {
                    return start;
                }
                start = index;
                if matches!(state, Joiner) && CodePointSetData::new::<EmojiModifier>().contains(ch)
                {
                    Modifier
                } else {
                    Emoji
                }
            }
            TagEnd | Tags if matches!(ch, '\u{e0020}'..='\u{e007e}') => {
                start = index;
                Tags
            }
            TagEnd => return start,
            Tags => {
                if CodePointSetData::new::<EmojiModifier>().contains(ch) {
                    start = index;
                    tag_base = true;
                    Modifier
                } else if variation(ch) {
                    start = index;
                    tag_base = true;
                    Variation
                } else {
                    if pictographic(ch) {
                        start = index;
                    }
                    return start;
                }
            }
        };
    }
    start
}
