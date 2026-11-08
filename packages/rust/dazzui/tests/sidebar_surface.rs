//! Compare the column's reserved space with its actual submitted surfaces.

#[allow(dead_code)]
mod support;

use dazzui::{
    ActiveTheme, Sidebar, SidebarCollapsible, SidebarInset, SidebarProvider, SidebarSide,
    SidebarVariant, Theme,
};
use gpui::{
    AnyElement, Bounds, Context, Hsla, IntoElement, ObservedPaint, ParentElement, Pixels, Render,
    Styled, TestAppContext, Window, div, point, prelude::*, px, size,
};
use std::{cell::RefCell, rc::Rc};
use support::paint::PaintProbe;

struct Harness {
    variant: SidebarVariant,
    collapsible: SidebarCollapsible,
    side: SidebarSide,
    open: bool,
    collapsed: Option<bool>,
    paints: [Rc<RefCell<Vec<ObservedPaint>>>; 3],
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let probe = |index: usize, child: AnyElement| PaintProbe {
            child,
            paints: self.paints[index].clone(),
        };
        let sidebar = Sidebar::new()
            .width(172.)
            .side(self.side)
            .variant(self.variant)
            .collapsible(self.collapsible)
            .when_some(self.collapsed, |sidebar, collapsed| {
                sidebar.collapsed(collapsed)
            })
            .child("Column");
        div().size_full().p(px(24.)).child(
            div().flex().w(px(420.)).h(px(180.)).child(probe(
                0,
                SidebarProvider::new("provider")
                    .open(self.open)
                    .child(probe(1, sidebar.into_any_element()))
                    .child(probe(
                        2,
                        SidebarInset::new().child("Content").into_any_element(),
                    ))
                    .into_any_element(),
            )),
        )
    }
}

fn bounds(x: f32, y: f32, width: f32, height: f32) -> Bounds<Pixels> {
    Bounds::new(point(px(x), px(y)), size(px(width), px(height)))
}

fn fill(paint: &ObservedPaint) -> Option<Hsla> {
    paint
        .surface
        .as_ref()?
        .style
        .background
        .as_ref()?
        .color()?
        .solid_color()
}

#[gpui::test]
fn sidebar_surfaces_and_independent_collapse_match_both_react_edges(cx: &mut TestAppContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("bright-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        variant: SidebarVariant::Sidebar,
        collapsible: SidebarCollapsible::Icon,
        side: SidebarSide::Left,
        open: true,
        collapsed: None,
        paints: std::array::from_fn(|_| Rc::new(RefCell::new(Vec::new()))),
    });
    for theme in ["bright-light", "studio-dark"] {
        cx.update(|_, cx| Theme::set(theme, cx));
        for variant in [
            SidebarVariant::Sidebar,
            SidebarVariant::Floating,
            SidebarVariant::Inset,
        ] {
            for collapsible in [
                SidebarCollapsible::None,
                SidebarCollapsible::Icon,
                SidebarCollapsible::Offcanvas,
            ] {
                for side in [SidebarSide::Left, SidebarSide::Right] {
                    for open in [true, false] {
                        for collapsed in [None, Some(true), Some(false)] {
                            view.update(cx, |view, cx| {
                                view.variant = variant;
                                view.collapsible = collapsible;
                                view.side = side;
                                view.open = open;
                                view.collapsed = collapsed;
                                view.paints
                                    .iter()
                                    .for_each(|paint| paint.borrow_mut().clear());
                                cx.notify();
                            });
                            cx.run_until_parked();
                            let label = format!(
                                "{theme} {variant:?} {collapsible:?} {side:?} {open} {collapsed:?}"
                            );
                            let paints = view.read_with(cx, |view, _| {
                                view.paints
                                    .each_ref()
                                    .map(|paints| paints.borrow().last().unwrap().clone())
                            });
                            let [provider, sidebar, inset] = &paints;
                            let away = collapsible != SidebarCollapsible::None
                                && collapsed.unwrap_or(!open);
                            let width = if !away {
                                172.
                            } else if collapsible == SidebarCollapsible::Offcanvas {
                                0.
                            } else if variant == SidebarVariant::Sidebar {
                                48.
                            } else {
                                64.
                            };
                            let raised = variant == SidebarVariant::Inset;
                            let lead = if raised && away { 8. } else { 0. };
                            assert_eq!(
                                sidebar.surface.as_ref().unwrap().bounds,
                                bounds(24., 24., width, 180.),
                                "{label}"
                            );
                            let root = &sidebar.surface.as_ref().unwrap().style;
                            assert_eq!(
                                root.padding
                                    .to_pixels(size(px(width).into(), px(180.).into()), px(16.))
                                    .left,
                                px(0.),
                                "{label}"
                            );
                            assert_eq!(
                                root.padding
                                    .to_pixels(size(px(width).into(), px(180.).into()), px(16.))
                                    .top,
                                px(0.),
                                "{label}"
                            );
                            let main = inset.surface.as_ref().unwrap();
                            assert_eq!(
                                main.bounds,
                                bounds(
                                    24. + width + lead,
                                    if raised { 32. } else { 24. },
                                    420. - width - lead - if raised { 8. } else { 0. },
                                    if raised { 164. } else { 180. }
                                ),
                                "{label}"
                            );
                            let (surface, column) = cx.update(|_, cx| {
                                (
                                    cx.theme().color_surface.hsla(),
                                    cx.theme().color_surface_column.hsla(),
                                )
                            });
                            assert_eq!(fill(provider), raised.then_some(column), "{label}");
                            assert_eq!(fill(inset), Some(surface), "{label}");
                            assert_eq!(
                                inset.surfaces.len(),
                                1,
                                "the content owns its surface: {label}"
                            );
                            assert_eq!(
                                main.style.corner_radii.top_left.to_pixels(px(16.)),
                                px(if raised { 12. } else { 0. }),
                                "{label}"
                            );
                            assert_eq!(main.style.box_shadow.is_empty(), !raised, "{label}");
                            if collapsible != SidebarCollapsible::None
                                && (!away || collapsible == SidebarCollapsible::Icon)
                            {
                                let container = &sidebar.surfaces[1];
                                let container_width = width
                                    + if away && variant != SidebarVariant::Sidebar {
                                        2.
                                    } else {
                                        0.
                                    };
                                let x = 24.
                                    - if side == SidebarSide::Right {
                                        container_width - width
                                    } else {
                                        0.
                                    };
                                assert_eq!(
                                    container.bounds,
                                    bounds(x, 24., container_width, 180.),
                                    "{label}"
                                );
                                let padding = container.style.padding.to_pixels(
                                    size(px(container_width).into(), px(180.).into()),
                                    px(16.),
                                );
                                assert_eq!(
                                    padding.left,
                                    px(if variant == SidebarVariant::Sidebar {
                                        0.
                                    } else {
                                        8.
                                    }),
                                    "{label}"
                                );
                                assert_eq!(padding.top, padding.left, "{label}");
                            }
                        }
                    }
                }
            }
        }
    }
}
