//! CSS Color 4 interpolation, including the legacy sRGB compatibility rule.
//! https://www.w3.org/TR/css-color-4/#interpolation-space
//! Oklab matrices: https://bottosson.github.io/posts/oklab/

use super::{Frame, Transition};
use gpui::{Hsla, Rgba};
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
pub(super) struct Color {
    pub value: Hsla,
    pub modern: bool,
}

impl Color {
    fn mix(self, other: Self, amount: f32) -> Self {
        let modern = self.modern || other.modern;
        let left = Rgba::from(self.value);
        let right = Rgba::from(other.value);
        let a = left.a + (right.a - left.a) * amount;
        if a <= 0. {
            return Self {
                value: Hsla::transparent_black(),
                modern,
            };
        }
        let coordinates = |color: Rgba| {
            let rgb = [color.r, color.g, color.b];
            if modern { to_oklab(rgb) } else { rgb }
        };
        let left_channels = coordinates(left);
        let right_channels = coordinates(right);
        let channels = std::array::from_fn(|i| {
            (left_channels[i] * left.a * (1. - amount) + right_channels[i] * right.a * amount) / a
        });
        let [r, g, b] = if modern {
            from_oklab(channels)
        } else {
            channels
        };
        Self {
            value: Rgba { r, g, b, a }.into(),
            modern,
        }
    }
}

pub(super) struct ColorTransition {
    from: Color,
    to: Color,
    progress: Transition,
}

impl ColorTransition {
    pub fn new(color: Color, now: Duration) -> Self {
        Self {
            from: color,
            to: color,
            progress: Transition::new(0., now),
        }
    }

    fn value(&self, now: Duration, duration: f32, curve: [f32; 4]) -> Color {
        let progress = self.progress.value(now, duration, curve);
        if progress == 0. {
            self.from
        } else if progress == 1. {
            self.to
        } else {
            self.from.mix(self.to, progress)
        }
    }

    pub fn sample(&mut self, target: Color, frame: Frame, duration: f32, curve: [f32; 4]) -> Hsla {
        if frame.reduced || frame.settled {
            *self = Self::new(target, frame.elapsed);
        } else if target == self.from {
            self.progress.target(0., frame, duration, curve);
        } else if target == self.to {
            self.progress.target(1., frame, duration, curve);
        } else {
            self.from = self.value(frame.elapsed, duration, curve);
            self.to = target;
            self.progress = Transition::new(0., frame.elapsed);
            self.progress.target(1., frame, duration, curve);
        }
        self.value(frame.elapsed, duration, curve).value
    }

    pub fn running(&self, frame: Frame, duration: f32) -> bool {
        self.progress.running(frame.elapsed, duration)
    }
}

fn multiply(matrix: [[f32; 3]; 3], channels: [f32; 3]) -> [f32; 3] {
    matrix.map(|row| row.into_iter().zip(channels).map(|(a, b)| a * b).sum())
}

fn to_oklab(rgb: [f32; 3]) -> [f32; 3] {
    let rgb = rgb.map(|value| {
        if value.abs() <= 0.04045 {
            value / 12.92
        } else {
            value.signum() * ((value.abs() + 0.055) / 1.055).powf(2.4)
        }
    });
    let lms = multiply(
        [
            [0.41222146, 0.53633255, 0.051445995],
            [0.2119035, 0.6806995, 0.10739696],
            [0.08830246, 0.28171885, 0.6299787],
        ],
        rgb,
    )
    .map(f32::cbrt);
    multiply(
        [
            [0.21045426, 0.7936178, -0.004072047],
            [1.9779985, -2.4285922, 0.4505937],
            [0.025904037, 0.78277177, -0.80867577],
        ],
        lms,
    )
}

fn from_oklab(lab: [f32; 3]) -> [f32; 3] {
    let lms = multiply(
        [
            [1., 0.39633778, 0.21580376],
            [1., -0.105561346, -0.06385417],
            [1., -0.08948418, -1.2914855],
        ],
        lab,
    )
    .map(|value| value.powi(3));
    multiply(
        [
            [4.0767417, -3.3077116, 0.23096994],
            [-1.268438, 2.6097574, -0.3413194],
            [-0.0041960863, -0.7034186, 1.7076147],
        ],
        lms,
    )
    .map(|value| {
        let value = if value.abs() <= 0.0031308 {
            value * 12.92
        } else {
            value.signum() * (1.055 * value.abs().powf(1. / 2.4) - 0.055)
        };
        value.clamp(0., 1.)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(ms: u64) -> Frame {
        Frame {
            elapsed: Duration::from_millis(ms),
            frozen: true,
            settled: false,
            reduced: false,
        }
    }

    fn assert_bytes(actual: Hsla, expected: u32) {
        let actual = Rgba::from(actual);
        for (actual, expected) in [actual.r, actual.g, actual.b, actual.a]
            .into_iter()
            .zip(expected.to_be_bytes())
        {
            assert!((actual * 255. - expected as f32).abs() < 0.6);
        }
    }

    #[test]
    fn legacy_and_modern_color_midpoints_match_chromiums_web_animations() {
        let red = Color {
            value: gpui::rgb(0xff0000).into(),
            modern: false,
        };
        // Actual Chromium Web Animation samples, paused at 500/1000ms:
        // legacy red/green, red/color(srgb 0 1 0), and an alpha-bearing blue.
        for (value, modern, expected) in [
            (0x00ff00ff, false, 0x808000ff),
            (0x00ff00ff, true, 0xd0a800ff),
            (0x0080ff33, true, 0xe5485199),
        ] {
            let target = Color {
                value: gpui::rgba(value).into(),
                modern,
            };
            let mut transition = ColorTransition::new(red, Duration::ZERO);
            transition.sample(target, frame(0), 1000., [0., 0., 1., 1.]);
            assert_bytes(
                transition.sample(target, frame(500), 1000., [0., 0., 1., 1.]),
                expected,
            );
        }
    }

    #[test]
    fn color_reversal_preserves_the_current_paint_and_shortens_the_return() {
        let red = Color {
            value: gpui::rgb(0xff0000).into(),
            modern: false,
        };
        let green = Color {
            value: gpui::rgb(0x00ff00).into(),
            modern: true,
        };
        let curve = [0., 0., 1., 1.];
        let mut transition = ColorTransition::new(red, Duration::ZERO);
        transition.sample(green, frame(0), 1000., curve);
        let middle = transition.sample(green, frame(500), 1000., curve);
        assert_eq!(transition.sample(red, frame(500), 1000., curve), middle);
        assert_eq!(transition.sample(red, frame(1000), 1000., curve), red.value);
        assert!(!transition.running(frame(1000), 1000.));
        transition.sample(green, frame(1000), 1000., curve);
        let mut reduced = frame(1100);
        reduced.reduced = true;
        assert_eq!(transition.sample(green, reduced, 1000., curve), green.value);
        assert!(!transition.running(reduced, 1000.));
    }
}
