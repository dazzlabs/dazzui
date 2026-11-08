use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    time::Duration,
};

use gpui::{
    App, Div, ElementId, InteractiveElement, Modifiers, MouseButton, Stateful,
    StatefulInteractiveElement, Task, Window,
};

type Tick = Rc<dyn Fn(Modifiers, &mut Window, &mut App) -> bool>;
type Stop = Rc<dyn Fn(&mut Window, &mut App)>;

#[derive(Default)]
struct Hold {
    pressed: bool,
    inside: bool,
    modifiers: Modifiers,
    tick: Option<Tick>,
    task: Option<Task<()>>,
    on_stop: Option<Stop>,
}

impl Hold {
    fn start(this: &Rc<RefCell<Self>>, window: &mut Window, cx: &mut App) {
        this.borrow_mut().task = None;
        if !Self::tick(this, window, cx) {
            return;
        }
        let weak = Rc::downgrade(this);
        let task = window.spawn(cx, async move |cx| {
            // Base UI starts the interval after 400ms. Its first repeat is
            // one 60ms interval later, rather than at the start deadline.
            cx.background_executor()
                .timer(Duration::from_millis(400))
                .await;
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(60))
                    .await;
                let repeat = cx.update(|window, cx| {
                    weak.upgrade()
                        .is_some_and(|hold| Self::tick(&hold, window, cx))
                });
                if !matches!(repeat, Ok(true)) {
                    break;
                }
            }
        });
        this.borrow_mut().task = Some(task);
    }

    fn tick(this: &Rc<RefCell<Self>>, window: &mut Window, cx: &mut App) -> bool {
        let hold = this.borrow();
        let (pressed, modifiers, tick) = (hold.pressed, hold.modifiers, hold.tick.clone());
        drop(hold);
        pressed && tick.is_some_and(|tick| tick(modifiers, window, cx))
    }

    fn release(this: &Rc<RefCell<Self>>, window: &mut Window, cx: &mut App) {
        let stop = {
            let mut hold = this.borrow_mut();
            let pressed = std::mem::take(&mut hold.pressed);
            hold.task = None;
            if pressed { hold.on_stop.clone() } else { None }
        };
        if let Some(stop) = stop {
            stop(window, cx);
        }
    }
}

pub(super) fn bind(
    button: Stateful<Div>,
    id: ElementId,
    tick: Option<Tick>,
    on_press: impl Fn(&mut Window, &mut App) + 'static,
    on_stop: impl Fn(&mut Window, &mut App) + 'static,
    window: &mut Window,
    cx: &mut App,
) -> Stateful<Div> {
    // Only the mounted dispatch listeners own the hold. Keyed state keeps
    // a weak reference, so removing a field cancels its pending timer.
    let slot = window.use_keyed_state(id, cx, |_, _| Weak::<RefCell<Hold>>::new());
    let hold = slot.update(cx, |slot, _| {
        slot.upgrade().unwrap_or_else(|| {
            let hold = Rc::new(RefCell::new(Hold::default()));
            *slot = Rc::downgrade(&hold);
            hold
        })
    });
    {
        let mut hold = hold.borrow_mut();
        if tick.is_none() {
            // A boundary stops repeats but the release still commits the
            // hold's final value, as the source's global pointerup does.
            hold.task = None;
        }
        hold.tick = tick;
        hold.on_stop = Some(Rc::new(on_stop));
    }
    let down = hold.clone();
    let up = hold.clone();
    let outside = hold.clone();
    button
        .on_mouse_down(MouseButton::Left, move |event, window, cx| {
            if down.borrow().tick.is_none() || window.default_prevented() {
                return;
            }
            window.prevent_default();
            cx.stop_propagation();
            on_press(window, cx);
            {
                let mut hold = down.borrow_mut();
                hold.pressed = true;
                hold.inside = true;
                hold.modifiers = event.modifiers;
            }
            Hold::start(&down, window, cx);
        })
        .capture_any_mouse_up(move |event, window, cx| {
            if event.button == MouseButton::Left {
                Hold::release(&up, window, cx);
            }
        })
        .on_mouse_up_out(MouseButton::Left, move |_, window, cx| {
            Hold::release(&outside, window, cx);
        })
        .on_hover(move |inside, window, cx| {
            let previous = {
                let mut hold = hold.borrow_mut();
                std::mem::replace(&mut hold.inside, *inside)
            };
            if *inside && !previous && hold.borrow().pressed {
                // Reentry restarts with an immediate tick and fresh delay.
                hold.borrow_mut().modifiers = window.modifiers();
                Hold::start(&hold, window, cx);
            } else if !*inside {
                hold.borrow_mut().task = None;
            }
        })
}
