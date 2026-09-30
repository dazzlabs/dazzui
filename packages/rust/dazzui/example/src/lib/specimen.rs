//! The conformance hook: every node codegen addressed (`"$at"`) is drawn
//! through here, so the collector can find the box drawn at the same
//! address React's was.
//!
//! Outside a collection this is the element itself. While collecting, it is
//! wrapped in an element that records the bounds its child laid out at.

use std::cell::RefCell;
use std::collections::BTreeMap;

use gpui::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, Pixels, Window,
};
use serde_json::Value;

thread_local! {
    /// `Some` while collecting: every specimen's bounds, by its index.
    static FRAMES: RefCell<Option<BTreeMap<usize, Bounds<Pixels>>>> = const { RefCell::new(None) };
}

/// Starts recording, clearing what an earlier scene recorded.
pub fn start() {
    FRAMES.with(|frames| *frames.borrow_mut() = Some(BTreeMap::new()));
}

/// What has been recorded since [`start`].
pub fn frames() -> BTreeMap<usize, Bounds<Pixels>> {
    FRAMES.with(|frames| frames.borrow().clone().unwrap_or_default())
}

pub fn wrap(at: Option<&Value>, element: impl IntoElement, _path: &str) -> AnyElement {
    let collecting = FRAMES.with(|frames| frames.borrow().is_some());
    match at.and_then(Value::as_u64) {
        Some(index) if collecting => Specimen {
            index: index as usize,
            child: element.into_any_element(),
        }
        .into_any_element(),
        _ => element.into_any_element(),
    }
}

/// Lays its child out as if it were not there — the child's own layout id
/// is its — and records where it landed.
struct Specimen {
    index: usize,
    child: AnyElement,
}

impl IntoElement for Specimen {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for Specimen {
    type RequestLayoutState = LayoutId;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, LayoutId) {
        let layout = self.child.request_layout(window, cx);
        (layout, layout)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut LayoutId,
        window: &mut Window,
        cx: &mut App,
    ) {
        FRAMES.with(|frames| {
            if let Some(frames) = frames.borrow_mut().as_mut() {
                // A specimen drawn twice (a trigger inside an open popup's
                // twin) keeps the first box, as the walk addresses it.
                frames.entry(self.index).or_insert(bounds);
            }
        });
        self.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut LayoutId,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);
    }
}
