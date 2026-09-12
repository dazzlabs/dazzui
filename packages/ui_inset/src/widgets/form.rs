use crate::{ControlState, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_rendering::{CrossAxisAlignment, MainAxisSize};
use inset_widgets::{
    BuildContext, Column, InheritedWidget, IntoWidget, StatelessWidget, Text, WidgetRef,
};
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
pub struct Form {
    pub errors: BTreeMap<String, String>,
    pub children: Vec<WidgetRef>,
}
impl Form {
    pub fn new(children: Vec<WidgetRef>) -> Self {
        Self {
            errors: BTreeMap::new(),
            children,
        }
    }
}
impl StatelessWidget for Form {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        FormErrors {
            errors: self.errors.clone(),
            child: Column::new()
                .main_axis_size(MainAxisSize::Min)
                .cross_axis_alignment(CrossAxisAlignment::Stretch)
                .spacing(v.spacing_4)
                .children(self.children.clone())
                .into_widget(),
        }
        .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct FormErrors {
    pub errors: BTreeMap<String, String>,
    pub child: WidgetRef,
}
impl FormErrors {
    pub fn of(app: &mut App, context: BuildContext, name: &str) -> Option<String> {
        context
            .depend_on_inherited_widget_of_exact_type::<Self>(app)
            .and_then(|v| v.errors.get(name).cloned())
    }
}
impl InheritedWidget for FormErrors {
    fn child(&self) -> &WidgetRef {
        &self.child
    }
    fn update_should_notify(&self, old: &Self) -> bool {
        self.errors != old.errors
    }
}
#[derive(Debug, Clone)]
pub struct FormField {
    pub label: Option<String>,
    pub hint: Option<String>,
    pub name: Option<String>,
    pub invalid: bool,
    pub child: WidgetRef,
}
impl FormField {
    pub fn new<K>(label: impl Into<String>, child: impl IntoWidget<K>) -> Self {
        Self {
            label: Some(label.into()),
            hint: None,
            name: None,
            invalid: false,
            child: child.into_widget(),
        }
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}
impl StatelessWidget for FormField {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let error = self
            .name
            .as_ref()
            .and_then(|n| FormErrors::of(app, context, n));
        let invalid = self.invalid || error.is_some();
        let hint = error.or(self.hint.clone());
        let ink = if invalid {
            v.control_colors(ControlVariant::Plain, Tint::Danger, ControlState::Normal)
                .content
        } else {
            v.color_content_subtle
        };
        let mut children = vec![];
        if let Some(label) = &self.label {
            children.push(
                Text::new(label)
                    .style(v.label_small.text_style().color(ink))
                    .into_widget(),
            );
        }
        children.push(self.child.clone());
        if let Some(hint) = hint {
            children.push(
                Text::new(hint)
                    .style(v.label_quiet.text_style().height(1.6).color(ink))
                    .into_widget(),
            );
        }
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .spacing(v.spacing_1_5)
            .children(children)
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Fieldset {
    pub legend: Option<String>,
    pub children: Vec<WidgetRef>,
}
impl Fieldset {
    pub fn new(legend: impl Into<String>, children: Vec<WidgetRef>) -> Self {
        Self {
            legend: Some(legend.into()),
            children,
        }
    }
}
impl StatelessWidget for Fieldset {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut children = vec![];
        if let Some(legend) = &self.legend {
            children.push(
                Text::new(legend)
                    .style(v.caption_small.text_style().color(v.color_content_subtle))
                    .into_widget(),
            );
        }
        children.extend(self.children.clone());
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .spacing(v.spacing_3)
            .children(children)
            .into_widget()
    }
}
