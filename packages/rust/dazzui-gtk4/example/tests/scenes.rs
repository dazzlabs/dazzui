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
                window.set_child(Some(&widget));
                window.present();
                settle();
                assert!(widget.width() > 0 && widget.height() > 0, "{}", scene.id);
                let mut seen = std::collections::HashSet::new();
                for (at, specimen) in specimens {
                    assert!(
                        specimen.compute_bounds(&widget).unwrap().width() > 0.0,
                        "{} / {at} has no width",
                        scene.id
                    );
                    assert!(seen.insert(at), "Duplicate address in {}", scene.id);
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
        .downcast::<gtk::Label>()
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
        .downcast::<gtk::Label>()
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
    window.destroy();
    println!("{count} generated scenes rendered in light and dark themes");
}
