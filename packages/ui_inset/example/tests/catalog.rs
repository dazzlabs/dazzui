#![feature(arbitrary_self_types)]
#[path = "../../tests/support/mod.rs"]
mod support;
use dazzui_inset::{THEMES, Theme};
use dazzui_inset_playground::{Demo, host, stories::PAGES, storybook::stage};
use inset::{Builder, Center, IntoWidget, SizedBox};
use std::time::Duration;
#[test]
fn every_component_page_renders_and_unmounts_in_every_theme() {
    for (name, vars) in THEMES {
        for page in PAGES {
            eprintln!("{name}/{}", page.name);
            let cell = support::test_cell();
            let demo = Demo::new(&mut cell.borrow_mut());
            let render = page.render;
            let specimen = Builder::new(move |app, _| render(app, &demo));
            support::build(
                &cell,
                host(Theme::new(
                    vars,
                    Center::new().child(
                        SizedBox::new()
                            .width(vars.spacing_20 * 4.0)
                            .child(stage::pane(specimen)),
                    ),
                ))
                .into_widget(),
            );
            for tick in [16, 160, 320] {
                support::pump(&mut cell.borrow_mut(), Duration::from_millis(tick));
            }
            support::build(&cell, host(SizedBox::shrink()).into_widget());
        }
    }
}
#[test]
fn story_ids_are_unique() {
    let ids: std::collections::BTreeSet<_> = PAGES
        .iter()
        .map(|p| dazzui_inset_playground::storybook::story_id(p.name, "Overview"))
        .collect();
    assert_eq!(ids.len(), PAGES.len());
}
