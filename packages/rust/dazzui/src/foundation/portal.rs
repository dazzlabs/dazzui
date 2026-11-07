//! GPUI cannot enqueue a deferred draw while painting another deferred tree.
//! Discover nested kit portals during layout, then enqueue them together.

use std::{cell::RefCell, rc::Rc};

use gpui::{
    AnyElement, App, AvailableSpace, Bounds, Element, ElementId, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, Pixels, Position, Style, Window,
};

use super::{provide, scoped};

struct Layer {
    child: AnyElement,
    priority: usize,
    namespace: Vec<ElementId>,
}

#[derive(Clone)]
struct Portals {
    layers: Rc<RefCell<Vec<Layer>>>,
    depth: usize,
    base_namespace_len: usize,
}

pub(crate) fn portal(child: impl IntoElement, priority: usize) -> impl IntoElement {
    Portal {
        child: Some(child.into_any_element()),
        priority,
        nested: false,
        portals: None,
    }
}

struct Portal {
    child: Option<AnyElement>,
    priority: usize,
    nested: bool,
    portals: Option<Portals>,
}

impl IntoElement for Portal {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for Portal {
    type RequestLayoutState = ();
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
    ) -> (LayoutId, ()) {
        let parent = scoped::<Portals>(cx);
        self.nested = parent.is_some();
        let namespace: Vec<_> = window.with_global_id("dz-portal".into(), |id, _| {
            id.iter().take(id.len() - 1).cloned().collect()
        });
        let portals = parent.unwrap_or_else(|| Portals {
            layers: Rc::default(),
            depth: 0,
            base_namespace_len: namespace.len(),
        });
        let mut child = provide(
            Portals {
                layers: portals.layers.clone(),
                depth: portals.depth + 1,
                base_namespace_len: portals.base_namespace_len,
            },
            self.child.take().unwrap(),
        )
        .into_any_element();
        // Preserve the global layer order (e.g. receipts above dialogs), and
        // place deeper portals after their own parent within the same layer.
        self.priority = self.priority * 1024 + portals.depth.min(1023);
        let layout = if self.nested {
            // Request its layout only after returning to the outer portal's
            // namespace. GPUI requires layout and prepaint to use the same
            // element namespace, including every stateful descendant.
            let child =
                crate::theme::themed(crate::Theme::entry(cx), super::scope::preserve(child, cx))
                    .into_any_element();
            portals.layers.borrow_mut().push(Layer {
                child,
                priority: self.priority,
                namespace: namespace[portals.base_namespace_len..].to_vec(),
            });
            let mut style = Style::default();
            style.position = Position::Absolute;
            window.request_layout(style, [], cx)
        } else {
            let layout = child.request_layout(window, cx);
            self.child = Some(child);
            layout
        };
        self.portals = Some(portals);
        (layout, ())
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
        if self.nested {
            return;
        }
        let offset = window.element_offset();
        window.defer_draw(self.child.take().unwrap(), offset, self.priority);
        let portals = self.portals.as_ref().unwrap();
        loop {
            let next = {
                let mut layers = portals.layers.borrow_mut();
                if layers.is_empty() {
                    None
                } else {
                    Some(layers.remove(0))
                }
            };
            let Some(layer) = next else {
                break;
            };
            let namespace = layer.namespace.clone();
            let mut layer = Some(layer);
            in_namespace(&namespace, window, &mut |window| {
                let mut layer = layer.take().unwrap();
                layer.child.layout_as_root(
                    window.viewport_size().map(AvailableSpace::Definite),
                    window,
                    cx,
                );
                window.defer_draw(layer.child, offset, layer.priority);
            });
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        _: &mut Window,
        _: &mut App,
    ) {
    }
}

fn in_namespace<R>(
    path: &[ElementId],
    window: &mut Window,
    f: &mut impl FnMut(&mut Window) -> R,
) -> R {
    if let Some((first, rest)) = path.split_first() {
        window.with_element_namespace(first.clone(), |window| in_namespace(rest, window, f))
    } else {
        f(window)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActiveTheme, Theme, themed};
    use gpui::{
        Context, InteractiveElement, ParentElement, Render, RenderOnce, Styled, TestAppContext,
        div, px,
    };

    struct Probe(u32);
    impl IntoElement for Probe {
        type Element = gpui::Component<Self>;
        fn into_element(self) -> Self::Element {
            gpui::Component::new(self)
        }
    }
    impl RenderOnce for Probe {
        fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
            let stored = window.use_keyed_state("probe", cx, |_, _| self.0);
            assert_eq!(
                *stored.read(cx),
                self.0,
                "sibling portal namespaces stay distinct"
            );
            assert_eq!(
                scoped::<u32>(cx),
                Some(self.0),
                "detached layout retains providers"
            );
            assert_eq!(
                cx.theme().color_surface,
                Theme::named("studio-dark").unwrap().tokens.color_surface,
                "detached layout retains its subtree theme"
            );
            div().size(px(8.))
        }
    }

    struct Stage;
    impl Render for Stage {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            themed(
                Theme::named("studio-dark").unwrap(),
                portal(
                    div()
                        .size(px(100.))
                        .children([("first", 11u32), ("second", 22u32)].map(|(id, value)| {
                            div().id(id).child(provide(value, portal(Probe(value), 1)))
                        })),
                    1,
                ),
            )
        }
    }

    #[gpui::test]
    fn nested_portals_keep_theme_providers_and_sibling_state(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (_, cx) = cx.add_window_view(|_, _| Stage);
        cx.run_until_parked();
        cx.update(|window, _| window.refresh());
        cx.run_until_parked();
    }
}
