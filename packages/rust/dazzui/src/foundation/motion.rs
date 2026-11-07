//! Shared loading clock, accessibility policy and deterministic capture time.

use gpui::App;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MotionPreference {
    #[default]
    System,
    Full,
    Reduced,
}

pub struct Motion {
    preference: MotionPreference,
    epoch: Instant,
    frozen: Option<Duration>,
    settle_transitions: bool,
    #[cfg(target_os = "macos")]
    _observer: macos::Observer,
}

impl gpui::Global for Motion {}

impl Motion {
    pub(crate) fn init(cx: &mut App) {
        if cx.has_global::<Self>() {
            return;
        }
        let state = Self {
            preference: MotionPreference::System,
            epoch: Instant::now(),
            frozen: None,
            settle_transitions: false,
            #[cfg(target_os = "macos")]
            _observer: macos::Observer::new(cx),
        };
        cx.set_global(state);
    }

    /// Follow the system or a host's explicit accessibility policy.
    pub fn set_preference(preference: MotionPreference, cx: &mut App) {
        Self::init(cx);
        cx.global_mut::<Self>().preference = preference;
        cx.refresh_windows();
    }

    /// Stop the loading clock at an exact time, without scheduling frames.
    pub fn freeze_at(time: Duration, cx: &mut App) {
        Self::init(cx);
        cx.global_mut::<Self>().frozen = Some(time);
        cx.global_mut::<Self>().settle_transitions = true;
        cx.refresh_windows();
    }

    /// Pause time while retaining transition state, for deterministic
    /// interaction playback. Unlike `freeze_at`, this does not settle values.
    pub fn seek(time: Duration, cx: &mut App) {
        Self::init(cx);
        let state = cx.global_mut::<Self>();
        state.frozen = Some(time);
        state.settle_transitions = false;
        cx.refresh_windows();
    }

    pub fn resume(cx: &mut App) {
        Self::init(cx);
        let state = cx.global_mut::<Self>();
        if let Some(time) = state.frozen.take() {
            state.epoch = Instant::now()
                .checked_sub(time)
                .unwrap_or_else(Instant::now);
        }
        cx.refresh_windows();
    }

    pub(crate) fn frame(cx: &mut App) -> Frame {
        Self::init(cx);
        let state = cx.global::<Self>();
        Frame {
            elapsed: state.frozen.unwrap_or_else(|| state.epoch.elapsed()),
            frozen: state.frozen.is_some(),
            settled: state.frozen.is_some() && state.settle_transitions,
            reduced: match state.preference {
                MotionPreference::Full => false,
                MotionPreference::Reduced => true,
                MotionPreference::System => system_reduced(),
            },
        }
    }
}

#[cfg(target_os = "macos")]
fn system_reduced() -> bool {
    macos::reduced()
}
#[cfg(not(target_os = "macos"))]
fn system_reduced() -> bool {
    false
}

#[derive(Clone, Copy)]
pub(crate) struct Frame {
    pub elapsed: Duration,
    pub frozen: bool,
    pub settled: bool,
    pub reduced: bool,
}

impl Frame {
    pub fn cycle(self, duration_ms: f32, reduced_duration_ms: f32) -> f32 {
        let duration = if self.reduced {
            reduced_duration_ms
        } else {
            duration_ms
        };
        if duration <= 0. {
            return 0.;
        }
        (self.elapsed.as_secs_f64() * 1000. / duration as f64).fract() as f32
    }
}

/// CSS cubic-bezier: invert x before evaluating y, rather than treating time
/// as the Bézier parameter. This also supports the ds transition curve.
pub(crate) fn cubic_bezier(curve: [f32; 4], progress: f32) -> f32 {
    let progress = progress.clamp(0., 1.);
    if progress == 0. || progress == 1. {
        return progress;
    }
    let sample =
        |t: f32, a, b| 3. * (1. - t).powi(2) * t * a + 3. * (1. - t) * t * t * b + t.powi(3);
    let (mut low, mut high) = (0., 1.);
    for _ in 0..20 {
        let t = (low + high) / 2.;
        if sample(t, curve[0], curve[2]) < progress {
            low = t;
        } else {
            high = t;
        }
    }
    sample((low + high) / 2., curve[1], curve[3])
}

pub(crate) struct Transition {
    from: f32,
    to: f32,
    started: Duration,
    reversing_start: f32,
    shortening: f32,
}

impl Transition {
    pub fn new(value: f32, now: Duration) -> Self {
        Self {
            from: value,
            to: value,
            started: now,
            reversing_start: value,
            shortening: 1.,
        }
    }

    pub fn value(&self, now: Duration, duration_ms: f32, curve: [f32; 4]) -> f32 {
        if duration_ms <= 0. {
            return self.to;
        }
        let duration_ms = duration_ms * self.shortening;
        if duration_ms <= 0. {
            return self.to;
        }
        let progress =
            (now.saturating_sub(self.started).as_secs_f32() * 1000. / duration_ms).clamp(0., 1.);
        self.from + (self.to - self.from) * cubic_bezier(curve, progress)
    }

    pub fn target(&mut self, value: f32, frame: Frame, duration_ms: f32, curve: [f32; 4]) {
        if frame.reduced || frame.settled {
            *self = Self::new(value, frame.elapsed);
        } else if value != self.to {
            let current = self.value(frame.elapsed, duration_ms, curve);
            let reversing =
                value == self.reversing_start && self.running(frame.elapsed, duration_ms);
            let shortening = if reversing {
                let elapsed = frame.elapsed.saturating_sub(self.started).as_secs_f32() * 1000.;
                let progress = cubic_bezier(curve, elapsed / (duration_ms * self.shortening));
                (progress * self.shortening + (1. - self.shortening))
                    .abs()
                    .clamp(0., 1.)
            } else {
                1.
            };
            self.reversing_start = if reversing { self.to } else { current };
            self.shortening = shortening;
            self.from = current;
            self.to = value;
            self.started = frame.elapsed;
        }
    }

    pub fn running(&self, now: Duration, duration_ms: f32) -> bool {
        self.from != self.to
            && now.saturating_sub(self.started).as_secs_f32() * 1000.
                < duration_ms * self.shortening
    }
}

pub(crate) struct Presence(Transition);

#[derive(Clone, Copy)]
pub(crate) struct PresenceSample {
    pub mounted: bool,
    pub amount: f32,
    pub animating: bool,
}

impl Presence {
    pub fn new(open: bool, now: Duration) -> Self {
        Self(Transition::new(if open { 1. } else { 0. }, now))
    }

    pub fn sample(
        &mut self,
        open: bool,
        frame: Frame,
        duration: f32,
        curve: [f32; 4],
    ) -> PresenceSample {
        self.0
            .target(if open { 1. } else { 0. }, frame, duration, curve);
        let amount = self.0.value(frame.elapsed, duration, curve);
        PresenceSample {
            mounted: open || amount > 0.,
            amount,
            animating: !frame.frozen && self.0.running(frame.elapsed, duration),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initially_open_presence_is_already_at_rest() {
        let frame = Frame {
            elapsed: Duration::ZERO,
            frozen: false,
            settled: false,
            reduced: false,
        };
        let mut presence = Presence::new(true, Duration::ZERO);
        let sample = presence.sample(true, frame, 150., [0.4, 0., 0.2, 1.]);
        assert_eq!(sample.amount, 1.);
        assert!(
            sample.mounted && !sample.animating,
            "initially open DOM has no starting style"
        );
    }

    #[test]
    fn easing_matches_chromiums_web_animation_samples() {
        // Computed opacity of a paused 0->1 Web Animation, sampled in Chromium.
        let times = [0., 0.1, 0.25, 0.5, 0.75, 0.9, 1.];
        for (curve, expected) in [
            (
                [0.42, 0., 0.58, 1.],
                [0., 0.0197225, 0.129162, 0.5, 0.870838, 0.980278, 1.],
            ),
            (
                [0.4, 0., 0.2, 1.],
                [0., 0.025863, 0.236587, 0.775561, 0.959368, 0.994354, 1.],
            ),
        ] {
            for (time, expected) in times.into_iter().zip(expected) {
                assert!((cubic_bezier(curve, time) - expected).abs() < 0.00001);
            }
        }
    }

    #[gpui::test]
    fn frozen_clock_and_reduced_policy_select_the_reference_periods(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            crate::init(cx);
            Motion::set_preference(MotionPreference::Full, cx);
            Motion::freeze_at(Duration::from_millis(600), cx);
            let full = Motion::frame(cx);
            assert!(full.frozen && !full.reduced);
            assert!((full.cycle(1000., 2400.) - 0.6).abs() < 0.00001);
            assert!((full.cycle(1200., 3000.) - 0.5).abs() < 0.00001);
            Motion::set_preference(MotionPreference::Reduced, cx);
            let reduced = Motion::frame(cx);
            assert!(reduced.frozen && reduced.reduced);
            assert!((reduced.cycle(1000., 2400.) - 0.25).abs() < 0.00001);
            assert!((reduced.cycle(1200., 3000.) - 0.2).abs() < 0.00001);
            Motion::resume(cx);
            let resumed = Motion::frame(cx);
            assert!(!resumed.frozen && resumed.reduced);
            assert!(resumed.elapsed >= Duration::from_millis(600));
        });
    }

    #[test]
    fn progress_updates_reverse_from_the_current_width_and_reduced_motion_jumps() {
        let curve = [0.4, 0., 0.2, 1.];
        let frame = |ms, reduced| Frame {
            elapsed: Duration::from_millis(ms),
            frozen: false,
            settled: false,
            reduced,
        };
        let mut transition = Transition::new(0.2, Duration::ZERO);
        transition.target(0.8, frame(0, false), 300., curve);
        let halfway = transition.value(Duration::from_millis(150), 300., curve);
        assert!((halfway - 0.6653366).abs() < 0.00001);
        transition.target(0.1, frame(150, false), 300., curve);
        assert!(
            (transition.value(Duration::from_millis(150), 300., curve) - halfway).abs() < 0.00001,
            "retargeting must not jump back to an old endpoint"
        );
        assert!((transition.value(Duration::from_millis(450), 300., curve) - 0.1).abs() < 0.00001);
        assert!(!transition.running(Duration::from_millis(450), 300.));
        transition.target(0.9, frame(450, true), 300., curve);
        assert_eq!(
            transition.value(Duration::from_millis(450), 300., curve),
            0.9
        );
        assert!(!transition.running(Duration::from_millis(450), 300.));
    }
}

#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
mod macos {
    use block::ConcreteBlock;
    use gpui::App;
    use objc::{
        class, msg_send,
        runtime::{BOOL, Object, YES},
        sel, sel_impl,
    };

    #[link(name = "AppKit", kind = "framework")]
    unsafe extern "C" {
        static NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification: *mut Object;
    }

    pub fn reduced() -> bool {
        unsafe {
            let workspace: *mut Object = msg_send![class!(NSWorkspace), sharedWorkspace];
            let reduced: BOOL = msg_send![workspace, accessibilityDisplayShouldReduceMotion];
            reduced == YES
        }
    }

    pub struct Observer {
        center: *mut Object,
        token: *mut Object,
    }

    impl Observer {
        pub fn new(cx: &App) -> Self {
            let app = cx.to_async();
            let callback = ConcreteBlock::new(move |_: *mut Object| {
                let _ = app.update(|cx| cx.refresh_windows());
            })
            .copy();
            unsafe {
                let workspace: *mut Object = msg_send![class!(NSWorkspace), sharedWorkspace];
                let center: *mut Object = msg_send![workspace, notificationCenter];
                let center: *mut Object = msg_send![center, retain];
                let queue: *mut Object = msg_send![class!(NSOperationQueue), mainQueue];
                let token: *mut Object = msg_send![center,
                    addObserverForName: NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification
                    object: std::ptr::null_mut::<Object>() queue: queue usingBlock: &*callback];
                let token: *mut Object = msg_send![token, retain];
                Self { center, token }
            }
        }
    }

    impl Drop for Observer {
        fn drop(&mut self) {
            unsafe {
                let _: () = msg_send![self.center, removeObserver: self.token];
                let _: () = msg_send![self.token, release];
                let _: () = msg_send![self.center, release];
            }
        }
    }
}
