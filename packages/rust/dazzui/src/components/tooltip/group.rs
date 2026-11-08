//! Base UI's TooltipProvider: one active hint and a short instant phase.

use crate::components::popup::PopupState;
use crate::foundation::provide;
use gpui::{
    AnyElement, App, Bounds, Element, ElementId, Entity, EntityId, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, Pixels, Task, WeakEntity, Window,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    time::Duration,
};

/// Shares timing across the tooltips in one content root, without adding a
/// layout box. A nested provider starts an independent group.
pub struct TooltipProvider {
    id: ElementId,
    child: Option<AnyElement>,
    delay: Option<Duration>,
    close_delay: Option<Duration>,
    timeout: Duration,
}

impl TooltipProvider {
    pub fn new(id: impl Into<ElementId>, child: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            child: Some(child.into_any_element()),
            delay: None,
            close_delay: None,
            timeout: Duration::from_millis(400),
        }
    }
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }
    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.close_delay = Some(delay);
        self
    }
    /// How long instant switching survives after the active hint closes.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

impl IntoElement for TooltipProvider {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for TooltipProvider {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn transparent_child_mut(&mut self) -> Option<&mut AnyElement> {
        self.child.as_mut()
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let group = window.use_keyed_state("tooltip-delay-group", cx, |_, _| DelayGroup::default());
        let group = group.read(cx).clone();
        {
            let mut state = group.0.borrow_mut();
            state.delay = self.delay;
            state.close_delay = self.close_delay;
            state.timeout = self.timeout;
        }
        self.child = Some(provide(group, self.child.take().unwrap()).into_any_element());
        (self.child.as_mut().unwrap().request_layout(window, cx), ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.as_mut().unwrap().prepaint(window, cx);
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.as_mut().unwrap().paint(window, cx);
    }
}

#[derive(Clone, Default)]
pub(super) struct DelayGroup(Rc<RefCell<GroupState>>);

#[derive(Default)]
struct GroupState {
    delay: Option<Duration>,
    close_delay: Option<Duration>,
    timeout: Duration,
    current: Option<ActiveHint>,
    reset: Option<Task<()>>,
}

struct ActiveHint {
    id: EntityId,
    popup: WeakEntity<PopupState>,
    token: Weak<()>,
    open: bool,
}

impl DelayGroup {
    pub fn delay(&self, own: Option<Duration>, default: Duration) -> Duration {
        let mut state = self.0.borrow_mut();
        // An unmounted open hint immediately releases its group's instant
        // phase. A closed one retains it until the reset timeout expires.
        if state
            .current
            .as_ref()
            .is_some_and(|current| current.open && current.token.strong_count() == 0)
        {
            state.current = None;
            state.reset = None;
        }
        if state.current.is_some() || state.delay == Some(Duration::ZERO) {
            Duration::ZERO
        } else {
            own.or(state.delay).unwrap_or(default)
        }
    }

    pub fn close_delay(&self, own: Option<Duration>, default: Duration) -> Duration {
        own.or(self.0.borrow().close_delay).unwrap_or(default)
    }

    pub fn changed(
        &self,
        popup: &Entity<PopupState>,
        token: Weak<()>,
        open: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let id = popup.entity_id();
        if open {
            let previous = {
                let mut state = self.0.borrow_mut();
                state.reset = None;
                state.current.replace(ActiveHint {
                    id,
                    popup: popup.downgrade(),
                    token,
                    open,
                })
            };
            if let Some(previous) = previous.filter(|previous| previous.id != id) {
                let _ = previous
                    .popup
                    .update(cx, |popup, cx| popup.hide(window, cx));
            }
            return;
        }
        let timeout = {
            let mut state = self.0.borrow_mut();
            let Some(current) = state.current.as_mut().filter(|current| current.id == id) else {
                return;
            };
            current.open = false;
            state.timeout
        };
        if timeout.is_zero() {
            let mut state = self.0.borrow_mut();
            state.current = None;
            state.reset = None;
            return;
        }
        let weak = Rc::downgrade(&self.0);
        let task = window.spawn(cx, async move |cx| {
            cx.background_executor().timer(timeout).await;
            if let Some(group) = weak.upgrade() {
                let mut state = group.borrow_mut();
                if state
                    .current
                    .as_ref()
                    .is_some_and(|current| current.id == id && !current.open)
                {
                    state.current = None;
                    state.reset = None;
                }
            }
        });
        self.0.borrow_mut().reset = Some(task);
    }
}
