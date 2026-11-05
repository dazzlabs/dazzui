use dazzui_gtk4::{gtk, prelude::*, *};
use dazzui_gtk4_example::{conformance::settle, stories, support::scene};
fn main() {
    gtk::init().expect("GTK display required; on Linux run under xvfb-run");
    gtk::Settings::default()
        .unwrap()
        .set_gtk_enable_animations(false);
    let window = gtk::Window::builder().default_width(640).build();
    let mut count = 0;
    for name in ["studio-light", "bright-dark"] {
        Theme::new(name).unwrap().apply(&window);
        for story in stories::all() {
            for scene in story.scenes() {
                if !scene.is_supported() {
                    continue;
                }
                let mut specimens = vec![];
                let widget = scene::render_scene(scene, &mut specimens)
                    .unwrap_or_else(|e| panic!("{}: {e}", scene.id));
                // Use the collector's frame so the previous scene's native
                // window minimum cannot become this scene's layout width.
                let (fixed, frame) = dazzui_gtk4_example::conformance::scene_frame(&widget);
                window.set_child(Some(&fixed));
                window.set_default_size(640, 1);
                window.present();
                settle();
                gtk::prelude::GtkWindowExt::set_focus(&window, None::<&gtk::Widget>);
                settle();
                assert!(
                    widget.width() > 0 && widget.height() > 0,
                    "{}: {}x{}, mapped={}",
                    scene.id,
                    widget.width(),
                    widget.height(),
                    widget.is_mapped()
                );
                let mut seen = std::collections::HashSet::new();
                for (at, specimen) in specimens {
                    assert!(seen.insert(at), "Duplicate address in {}", scene.id);
                    // Closed disclosures and portals still have addresses,
                    // but no initial reading until their native widget maps.
                    if !specimen.is_mapped() {
                        continue;
                    }
                    assert!(
                        specimen.compute_bounds(&frame).unwrap().width() > 0.0,
                        "{} / {at} has no width",
                        scene.id
                    );
                }
                count += 1;
            }
        }
    }
    // A native edit updates another node without replacing the input or
    // throwing away its selection; an external action updates the same input.
    let live: dazzui_gtk4_example::support::story::Scene = serde_json::from_value(serde_json::json!({
        "id":"live", "title":"State", "gaps":[], "state":{"query":"","checked":true},
        "content":{"$el":"Col","children":[
            {"$el":"SearchField","$at":0,"props":{"value":{"$state":"query"},"onValueChange":{"$do":[{"set":"query","to":{"$arg":0}}]}}},
            {"$el":"Text","children":[{"$concat":["Query: ",{"$state":"query"}]}]},
            {"$el":"Switch","$at":1,"props":{"checked":{"$state":"checked"},"onCheckedChange":{"$do":[{"set":"checked","to":{"$arg":0}}]}}},
            {"$el":"Button","$at":2,"props":{"onClick":{"$do":[{"set":"query","to":"Reset"},{"set":"checked","to":false}]}},"children":["Reset"]}
        ]}
    })).unwrap();
    let mut specimens = vec![];
    let root = scene::render_scene(&live, &mut specimens).unwrap();
    window.set_child(Some(&root));
    window.present();
    settle();
    let input = specimens
        .iter()
        .find(|(at, _)| *at == 0)
        .unwrap()
        .1
        .clone()
        .downcast::<gtk::SearchEntry>()
        .unwrap();
    let check = specimens
        .iter()
        .find(|(at, _)| *at == 1)
        .unwrap()
        .1
        .clone()
        .downcast::<gtk::Switch>()
        .unwrap();
    let reset = specimens
        .iter()
        .find(|(at, _)| *at == 2)
        .unwrap()
        .1
        .clone()
        .downcast::<gtk::Button>()
        .unwrap();
    let output = input
        .next_sibling()
        .unwrap()
        .first_child()
        .unwrap()
        .downcast::<TextBox>()
        .unwrap();
    input.set_text("Typed");
    input.select_region(1, 3);
    settle();
    assert_eq!(output.text(), "Query: Typed");
    assert_eq!(input.selection_bounds(), Some((1, 3)));
    assert!(check.is_active());
    reset.emit_clicked();
    settle();
    assert_eq!(input.text(), "Reset");
    assert_eq!(output.text(), "Query: Reset");
    assert!(!check.is_active());
    assert_eq!(
        root.first_child().unwrap(),
        input.clone().upcast::<gtk::Widget>()
    );
    let choices: dazzui_gtk4_example::support::story::Scene = serde_json::from_value(serde_json::json!({
        "id":"choices", "title":"Choices", "gaps":[], "state":{"plan":"one"},
        "content":{"$el":"Row","props":{"align":"stretch","width":"100%"},"children":[
            {"$el":"OptionCard","$at":0,"props":{"title":"One","description":"First plan","selected":{"$eq":[{"$state":"plan"},"one"]},"onClick":{"$do":[{"set":"plan","to":"one"}]}}},
            {"$el":"OptionCard","$at":1,"props":{"title":"Two","description":"Second plan","selected":{"$eq":[{"$state":"plan"},"two"]},"onClick":{"$do":[{"set":"plan","to":"two"}]}}}
        ]}
    })).unwrap();
    let mut specimens = vec![];
    let root = scene::render_scene(&choices, &mut specimens).unwrap();
    let (fixed, frame) = dazzui_gtk4_example::conformance::scene_frame(&root);
    window.set_default_size(1200, 600);
    window.set_child(Some(&fixed));
    window.present();
    settle();
    let one = specimens[0]
        .1
        .clone()
        .downcast::<gtk::CheckButton>()
        .unwrap();
    let two = specimens[1]
        .1
        .clone()
        .downcast::<gtk::CheckButton>()
        .unwrap();
    assert_eq!(
        frame.compute_bounds(&frame).unwrap().width(),
        640.,
        "a larger native window keeps the reference measure"
    );
    assert_eq!(root.width(), 592);
    assert_eq!(one.compute_bounds(&frame).unwrap().width(), 290.);
    assert_eq!(
        one.width(),
        two.width(),
        "option cards share available width equally"
    );
    assert!(one.is_active() && !two.is_active());
    two.emit_by_name::<()>("activate", &[]);
    assert!(!one.is_active() && two.is_active());
    two.emit_by_name::<()>("activate", &[]);
    assert!(
        !one.is_active() && two.is_active(),
        "pressing the selected card preserves its selection"
    );
    let selector: dazzui_gtk4_example::support::story::Scene = serde_json::from_value(serde_json::json!({
        "id":"selector", "title":"Selector", "gaps":[], "state":{"person":"ada"},
        "content":{"$el":"Col","children":[
            {"$el":"Select","$at":0,"props":{"value":{"$state":"person"},"onValueChange":{"$do":[{"set":"person","to":{"$arg":0}}]},"placeholder":"Choose","options":[{"label":"People","options":[{"value":"ada","label":"Ada"},{"value":"grace","label":"Grace"}]}]}},
            {"$el":"Text","children":[{"$state":"person"}]},
            {"$el":"Button","$at":1,"props":{"onClick":{"$do":[{"set":"person","to":"ada"}]}},"children":["Reset"]},
            {"$el":"Select","$at":2,"props":{"value":"ada","readOnly":true,"options":[{"value":"ada","label":"Ada"}]}}
        ]}
    })).unwrap();
    let mut specimens = vec![];
    let root = scene::render_scene(&selector, &mut specimens).unwrap();
    window.set_child(Some(&root));
    window.present();
    settle();
    let select = specimens[0].1.clone().downcast::<gtk::DropDown>().unwrap();
    let reset = specimens[1].1.clone().downcast::<gtk::Button>().unwrap();
    let readonly = specimens[2].1.clone().downcast::<gtk::DropDown>().unwrap();
    let output = select
        .next_sibling()
        .unwrap()
        .first_child()
        .unwrap()
        .downcast::<TextBox>()
        .unwrap();
    assert_eq!(select.selected(), 2);
    select.set_selected(3);
    assert_eq!(output.text(), "grace");
    reset.emit_clicked();
    assert_eq!(select.selected(), 2);
    assert_eq!(output.text(), "ada");
    select.set_selected(1);
    assert_eq!(
        select.selected(),
        2,
        "group heading cannot clear a controlled value"
    );
    assert!(!readonly.first_child().unwrap().is_sensitive());
    assert_eq!(root.first_child().unwrap(), select.upcast::<gtk::Widget>());
    let placeholder = select_options(&[], "Choose a language", Size::Medium);
    let nested = button("Nested label", Variant::Normal, Tint::Primary, Size::Medium);
    let outer = gtk::Box::new(gtk::Orientation::Vertical, 0);
    outer.append(&nested);
    outer.append(&placeholder);
    window.set_child(Some(&outer));
    window.present();
    settle();
    let specimens = vec![nested.clone().upcast::<gtk::Widget>()];
    let text = dazzui_gtk4_example::conformance::first_text(outer.upcast_ref(), &specimens)
        .unwrap()
        .downcast::<gtk::Label>()
        .unwrap();
    assert_eq!(text.text(), "Choose a language");
    assert!(text.has_css_class("dz-select-placeholder"));
    assert_ne!(text.color(), placeholder.color());
    outer.remove(&placeholder);
    settle();
    assert!(
        dazzui_gtk4_example::conformance::first_text(outer.upcast_ref(), &specimens).is_none(),
        "a nested specimen owns its label"
    );

    // A story width is exact, rather than GTK's size-request minimum. Rows
    // wrap unequal native controls while retaining their own widths and gap.
    let row: dazzui_gtk4_example::support::story::Scene =
        serde_json::from_value(serde_json::json!({
            "id":"wrap", "title":"Wrap", "gaps":[],
            "content":{"$el":"Row","props":{"width":"6rem","gap":"3"},"children":[
                {"$el":"Button","$at":0,"children":["One"]},
                {"$el":"Button","$at":1,"children":["Two"]},
                {"$el":"Button","$at":2,"children":["Three"]}
            ]}
        }))
        .unwrap();
    let mut specimens = vec![];
    let root = scene::render_scene(&row, &mut specimens).unwrap();
    let (fixed, frame) = dazzui_gtk4_example::conformance::scene_frame(&root);
    window.set_child(Some(&fixed));
    settle();
    assert_eq!(
        root.width(),
        96,
        "explicit width survives a wider native window"
    );
    let first = specimens[0].1.compute_bounds(&root).unwrap();
    let second = specimens[1].1.compute_bounds(&root).unwrap();
    assert!(second.y() > first.y(), "the second button wraps");
    assert_eq!(
        second.y() - first.y() - first.height(),
        12.,
        "wrapped row gap"
    );
    for (_, child) in &specimens {
        let bounds = child.compute_bounds(&root).unwrap();
        assert!(bounds.x() >= 0. && bounds.x() + bounds.width() <= 96.);
    }
    assert_eq!(frame.compute_bounds(&frame).unwrap().width(), 640.);

    for (story, index, expected) in [
        ("preferences", None, 320.),
        ("preferences", Some(0), 480.),
        ("card", None, 240.),
    ] {
        let story = stories::story_of(story).unwrap();
        let value = index.map(|i| &story.examples[i]).unwrap_or(&story.preview);
        let mut specimens = vec![];
        let root = scene::render_scene(value, &mut specimens).unwrap();
        let (fixed, frame) = dazzui_gtk4_example::conformance::scene_frame(&root);
        window.set_child(Some(&fixed));
        settle();
        assert_eq!(
            specimens[0].1.compute_bounds(&frame).unwrap().width(),
            expected,
            "{}: measure",
            value.id
        );
    }
    let value = &stories::story_of("text-field").unwrap().examples[2];
    let mut specimens = vec![];
    let root = scene::render_scene(value, &mut specimens).unwrap();
    let (fixed, _) = dazzui_gtk4_example::conformance::scene_frame(&root);
    window.set_child(Some(&fixed));
    settle();
    for ((_, widget), height) in specimens.iter().zip([80., 118.]) {
        assert_eq!(
            widget.compute_bounds(&root).unwrap().height(),
            height,
            "textarea rows and floor"
        );
    }
    let field = specimens[1]
        .1
        .clone()
        .downcast::<gtk::ScrolledWindow>()
        .unwrap();
    let input = field.child().and_downcast::<gtk::TextView>().unwrap();
    input
        .buffer()
        .set_text(&"Long content must scroll rather than increase the field height.\n".repeat(80));
    input.buffer().place_cursor(&input.buffer().end_iter());
    input.scroll_mark_onscreen(&input.buffer().get_insert());
    settle();
    assert_eq!(field.compute_bounds(&root).unwrap().height(), 118.);
    assert!(field.vadjustment().upper() > field.vadjustment().page_size());
    assert!(
        input.grab_focus(),
        "the scrolling field retains native editing focus"
    );
    window.destroy();
    println!("{count} generated scenes rendered in light and dark themes");
}
