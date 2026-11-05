use crate::Theme;
use nsh_core::{
    element::{ImageSrc, Svg, svg},
    prelude::*,
};
#[derive(Clone)]
pub(crate) struct IconSize(pub std::rc::Rc<dyn Fn() -> f32>);
/// A Fluent icon, using the exact SVG path imported by the React package.
/// Unknown names fail loudly; applications may also pass their own SVG to NSH.
pub fn icon(name: &str) -> Svg {
    let markup = crate::generated::GLYPHS
        .iter()
        .find(|(key, _)| *key == name)
        .unwrap_or_else(|| panic!("unknown Fluent icon: {name}"))
        .1;
    let theme = Theme::current();
    let size = use_context::<IconSize>();
    svg(ImageSrc::svg_markup(markup))
        .size(move || {
            px(size
                .as_ref()
                .map_or_else(|| theme.tokens().icon_medium, |size| size.0()))
        })
        .flex_shrink_0()
}
