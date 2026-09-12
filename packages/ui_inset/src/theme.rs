use crate::generated::theme_variables::*;
use inset_foundation::App;
use inset_widgets::{
    BuildContext, DefaultTextStyle, InheritedWidget, IntoWidget, StatelessWidget, WidgetRef,
};

pub const THEMES: [(&str, &ThemeVariables); 12] = [
    ("studio-light", &STUDIO_LIGHT),
    ("studio-dark", &STUDIO_DARK),
    ("bright-light", &BRIGHT_LIGHT),
    ("bright-dark", &BRIGHT_DARK),
    ("frost-light", &FROST_LIGHT),
    ("frost-dark", &FROST_DARK),
    ("graphite-light", &GRAPHITE_LIGHT),
    ("graphite-dark", &GRAPHITE_DARK),
    ("ember-light", &EMBER_LIGHT),
    ("ember-dark", &EMBER_DARK),
    ("nocturne-light", &NOCTURNE_LIGHT),
    ("nocturne-dark", &NOCTURNE_DARK),
];
#[derive(Debug, Clone)]
pub struct Theme {
    pub vars: std::rc::Rc<ThemeVariables>,
    pub child: WidgetRef,
}
impl Theme {
    pub fn new<K>(vars: &ThemeVariables, child: impl IntoWidget<K>) -> Self {
        Self {
            vars: std::rc::Rc::new(*vars),
            child: child.into_widget(),
        }
    }
    pub fn of(app: &mut App, context: BuildContext) -> std::rc::Rc<ThemeVariables> {
        context
            .depend_on_inherited_widget_of_exact_type::<ThemeScope>(app)
            .map(|s| s.vars.clone())
            .unwrap_or_else(|| std::rc::Rc::new(STUDIO_LIGHT))
    }
}
impl StatelessWidget for Theme {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let style = DefaultTextStyle::of(app, context).style.merge(Some(
            &self
                .vars
                .body_medium
                .text_style()
                .color(self.vars.color_content),
        ));
        ThemeScope {
            vars: self.vars.clone(),
            child: DefaultTextStyle::new(style, self.child.clone()).into_widget(),
        }
        .into_widget()
    }
}
#[derive(Debug)]
struct ThemeScope {
    vars: std::rc::Rc<ThemeVariables>,
    child: WidgetRef,
}
impl InheritedWidget for ThemeScope {
    fn child(&self) -> &WidgetRef {
        &self.child
    }
    fn update_should_notify(&self, old: &Self) -> bool {
        self.vars != old.vars
    }
}
