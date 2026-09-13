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
            support::build(&cell, SizedBox::shrink().into_widget());
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

#[test]
fn every_generated_story_renders_and_unmounts_in_every_theme() {
    use dazzui_inset_playground::generated_stories;
    let ids: std::collections::BTreeSet<_> = generated_stories::STORIES
        .iter()
        .map(|(id, _)| *id)
        .collect();
    assert_eq!(ids.len(), generated_stories::STORIES.len());
    for (_, vars) in THEMES {
        for (id, _) in generated_stories::STORIES {
            eprintln!("generated {id}");
            let cell = support::test_cell();
            let key = std::rc::Rc::new(inset::GlobalKey::new());
            let specimen = inset::KeyedSubtree::new(
                generated_stories::build(id).expect("generated story has a builder"),
            )
            .key(key.clone());
            support::build(
                &cell,
                host(Theme::new(vars, Center::new().child(specimen))).into_widget(),
            );
            support::pump(&mut cell.borrow_mut(), Duration::from_millis(160));
            support::build(&cell, SizedBox::shrink().into_widget());
            assert!(
                key.current_context(&mut cell.borrow_mut()).is_none(),
                "{id} unmounted"
            );
        }
    }
}

#[test]
fn generated_overviews_replace_legacy_catalog_entries() {
    use dazzui_inset_playground::{generated_stories, legacy_pages};
    let mut ids = std::collections::BTreeSet::new();
    for (id, _) in generated_stories::STORIES {
        assert!(ids.insert(id.to_string()));
    }
    for page in legacy_pages() {
        assert!(
            ids.insert(dazzui_inset_playground::storybook::story_id(
                page.name, "Overview"
            ))
        );
    }
    for page in PAGES {
        assert!(
            ids.contains(&dazzui_inset_playground::storybook::story_id(
                page.name, "Overview"
            ))
        );
    }
    assert_eq!(
        ids.len(),
        generated_stories::STORIES.len() + legacy_pages().len()
    );
}

#[test]
fn generated_grid_adapts_tracks_and_stretches_each_row() {
    use inset::{GlobalKey, Offset};
    use std::rc::Rc;
    for (width, columns) in [(380.0, 3usize), (250.0, 2usize)] {
        let cell = support::test_cell();
        let keys: Vec<_> = (0..5).map(|_| Rc::new(GlobalKey::new())).collect();
        let children = keys
            .iter()
            .enumerate()
            .map(|(i, key)| {
                SizedBox::new()
                    .key(key.clone())
                    .height(20.0 + i as f64 * 10.0)
                    .into_widget()
            })
            .collect();
        support::build(
            &cell,
            host(Theme::new(
                THEMES[0].1,
                Center::new().child(
                    SizedBox::new()
                        .width(width)
                        .child(stage::grid(100.0, children)),
                ),
            ))
            .into_widget(),
        );
        let mut app = cell.borrow_mut();
        let boxes: Vec<_> = keys
            .iter()
            .map(|key| {
                let render = key
                    .current_context(&mut app)
                    .unwrap()
                    .find_render_object(&app)
                    .unwrap()
                    .as_box()
                    .unwrap();
                (
                    render.size(&app),
                    render.local_to_global(&app, Offset::ZERO, None),
                )
            })
            .collect();
        let gap = THEMES[0].1.spacing_4;
        let track = (width - gap * (columns - 1) as f64) / columns as f64;
        for (i, (size, origin)) in boxes.iter().enumerate() {
            assert!((size.width() - track).abs() < 0.001);
            assert!(
                (origin.dx() - boxes[0].1.dx() - (i % columns) as f64 * (track + gap)).abs()
                    < 0.001
            );
            let end = ((i / columns + 1) * columns).min(boxes.len()) - 1;
            assert_eq!(size.height(), 20.0 + end as f64 * 10.0);
            if i >= columns {
                assert!(origin.dy() > boxes[i - columns].1.dy());
            }
        }
    }
}

#[test]
fn action_bar_distributes_native_children_at_the_declared_alignment() {
    use dazzui_inset::ActionBar;
    use inset::{GlobalKey, MainAxisAlignment, Offset};
    use std::rc::Rc;
    let gap = THEMES[0].1.control_small_gap;
    for (align, offsets) in [
        (MainAxisAlignment::Start, [0.0, 20.0 + gap]),
        (MainAxisAlignment::End, [250.0 - gap, 270.0]),
        (MainAxisAlignment::SpaceBetween, [0.0, 270.0]),
    ] {
        let cell = support::test_cell();
        let frame = Rc::new(GlobalKey::new());
        let keys: Vec<_> = (0..2).map(|_| Rc::new(GlobalKey::new())).collect();
        let children = keys
            .iter()
            .enumerate()
            .map(|(i, key)| {
                SizedBox::new()
                    .key(key.clone())
                    .width(20.0 + i as f64 * 10.0)
                    .height(20.0)
                    .into_widget()
            })
            .collect();
        support::build(
            &cell,
            host(Theme::new(
                THEMES[0].1,
                Center::new().child(
                    SizedBox::new()
                        .key(frame.clone())
                        .width(300.0)
                        .child(ActionBar::new(children).align(align)),
                ),
            ))
            .into_widget(),
        );
        let mut app = cell.borrow_mut();
        let parent = frame
            .current_context(&mut app)
            .unwrap()
            .find_render_object(&app)
            .unwrap();
        for (key, expected) in keys.iter().zip(offsets) {
            let child = key
                .current_context(&mut app)
                .unwrap()
                .find_render_object(&app)
                .unwrap()
                .as_box()
                .unwrap();
            let offset = child.local_to_global(&app, Offset::ZERO, Some(parent));
            assert!(
                (offset.dx() - expected).abs() < 0.001,
                "{align:?}: {offset:?}"
            );
        }
    }
}

#[test]
fn table_header_cells_share_tracks_and_honor_trailing_alignment() {
    use dazzui_inset::{TableCell, TableHead};
    use inset::{GlobalKey, Offset};
    use std::rc::Rc;
    let cell = support::test_cell();
    let frame = Rc::new(GlobalKey::new());
    let keys: Vec<_> = (0..2).map(|_| Rc::new(GlobalKey::new())).collect();
    let children = keys
        .iter()
        .enumerate()
        .map(|(i, key)| {
            TableCell::new(SizedBox::new().key(key.clone()).width(10.0).height(20.0))
                .head(true)
                .end(i == 1)
                .into_widget()
        })
        .collect();
    support::build(
        &cell,
        host(Theme::new(
            THEMES[0].1,
            Center::new().child(
                SizedBox::new()
                    .key(frame.clone())
                    .width(300.0)
                    .child(TableHead::new(children)),
            ),
        ))
        .into_widget(),
    );
    let mut app = cell.borrow_mut();
    let parent = frame
        .current_context(&mut app)
        .unwrap()
        .find_render_object(&app)
        .unwrap();
    for (key, expected) in keys
        .iter()
        .zip([THEMES[0].1.spacing_5, 290.0 - THEMES[0].1.spacing_5])
    {
        let child = key
            .current_context(&mut app)
            .unwrap()
            .find_render_object(&app)
            .unwrap()
            .as_box()
            .unwrap();
        let offset = child.local_to_global(&app, Offset::ZERO, Some(parent));
        assert!((offset.dx() - expected).abs() < 0.001);
        assert_eq!(offset.dy(), THEMES[0].1.spacing_2);
    }
}

#[test]
fn specimen_keeps_label_track_and_gives_the_stage_remaining_width() {
    use inset::{GlobalKey, Offset};
    use std::rc::Rc;
    let v = THEMES[0].1;
    for width in [400.0, 250.0] {
        let cell = support::test_cell();
        let frame = Rc::new(GlobalKey::new());
        let child = Rc::new(GlobalKey::new());
        support::build(
            &cell,
            host(Theme::new(
                v,
                Center::new().child(
                    SizedBox::new()
                        .key(frame.clone())
                        .width(width)
                        .child(stage::specimen(
                            "default",
                            SizedBox::new()
                                .key(child.clone())
                                .height(20.0)
                                .into_widget(),
                        )),
                ),
            ))
            .into_widget(),
        );
        let mut app = cell.borrow_mut();
        let parent = frame
            .current_context(&mut app)
            .unwrap()
            .find_render_object(&app)
            .unwrap();
        let stage = child
            .current_context(&mut app)
            .unwrap()
            .find_render_object(&app)
            .unwrap()
            .as_box()
            .unwrap();
        let left = v.spacing_20 + v.spacing_4 * 2.0;
        assert_eq!(
            stage.local_to_global(&app, Offset::ZERO, Some(parent)).dx(),
            left
        );
        assert_eq!(stage.size(&app).width(), width - left);
    }
}

#[test]
fn vertical_action_separator_stretches_to_the_controls() {
    use dazzui_inset::{ActionBar, Divider};
    use inset::{GlobalKey, IntrinsicHeight, KeyedSubtree};
    use std::rc::Rc;
    let cell = support::test_cell();
    let key = Rc::new(GlobalKey::new());
    let children = vec![
        SizedBox::new().width(20.0).height(28.0).into_widget(),
        KeyedSubtree::new(Divider {
            vertical: true,
            muted: false,
        })
        .key(key.clone())
        .into_widget(),
        SizedBox::new().width(20.0).height(20.0).into_widget(),
    ];
    support::build(
        &cell,
        host(Theme::new(
            THEMES[0].1,
            Center::new().child(
                SizedBox::new()
                    .width(300.0)
                    .child(IntrinsicHeight::new().child(ActionBar::new(children).stretch(true))),
            ),
        ))
        .into_widget(),
    );
    let mut app = cell.borrow_mut();
    let divider = key
        .current_context(&mut app)
        .unwrap()
        .find_render_object(&app)
        .unwrap()
        .as_box()
        .unwrap();
    assert_eq!(divider.size(&app).height(), 28.0);
    assert_eq!(divider.size(&app).width(), THEMES[0].1.stroke_hairline);
}

#[test]
fn callout_sizes_preserve_icon_and_trailing_action_slots() {
    use dazzui_inset::{Callout, ControlSize};
    use inset::{GlobalKey, Offset};
    use std::rc::Rc;
    let v = THEMES[0].1;
    for (size, y, x) in [
        (ControlSize::Small, v.spacing_2, v.spacing_3),
        (ControlSize::Medium, v.spacing_3, v.spacing_3_5),
        (ControlSize::Large, v.spacing_4, v.spacing_4),
    ] {
        let cell = support::test_cell();
        let frame = Rc::new(GlobalKey::new());
        let icon = Rc::new(GlobalKey::new());
        let action = Rc::new(GlobalKey::new());
        let mut callout = Callout::new("Status", "");
        callout.size = size;
        callout.icon = Some(
            SizedBox::new()
                .key(icon.clone())
                .width(16.0)
                .height(16.0)
                .into_widget(),
        );
        callout.actions = vec![
            SizedBox::new()
                .key(action.clone())
                .width(24.0)
                .height(20.0)
                .into_widget(),
        ];
        support::build(
            &cell,
            host(Theme::new(
                v,
                Center::new().child(
                    SizedBox::new()
                        .key(frame.clone())
                        .width(300.0)
                        .child(callout),
                ),
            ))
            .into_widget(),
        );
        let mut app = cell.borrow_mut();
        let parent = frame
            .current_context(&mut app)
            .unwrap()
            .find_render_object(&app)
            .unwrap();
        let left = icon
            .current_context(&mut app)
            .unwrap()
            .find_render_object(&app)
            .unwrap()
            .as_box()
            .unwrap();
        let right = action
            .current_context(&mut app)
            .unwrap()
            .find_render_object(&app)
            .unwrap()
            .as_box()
            .unwrap();
        assert_eq!(
            left.local_to_global(&app, Offset::ZERO, Some(parent)).dx(),
            x + v.stroke_hairline
        );
        assert_eq!(
            right.local_to_global(&app, Offset::ZERO, Some(parent)).dx() + right.size(&app).width(),
            300.0 - x - v.stroke_hairline
        );
        assert_eq!(
            parent.as_box().unwrap().size(&app).height(),
            20.0 + 2.0 * (y + v.stroke_hairline)
        );
    }
}

#[test]
fn active_steps_use_the_declared_tint_without_retinting_finished_steps() {
    use dazzui_inset::{ControlState, ControlVariant, Spinner, Step, StepStatus, Tint};
    use inset::{App, BuildContext, GlobalKey, KeyedSubtree, Text, WidgetRef};
    use std::rc::Rc;
    fn widgets(app: &App, element: BuildContext, found: &mut Vec<WidgetRef>) {
        found.push(element.widget(app).clone());
        element.visit_child_elements(app, &mut |child| widgets(app, child, found));
    }
    for (_, theme) in THEMES {
        for tint in Tint::ALL {
            for status in [StepStatus::Idle, StepStatus::Active, StepStatus::Done] {
                let cell = support::test_cell();
                let key = Rc::new(GlobalKey::new());
                let mut step = Step::new("Stage", status);
                step.meta = Some("detail".into());
                step.tint = tint;
                support::build(
                    &cell,
                    host(Theme::new(
                        theme,
                        Center::new().child(
                            SizedBox::new()
                                .width(300.0)
                                .child(KeyedSubtree::new(step).key(key.clone())),
                        ),
                    ))
                    .into_widget(),
                );
                let mut app = cell.borrow_mut();
                let mut found = vec![];
                let context = key.current_context(&mut app).unwrap();
                widgets(&app, context, &mut found);
                let spinner = found
                    .iter()
                    .find_map(|w| w.as_any().downcast_ref::<Spinner>());
                assert_eq!(
                    spinner.map(|s| s.tint),
                    if status == StepStatus::Active {
                        Some(tint)
                    } else {
                        None
                    }
                );
                let meta = found
                    .iter()
                    .filter_map(|w| w.as_any().downcast_ref::<Text>())
                    .find(|t| t.data.as_deref() == Some("detail"))
                    .unwrap();
                let expected = if status == StepStatus::Active {
                    theme
                        .control_colors(ControlVariant::Plain, tint, ControlState::Normal)
                        .content
                } else {
                    theme.color_content_faint
                };
                assert_eq!(
                    meta.style.as_ref().unwrap().color.as_ref().unwrap().color(),
                    expected
                );
            }
        }
    }
}
