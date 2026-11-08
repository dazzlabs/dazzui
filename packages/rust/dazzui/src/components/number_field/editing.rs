use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};

const MINUS: &str = "-−－‒–—﹣";
const PLUS: &str = "+＋﹢";
const HAN: &str = "零〇一二三四五六七八九";

pub(super) fn numeral(ch: char) -> bool {
    ch.is_ascii_digit() || matches!(ch, '٠'..='٩' | '۰'..='۹' | '０'..='９') || HAN.contains(ch)
}

pub(super) fn minus(ch: char) -> bool {
    MINUS.contains(ch)
}
pub(super) fn plus(ch: char) -> bool {
    PLUS.contains(ch)
}

pub(super) fn allowed(ch: char) -> bool {
    numeral(ch)
        || minus(ch)
        || plus(ch)
        || ".,．，٫٬".contains(ch)
        || ch.general_category() == GeneralCategory::Format
}

pub(super) fn valid_text(text: &str) -> bool {
    text.trim().is_empty() || text.chars().all(allowed)
}

pub(super) fn allowed_key(
    key: &str,
    value: &str,
    selection: std::ops::Range<usize>,
    min: Option<f64>,
    allow_out_of_range: bool,
) -> bool {
    let key = if key == "space" { " " } else { key };
    let mut chars = key.chars();
    let Some(ch) = chars.next() else {
        return true;
    };
    if chars.next().is_some() {
        return true;
    }
    if !allowed(ch) || minus(ch) && min.is_some_and(|min| min >= 0.) && !allow_out_of_range {
        return false;
    }
    let all = selection == (0..value.len());
    if minus(ch) || plus(ch) {
        let existing = value.char_indices().find(|(_, existing)| {
            if minus(ch) {
                minus(*existing)
            } else {
                plus(*existing)
            }
        });
        return !value.chars().any(|ch| minus(ch) || plus(ch))
            || all
            || existing.is_some_and(|(at, _)| selection.contains(&at));
    }
    if ch == '.' {
        return value
            .find('.')
            .is_none_or(|at| all || selection.contains(&at));
    }
    true
}

/// The source input's default-format parser: normalize supported numeral
/// systems and signs, then read the numeric prefix as JS parseFloat does.
/// Pasting uses this independently from ordinary input character validation.
pub(super) fn parsed(text: &str) -> Option<f64> {
    let clean: String = text
        .chars()
        .filter(|ch| ch.general_category() != GeneralCategory::Format)
        .map(|ch| {
            if minus(ch) {
                '-'
            } else if plus(ch) {
                '+'
            } else {
                ch
            }
        })
        .collect();
    let mut input = clean.trim();
    let mut negative = false;
    if let Some(sign) = input.chars().last().filter(|ch| matches!(ch, '-' | '+')) {
        negative |= sign == '-';
        input = &input[..input.len() - 1];
    }
    input = input.trim_start();
    if let Some(sign) = input.chars().next().filter(|ch| matches!(ch, '-' | '+')) {
        negative |= sign == '-';
        input = &input[1..];
    }
    if clean.trim().eq_ignore_ascii_case("Infinity")
        || clean.trim().eq_ignore_ascii_case("+Infinity")
        || clean.trim().eq_ignore_ascii_case("-Infinity")
        || input.contains('∞')
    {
        return None;
    }
    // Base UI infers the Arabic decimal/group separators from Arabic or
    // Persian digits. Otherwise the package's default formatter uses English.
    let arabic = input.chars().any(|ch| matches!(ch, '٠'..='٩' | '۰'..='۹'));
    let mut normalized = String::new();
    for ch in input.chars() {
        match ch {
            ',' if !arabic => {}
            '，' | '٬' | '%' | '٪' | '％' | '﹪' | '‰' | '؉' => {}
            '．' | '٫' => normalized.push('.'),
            '٠'..='٩' | '۰'..='۹' | '０'..='９' => {
                normalized.push(char::from(b'0' + (ch as u32 % 16) as u8))
            }
            ch if HAN.contains(ch) => {
                let index = HAN.chars().position(|digit| digit == ch).unwrap();
                normalized.push(char::from(b'0' + index.saturating_sub(1) as u8));
            }
            ch => normalized.push(ch),
        }
    }
    // Mixed-locale input retains the last decimal point and drops earlier
    // ones, exactly as the source parser does before reading the prefix.
    if let Some(last) = normalized.rfind('.') {
        normalized = normalized
            .char_indices()
            .filter_map(|(index, ch)| (ch != '.' || index == last).then_some(ch))
            .collect();
    }
    if negative {
        normalized.insert(0, '-');
    }
    let value = float_prefix(normalized.trim_start())?;
    let shift = if text.contains(['‰', '؉']) {
        3
    } else if text.contains(['%', '٪', '％', '﹪']) {
        2
    } else {
        0
    };
    let value = if shift == 0 {
        value
    } else {
        // Shift the decimal representation, rather than introducing another
        // floating-point operation for percent and permille conversion.
        let value = value.to_string();
        let (coefficient, exponent) = value.split_once(['e', 'E']).unwrap_or((&value, "0"));
        format!("{coefficient}e{}", exponent.parse::<i32>().ok()? - shift)
            .parse::<f64>()
            .ok()?
    };
    value.is_finite().then_some(value)
}

fn float_prefix(text: &str) -> Option<f64> {
    let bytes = text.as_bytes();
    let mut at = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let start = at;
    while bytes.get(at).is_some_and(u8::is_ascii_digit) {
        at += 1;
    }
    let mut digits = at - start;
    if bytes.get(at) == Some(&b'.') {
        at += 1;
        let start = at;
        while bytes.get(at).is_some_and(u8::is_ascii_digit) {
            at += 1;
        }
        digits += at - start;
    }
    if digits == 0 {
        return None;
    }
    let end = at;
    if matches!(bytes.get(at), Some(b'e' | b'E')) {
        at += 1;
        if matches!(bytes.get(at), Some(b'+' | b'-')) {
            at += 1;
        }
        let start = at;
        while bytes.get(at).is_some_and(u8::is_ascii_digit) {
            at += 1;
        }
        if at == start {
            at = end;
        }
    }
    text[..at].parse::<f64>().ok()
}
