//! Several disclosures that know about each other.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, KeyDownEvent, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::components::collapsible::{disclosure_panel, disclosure_row, kept_focus, part_id};
use crate::theme::ActiveTheme;

/// The open sections after a change, as a controlled `value` hears it.
pub type OnValueChange = Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App)>;

/// Several disclosures that know about each other: the arrow keys move
/// between the headers, Home and End reach the ends, and by default opening
/// one shuts the last — `multiple` lets a reader hold two open at once.
#[derive(IntoElement)]
pub struct Accordion {
    id: ElementId,
    items: Vec<AccordionItem>,
    value: Option<Vec<SharedString>>,
    default_value: Vec<SharedString>,
    on_value_change: Option<OnValueChange>,
    multiple: bool,
    divided: bool,
}

impl Accordion {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Accordion {
            id: id.into(),
            items: Vec::new(),
            value: None,
            default_value: Vec::new(),
            on_value_change: None,
            multiple: false,
            divided: false,
        }
    }

    pub fn item(mut self, item: AccordionItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = AccordionItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Controlled: the values of the open sections.
    pub fn value(mut self, value: Vec<SharedString>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: Vec<SharedString>) -> Self {
        self.default_value = value;
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// A hairline between the sections, for a run that reads as one list.
    pub fn divided(mut self, divided: bool) -> Self {
        self.divided = divided;
        self
    }
}

/// One section: the header that opens it, and everything it discloses.
pub struct AccordionItem {
    value: SharedString,
    title: AnyElement,
    icon: Option<AnyElement>,
    meta: Option<AnyElement>,
    disabled: bool,
    children: Vec<AnyElement>,
}

impl AccordionItem {
    pub fn new(value: impl Into<SharedString>, title: impl IntoElement) -> Self {
        AccordionItem {
            value: value.into(),
            title: title.into_any_element(),
            icon: None,
            meta: None,
            disabled: false,
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
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

impl ParentElement for AccordionItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Accordion {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let default_value = self.default_value.clone();
        let own = window.use_keyed_state(part_id(&self.id, "value"), cx, |_, _| default_value);
        let open: Vec<SharedString> = self.value.clone().unwrap_or_else(|| own.read(cx).clone());
        let controlled = self.value.is_some();
        let multiple = self.multiple;

        // Every header's focus, so the arrows can walk between them.
        let focuses: Rc<Vec<(gpui::FocusHandle, bool)>> = Rc::new(
            self.items
                .iter()
                .map(|item| {
                    (
                        kept_focus(
                            part_id(&self.id, format!("focus-{}", item.value)),
                            window,
                            cx,
                        ),
                        item.disabled,
                    )
                })
                .collect(),
        );

        let mut root = div().flex().flex_col().w_full();
        for (index, item) in self.items.into_iter().enumerate() {
            let is_open = open.contains(&item.value);
            let focus = focuses[index].0.clone();
            let mut row = disclosure_row(
                part_id(&self.id, format!("trigger-{}", item.value)),
                &focus,
                is_open,
                item.disabled,
                item.icon,
                Some(item.title),
                item.meta,
                window,
                cx,
            );
            if !item.disabled {
                let value = item.value.clone();
                let open = open.clone();
                let own = own.clone();
                let on_change = self.on_value_change.clone();
                row = row.on_click(move |_, window, cx| {
                    let next: Vec<SharedString> = if open.contains(&value) {
                        open.iter().filter(|v| **v != value).cloned().collect()
                    } else if multiple {
                        open.iter().cloned().chain([value.clone()]).collect()
                    } else {
                        vec![value.clone()]
                    };
                    if !controlled {
                        let next = next.clone();
                        own.update(cx, |own, cx| {
                            *own = next;
                            cx.notify();
                        });
                    }
                    if let Some(on_change) = &on_change {
                        on_change(next, window, cx);
                    }
                    window.refresh();
                });
                let focuses = focuses.clone();
                row = row.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let enabled: Vec<usize> = (0..focuses.len())
                        .filter(|index| !focuses[*index].1)
                        .collect();
                    let here = enabled.iter().position(|i| *i == index).unwrap_or(0);
                    let target = match event.keystroke.key.as_str() {
                        "down" => enabled.get((here + 1) % enabled.len()),
                        "up" => enabled.get((here + enabled.len() - 1) % enabled.len()),
                        "home" => enabled.first(),
                        "end" => enabled.last(),
                        _ => return,
                    };
                    if let Some(target) = target {
                        window.focus(&focuses[*target].0);
                    }
                    cx.stop_propagation();
                });
            }

            let section = div()
                .flex()
                .flex_col()
                .w_full()
                .when(self.divided && index > 0, |this| {
                    this.border_t(px(t.stroke_hairline))
                        .border_color(t.color_border.hsla())
                })
                .child(row)
                .when(is_open, |this| {
                    this.child(disclosure_panel(item.children, cx))
                });
            root = root.child(section);
        }
        root
    }
}
