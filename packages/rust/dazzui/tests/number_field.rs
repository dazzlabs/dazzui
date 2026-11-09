use dazzui::foundation::text_input::{InputState, input_state};
use dazzui::{
    NumberField, NumberFieldChangeReason as ChangeReason, NumberFieldCommitReason as CommitReason,
    Theme,
};
use gpui::StatefulInteractiveElement;
use gpui::{
    Context, Entity, EntityInputHandler, InteractiveElement, IntoElement, ParentElement, Render,
    ScrollDelta, ScrollHandle, ScrollWheelEvent, Styled, TestAppContext, TouchPhase,
    VisualTestContext, Window, div, px,
};
use std::time::Duration;

#[derive(Clone, Copy, Default)]
enum Veto {
    #[default]
    None,
    All,
    Reason(ChangeReason),
    Normalization,
}
type Proposal = (Option<f64>, ChangeReason, Option<i8>, bool);

fn proposals(view: &Entity<Harness>, cx: &VisualTestContext) -> Vec<Proposal> {
    view.read_with(cx, |view, _| view.proposals.clone())
}
fn veto(view: &Entity<Harness>, cx: &mut VisualTestContext, rule: Veto) {
    // Change the live callback policy without rendering, as in the browser
    // reference fixture. A draft's authority must not depend on a repaint.
    view.update(cx, |view, _| view.options.veto = rule);
}
fn refocus(view: &Entity<Harness>, cx: &mut VisualTestContext) {
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|window, cx| input.read(cx).focus(window));
    cx.run_until_parked();
}

#[gpui::test]
fn canceled_keyboard_wheel_and_empty_seed_leave_numeric_state_unchanged(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            veto: Veto::Reason(ChangeReason::Keyboard),
            min: Some(0.),
            max: Some(30.),
            ..Default::default()
        },
    );
    cx.simulate_keystrokes("up down home end");
    assert_eq!(
        proposals(&view, cx),
        [
            (Some(17.), ChangeReason::Keyboard, Some(1), true),
            (Some(7.), ChangeReason::Keyboard, Some(-1), true),
            (Some(0.), ChangeReason::Keyboard, None, true),
            (Some(30.), ChangeReason::Keyboard, None, true)
        ]
    );
    assert_eq!(display(&view, cx), "12");
    assert!(commits(&view, cx).is_empty());
    veto(&view, cx, Veto::None);
    cx.simulate_keystrokes("up");
    assert_eq!(commits(&view, cx), [(Some(17.), CommitReason::Keyboard)]);
    let (view, cx) = setup(
        cx,
        Options {
            wheel: true,
            veto: Veto::Reason(ChangeReason::Wheel),
            ..Default::default()
        },
    );
    wheel_at_input(
        &view,
        cx,
        ScrollDelta::Lines(gpui::point(0., 1.)),
        gpui::Modifiers::none(),
    );
    assert_eq!(
        proposals(&view, cx),
        [(Some(17.), ChangeReason::Wheel, Some(1), true)]
    );
    assert_eq!(display(&view, cx), "12");
    assert!(commits(&view, cx).is_empty());
    veto(&view, cx, Veto::None);
    wheel_at_input(
        &view,
        cx,
        ScrollDelta::Lines(gpui::point(0., 1.)),
        gpui::Modifiers::none(),
    );
    assert_eq!(commits(&view, cx), [(Some(17.), CommitReason::Wheel)]);
    let (view, cx) = setup(
        cx,
        Options {
            initial: None,
            veto: Veto::Reason(ChangeReason::Keyboard),
            ..Default::default()
        },
    );
    cx.simulate_keystrokes("up");
    assert_eq!(
        proposals(&view, cx),
        [(Some(0.), ChangeReason::Keyboard, None, true)]
    );
    veto(&view, cx, Veto::None);
    cx.simulate_keystrokes("up");
    assert_eq!(commits(&view, cx), [(Some(0.), CommitReason::Keyboard)]);
}

#[gpui::test]
fn canceled_text_and_paste_preserve_drafts_without_reusing_a_canceled_step_base(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(
        cx,
        Options {
            veto: Veto::Reason(ChangeReason::InputChange),
            ..Default::default()
        },
    );
    fill_value(&view, cx, "20");
    assert_eq!(display(&view, cx), "20");
    veto(&view, cx, Veto::Reason(ChangeReason::Keyboard));
    cx.simulate_keystrokes("up");
    assert_eq!(display(&view, cx), "20");
    veto(&view, cx, Veto::None);
    cx.simulate_keystrokes("up");
    cx.run_until_parked();
    assert_eq!(
        proposals(&view, cx),
        [
            (Some(20.), ChangeReason::InputChange, None, true),
            (Some(25.), ChangeReason::Keyboard, Some(1), true),
            (Some(17.), ChangeReason::Keyboard, Some(1), false)
        ]
    );
    assert_eq!(display(&view, cx), "17");
    assert_eq!(commits(&view, cx), [(Some(17.), CommitReason::Keyboard)]);
    let (view, cx) = setup(
        cx,
        Options {
            veto: Veto::All,
            ..Default::default()
        },
    );
    fill_value(&view, cx, "20");
    blur(cx);
    assert_eq!(display(&view, cx), "20");
    assert_eq!(
        proposals(&view, cx),
        [
            (Some(20.), ChangeReason::InputChange, None, true),
            (Some(20.), ChangeReason::InputBlur, None, true)
        ]
    );
    assert!(commits(&view, cx).is_empty());
    refocus(&view, cx);
    veto(&view, cx, Veto::None);
    cx.simulate_keystrokes("up");
    cx.run_until_parked();
    assert_eq!(display(&view, cx), "17");
    let (view, cx) = setup(
        cx,
        Options {
            veto: Veto::Reason(ChangeReason::InputClear),
            ..Default::default()
        },
    );
    fill_value(&view, cx, "");
    blur(cx);
    assert_eq!(display(&view, cx), "");
    assert!(commits(&view, cx).is_empty());
    refocus(&view, cx);
    veto(&view, cx, Veto::None);
    cx.simulate_keystrokes("up");
    cx.run_until_parked();
    assert_eq!(display(&view, cx), "17");
    let (view, cx) = setup(
        cx,
        Options {
            veto: Veto::Reason(ChangeReason::InputPaste),
            ..Default::default()
        },
    );
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
    paste(cx, "12.2");
    assert_eq!(display(&view, cx), "12.2");
    assert_eq!(
        proposals(&view, cx),
        [(Some(12.2), ChangeReason::InputPaste, None, true)]
    );
    cx.simulate_keystrokes("up");
    assert_eq!(commits(&view, cx), [(Some(17.2), CommitReason::Keyboard)]);
}

#[gpui::test]
fn canceled_keyboard_does_not_commit_stale_manual_history_on_blur(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            controlled: true,
            veto: Veto::Reason(ChangeReason::Keyboard),
            ..Default::default()
        },
    );
    fill_value(&view, cx, "20");
    cx.simulate_keystrokes("up");
    blur(cx);
    assert_eq!(commits(&view, cx), [(Some(12.), CommitReason::InputBlur)]);
    assert_eq!(display(&view, cx), "12");
}

#[gpui::test]
fn canceled_pointer_ticks_commit_only_the_current_or_last_applied_value(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            veto: Veto::All,
            ..Default::default()
        },
    );
    let at = cx.debug_bounds("number-increment").unwrap().center();
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    wait(cx, 400);
    wait(cx, 60);
    assert_eq!(
        proposals(&view, cx),
        [(Some(17.), ChangeReason::IncrementPress, Some(1), true)]
    );
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    assert_eq!(
        commits(&view, cx),
        [(Some(12.), CommitReason::IncrementPress)]
    );
    let (view, cx) = setup(
        cx,
        Options {
            controlled: true,
            veto: Veto::Normalization,
            ..Default::default()
        },
    );
    fill_value(&view, cx, "12.2");
    let at = cx.debug_bounds("number-increment").unwrap().center();
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(
        proposals(&view, cx),
        [
            (Some(12.2), ChangeReason::InputChange, None, false),
            (Some(12.2), ChangeReason::IncrementPress, None, true),
            (Some(17.), ChangeReason::IncrementPress, Some(1), false)
        ]
    );
    assert_eq!(
        commits(&view, cx),
        [(Some(17.), CommitReason::IncrementPress)]
    );
    let (view, cx) = setup(cx, Options::default());
    let at = cx.debug_bounds("number-increment").unwrap().center();
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    veto(&view, cx, Veto::Reason(ChangeReason::IncrementPress));
    wait(cx, 400);
    wait(cx, 60);
    wait(cx, 60);
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    assert_eq!(
        proposals(&view, cx),
        [
            (Some(17.), ChangeReason::IncrementPress, Some(1), false),
            (Some(22.), ChangeReason::IncrementPress, Some(1), true)
        ]
    );
    assert_eq!(
        commits(&view, cx),
        [(Some(17.), CommitReason::IncrementPress)]
    );
}

fn wait(cx: &mut VisualTestContext, millis: u64) {
    cx.executor().advance_clock(Duration::from_millis(millis));
    cx.run_until_parked();
}

fn changes(view: &Entity<Harness>, cx: &VisualTestContext) -> Vec<Option<f64>> {
    view.read_with(cx, |view, _| view.changes.clone())
}

fn commits(view: &Entity<Harness>, cx: &VisualTestContext) -> Vec<(Option<f64>, CommitReason)> {
    view.read_with(cx, |view, _| view.commits.clone())
}

fn blur(cx: &mut VisualTestContext) {
    cx.update(|window, _| window.blur());
    wait(cx, 16);
}

#[gpui::test]
fn direct_entry_can_exceed_bounds_but_interactive_steps_still_clamp(cx: &mut TestAppContext) {
    for allow_out_of_range in [false, true] {
        let (view, cx) = setup(
            cx,
            Options {
                min: Some(0.),
                max: Some(10.),
                step: 1.,
                allow_out_of_range,
                ..Default::default()
            },
        );
        fill_value(&view, cx, "20");
        assert_eq!(
            changes(&view, cx),
            [Some(if allow_out_of_range { 20. } else { 10. })]
        );
        cx.simulate_keystrokes("enter");
        assert!(commits(&view, cx).is_empty());
        blur(cx);
        assert_eq!(
            commits(&view, cx),
            [(
                Some(if allow_out_of_range { 20. } else { 10. }),
                CommitReason::InputBlur
            )]
        );
        assert_eq!(
            changes(&view, cx),
            if allow_out_of_range {
                vec![Some(20.)]
            } else {
                vec![Some(10.), Some(10.)]
            }
        );
        let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
        cx.update(|window, cx| input.read(cx).focus(window));
        cx.simulate_keystrokes("down");
        cx.run_until_parked();
        assert_eq!(
            changes(&view, cx).last(),
            Some(&Some(if allow_out_of_range { 10. } else { 9. }))
        );
        assert_eq!(commits(&view, cx).last().unwrap().1, CommitReason::Keyboard);
    }
    let (view, cx) = setup(
        cx,
        Options {
            min: Some(0.),
            allow_out_of_range: true,
            ..Default::default()
        },
    );
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
    cx.simulate_keystrokes("-");
    assert_eq!(display(&view, cx), "-");
    blur(cx);
    assert!(commits(&view, cx).is_empty());

    let (view, cx) = setup(
        cx,
        Options {
            min: Some(0.),
            max: Some(10.),
            step: 1.,
            allow_out_of_range: true,
            ..Default::default()
        },
    );
    fill_value(&view, cx, "20");
    let at = cx.debug_bounds("number-decrement").unwrap().center();
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(changes(&view, cx), [Some(20.), Some(10.), Some(10.)]);
    assert!(commits(&view, cx).is_empty());
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(
        commits(&view, cx),
        [(Some(10.), CommitReason::DecrementPress)]
    );
}

#[gpui::test]
fn blur_commits_manual_edits_and_preserves_controlled_proposals(cx: &mut TestAppContext) {
    for initial in [Some(12.), None, Some(0.123456789)] {
        let (view, cx) = setup(
            cx,
            Options {
                initial,
                ..Default::default()
            },
        );
        blur(cx);
        assert!(commits(&view, cx).is_empty());
    }
    let (view, cx) = setup(cx, Options::default());
    fill_value(&view, cx, "");
    blur(cx);
    assert_eq!(commits(&view, cx), [(None, CommitReason::InputClear)]);
    let (view, cx) = setup(
        cx,
        Options {
            controlled: true,
            allow_out_of_range: true,
            ..Default::default()
        },
    );
    fill_value(&view, cx, "20");
    blur(cx);
    assert_eq!(commits(&view, cx), [(Some(20.), CommitReason::InputBlur)]);
    assert_eq!(changes(&view, cx), [Some(20.), Some(20.)]);
    assert_eq!(display(&view, cx), "12");
}

#[gpui::test]
fn changed_keyboard_and_wheel_values_commit_immediately_and_only_once(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            wheel: true,
            ..Default::default()
        },
    );
    cx.simulate_keystrokes("up");
    cx.run_until_parked();
    assert_eq!(commits(&view, cx), [(Some(17.), CommitReason::Keyboard)]);
    wheel_at_input(
        &view,
        cx,
        ScrollDelta::Lines(gpui::point(0., -1.)),
        gpui::Modifiers::none(),
    );
    assert_eq!(
        commits(&view, cx),
        [
            (Some(17.), CommitReason::Keyboard),
            (Some(12.), CommitReason::Wheel)
        ]
    );
    blur(cx);
    assert_eq!(commits(&view, cx).len(), 2);
    view.update(cx, |view, cx| {
        view.options.max = Some(12.);
        cx.notify();
    });
    cx.run_until_parked();
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|window, cx| input.read(cx).focus(window));
    cx.simulate_keystrokes("up");
    cx.run_until_parked();
    wheel_at_input(
        &view,
        cx,
        ScrollDelta::Lines(gpui::point(0., 1.)),
        gpui::Modifiers::none(),
    );
    assert_eq!(commits(&view, cx).len(), 2, "boundary no-ops do not commit");
}

#[gpui::test]
fn pointer_release_commits_after_holds_bounds_and_controlled_refusal(cx: &mut TestAppContext) {
    for (options, expected, count) in [
        (
            Options {
                max: Some(22.),
                ..Default::default()
            },
            22.,
            2,
        ),
        (
            Options {
                step: 0.,
                ..Default::default()
            },
            12.,
            0,
        ),
        (
            Options {
                controlled: true,
                ..Default::default()
            },
            17.,
            2,
        ),
    ] {
        let (view, cx) = setup(cx, options);
        let at = cx.debug_bounds("number-increment").unwrap().center();
        cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
        cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
        cx.run_until_parked();
        assert!(commits(&view, cx).is_empty());
        wait(cx, 400);
        wait(cx, 60);
        assert_eq!(changes(&view, cx).len(), count);
        assert!(commits(&view, cx).is_empty());
        cx.simulate_mouse_up(
            gpui::point(px(500.), px(400.)),
            gpui::MouseButton::Left,
            gpui::Modifiers::none(),
        );
        cx.run_until_parked();
        assert_eq!(
            commits(&view, cx),
            [(Some(expected), CommitReason::IncrementPress)]
        );
        blur(cx);
        assert_eq!(commits(&view, cx).len(), 1);
    }
}

#[gpui::test]
fn decrement_release_has_its_own_reason_and_unmount_does_not_commit(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, Options::default());
    let at = cx.debug_bounds("number-decrement").unwrap().center();
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(
        commits(&view, cx),
        [(Some(7.), CommitReason::DecrementPress)]
    );
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    view.update(cx, |view, cx| {
        view.mounted = false;
        cx.notify();
    });
    cx.run_until_parked();
    wait(cx, 400);
    wait(cx, 60);
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(commits(&view, cx).len(), 1);
}

#[gpui::test]
fn pointer_steps_on_down_then_repeats_without_an_extra_release_step(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, Options::default());
    let at = cx.debug_bounds("number-increment").unwrap().center();
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(changes(&view, cx), [Some(17.)]);
    wait(cx, 400);
    wait(cx, 59);
    assert_eq!(changes(&view, cx), [Some(17.)]);
    wait(cx, 1);
    assert_eq!(changes(&view, cx), [Some(17.), Some(22.)]);
    wait(cx, 60);
    assert_eq!(changes(&view, cx), [Some(17.), Some(22.), Some(27.)]);
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    wait(cx, 1000);
    assert_eq!(changes(&view, cx), [Some(17.), Some(22.), Some(27.)]);
}

#[gpui::test]
fn leaving_pauses_a_hold_and_reentry_steps_immediately(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, Options::default());
    let at = cx.debug_bounds("number-decrement").unwrap().center();
    let outside = gpui::point(px(500.), px(400.));
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(changes(&view, cx), [Some(7.)]);
    cx.simulate_mouse_move(
        outside,
        Some(gpui::MouseButton::Left),
        gpui::Modifiers::none(),
    );
    wait(cx, 1000);
    assert_eq!(changes(&view, cx), [Some(7.)]);
    cx.simulate_mouse_move(at, Some(gpui::MouseButton::Left), gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(changes(&view, cx), [Some(7.), Some(2.)]);
    wait(cx, 400);
    wait(cx, 60);
    assert_eq!(changes(&view, cx), [Some(7.), Some(2.), Some(-3.)]);
    cx.simulate_mouse_up(outside, gpui::MouseButton::Left, gpui::Modifiers::none());
    wait(cx, 1000);
    assert_eq!(changes(&view, cx).len(), 3);
}

#[gpui::test]
fn holds_use_press_modifiers_and_stop_at_bounds_or_unmount(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            max: Some(30.),
            ..Default::default()
        },
    );
    let at = cx.debug_bounds("number-increment").unwrap().center();
    let modifiers = gpui::Modifiers {
        shift: true,
        ..Default::default()
    };
    cx.simulate_mouse_move(at, None, modifiers);
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, modifiers);
    cx.run_until_parked();
    assert_eq!(changes(&view, cx), [Some(22.)]);
    wait(cx, 400);
    wait(cx, 60);
    assert_eq!(changes(&view, cx), [Some(22.), Some(30.)]);
    wait(cx, 1000);
    assert_eq!(changes(&view, cx).len(), 2);
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, modifiers);
    let at = cx.debug_bounds("number-decrement").unwrap().center();
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(changes(&view, cx).last(), Some(&Some(25.)));
    view.update(cx, |view, cx| {
        view.mounted = false;
        cx.notify();
    });
    cx.run_until_parked();
    wait(cx, 400);
    wait(cx, 60);
    assert_eq!(changes(&view, cx).len(), 3);
}

#[gpui::test]
fn holds_honor_live_read_only_disabled_and_controlled_refusal(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            controlled: true,
            ..Default::default()
        },
    );
    let at = cx.debug_bounds("number-increment").unwrap().center();
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    wait(cx, 400);
    wait(cx, 60);
    assert_eq!(changes(&view, cx), [Some(17.), Some(17.)]);
    view.update(cx, |view, cx| {
        view.options.read_only = true;
        cx.notify();
    });
    cx.run_until_parked();
    wait(cx, 1000);
    assert_eq!(changes(&view, cx).len(), 2);
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    wait(cx, 400);
    wait(cx, 60);
    assert_eq!(changes(&view, cx).len(), 2);
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    view.update(cx, |view, cx| {
        view.options.read_only = false;
        view.options.disabled = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    wait(cx, 400);
    wait(cx, 60);
    assert_eq!(changes(&view, cx).len(), 2);
}

#[gpui::test]
fn pointer_commits_dirty_controlled_text_before_stepping(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            controlled: true,
            ..Default::default()
        },
    );
    type_value(&view, cx, "12.2");
    view.update(cx, |view, _| view.changes.clear());
    let at = cx.debug_bounds("number-increment").unwrap().center();
    cx.simulate_mouse_move(at, None, gpui::Modifiers::none());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(changes(&view, cx), [Some(12.2), Some(17.2)]);
    assert_eq!(display(&view, cx), "12");
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, gpui::Modifiers::none());
    assert_eq!(changes(&view, cx).len(), 2);
}
struct Options {
    initial: Option<f64>,
    step: f64,
    min: Option<f64>,
    max: Option<f64>,
    snap: bool,
    controlled: bool,
    read_only: bool,
    disabled: bool,
    wheel: bool,
    allow_out_of_range: bool,
    veto: Veto,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            initial: Some(12.),
            step: 5.,
            min: None,
            max: None,
            snap: false,
            controlled: false,
            read_only: false,
            disabled: false,
            wheel: false,
            allow_out_of_range: false,
            veto: Veto::None,
        }
    }
}
struct Harness {
    options: Options,
    mounted: bool,
    input: Option<Entity<InputState>>,
    changes: Vec<Option<f64>>,
    commits: Vec<(Option<f64>, CommitReason)>,
    proposals: Vec<Proposal>,
    scroll: ScrollHandle,
    wheel_prevented: Vec<bool>,
}
impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.mounted {
            return div().w(px(240.)).child("Unmounted").into_any_element();
        }
        let input = window.with_id(gpui::ElementId::from("scroll-parent"), |window| {
            window.with_id(
                gpui::ElementId::Name(std::any::type_name::<NumberField>().into()),
                |window| input_state("number", window, cx),
            )
        });
        self.input = Some(input);
        let owner = cx.entity().downgrade();
        let commit_owner = owner.clone();
        let mut field = NumberField::new("number")
            .step(self.options.step)
            .snap_on_step(self.options.snap)
            .read_only(self.options.read_only)
            .disabled(self.options.disabled)
            .allow_wheel_scrub(self.options.wheel)
            .allow_out_of_range(self.options.allow_out_of_range)
            .on_value_committed(move |value, reason, _, cx| {
                let _ = commit_owner.update(cx, |view, _| view.commits.push((value, reason)));
            })
            .on_value_change_with_details(move |value, details, _, cx| {
                let _ = owner.update(cx, |view, _| {
                    let reject = match view.options.veto {
                        Veto::None => false,
                        Veto::All => true,
                        Veto::Reason(reason) => details.reason == reason,
                        Veto::Normalization => {
                            matches!(
                                details.reason,
                                ChangeReason::IncrementPress | ChangeReason::DecrementPress
                            ) && details.direction.is_none()
                        }
                    };
                    if reject {
                        details.cancel();
                    }
                    view.changes.push(value);
                    view.proposals.push((
                        value,
                        details.reason,
                        details.direction,
                        details.is_canceled(),
                    ));
                });
            });
        if self.options.controlled {
            field = field.value(self.options.initial);
        } else if let Some(value) = self.options.initial {
            field = field.default_value(value);
        }
        if let Some(value) = self.options.min {
            field = field.min(value);
        }
        if let Some(value) = self.options.max {
            field = field.max(value);
        }
        let owner = cx.entity().downgrade();
        div()
            .id("scroll-parent")
            .w(px(240.))
            .h(px(120.))
            .overflow_y_scroll()
            .track_scroll(&self.scroll)
            .on_scroll_wheel(move |_, window, cx| {
                let _ = owner.update(cx, |view, _| {
                    view.wheel_prevented.push(window.default_prevented())
                });
            })
            .child(field)
            .child(div().h(px(800.)))
            .into_any_element()
    }
}
fn setup(cx: &mut TestAppContext, options: Options) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        options,
        mounted: true,
        input: None,
        changes: vec![],
        commits: vec![],
        proposals: vec![],
        scroll: ScrollHandle::new(),
        wheel_prevented: vec![],
    });
    cx.run_until_parked();
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|window, cx| {
        window.activate_window();
        input.read(cx).focus(window);
    });
    cx.run_until_parked();
    (view, cx)
}
fn display(view: &Entity<Harness>, cx: &VisualTestContext) -> String {
    view.read_with(cx, |view, cx| {
        view.input.as_ref().unwrap().read(cx).value().to_string()
    })
}
fn type_value(view: &Entity<Harness>, cx: &mut VisualTestContext, text: &str) {
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
    if text.is_empty() {
        cx.simulate_keystrokes("backspace");
    } else {
        cx.simulate_input(text);
    }
    cx.run_until_parked();
}

fn fill_value(view: &Entity<Harness>, cx: &mut VisualTestContext, text: &str) {
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            let length = input.value().encode_utf16().count();
            input.replace_text_in_range(Some(0..length), text, window, cx);
        })
    });
    cx.run_until_parked();
}

fn paste(cx: &mut VisualTestContext, text: &str) {
    cx.write_to_clipboard(gpui::ClipboardItem::new_string(text.into()));
    cx.simulate_keystrokes(if cfg!(target_os = "macos") {
        "cmd-v"
    } else {
        "ctrl-v"
    });
    cx.run_until_parked();
}

#[gpui::test]
fn typed_text_rejects_invalid_characters_and_preserves_partial_and_unicode_numbers(
    cx: &mut TestAppContext,
) {
    for text in ["abc", "12e3", "12%", "12 3", "∞"] {
        let (view, cx) = setup(cx, Options::default());
        fill_value(&view, cx, text);
        assert_eq!(display(&view, cx), "12", "{text}");
        assert!(changes(&view, cx).is_empty(), "{text}");
    }
    for (text, value) in [
        ("+", None),
        ("-", None),
        (".", None),
        ("12.", Some(12.)),
        ("1.2.3", Some(12.3)),
        ("1,2,3", Some(123.)),
        ("12-", Some(-12.)),
        ("--12", None),
        ("+-12", Some(-12.)),
        ("12+3", Some(12.)),
        ("１２．３", Some(12.3)),
        ("١٢٫٣", Some(12.3)),
        ("۱۲٫۳", Some(12.3)),
        ("一二．三", Some(12.3)),
        ("\u{200e}12\u{200f}", Some(12.)),
    ] {
        let (view, cx) = setup(cx, Options::default());
        fill_value(&view, cx, text);
        assert_eq!(display(&view, cx), text);
        assert_eq!(
            changes(&view, cx),
            value.map(|value| vec![Some(value)]).unwrap_or_default(),
            "{text}"
        );
    }
}

#[gpui::test]
fn paste_accepts_numeric_prefixes_and_preserves_insertion_and_utf16_selection(
    cx: &mut TestAppContext,
) {
    for (text, value) in [
        ("12e3", 12000.),
        ("12%", 0.12),
        ("12‰", 0.012),
        ("12foo", 12.),
        ("12 3", 12.),
        ("１２．３", 12.3),
        ("١٢٫٣", 12.3),
    ] {
        let (view, cx) = setup(cx, Options::default());
        let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
        cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
        paste(cx, text);
        assert_eq!(display(&view, cx), text);
        assert_eq!(changes(&view, cx), [Some(value)], "{text}");
        assert_eq!(
            input.read_with(cx, |input, _| input.selected_range()),
            text.len()..text.len()
        );
    }
    let (view, cx) = setup(cx, Options::default());
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
    paste(cx, "abc");
    assert_eq!(display(&view, cx), "12");
    assert!(changes(&view, cx).is_empty());
    cx.simulate_keystrokes("left right");
    paste(cx, "5");
    assert_eq!(display(&view, cx), "152");
    assert_eq!(changes(&view, cx), [Some(152.)]);
    assert_eq!(input.read_with(cx, |input, _| input.selected_range()), 2..2);
    cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
    cx.simulate_keystrokes("left right shift-right");
    paste(cx, "９");
    assert_eq!(display(&view, cx), "1９2");
    assert_eq!(changes(&view, cx).last(), Some(&Some(192.)));
    let range = cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input.selected_text_range(false, window, cx).unwrap().range
        })
    });
    assert_eq!(range, 2..2);
}

#[gpui::test]
fn composition_rejects_invalid_marked_text_without_losing_the_committed_numeral(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx, Options::default());
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input.replace_and_mark_text_in_range(Some(0..2), "yi", Some(2..2), window, cx)
        })
    });
    cx.run_until_parked();
    assert_eq!(display(&view, cx), "12");
    assert!(changes(&view, cx).is_empty());
    cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input.replace_text_in_range(None, "一", window, cx)
        })
    });
    cx.run_until_parked();
    assert_eq!(display(&view, cx), "12一");
    assert_eq!(changes(&view, cx), [Some(121.)]);
}

#[gpui::test]
fn enter_preserves_the_draft_and_blur_only_normalizes_parseable_text(cx: &mut TestAppContext) {
    for (text, blurred, expected) in [
        ("-", "-", None),
        ("+", "+", None),
        (".", ".", None),
        ("12.", "12", Some(12.)),
        ("1.2.3", "12.3", Some(12.3)),
    ] {
        let (view, cx) = setup(cx, Options::default());
        fill_value(&view, cx, text);
        cx.simulate_keystrokes("enter");
        cx.run_until_parked();
        assert_eq!(display(&view, cx), text, "Enter preserves editing text");
        cx.update(|window, _| window.blur());
        wait(cx, 16);
        cx.update(|window, _| assert!(window.is_window_active()));
        assert_eq!(display(&view, cx), blurred);
        assert_eq!(
            changes(&view, cx),
            expected.map(|value| vec![Some(value)]).unwrap_or_default()
        );
    }
}

#[gpui::test]
fn enter_in_a_number_field_submits_the_enclosing_form(cx: &mut TestAppContext) {
    struct NumberForm(usize);
    impl Render for NumberForm {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let owner = cx.entity().downgrade();
            dazzui::Form::new()
                .on_submit(move |_, cx| {
                    let _ = owner.update(cx, |form, _| form.0 += 1);
                })
                .child(NumberField::new("form-number").default_value(12.))
                .child(dazzui::Button::new("submit").child("Submit"))
        }
    }
    cx.update(dazzui::init);
    let (view, cx) = cx.add_window_view(|_, _| NumberForm(0));
    cx.run_until_parked();
    cx.simulate_keystrokes("tab enter");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |form, _| form.0), 1);
}

#[gpui::test]
fn key_filter_blocks_duplicate_symbols_and_preserves_navigation_and_replacement(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx, Options::default());
    type_value(&view, cx, "1.2");
    cx.simulate_keystrokes(". e space");
    assert_eq!(display(&view, cx), "1.2");
    cx.simulate_keystrokes("left right backspace");
    assert_eq!(display(&view, cx), "1.");
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
    cx.simulate_keystrokes("-");
    assert_eq!(display(&view, cx), "-");
    cx.simulate_keystrokes("-");
    assert_eq!(display(&view, cx), "-");
    cx.simulate_keystrokes("1");
    assert_eq!(display(&view, cx), "-1");
    view.update(cx, |view, cx| {
        view.options.min = Some(0.);
        cx.notify();
    });
    cx.run_until_parked();
    cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
    cx.simulate_keystrokes("-");
    assert_eq!(
        display(&view, cx),
        "-1",
        "non-negative bounds disallow a newly typed minus"
    );
}

fn wheel_at_input(
    view: &Entity<Harness>,
    cx: &mut VisualTestContext,
    delta: ScrollDelta,
    modifiers: gpui::Modifiers,
) {
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    let caret = cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input
                .bounds_for_range(0..0, Default::default(), window, cx)
                .unwrap()
        })
    });
    cx.simulate_event(ScrollWheelEvent {
        position: caret.origin + gpui::point(px(2.), caret.size.height / 2.),
        delta,
        modifiers,
        touch_phase: TouchPhase::Moved,
    });
    cx.run_until_parked();
}

#[gpui::test]
fn wheel_is_opt_in_focused_and_leaves_zoom_and_page_scroll_available(cx: &mut TestAppContext) {
    for (options, blur, modifiers) in [
        (Options::default(), false, gpui::Modifiers::none()),
        (
            Options {
                wheel: true,
                ..Default::default()
            },
            true,
            gpui::Modifiers::none(),
        ),
        (
            Options {
                wheel: true,
                read_only: true,
                ..Default::default()
            },
            false,
            gpui::Modifiers::none(),
        ),
        (
            Options {
                wheel: true,
                disabled: true,
                ..Default::default()
            },
            false,
            gpui::Modifiers::none(),
        ),
        (
            Options {
                wheel: true,
                ..Default::default()
            },
            false,
            gpui::Modifiers {
                control: true,
                ..Default::default()
            },
        ),
    ] {
        let (view, cx) = setup(cx, options);
        if blur {
            cx.update(|window, _| window.blur());
            cx.run_until_parked();
        }
        wheel_at_input(
            &view,
            cx,
            ScrollDelta::Pixels(gpui::point(px(0.), px(-80.))),
            modifiers,
        );
        assert!(changes(&view, cx).is_empty());
        assert_eq!(
            view.read_with(cx, |view, _| view.wheel_prevented.clone()),
            [false]
        );
        assert!(view.read_with(cx, |view, _| view.scroll.offset().y) < px(0.));
    }
}

#[gpui::test]
fn focused_wheel_steps_once_per_event_and_prevents_only_default_scrolling(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            wheel: true,
            ..Default::default()
        },
    );
    for (delta, modifiers, expected) in [
        (
            ScrollDelta::Pixels(gpui::point(px(0.), px(0.1))),
            gpui::Modifiers::none(),
            17.,
        ),
        (
            ScrollDelta::Pixels(gpui::point(px(0.), px(-800.))),
            gpui::Modifiers::none(),
            12.,
        ),
        (
            ScrollDelta::Lines(gpui::point(0., 4.)),
            gpui::Modifiers {
                shift: true,
                ..Default::default()
            },
            22.,
        ),
        (
            ScrollDelta::Lines(gpui::point(0., -1.)),
            gpui::Modifiers {
                alt: true,
                ..Default::default()
            },
            21.9,
        ),
        (
            ScrollDelta::Pixels(gpui::point(px(-90.), px(0.))),
            gpui::Modifiers::none(),
            26.9,
        ),
    ] {
        wheel_at_input(&view, cx, delta, modifiers);
        assert_eq!(changes(&view, cx).last(), Some(&Some(expected)));
        assert_eq!(
            view.read_with(cx, |view, _| view.wheel_prevented.last().copied()),
            Some(true),
            "ancestor observers must still receive the prevented event"
        );
        assert_eq!(view.read_with(cx, |view, _| view.scroll.offset().y), px(0.));
    }
    let at = cx.debug_bounds("number-increment").unwrap().center();
    cx.simulate_event(ScrollWheelEvent {
        position: at,
        delta: ScrollDelta::Pixels(gpui::point(px(0.), px(-80.))),
        modifiers: gpui::Modifiers::none(),
        touch_phase: TouchPhase::Moved,
    });
    cx.run_until_parked();
    assert_eq!(
        changes(&view, cx).len(),
        5,
        "the stepper is outside the input's wheel target"
    );
    assert!(view.read_with(cx, |view, _| view.scroll.offset().y) < px(0.));
}

#[gpui::test]
fn wheel_uses_authoritative_controlled_values_and_consumes_boundary_no_ops(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(
        cx,
        Options {
            wheel: true,
            controlled: true,
            ..Default::default()
        },
    );
    type_value(&view, cx, "12.2");
    view.update(cx, |view, _| view.changes.clear());
    for _ in 0..2 {
        wheel_at_input(
            &view,
            cx,
            ScrollDelta::Lines(gpui::point(0., 1.)),
            gpui::Modifiers::none(),
        );
    }
    assert_eq!(changes(&view, cx), [Some(17.), Some(17.)]);
    assert_eq!(display(&view, cx), "12");
    view.update(cx, |view, cx| {
        view.options.max = Some(12.);
        cx.notify();
    });
    cx.run_until_parked();
    wheel_at_input(
        &view,
        cx,
        ScrollDelta::Lines(gpui::point(0., 1.)),
        gpui::Modifiers::none(),
    );
    assert_eq!(changes(&view, cx).len(), 2);
    assert_eq!(
        view.read_with(cx, |view, _| view.wheel_prevented.last().copied()),
        Some(true)
    );
    assert_eq!(view.read_with(cx, |view, _| view.scroll.offset().y), px(0.));
    view.update(cx, |view, cx| {
        view.options.wheel = false;
        cx.notify();
    });
    cx.run_until_parked();
    wheel_at_input(
        &view,
        cx,
        ScrollDelta::Pixels(gpui::point(px(0.), px(-80.))),
        gpui::Modifiers::none(),
    );
    assert_eq!(
        view.read_with(cx, |view, _| view.wheel_prevented.last().copied()),
        Some(false)
    );
    assert!(view.read_with(cx, |view, _| view.scroll.offset().y) < px(0.));
}
#[gpui::test]
fn keyboard_uses_unsnapped_steps_modifiers_and_explicit_boundaries(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            min: Some(-2.),
            max: Some(30.),
            ..Default::default()
        },
    );
    for (key, want) in [
        ("up", 17.),
        ("down", 12.),
        ("shift-up", 22.),
        ("alt-down", 21.9),
        ("home", -2.),
        ("end", 30.),
        ("alt-shift-down", 29.9),
    ] {
        cx.simulate_keystrokes(key);
        cx.run_until_parked();
        assert_eq!(
            view.read_with(cx, |view, _| view.changes.last().cloned()),
            Some(Some(want)),
            "{key}"
        );
    }
}
#[gpui::test]
fn explicit_snapping_empty_ranges_and_zero_steps_match_number_semantics(cx: &mut TestAppContext) {
    for (options, keys, want) in [
        (
            Options {
                snap: true,
                min: Some(0.),
                ..Default::default()
            },
            "up down shift-up alt-down",
            Some(19.9),
        ),
        (
            Options {
                initial: None,
                min: Some(-10.),
                max: Some(-2.),
                ..Default::default()
            },
            "up",
            Some(-2.),
        ),
        (
            Options {
                initial: None,
                min: Some(-10.),
                max: Some(10.),
                ..Default::default()
            },
            "down",
            Some(0.),
        ),
        (
            Options {
                step: 0.,
                ..Default::default()
            },
            "up down",
            None,
        ),
    ] {
        let (view, cx) = setup(cx, options);
        cx.simulate_keystrokes(keys);
        cx.run_until_parked();
        assert_eq!(
            view.read_with(cx, |view, _| view.changes.last().copied().flatten()),
            want
        );
    }
}
#[gpui::test]
fn controlled_rejection_steps_from_dirty_text_without_an_intermediate_commit(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(
        cx,
        Options {
            controlled: true,
            ..Default::default()
        },
    );
    cx.simulate_keystrokes("up up");
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.clone()),
        [Some(17.), Some(17.)]
    );
    type_value(&view, cx, "12.2");
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.last().copied()),
        Some(Some(12.2))
    );
    view.update(cx, |view, _| view.changes.clear());
    cx.simulate_keystrokes("up");
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.clone()),
        [Some(17.2)]
    );
    assert_eq!(display(&view, cx), "12");
}
#[gpui::test]
fn display_rounding_does_not_replace_precise_numeric_state(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            initial: Some(0.123456789),
            step: 0.00000001,
            ..Default::default()
        },
    );
    assert_eq!(display(&view, cx), "0.123");
    cx.simulate_keystrokes("up");
    cx.run_until_parked();
    assert_eq!(display(&view, cx), "0.123");
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.last().copied()),
        Some(Some(0.123456799))
    );
    type_value(&view, cx, "");
    cx.simulate_keystrokes("down");
    cx.run_until_parked();
    assert_eq!(display(&view, cx), "0");
    let (view, cx) = setup(
        cx,
        Options {
            initial: Some(1234.56789),
            ..Default::default()
        },
    );
    assert_eq!(display(&view, cx), "1,234.568");
}
#[gpui::test]
fn read_only_fields_ignore_numeric_keyboard_changes(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            min: Some(0.),
            max: Some(30.),
            read_only: true,
            ..Default::default()
        },
    );
    cx.simulate_keystrokes("up shift-down alt-up home end");
    cx.run_until_parked();
    assert!(view.read_with(cx, |view, _| view.changes.is_empty()));
    assert_eq!(display(&view, cx), "12");
}

#[gpui::test]
fn keyboard_scope_uses_updated_rules_and_releases_on_unmount(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, Options::default());
    view.update(cx, |view, cx| {
        view.options.step = 2.;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("up");
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.clone()),
        [Some(14.)]
    );
    view.update(cx, |view, cx| {
        view.mounted = false;
        view.changes.clear();
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("up alt-down home end");
    cx.run_until_parked();
    assert!(view.read_with(cx, |view, _| view.changes.is_empty()));
}

#[gpui::test]
fn undo_redo_restore_numeric_drafts_without_committing_and_keep_step_authority(
    cx: &mut TestAppContext,
) {
    let undo = if cfg!(target_os = "macos") {
        "cmd-z"
    } else {
        "ctrl-z"
    };
    let redo = if cfg!(target_os = "macos") {
        "cmd-shift-z"
    } else {
        "ctrl-shift-z"
    };
    for (draft, controlled, expected_changes, stepped) in [
        ("20", false, vec![Some(20.), Some(12.), Some(20.)], 21.),
        ("-", false, vec![Some(12.)], 13.),
        ("20", true, vec![Some(20.), Some(12.), Some(20.)], 21.),
    ] {
        let (view, cx) = setup(
            cx,
            Options {
                step: 1.,
                controlled,
                ..Default::default()
            },
        );
        fill_value(&view, cx, draft);
        cx.simulate_keystrokes(undo);
        assert_eq!(display(&view, cx), "12");
        cx.simulate_keystrokes(redo);
        assert_eq!(display(&view, cx), draft);
        assert_eq!(changes(&view, cx), expected_changes);
        assert!(commits(&view, cx).is_empty());
        cx.simulate_keystrokes("up");
        let display_after_step = if controlled {
            "12".to_owned()
        } else {
            stepped.to_string()
        };
        assert_eq!(display(&view, cx), display_after_step);
        assert_eq!(
            commits(&view, cx),
            [(Some(stepped), CommitReason::Keyboard)]
        );
        cx.simulate_keystrokes(undo);
        assert_eq!(display(&view, cx), display_after_step);
        cx.simulate_keystrokes(redo);
        assert_eq!(display(&view, cx), display_after_step);
    }
    let (view, cx) = setup(
        cx,
        Options {
            step: 1.,
            ..Default::default()
        },
    );
    type_value(&view, cx, "345");
    cx.simulate_keystrokes(undo);
    assert_eq!(display(&view, cx), "34");
    cx.simulate_keystrokes(redo);
    assert_eq!(display(&view, cx), "345");
    cx.simulate_keystrokes("up");
    assert_eq!(display(&view, cx), "346");
}

#[gpui::test]
fn refused_numeric_changes_preserve_the_typing_group(cx: &mut TestAppContext) {
    let (view, cx) = setup(
        cx,
        Options {
            controlled: true,
            ..Default::default()
        },
    );
    type_value(&view, cx, "345");
    cx.simulate_keystrokes(if cfg!(target_os = "macos") {
        "cmd-z"
    } else {
        "ctrl-z"
    });
    assert_eq!(display(&view, cx), "12");
    cx.simulate_keystrokes(if cfg!(target_os = "macos") {
        "cmd-shift-z"
    } else {
        "ctrl-shift-z"
    });
    assert_eq!(display(&view, cx), "345");
}
