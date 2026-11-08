//! A box-shadow list is one CSS transition, including its geometry and color.

use super::{Frame, Transition, color::Color};
use gpui::{BoxShadow, Hsla, Pixels, point, px};
use std::time::Duration;

#[derive(Clone, PartialEq)]
pub(super) struct Shadows(pub Vec<(BoxShadow, bool)>);

impl Shadows {
    fn mix(&self, other: &Self, amount: f32) -> Self {
        let clear = BoxShadow {
            color: Hsla::transparent_black(),
            offset: point(px(0.), px(0.)),
            blur_radius: px(0.),
            spread_radius: px(0.),
        };
        let zero = (clear, false);
        Self(
            (0..self.0.len().max(other.0.len()))
                .map(|index| {
                    let (from, from_modern) = self.0.get(index).unwrap_or(&zero);
                    let (to, to_modern) = other.0.get(index).unwrap_or(&zero);
                    let color = Color {
                        value: from.color,
                        modern: *from_modern,
                    }
                    .mix(
                        Color {
                            value: to.color,
                            modern: *to_modern,
                        },
                        amount,
                    );
                    let mix = |a: Pixels, b: Pixels| a + (b - a) * amount;
                    (
                        BoxShadow {
                            color: color.value,
                            offset: point(
                                mix(from.offset.x, to.offset.x),
                                mix(from.offset.y, to.offset.y),
                            ),
                            blur_radius: mix(from.blur_radius, to.blur_radius),
                            spread_radius: mix(from.spread_radius, to.spread_radius),
                        },
                        color.modern,
                    )
                })
                .collect(),
        )
    }
}

pub(super) struct ShadowTransition {
    from: Shadows,
    to: Shadows,
    progress: Transition,
}

impl ShadowTransition {
    pub fn new(value: Shadows, now: Duration) -> Self {
        Self {
            from: value.clone(),
            to: value,
            progress: Transition::new(0., now),
        }
    }

    fn value(&self, now: Duration, duration: f32, curve: [f32; 4]) -> Shadows {
        let progress = self.progress.value(now, duration, curve);
        if progress == 0. {
            self.from.clone()
        } else if progress == 1. {
            self.to.clone()
        } else {
            self.from.mix(&self.to, progress)
        }
    }

    pub fn sample(
        &mut self,
        target: Shadows,
        frame: Frame,
        duration: f32,
        curve: [f32; 4],
    ) -> Vec<BoxShadow> {
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
        self.value(frame.elapsed, duration, curve)
            .0
            .into_iter()
            .map(|(shadow, _)| shadow)
            .collect()
    }

    pub fn running(&self, frame: Frame, duration: f32) -> bool {
        self.progress.running(frame.elapsed, duration)
    }
}
