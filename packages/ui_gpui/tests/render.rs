//! What a design system can be tested for without a screen.
//!
//! Two things: that the token graph resolves the way the design says it does,
//! and that every component lays out and paints. The second is worth more
//! than it sounds — GPUI does the real layout and paint here, so a component
//! that asks for a bad length, loses its element state, or panics in a custom
//! element fails the test rather than the window.

use dazzui::assets::icons;
use dazzui::prelude::*;
use dazzui::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use gpui::{
    AnyElement, App, Context, IntoElement, ParentElement, Render, SharedString, Styled,
    TestAppContext, Window, div, px,
};

/// How many times the gallery has been rendered, so the test can tell a
/// window that drew from one that never got round to it.
static RENDERS: AtomicUsize = AtomicUsize::new(0);

/// The gallery has to be a view rather than a bare element: GPUI's click and
/// hover listeners are registered against the view that is rendering, so an
/// interactive component painted outside one has nothing to belong to.
struct Gallery {
    theme: ThemeName,
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        RENDERS.fetch_add(1, Ordering::SeqCst);
        themed(self.theme, gallery(window, cx))
    }
}

#[gpui::test]
fn every_component_draws(cx: &mut TestAppContext) {
    for theme in ThemeName::ALL {
        let drawn = RENDERS.load(Ordering::SeqCst);
        let (_gallery, cx) = cx.add_window_view(|_, _| Gallery { theme });
        cx.run_until_parked();
        assert!(
            RENDERS.load(Ordering::SeqCst) > drawn,
            "{} never drew",
            theme.label()
        );
    }
}

/// One of everything, which is what the playground's pages add up to.
fn gallery(_window: &mut Window, cx: &mut App) -> AnyElement {
    let field = cx.new(|cx| TextBuffer::new(cx, "Search").with_value("aurora"));
    let vars = cx.theme().vars;

    div()
        .flex()
        .flex_col()
        .gap(vars.spacing_4)
        .child(
            div()
                .flex()
                .gap(vars.spacing_2)
                .children(
                    ControlVariant::ALL.map(|variant| {
                        Button::new(variant.name(), variant.name()).variant(variant)
                    }),
                )
                .children(ControlSize::ALL.map(|size| {
                    Button::new(size.name(), size.name())
                        .size(size)
                        .variant(ControlVariant::Filled)
                        .shortcut("⌘S")
                }))
                .child(
                    Button::new("disabled", "Disabled")
                        .disabled(true)
                        .icon(Icon::new(icons::STAR_16_REGULAR)),
                ),
        )
        .child(
            div()
                .flex()
                .gap(vars.spacing_2)
                .children(Tint::ALL.map(|tint| {
                    IconButton::new(tint.name(), tint.name(), Icon::new(icons::PIN_16_REGULAR))
                        .tint(tint)
                }))
                .children(
                    BadgeVariant::ALL
                        .map(|variant| Badge::new(variant_name(variant)).variant(variant)),
                )
                .child(KeyCap::new("⌘K").variant(KeyCapVariant::Key))
                .child(SectionLabel::new("Section")),
        )
        .child(div().flex().gap(vars.spacing_2).children(
            CardVariant::ALL.map(|variant| Card::new().variant(variant).child("A surface.")),
        ))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(vars.spacing_2)
                .child(Checkbox::new("check", "Checked", true).note("A note"))
                .child(
                    RadioGroup::new("plan", "team")
                        .option("solo", "Solo")
                        .option("team", "Team")
                        .disabled_option("enterprise", "Enterprise"),
                )
                .child(Switch::new("switch", true))
                .child(
                    SegmentedControl::new("mode", "code")
                        .item("design", "Design")
                        .item("code", "Code"),
                )
                .child(
                    PillTabs::new("tabs", "all")
                        .counted_item("all", "All", 24)
                        .item("open", "Open"),
                )
                .child(
                    OptionCard::new("auto", "Automatic")
                        .description("Follow the system appearance.")
                        .selected(true),
                ),
        )
        .child(
            FormField::new("Workspace")
                .hint("We could not reach this address.")
                .state(TextFieldState::Error)
                .child(TextField::new(&field).state(TextFieldState::Error))
                .child(TextField::new(&field).multiline(true))
                .child(FieldValue::new("ff_live_51H").mono(true))
                .child(
                    Select::new("region", "us")
                        .option("us", "United States")
                        .option("eu", "Europe"),
                )
                .child(SearchField::new("search", &field).hint(KeyCap::new("⌘F")))
                .child(ShortcutRecorder::new("shortcut", "⌥⇧2")),
        )
        .child(
            Table::new()
                .child(
                    TableHead::new()
                        .child(TableCell::new().head(true).child("Build"))
                        .child(
                            TableCell::new()
                                .head(true)
                                .align(TableAlign::End)
                                .child("Status"),
                        ),
                )
                .child(
                    TableRow::new("aurora")
                        .active(true)
                        .interactive(true)
                        .child(TableCell::new().child("aurora"))
                        .child(
                            TableCell::new()
                                .align(TableAlign::End)
                                .child(Badge::new("Passed").tint(Tint::Success)),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .gap(vars.spacing_2)
                .child(Progress::new(35.))
                .child(Progress::indeterminate().gradient(true))
                .child(Spinner::new())
                .child(Divider::vertical()),
        )
        .child(
            StepList::new()
                .child(Step::new("Resolve the token graph", StepStatus::Done))
                .child(Step::new("Compile", StepStatus::Active).meta("14s"))
                .child(Step::new("Publish", StepStatus::Idle)),
        )
        .child(
            Callout::new()
                .tint(Tint::Warning)
                .title("Ramp moves only part-way")
                .description("color.warning cannot move whole under bright-dark.")
                .icon(Icon::new(icons::ALERT_16_REGULAR))
                .action(Button::new("fix", "Fix")),
        )
        .child(
            Menu::new("actions")
                .item(MenuItem::new("Rename").shortcut("⌘R").checked(true))
                .item(MenuItem::new("Delete").disabled(true))
                .trigger(|_, _| Button::new("trigger", "Actions").into_any_element()),
        )
        .child(
            Sidebar::new("sidebar")
                .resizable(true)
                .header(SectionLabel::new("Workspace"))
                .child(
                    SidebarGroup::new()
                        .label("Components")
                        .first(true)
                        .child(
                            NavItem::new("controls", "Controls")
                                .icon(Icon::new(icons::TABLE_16_REGULAR))
                                .active(true),
                        )
                        .child(NavItem::action("new", "New")),
                )
                .footer(SidebarCard::new().label("Queue").child("3 jobs")),
        )
        .child(
            Rail::new().child(
                RailGroup::new()
                    .label("Pages")
                    .child(NavItem::new("page", "Page").size(NavItemSize::Small)),
            ),
        )
        .child(Aside::new().child(SectionLabel::new("Details")))
        .child(
            Preferences::new().child(
                PreferenceGroup::new("Application")
                    .description("Settings that apply to every window.")
                    .child(
                        PreferenceSection::new()
                            .label("General")
                            .footer("Automatic follows the system.")
                            .action(Button::new("reset", "Reset"))
                            .child(
                                PreferenceRow::new("launch", "Launch at login")
                                    .subtitle("Every window")
                                    .child(Switch::new("launch-switch", true)),
                            )
                            .child(
                                PreferenceRow::new("open", "Open something")
                                    .on_open(|_, _| {})
                                    .child(KeyCap::new("⌘K")),
                            ),
                    ),
            ),
        )
        .child(
            ActionBar::new()
                .align(ActionBarAlign::End)
                .child(Button::new("cancel", "Cancel")),
        )
        .child(
            DialogScrim::new().child(
                Dialog::new()
                    .tone(DialogTone::Danger)
                    .child(DialogHeader::new("Discard?").subtitle("This cannot be undone."))
                    .child(DialogBody::new().child("Body"))
                    .child(DialogFooter::new().child(Button::new("keep", "Keep"))),
            ),
        )
        .child(
            ToastViewport::new().child(
                Toast::new("toast", "Tokens rebuilt")
                    .tint(Tint::Success)
                    .icon(Spinner::new())
                    .action(Button::new("undo", "Undo"))
                    .on_dismiss(|_, _| {}),
            ),
        )
        .child(EmptyState::new("Nothing here").action(Button::new("clear", "Clear")))
        .child(
            div()
                .flex()
                .gap(vars.spacing_2)
                .children(
                    [AvatarSize::Small, AvatarSize::Medium, AvatarSize::Large]
                        .map(|size| Avatar::new("Ada Lovelace").size(size)),
                )
                .child(Avatar::new("DazzUI").shape(AvatarShape::Square))
                .child(
                    AvatarGroup::new()
                        .max(2)
                        .avatar(Avatar::new("Ada Lovelace"))
                        .avatar(Avatar::new("Grace Hopper").tint(Tint::Info))
                        .avatar(Avatar::new("Alan Turing").tint(Tint::Success)),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(vars.spacing_2)
                .w(px(320.))
                .child(Meter::new(62.).label("Storage").value_label("62%"))
                .children(
                    [MeterSize::Small, MeterSize::Medium, MeterSize::Large]
                        .map(|size| Meter::new(62.).size(size).tint(Tint::Warning)),
                )
                .children(
                    [SliderSize::Small, SliderSize::Medium, SliderSize::Large].map(|size| {
                        Slider::new(SharedString::from(format!("slider-{size:?}")), 40.)
                            .size(size)
                            .value_label("40")
                    }),
                )
                .child(Slider::range("range", vec![20., 70.]))
                .child(Slider::new("disabled-slider", 40.).disabled(true)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(vars.spacing_2)
                .w(px(360.))
                // The seven that landed with the Base UI coverage sweep. A
                // component that is not on this page is a component no theme
                // has ever drawn.
                .child(
                    div()
                        .flex()
                        .gap(vars.spacing_2)
                        .child(Tag::new("tag-tinted", "bug"))
                        .child(
                            Tag::new("tag-selected", "blocked")
                                .variant(TagVariant::Outlined)
                                .selected(true),
                        )
                        .child(Tag::new("tag-removable", "needs design").on_remove(|_, _| {}))
                        .child(Toggle::new("toggle-up", "Wrap"))
                        .child(Toggle::new("toggle-down", "Wrap").pressed(true))
                        .child(Link::new("link", "Base UI").external(true))
                        .child(Code::new("pnpm parity")),
                )
                .child(
                    ToggleGroup::new(
                        "toggle-group",
                        vec![
                            ToggleItem::new("whitespace", "Whitespace"),
                            ToggleItem::new("minimap", "Minimap"),
                        ],
                    )
                    .value(vec![SharedString::from("whitespace")]),
                )
                .child(Skeleton::new().shape(SkeletonShape::Circle).width(px(40.)))
                .child(SkeletonText::new().lines(2))
                .child(
                    CodeBlock::new("pnpm install\npnpm tokens")
                        .title("install.sh")
                        .language("bash")
                        .line_numbers(true)
                        .copyable(true),
                )
                .child(
                    Tree::new(
                        "tree",
                        vec![
                            TreeNode::branch(
                                "src",
                                "src",
                                vec![TreeNode::leaf("badge", "badge.tsx").meta("M")],
                            )
                            .icon(icons::FOLDER_16_REGULAR),
                            TreeNode::leaf("readme", "README.md").icon(icons::DOCUMENT_16_REGULAR),
                        ],
                    )
                    .expanded(vec![SharedString::from("src")])
                    .selected("badge"),
                )
                .child(div().h(px(120.)).child(Splitter::new(
                    "splitter",
                    vec![
                        SplitterPanel::new(div().child("Files")).default_size(30.),
                        SplitterPanel::new(div().child("Document")).default_size(70.),
                    ],
                )))
                .child(
                    Collapsible::new("collapsible", "Environment")
                        .initially_open(true)
                        .meta("3 items")
                        .child("Node 24.20, pnpm 10.4"),
                )
                .child(
                    Accordion::new("accordion")
                        .divided(true)
                        .item(
                            AccordionItem::new("where", "Where do the values come from?")
                                .child("Every value is a token."),
                        )
                        .item(
                            AccordionItem::new("themes", "How many themes?")
                                .child("Twelve, as six families."),
                        ),
                )
                .child(
                    CheckboxGroup::new()
                        .parent(
                            Checkbox::new("every", "Every platform", false)
                                .indeterminate(true)
                                .note("2 of 4"),
                        )
                        .child(Checkbox::new("react", "react", true))
                        .child(Checkbox::new("gpui", "gpui", false)),
                )
                .child(
                    Form::new()
                        .child(
                            Fieldset::new()
                                .legend("Billing address")
                                .child(FormField::new("Street").child(TextField::new(&field))),
                        )
                        .child(FormField::new("City").child(TextField::new(&field))),
                )
                .child(NumberField::new("port", &field).unit("px").bounds(8., 24.))
                .child(OtpField::new("code", &field, 6).group_after(3))
                .child(
                    Combobox::new("zone", &field)
                        .value("utc")
                        .option(ComboboxOption::new("utc", "UTC").group("Global"))
                        .option(ComboboxOption::new("cet", "CET").group("Europe")),
                )
                .child(
                    Autocomplete::new("branch", &field)
                        .mono(true)
                        .item("main")
                        .item("release/4.2"),
                ),
        )
        .child(
            div()
                .flex()
                .gap(vars.spacing_2)
                .child(
                    Popover::new("about")
                        .title("Build 4.2.1")
                        .description("Published 3 days ago.")
                        .trigger(|_, _| Button::new("about-trigger", "About").into_any_element()),
                )
                .child(
                    PreviewCard::new("preview")
                        .trigger(Button::new("preview-trigger", "@ada"))
                        .child("Ada Lovelace"),
                )
                .child(
                    Menubar::new("menubar")
                        .menu(MenubarMenu::new("File").item(MenuItem::new("New")))
                        .menu(MenubarMenu::new("Edit").item(MenuItem::new("Undo"))),
                )
                .child(
                    ContextMenu::new("context")
                        .item(MenuItem::new("Rename"))
                        .child(Card::new().child("Right-click")),
                ),
        )
        .child(
            div().relative().w(px(480.)).h(px(240.)).child(
                Drawer::new()
                    .title("Filters")
                    .description("Narrow what the list is showing.")
                    .footer(Button::new("apply", "Apply"))
                    .child("Open, merged, closed."),
            ),
        )
        .into_any_element()
}

/// The playground prints these names too; the test only needs one of them.
fn variant_name(variant: BadgeVariant) -> &'static str {
    match variant {
        BadgeVariant::Filled => "Filled",
        BadgeVariant::Tinted => "Tinted",
        BadgeVariant::Outlined => "Outlined",
        BadgeVariant::Raised => "Raised",
    }
}

#[test]
fn a_recipe_resolves_against_the_tint_it_is_given() {
    let vars = ThemeName::StudioLight.variables();

    // `filled` fills with the ramp's 600 and prints what sits on an accent.
    let filled = vars.control_colors(ControlVariant::Filled, Tint::Danger, ControlState::Normal);
    assert_eq!(filled.surface, vars.color_danger.shade(600));
    assert_eq!(filled.content, vars.color_on_accent);

    // The same recipe against another ramp is the same shade of another hue.
    let success = vars.control_colors(ControlVariant::Filled, Tint::Success, ControlState::Normal);
    assert_eq!(success.surface, vars.color_success.shade(600));

    // Accent text is a grade darker than accent fill.
    let tinted = vars.control_colors(ControlVariant::Tinted, Tint::Primary, ControlState::Normal);
    assert_eq!(tinted.content, vars.color_primary.shade(700));

    // Disabled flattens to the groove, whatever the variant asked for.
    for variant in ControlVariant::ALL {
        let disabled = vars.control_colors(variant, Tint::Primary, ControlState::Disabled);
        assert_eq!(disabled.content, vars.color_content_faint);
    }
}

#[test]
fn a_theme_moves_what_it_names_and_what_follows_from_it() {
    let light = ThemeName::StudioLight.variables();
    let dark = ThemeName::StudioDark.variables();
    let bright = ThemeName::BrightLight.variables();

    // A role a theme re-points moves.
    assert_ne!(light.color_surface, dark.color_surface);

    // And so does everything derived from it, without the theme naming it:
    // the neutral control's resting surface *is* `color.surface`.
    assert_eq!(
        dark.control_colors(ControlVariant::Normal, Tint::Primary, ControlState::Normal)
            .surface,
        dark.color_surface,
    );

    // Bright re-points the primary ramp itself, so every recipe resolved
    // against it lands on another ladder.
    assert_ne!(
        light.color_primary.shade(600),
        bright.color_primary.shade(600)
    );

    // A token no theme touches is the same value in all four.
    for theme in ThemeName::ALL {
        assert_eq!(theme.variables().spacing_4, light.spacing_4);
    }
}

#[test]
fn a_ramp_is_a_role_ladder() {
    // Bright's ink ladder puts a *lighter* colour at 700 than at 600, which
    // is the thing the docs keep warning about: a step means the role the
    // recipe reaching for it gives it.
    let bright = ThemeName::BrightLight.variables();
    let ink = bright.color_primary;
    assert!(ink.shade(700).l > ink.shade(600).l);

    // A step nothing generated resolves to nothing rather than panicking.
    assert_eq!(ink.shade(1234).a, 0.);
}

#[test]
fn a_shortcut_needs_more_than_a_letter() {
    use gpui::Keystroke;

    // ⇧ alone is still typing.
    assert_eq!(format_shortcut(&Keystroke::parse("shift-a").unwrap()), None);
    // A function key is a shortcut by itself.
    assert_eq!(
        format_shortcut(&Keystroke::parse("f5").unwrap())
            .as_ref()
            .map(|k| k.as_ref()),
        Some("F5")
    );
    // Modifiers print in the order macOS prints them, and a word key gets a
    // space so `⌥ Space` reads as two things.
    assert_eq!(
        format_shortcut(&Keystroke::parse("cmd-shift-2").unwrap())
            .as_ref()
            .map(|k| k.as_ref()),
        Some("⇧⌘2")
    );
    assert_eq!(
        format_shortcut(&Keystroke::parse("alt-space").unwrap())
            .as_ref()
            .map(|k| k.as_ref()),
        Some("⌥ Space")
    );
}

/// The heights a sized control lands on, measured rather than asserted about.
///
/// This is what a screenshot would tell you and a type check would not: that
/// the profile a component binds is the one that ends up on the screen.
#[gpui::test]
fn a_control_is_the_height_its_profile_says(cx: &mut TestAppContext) {
    let measured = Measured::default();
    let vars = ThemeName::StudioLight.variables();

    {
        let (_ruler, cx) = cx.add_window_view({
            let measured = measured.clone();
            |_, _| Ruler {
                measured,
                build: |vars| {
                    ControlSize::ALL
                        .map(|size| {
                            Button::new(size.name(), size.name())
                                .size(size)
                                .into_any_element()
                        })
                        .into_iter()
                        .chain([
                            Switch::new("small", true)
                                .size(SwitchSize::Small)
                                .into_any_element(),
                            Switch::new("large", true)
                                .size(SwitchSize::Large)
                                .into_any_element(),
                            Progress::new(50.)
                                .size(ProgressSize::Large)
                                .into_any_element(),
                            Divider::horizontal().into_any_element(),
                            Spinner::new().into_any_element(),
                            Badge::new("Tag").into_any_element(),
                            known_square(vars),
                        ])
                        .collect()
                },
            }
        });
        cx.run_until_parked();
    }

    let bounds = measured.0.borrow().clone();
    assert_eq!(bounds.len(), 11, "every specimen was measured");

    // The four control heights: 24 · 26 · 28 · 32, off `base.control-height`.
    for (index, size) in ControlSize::ALL.into_iter().enumerate() {
        assert_eq!(
            bounds[index].size.height,
            vars.metrics(size).size,
            "a {} button",
            size.name()
        );
    }

    // A switch is its own box on both axes.
    assert_eq!(bounds[4].size.width, vars.switch_small_width);
    assert_eq!(bounds[4].size.height, vars.switch_small_height);
    assert_eq!(bounds[5].size.width, vars.switch_large_width);
    assert_eq!(bounds[5].size.height, vars.switch_large_height);

    // A progress bar is a rule cut into the surface, not a control.
    assert_eq!(bounds[6].size.height, vars.spacing_2);

    // A divider is one CSS pixel, where a hairline halves on Retina.
    assert_eq!(bounds[7].size.height, vars.spacing_px);

    // A spinner is square, at the medium step.
    assert_eq!(bounds[8].size.width, vars.spacing_4);
    assert_eq!(bounds[8].size.height, vars.spacing_4);

    // A badge never reaches control height: it is an annotation.
    assert!(bounds[9].size.height < vars.control_tiny_size);

    // And the square is where the ruler says it is.
    assert_eq!(bounds[10].size.width, vars.spacing_8);
}

/// A square of a known size, so a measurement that came back empty or in the
/// wrong order fails loudly rather than passing by accident.
fn known_square(vars: &'static ThemeVariables) -> AnyElement {
    div().w(vars.spacing_8).h(vars.spacing_8).into_any_element()
}

/// The bounds the ruler collected, in the order the children were built.
#[derive(Clone, Default)]
struct Measured(std::rc::Rc<std::cell::RefCell<Vec<gpui::Bounds<gpui::Pixels>>>>);

/// A view that lays a row of specimens out and writes down where they landed.
struct Ruler {
    measured: Measured,
    build: fn(&'static ThemeVariables) -> Vec<AnyElement>,
}

impl Render for Ruler {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let vars = cx.theme().vars;
        let measured = self.measured.clone();

        div()
            .flex()
            .flex_col()
            .items_start()
            .gap(vars.spacing_2)
            .children((self.build)(vars))
            .on_children_prepainted(move |bounds, _, _| {
                *measured.0.borrow_mut() = bounds;
            })
    }
}

/// The pointer, end to end: bounds from the last frame, a hover state before
/// the subtree is built, and a click that reaches the listener.
///
/// This is the piece with the most machinery behind it — `pressable` is what
/// stands in for `:hover` and `:active` — so it is the piece worth simulating
/// rather than reasoning about.
#[gpui::test]
fn a_control_answers_the_pointer(cx: &mut TestAppContext) {
    let measured = Measured::default();
    let seen = std::rc::Rc::new(std::cell::Cell::new(PressableState::default()));
    let clicked = std::rc::Rc::new(std::cell::Cell::new(false));

    let (_ruler, cx) = cx.add_window_view({
        let measured = measured.clone();
        let seen = seen.clone();
        let clicked = clicked.clone();
        |_, _| Prober {
            measured,
            seen,
            clicked,
        }
    });
    cx.run_until_parked();

    let bounds = measured.0.borrow()[0];
    let away = gpui::point(bounds.right() + px(200.), bounds.bottom() + px(200.));

    cx.simulate_mouse_move(away, None, gpui::Modifiers::default());
    cx.run_until_parked();
    assert!(
        !seen.get().hovered,
        "nothing is hovered before the pointer is"
    );

    cx.simulate_mouse_move(bounds.center(), None, gpui::Modifiers::default());
    cx.run_until_parked();
    assert!(seen.get().hovered, "the pointer is over the control");

    cx.simulate_click(bounds.center(), gpui::Modifiers::default());
    cx.run_until_parked();
    assert!(clicked.get(), "the click reached the listener");

    cx.simulate_mouse_move(away, None, gpui::Modifiers::default());
    cx.run_until_parked();
    assert!(!seen.get().hovered, "and the pointer left again");
}

/// A view that writes down the state its control was handed.
struct Prober {
    measured: Measured,
    seen: std::rc::Rc<std::cell::Cell<PressableState>>,
    clicked: std::rc::Rc<std::cell::Cell<bool>>,
}

impl Render for Prober {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let vars = cx.theme().vars;
        let measured = self.measured.clone();
        let seen = self.seen.clone();
        let clicked = self.clicked.clone();

        div()
            .flex()
            .items_start()
            .child(pressable("probe", true, move |state, _, _| {
                seen.set(state);
                div()
                    .id("probe-body")
                    .w(vars.spacing_20)
                    .h(vars.control_medium_size)
                    .bg(vars.color_surface)
                    .on_click(move |_, _, _| clicked.set(true))
                    .into_any_element()
            }))
            .on_children_prepainted(move |bounds, _, _| {
                *measured.0.borrow_mut() = bounds;
            })
    }
}

/// Typing, end to end: the field takes the keyboard, the platform's text
/// reaches the input handler, and the editing keys move the caret.
#[gpui::test]
fn a_field_takes_what_is_typed_into_it(cx: &mut TestAppContext) {
    let (typist, cx) = cx.add_window_view(|_, cx| Typist {
        field: cx.new(|cx| TextBuffer::new(cx, "Search")),
    });
    cx.run_until_parked();

    let field = typist.read_with(cx, |typist, _| typist.field.clone());
    cx.update(|window, cx| field.read(cx).focus(window));
    cx.run_until_parked();

    cx.simulate_input("main");
    cx.run_until_parked();
    assert_eq!(
        field.read_with(cx, |field, _| field.value().clone()),
        "main"
    );

    cx.simulate_keystrokes("backspace");
    cx.run_until_parked();
    assert_eq!(field.read_with(cx, |field, _| field.value().clone()), "mai");

    // Select it all and type over it, which is what a field is for.
    cx.simulate_keystrokes("cmd-a");
    cx.simulate_input("v2");
    cx.run_until_parked();
    assert_eq!(field.read_with(cx, |field, _| field.value().clone()), "v2");
}

/// A view holding one field, because a field belongs to whoever owns it.
struct Typist {
    field: gpui::Entity<TextBuffer>,
}

impl Render for Typist {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().w(px(300.)).child(TextField::new(&self.field))
    }
}

/// The other keyboard control: click it, press a chord, and it keeps what a
/// menu would print.
#[gpui::test]
fn a_recorder_keeps_the_chord_it_is_given(cx: &mut TestAppContext) {
    let measured = Measured::default();
    let bound = std::rc::Rc::new(std::cell::RefCell::new(SharedString::default()));

    let (_recorder, cx) = cx.add_window_view({
        let measured = measured.clone();
        let bound = bound.clone();
        |_, _| Rebinder { measured, bound }
    });
    cx.run_until_parked();

    let bounds = measured.0.borrow()[0];
    cx.simulate_click(bounds.center(), gpui::Modifiers::default());
    cx.run_until_parked();

    cx.simulate_keystrokes("cmd-shift-2");
    cx.run_until_parked();
    assert_eq!(bound.borrow().as_ref(), "⇧⌘2");

    // A bare letter is typing rather than a shortcut, so it is not taken.
    cx.simulate_click(bounds.center(), gpui::Modifiers::default());
    cx.run_until_parked();
    cx.simulate_keystrokes("k");
    cx.run_until_parked();
    assert_eq!(bound.borrow().as_ref(), "⇧⌘2");
}

/// A view holding one recorder and the binding it reports.
struct Rebinder {
    measured: Measured,
    bound: std::rc::Rc<std::cell::RefCell<SharedString>>,
}

impl Render for Rebinder {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let measured = self.measured.clone();
        let bound = self.bound.clone();
        let value = self.bound.borrow().clone();

        div()
            .flex()
            .items_start()
            .child(
                ShortcutRecorder::new("shortcut", value).on_change(move |next, _, _| {
                    *bound.borrow_mut() = next.clone();
                }),
            )
            .on_children_prepainted(move |bounds, _, _| {
                *measured.0.borrow_mut() = bounds;
            })
    }
}

/// A popover, end to end: the trigger opens it, the panel paints over what it
/// hangs across, and choosing an item closes it.
#[gpui::test]
fn a_menu_opens_and_chooses(cx: &mut TestAppContext) {
    let measured = Measured::default();
    let chosen = std::rc::Rc::new(std::cell::Cell::new(0usize));

    let (_menu, cx) = cx.add_window_view({
        let measured = measured.clone();
        let chosen = chosen.clone();
        |_, _| Chooser { measured, chosen }
    });
    cx.run_until_parked();

    // Closed, the menu is only its trigger.
    let closed = measured.0.borrow()[0];
    cx.simulate_click(closed.center(), gpui::Modifiers::default());
    cx.run_until_parked();

    // Open, the panel hangs below it — so the first item sits under the
    // trigger's own bottom edge.
    let item = gpui::point(closed.center().x, closed.bottom() + px(24.));
    cx.simulate_click(item, gpui::Modifiers::default());
    cx.run_until_parked();
    assert_eq!(chosen.get(), 1, "the item under the pointer was chosen");

    // And the menu closed behind it: the same click lands on nothing now.
    cx.simulate_click(item, gpui::Modifiers::default());
    cx.run_until_parked();
    assert_eq!(chosen.get(), 1, "a closed menu has nothing to click");
}

/// A view holding one menu and a count of what was chosen from it.
struct Chooser {
    measured: Measured,
    chosen: std::rc::Rc<std::cell::Cell<usize>>,
}

impl Render for Chooser {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let measured = self.measured.clone();
        let chosen = self.chosen.clone();

        div()
            .flex()
            .items_start()
            .child(
                Menu::new("actions")
                    .align(MenuAlign::Start)
                    .item(MenuItem::new("Rename").on_select(move |_, _| {
                        chosen.set(chosen.get() + 1);
                    }))
                    .item(MenuItem::new("Duplicate"))
                    .trigger(|_, _| Button::new("trigger", "Actions").into_any_element()),
            )
            .on_children_prepainted(move |bounds, _, _| {
                *measured.0.borrow_mut() = bounds;
            })
    }
}

/// Every theme on one screen, which is the claim the Themes page makes.
///
/// A theme is a value scoped to a subtree rather than a mode the process is
/// in, so a component resolves against whichever one wraps it — including
/// when one wraps another.
#[gpui::test]
fn a_theme_scopes_to_its_own_subtree(cx: &mut TestAppContext) {
    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

    let (_scoped, cx) = cx.add_window_view({
        let seen = seen.clone();
        |_, _| Scoped { seen }
    });
    cx.run_until_parked();

    // The window's own theme, then one card per theme, then the window's
    // again — the stack unwinds where each card ends. Composed from `ALL`
    // rather than written out, so a theme added to the graph is covered here
    // the moment it reaches the enum.
    let mut expected = vec![ThemeName::StudioDark];
    expected.extend(ThemeName::ALL);
    expected.push(ThemeName::StudioDark);

    assert_eq!(*seen.borrow(), expected);
}

/// A view that renders a probe outside the cards and one inside each.
struct Scoped {
    seen: std::rc::Rc<std::cell::RefCell<Vec<ThemeName>>>,
}

impl Render for Scoped {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let seen = self.seen.clone();

        themed(
            ThemeName::StudioDark,
            div()
                .flex()
                .child(probe(seen.clone()))
                .children(ThemeName::ALL.map(|name| themed(name, probe(seen.clone()))))
                .child(probe(seen)),
        )
    }
}

/// An element that writes down the theme it resolved against.
fn probe(seen: std::rc::Rc<std::cell::RefCell<Vec<ThemeName>>>) -> AnyElement {
    pressable("probe", true, move |_, _, cx| {
        seen.borrow_mut().push(cx.theme().name);
        div().into_any_element()
    })
    .into_any_element()
}
