use super::{Popover, PopoverPanel, PopoverSide};
use inset_foundation::App;
use inset_widgets::*;
#[derive(Debug, Clone)]
pub struct Tooltip {
    pub message: String,
    pub child: WidgetRef,
}
impl Tooltip {
    pub fn new<K>(message: impl Into<String>, child: impl IntoWidget<K>) -> Self {
        Self {
            message: message.into(),
            child: child.into_widget(),
        }
    }
}
impl StatelessWidget for Tooltip {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let child = self.child.clone();
        let mut popup = Popover::new(
            move |_, _, _, _| child.clone(),
            PopoverPanel::new(Text::new(&self.message)),
        );
        popup.hover = true;
        popup.dismiss_on_outside = false;
        popup.focus = false;
        popup.side = PopoverSide::Top;
        popup.into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct PreviewCard {
    pub trigger: WidgetRef,
    pub title: String,
    pub description: String,
    pub child: WidgetRef,
}
impl PreviewCard {
    pub fn new<K, L>(trigger: impl IntoWidget<K>, child: impl IntoWidget<L>) -> Self {
        Self {
            trigger: trigger.into_widget(),
            title: String::new(),
            description: String::new(),
            child: child.into_widget(),
        }
    }
}
impl StatelessWidget for PreviewCard {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = crate::Theme::of(app, context);
        let child = self.trigger.clone();
        let mut panel = PopoverPanel::new(self.child.clone());
        panel.title = (!self.title.is_empty()).then(|| self.title.clone());
        panel.description = (!self.description.is_empty()).then(|| self.description.clone());
        panel.width = Some(v.preview_card_width);
        let mut popup = Popover::new(move |_, _, _, _| child.clone(), panel);
        popup.hover = true;
        popup.dismiss_on_outside = false;
        popup.focus = false;
        popup.into_widget()
    }
}
