use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div,
};

use crate::components::Icon;
use crate::components::collapsible::{DisclosureHeader, DisclosurePanel};
use crate::prelude::*;
use crate::{SharedValueListener, ValueListener, hairline, latched};

/// One section of an [`Accordion`]: the header that opens it, and everything
/// it discloses.
pub struct AccordionItem {
    /// Identifies the section. It is what [`Accordion::open`] holds, so it
    /// survives a re-order where an index would not.
    value: SharedString,
    title: SharedString,
    child: Option<AnyElement>,
    icon: Option<Icon>,
    meta: Option<AnyElement>,
    disabled: bool,
}

impl AccordionItem {
    pub fn new(value: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        AccordionItem {
            value: value.into(),
            title: title.into(),
            child: None,
            icon: None,
            meta: None,
            disabled: false,
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn meta(mut self, meta: impl IntoElement) -> Self {
        self.meta = Some(meta.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Several disclosures that know about each other.
///
/// That knowledge is the whole difference from a run of
/// [`Collapsible`](crate::Collapsible)s: by default opening one shuts the
/// last, so a reader is always looking at one section rather than at a page
/// that grew while they were reading it. `multiple` is the deliberate opt-in
/// for the case where they are comparing rather than choosing.
///
/// The rows and the panels are the same drawing a collapsible wears. What is
/// the accordion's own is the run between the sections — the thing a single
/// disclosure has no opinion about.
#[derive(IntoElement)]
pub struct Accordion {
    id: ElementId,
    items: Vec<AccordionItem>,
    /// Drive it from outside. Left `None` the element holds its own: the
    /// index of the one open section, since a self-driven accordion is the
    /// single-open case.
    open: Option<Vec<SharedString>>,
    initially_open: usize,
    multiple: bool,
    /// Draws a hairline between the sections, for a run that has to read as
    /// one list rather than as a stack of separate disclosures.
    divided: bool,
    on_toggle: Option<ValueListener<SharedString>>,
}

impl Accordion {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Accordion {
            id: id.into(),
            items: Vec::new(),
            open: None,
            initially_open: 0,
            multiple: false,
            divided: false,
            on_toggle: None,
        }
    }

    pub fn item(mut self, item: AccordionItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn open(mut self, open: Vec<SharedString>) -> Self {
        self.open = Some(open);
        self
    }

    pub fn initially_open(mut self, index: usize) -> Self {
        self.initially_open = index;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn divided(mut self, divided: bool) -> Self {
        self.divided = divided;
        self
    }

    pub fn on_toggle(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Accordion {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Accordion {
            id,
            items,
            open,
            initially_open,
            multiple,
            divided,
            on_toggle,
        } = self;
        let on_toggle: Option<SharedValueListener<SharedString>> = on_toggle.map(std::rc::Rc::from);
        let rule = hairline(vars);

        // `(touched, index)`: which section this element has opened for
        // itself. A driven accordion ignores it, and `multiple` is only
        // meaningful when the host is holding the set — an element that owns
        // one index is by definition the single-open case.
        // Whether the host is holding the open set at all: read once, so the
        // closure below can move the set itself into the loop.
        let driven = open.is_some();

        latched::<(bool, usize)>(id.clone(), move |cell, _window, _cx| {
            let (touched, latched_index) = cell.get();
            let self_open = if touched {
                latched_index
            } else {
                initially_open
            };

            let mut column = div().flex().flex_col().w_full();

            for (index, item) in items.into_iter().enumerate() {
                let AccordionItem {
                    value,
                    title,
                    child,
                    icon,
                    meta,
                    disabled,
                } = item;
                let is_open = match &open {
                    Some(values) => values.contains(&value),
                    None => index == self_open,
                };

                let section = div()
                    .flex()
                    .flex_col()
                    .w_full()
                    // Undivided, the sections sit on one run with nothing
                    // between them: the rows are already separated by their
                    // own hover wash, and a rule under every one would draw a
                    // table.
                    .when(divided && index > 0, |el| {
                        el.border_t(rule).border_color(vars.color_border)
                    })
                    .child(
                        DisclosureHeader::new(
                            SharedString::from(format!("{id:?}-header-{index}")),
                            title,
                            is_open,
                        )
                        .icon(icon)
                        .meta(meta)
                        .disabled(disabled)
                        .on_toggle({
                            let cell = cell.clone();
                            let on_toggle = on_toggle.clone();
                            let value = value.clone();
                            move |window, cx| {
                                if !multiple || !driven {
                                    cell.set((true, if is_open { usize::MAX } else { index }));
                                }
                                window.refresh();
                                if let Some(listener) = on_toggle.as_ref() {
                                    listener(&value, window, cx);
                                }
                            }
                        }),
                    )
                    .when_some(child.filter(|_| is_open), |el, child| {
                        el.child(DisclosurePanel::new(child))
                    });

                column = column.child(section);
            }

            column.into_any_element()
        })
    }
}
