//! Lengths named the way the web names them.

use crate::generated::ThemeTokens;

impl ThemeTokens {
    /// A spacing step by its CSS name: `"3"` is `--spacing-3`, `"0-5"` is
    /// `--spacing-0-5`.
    pub fn spacing(&self, step: &str) -> Option<f32> {
        Some(match step {
            "0" => self.spacing_0,
            "0-5" => self.spacing_0_5,
            "px" => self.spacing_px,
            "1" => self.spacing_1,
            "1-5" => self.spacing_1_5,
            "2" => self.spacing_2,
            "2-5" => self.spacing_2_5,
            "3" => self.spacing_3,
            "3-5" => self.spacing_3_5,
            "4" => self.spacing_4,
            "5" => self.spacing_5,
            "6" => self.spacing_6,
            "7" => self.spacing_7,
            "8" => self.spacing_8,
            "9" => self.spacing_9,
            "10" => self.spacing_10,
            "11" => self.spacing_11,
            "12" => self.spacing_12,
            "14" => self.spacing_14,
            "16" => self.spacing_16,
            "20" => self.spacing_20,
            _ => return None,
        })
    }
}

/// A CSS length as the stories write one: `"20rem"`, `"240px"`, or a
/// fraction of the container, `"100%"`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    Px(f32),
    Fraction(f32),
}

impl Length {
    pub fn parse(text: &str) -> Option<Length> {
        let text = text.trim();
        if let Some(rem) = text.strip_suffix("rem") {
            return rem.parse::<f32>().ok().map(|n| Length::Px(n * 16.0));
        }
        if let Some(px) = text.strip_suffix("px") {
            return px.parse::<f32>().ok().map(Length::Px);
        }
        if let Some(percent) = text.strip_suffix('%') {
            return percent
                .parse::<f32>()
                .ok()
                .map(|n| Length::Fraction(n / 100.0));
        }
        text.parse::<f32>().ok().map(Length::Px)
    }

    pub fn definite(self) -> gpui::DefiniteLength {
        match self {
            Length::Px(px) => gpui::px(px).into(),
            Length::Fraction(fraction) => gpui::relative(fraction),
        }
    }
}
