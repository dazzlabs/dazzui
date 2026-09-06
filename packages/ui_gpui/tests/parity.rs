//! The React reference, measured against.
//!
//! `tests/parity/reference/*.json` is what `apps/storybook` rendered: every
//! component root on every story, in every theme, with its `data-*` axes and
//! the box, border, corner, colours and type the browser computed. This test
//! reads those probes back as *specimens* — a class and its axes — builds
//! each one here with the matching component, lays it out under the same
//! theme, and measures what it can. Where the two disagree is where the port
//! has drifted from the standard, and it is listed rather than argued about.
//!
//! What can be read here without a screen is the box, which GPUI reports
//! after prepaint, and — for the controls that read the recipe table — the
//! colours the theme resolves for the variant, tint and state, which is what
//! the component paints with. A width that depends on text is skipped, since
//! the text system under test is not the browser's; a width that varies
//! between the stories a specimen appears in is skipped, because it belonged
//! to the story rather than the component.
//!
//! `tests/parity/drift.txt` is the ratchet: every check that fails today is
//! named there, held both ways, so the list can only shrink. Bringing a
//! component into line means deleting its lines in the same change.
//!
//! None of this is committed: the reference is what `pnpm parity` writes,
//! and the list is recorded by the first run that finds it missing. A
//! checkout without the reference passes with a note rather than failing.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use dazzui::assets::icons;
use dazzui::prelude::*;
use dazzui::*;
use gpui::{
    AnyElement, App, Bounds, Context, IntoElement, ParentElement, Pixels, Render, Rgba, Styled,
    TestAppContext, Window, div, px,
};
use serde_json::Value;

/// One distinct thing React drew: a class and its axes, with the first text
/// it was seen with. Text is not part of the identity — a badge reading
/// Beta and one reading Failed are the same specimen.
#[derive(Clone, Debug)]
struct Probe {
    class: String,
    data: BTreeMap<String, String>,
    text: String,
    /// The fields the React component's own stylesheet decides.
    ///
    /// A computed style always answers — a callout root reports the page's
    /// 13px because it inherits it, having no type of its own — so holding
    /// a port to one is holding it to a value the component never chose.
    /// Only what the stylesheet declares is the component's own decision.
    declares: BTreeSet<String>,
}

impl Probe {
    fn key(&self) -> String {
        let axes: Vec<String> = self.data.iter().map(|(k, v)| format!("{k}={v}")).collect();
        if axes.is_empty() {
            self.class.clone()
        } else {
            format!("{} {}", self.class, axes.join(" "))
        }
    }

    fn has(&self, axis: &str) -> bool {
        self.data.contains_key(axis)
    }

    fn get(&self, axis: &str) -> Option<&str> {
        self.data.get(axis).map(String::as_str)
    }

    fn disabled(&self) -> bool {
        self.has("disabled") || self.get("aria-disabled") == Some("true")
    }

    /// Components whose height is what they hold rather than what they are.
    ///
    /// A button is its size profile's height whatever its label; a card is
    /// as tall as its children, and those came from the story rather than
    /// from any axis this specimen carries. Comparing one is comparing two
    /// different contents, so the height is left to the screenshot layer.
    fn height_is_content_driven(&self) -> bool {
        matches!(
            self.class.as_str(),
            "dz-card" | "dz-callout" | "dz-option-card" | "dz-toast" | "dz-step"
        )
    }

    fn is_field(&self) -> bool {
        matches!(
            self.class.as_str(),
            "dz-text-field" | "dz-search-field" | "dz-select" | "dz-shortcut-recorder"
        )
    }
}

/// What React measured for a specimen in one theme. A value is `None` where
/// the specimen's appearances disagreed, which means it was the story's.
#[derive(Clone, Debug, Default)]
struct Expected {
    width: Option<f32>,
    height: Option<f32>,
    background: Option<String>,
    color: Option<String>,
    border_color: Option<String>,
}

struct Specimen {
    key: String,
    probe: Probe,
    themes: BTreeMap<String, Expected>,
}

fn load_specimens() -> Vec<Specimen> {
    let dir = format!("{}/tests/parity/reference", env!("CARGO_MANIFEST_DIR"));
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("the reference directory exists")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    files.sort();

    let mut probes: BTreeMap<String, Probe> = BTreeMap::new();
    let mut seen: BTreeMap<String, BTreeMap<String, Vec<Value>>> = BTreeMap::new();

    for file in files {
        let page: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
        for story in page["stories"].as_object().unwrap().values() {
            for (theme, drawn) in story["themes"].as_object().unwrap() {
                for raw in drawn["probes"].as_array().unwrap() {
                    let data: BTreeMap<String, String> = raw["data"]
                        .as_object()
                        .unwrap()
                        .iter()
                        .filter(|(k, _)| !k.starts_with("base-ui") && k.as_str() != "control")
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect();
                    let probe = Probe {
                        class: raw["class"].as_str().unwrap().to_string(),
                        data,
                        text: raw["text"].as_str().unwrap_or("").to_string(),
                        declares: raw["declares"]
                            .as_array()
                            .map(|list| {
                                list.iter()
                                    .filter_map(|field| field.as_str())
                                    .map(str::to_string)
                                    .collect()
                            })
                            .unwrap_or_default(),
                    };
                    let key = probe.key();
                    probes.entry(key.clone()).or_insert(probe);
                    seen.entry(key)
                        .or_default()
                        .entry(theme.clone())
                        .or_default()
                        .push(raw.clone());
                }
            }
        }
    }

    seen.into_iter()
        .map(|(key, themes)| Specimen {
            probe: probes[&key].clone(),
            themes: themes
                .into_iter()
                .map(|(t, list)| (t, fold(&list)))
                .collect(),
            key,
        })
        .collect()
}

/// The value every appearance agrees on, or `None`.
fn fold(probes: &[Value]) -> Expected {
    fn agreed_num(probes: &[Value], read: impl Fn(&Value) -> Option<f64>) -> Option<f32> {
        let first = read(&probes[0])?;
        for probe in &probes[1..] {
            if (read(probe)? - first).abs() > 0.5 {
                return None;
            }
        }
        Some(first as f32)
    }
    fn agreed_str(probes: &[Value], read: impl Fn(&Value) -> Option<&str>) -> Option<String> {
        let first = read(&probes[0])?;
        for probe in &probes[1..] {
            if read(probe)? != first {
                return None;
            }
        }
        Some(first.to_string())
    }
    Expected {
        width: agreed_num(probes, |p| p["rect"]["width"].as_f64()),
        height: agreed_num(probes, |p| p["rect"]["height"].as_f64()),
        background: agreed_str(probes, |p| p["style"]["backgroundColor"].as_str()),
        color: agreed_str(probes, |p| p["style"]["color"].as_str()),
        border_color: agreed_str(probes, |p| p["style"]["borderTopColor"].as_str()),
    }
}

// ---------------------------------------------------------------------------
// Axes, spelled the way the reference spells them.

fn tint(name: Option<&str>) -> Option<Tint> {
    Tint::ALL.into_iter().find(|t| Some(t.name()) == name)
}

fn variant(name: Option<&str>) -> Option<ControlVariant> {
    ControlVariant::ALL
        .into_iter()
        .find(|v| Some(v.name()) == name)
}

fn control_size(name: Option<&str>) -> Option<ControlSize> {
    ControlSize::ALL
        .into_iter()
        .find(|s| Some(s.name()) == name)
}

/// The three-step size most components carry, spelled per component.
macro_rules! sized {
    ($name:expr, $ty:ident) => {
        match $name {
            Some("small") => Some($ty::Small),
            Some("medium") => Some($ty::Medium),
            Some("large") => Some($ty::Large),
            _ => None,
        }
    };
}

/// The component a probe names, or `None` where this port cannot build it.
///
/// Only *leaf* components are built: the ones whose look is a function of
/// their own axes. A container takes its size from what it holds, so a probe
/// of one says more about the story than about the component.
fn build(probe: &Probe, cx: &mut App) -> Option<AnyElement> {
    // An id of its own per specimen, which is load bearing: GPUI keys its
    // element state by id, so a shared one would hand the next specimen the
    // last one's press and hover state.
    let id: gpui::SharedString = probe.key().into();
    let text: gpui::SharedString = probe.text.clone().into();
    let disabled = probe.disabled();
    let t = tint(probe.get("tint"));

    let element = match probe.class.as_str() {
        "dz-button" => {
            let mut button = Button::new(id.clone(), text)
                .disabled(disabled)
                .full_width(probe.get("full-width") == Some("true"));
            if let Some(v) = variant(probe.get("variant")) {
                button = button.variant(v);
            }
            if let Some(s) = control_size(probe.get("size")) {
                button = button.size(s);
            }
            if let Some(t) = t {
                button = button.tint(t);
            }
            button.into_any_element()
        }
        "dz-icon-button" => {
            let mut button = IconButton::new(id.clone(), "Star", Icon::new(icons::STAR_16_REGULAR))
                .disabled(disabled);
            if let Some(v) = variant(probe.get("variant")) {
                button = button.variant(v);
            }
            if let Some(s) = control_size(probe.get("size")) {
                button = button.size(s);
            }
            if let Some(t) = t {
                button = button.tint(t);
            }
            button.into_any_element()
        }
        "dz-badge" => {
            let mut badge = Badge::new(text);
            if let Some(s) = sized!(probe.get("size"), BadgeSize) {
                badge = badge.size(s);
            }
            badge = match probe.get("variant") {
                Some("filled") => badge.variant(BadgeVariant::Filled),
                Some("tinted") => badge.variant(BadgeVariant::Tinted),
                Some("outlined") => badge.variant(BadgeVariant::Outlined),
                Some("raised") => badge.variant(BadgeVariant::Raised),
                _ => badge,
            };
            if let Some(t) = t {
                badge = badge.tint(t);
            }
            badge.into_any_element()
        }
        "dz-checkbox" => {
            let mut checkbox =
                Checkbox::new(id.clone(), text, probe.has("checked")).disabled(disabled);
            if let Some(s) = sized!(probe.get("size"), CheckboxSize) {
                checkbox = checkbox.size(s);
            }
            if let Some(t) = t {
                checkbox = checkbox.tint(t);
            }
            checkbox.into_any_element()
        }
        "dz-radio" => {
            let mut radio = Radio::new(id.clone(), text, probe.has("checked")).disabled(disabled);
            if let Some(s) = sized!(probe.get("size"), RadioSize) {
                radio = radio.size(s);
            }
            if let Some(t) = t {
                radio = radio.tint(t);
            }
            radio.into_any_element()
        }
        "dz-switch" => {
            let mut switch = Switch::new(id.clone(), probe.has("checked")).disabled(disabled);
            if let Some(s) = sized!(probe.get("size"), SwitchSize) {
                switch = switch.size(s);
            }
            if let Some(t) = t {
                switch = switch.tint(t);
            }
            switch.into_any_element()
        }
        "dz-key-cap" => {
            let mut cap = KeyCap::new(text);
            if let Some(s) = sized!(probe.get("size"), KeyCapSize) {
                cap = cap.size(s);
            }
            cap = match probe.get("variant") {
                Some("hint") => cap.variant(KeyCapVariant::Hint),
                Some("strong") => cap.variant(KeyCapVariant::Strong),
                Some("key") => cap.variant(KeyCapVariant::Key),
                _ => cap,
            };
            cap.into_any_element()
        }
        "dz-card" => {
            let mut card = Card::new().child(text);
            if let Some(s) = sized!(probe.get("size"), CardSize) {
                card = card.size(s);
            }
            card = match probe.get("variant") {
                Some("raised") => card.variant(CardVariant::Raised),
                Some("sunken") => card.variant(CardVariant::Sunken),
                Some("outlined") => card.variant(CardVariant::Outlined),
                Some("tinted") => card.variant(CardVariant::Tinted),
                _ => card,
            };
            if let Some(t) = t {
                card = card.tint(t);
            }
            card.into_any_element()
        }
        "dz-callout" => {
            // `has-description` means a title *and* a body: React sets it
            // only when both are given, so a callout carrying it is two
            // lines.
            let mut callout = Callout::new().description(text);
            if probe.has("has-description") {
                callout = callout.title("Title");
            }
            if let Some(s) = sized!(probe.get("size"), CalloutSize) {
                callout = callout.size(s);
            }
            if let Some(t) = t {
                callout = callout.tint(t);
            }
            callout.into_any_element()
        }
        "dz-progress" => {
            let mut progress = if probe.has("indeterminate") {
                Progress::indeterminate()
            } else {
                Progress::new(35.)
            };
            if let Some(s) = sized!(probe.get("size"), ProgressSize) {
                progress = progress.size(s);
            }
            if let Some(t) = t {
                progress = progress.tint(t);
            }
            progress.into_any_element()
        }
        "dz-spinner" => {
            let mut spinner = Spinner::new();
            if let Some(s) = sized!(probe.get("size"), SpinnerSize) {
                spinner = spinner.size(s);
            }
            if let Some(t) = t {
                spinner = spinner.tint(t);
            }
            spinner.into_any_element()
        }
        "dz-section-label" => {
            let mut label = SectionLabel::new(text);
            if let Some(t) = t {
                label = label.tint(t);
            }
            label.into_any_element()
        }
        "dz-toast" => {
            let mut toast = Toast::new(id.clone(), text);
            if let Some(t) = t {
                toast = toast.tint(t);
            }
            toast.into_any_element()
        }
        "dz-nav-item" => {
            let mut item = NavItem::new(id.clone(), text).disabled(disabled);
            item = match probe.get("size") {
                Some("small") => item.size(NavItemSize::Small),
                Some("medium") => item.size(NavItemSize::Medium),
                _ => item,
            };
            if let Some(t) = t {
                item = item.tint(t);
            }
            item.into_any_element()
        }
        "dz-option-card" => {
            let mut card = OptionCard::new(id.clone(), text)
                .selected(probe.get("aria-checked") == Some("true"))
                .disabled(disabled);
            if let Some(t) = t {
                card = card.tint(t);
            }
            card.into_any_element()
        }
        "dz-text-field" => {
            let field = cx.new(|cx| TextBuffer::new(cx, "Placeholder"));
            let mut text_field = TextField::new(&field)
                .multiline(probe.has("multiline"))
                .mono(probe.get("mono") == Some("true"))
                .disabled(disabled);
            if let Some(s) = sized!(probe.get("size"), TextFieldSize) {
                text_field = text_field.size(s);
            }
            if probe.has("invalid") {
                text_field = text_field.state(TextFieldState::Error);
            }
            if let Some(t) = t {
                text_field = text_field.tint(t);
            }
            text_field.into_any_element()
        }
        "dz-search-field" => {
            let field = cx.new(|cx| TextBuffer::new(cx, "Search"));
            let mut search = SearchField::new(id.clone(), &field);
            if let Some(s) = sized!(probe.get("size"), TextFieldSize) {
                search = search.size(s);
            }
            if let Some(t) = t {
                search = search.tint(t);
            }
            search.into_any_element()
        }
        "dz-select" => {
            let mut select = Select::new(id.clone(), "us")
                .option("us", "United States")
                .option("eu", "Europe");
            if let Some(s) = sized!(probe.get("size"), TextFieldSize) {
                select = select.size(s);
            }
            if probe.has("invalid") {
                select = select.state(TextFieldState::Error);
            }
            select.into_any_element()
        }
        "dz-shortcut-recorder" => {
            let mut recorder = ShortcutRecorder::new(id.clone(), "⌘K").disabled(disabled);
            if let Some(t) = t {
                recorder = recorder.tint(t);
            }
            recorder.into_any_element()
        }
        "dz-field-value" => {
            let mut value = FieldValue::new(text).mono(probe.get("mono") == Some("true"));
            if let Some(s) = sized!(probe.get("size"), TextFieldSize) {
                value = value.size(s);
            }
            value.into_any_element()
        }
        "dz-divider" => {
            let mut divider = if probe.get("orientation") == Some("vertical") {
                Divider::vertical()
            } else {
                Divider::horizontal()
            };
            if probe.get("tone") == Some("muted") {
                divider = divider.tone(DividerTone::Muted);
            }
            divider.into_any_element()
        }
        "dz-segmented-control" => {
            let mut control = SegmentedControl::new(id.clone(), "design")
                .item("design", "Design")
                .item("code", "Code")
                .stretch(probe.get("stretch") == Some("true"));
            if let Some(s) = sized!(probe.get("size"), SegmentedControlSize) {
                control = control.size(s);
            }
            if probe.get("active-style") == Some("filled") {
                control = control.active_style(SegmentedActiveStyle::Filled);
            }
            if let Some(t) = t {
                control = control.tint(t);
            }
            control.into_any_element()
        }
        "dz-pill-tabs" => {
            let mut tabs = PillTabs::new(id.clone(), "all")
                .item("all", "All")
                .item("open", "Open");
            if let Some(t) = t {
                tabs = tabs.tint(t);
            }
            tabs.into_any_element()
        }
        "dz-step" => {
            let status = match probe.get("status") {
                Some("done") => StepStatus::Done,
                Some("active") => StepStatus::Active,
                _ => StepStatus::Idle,
            };
            let mut step = Step::new(text, status);
            if let Some(t) = t {
                step = step.tint(t);
            }
            step.into_any_element()
        }
        _ => return None,
    };
    Some(element)
}

// ---------------------------------------------------------------------------
// Measuring.

/// The bounds the ruler collected, in the order the specimens were built.
type Measured = Rc<RefCell<Vec<Bounds<Pixels>>>>;

type Builder = Rc<dyn Fn(&mut App) -> Vec<AnyElement>>;

/// A view that lays a column of specimens out under one theme and writes
/// down where each landed.
struct Ruler {
    theme: ThemeName,
    build: Builder,
    measured: Measured,
}

impl Render for Ruler {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let measured = self.measured.clone();
        let children = (self.build)(cx);
        themed(
            self.theme,
            div()
                .flex()
                .flex_col()
                .items_start()
                .children(children)
                .on_children_prepainted(move |bounds, _, _| {
                    *measured.borrow_mut() = bounds;
                }),
        )
    }
}

/// `#rrggbb`, or `#rrggbbaa` when translucent — the reference's spelling.
fn hex(color: gpui::Hsla) -> String {
    let rgba: Rgba = color.into();
    let channel = |value: f32| format!("{:02x}", (value * 255.).round() as u8);
    let rgb = format!("#{}{}{}", channel(rgba.r), channel(rgba.g), channel(rgba.b));
    if rgba.a >= 1. {
        rgb
    } else {
        format!("{rgb}{}", channel(rgba.a))
    }
}

/// Equal to the eye: the same channels, with one step of slack on alpha,
/// which is where `0.12 × 255` rounds differently on two platforms.
fn same_colour(a: &str, b: &str) -> bool {
    let alpha = |v: &str| {
        if v.len() == 9 {
            u8::from_str_radix(&v[7..], 16).unwrap_or(255)
        } else {
            255
        }
    };
    if alpha(a) == 0 && alpha(b) == 0 {
        return true;
    }
    a[..7] == b[..7] && alpha(a).abs_diff(alpha(b)) <= 1
}

/// The colours a control paints with: the recipe the theme resolves for its
/// variant, tint and state. This is what the component reads, so it is what
/// the browser's computed colour is held to.
fn recipe(probe: &Probe, vars: &ThemeVariables) -> Option<ControlColors> {
    if !matches!(probe.class.as_str(), "dz-button" | "dz-icon-button") {
        return None;
    }
    // The one control that deliberately leaves the recipe: an icon button's
    // default posture — plain, neutral — is subtle ink rather than the
    // neutral ramp, which `icon-button.css` says is too flat a grey for a
    // toolbar that has to read as chrome. Every port draws it that way; the
    // recipe table is simply not what it is drawn from.
    if probe.class == "dz-icon-button"
        && probe.get("variant") == Some("plain")
        && probe.get("tint") == Some("neutral")
    {
        return None;
    }
    let state = if probe.disabled() {
        ControlState::Disabled
    } else {
        ControlState::Normal
    };
    Some(vars.control_colors(
        variant(probe.get("variant"))?,
        tint(probe.get("tint"))?,
        state,
    ))
}

#[gpui::test]
fn every_specimen_the_port_can_build_measures_as_react_drew_it(cx: &mut TestAppContext) {
    let reference = format!("{}/tests/parity/reference", env!("CARGO_MANIFEST_DIR"));
    if !std::path::Path::new(&reference).is_dir() {
        println!("parity: no tests/parity/reference — run `pnpm parity` at the root; skipped");
        return;
    }
    let specimens = load_specimens();
    let drift_path = format!("{}/tests/parity/drift.txt", env!("CARGO_MANIFEST_DIR"));
    let recorded: Option<BTreeSet<String>> =
        std::fs::read_to_string(&drift_path).ok().map(|text| {
            text.lines()
                .map(str::trim)
                .filter(|line| !line.is_empty() && !line.starts_with('#'))
                .map(str::to_string)
                .collect()
        });

    let mut failing: BTreeSet<String> = BTreeSet::new();
    let mut detail: BTreeMap<String, String> = BTreeMap::new();
    let mut unsupported: BTreeSet<String> = BTreeSet::new();
    // What this port measured, written beside the drift list for the viewer
    // (`pnpm parity:view`): the value on this side of every check.
    let mut measured_all: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();

    // The specimens this port can build, in a fixed order, so the bounds
    // the ruler hands back line up with them.
    let buildable: Vec<&Specimen> = specimens
        .iter()
        .filter(|specimen| {
            let ok = cx.update(|cx| build(&specimen.probe, cx).is_some());
            if !ok {
                unsupported.insert(specimen.probe.class.clone());
            }
            ok
        })
        .collect();

    for theme in ThemeName::ALL {
        let name = theme.attribute();
        let vars = theme.variables();
        let measured: Measured = Rc::default();

        let expected: Vec<Option<f32>> = buildable
            .iter()
            .map(|s| s.themes.get(name).and_then(|e| e.width))
            .collect();
        let probes: Vec<Probe> = buildable.iter().map(|s| s.probe.clone()).collect();

        let build_all: Builder = Rc::new(move |cx: &mut App| {
            probes
                .iter()
                .zip(&expected)
                .map(|(probe, width)| {
                    let element = build(probe, cx).expect("filtered to buildable");
                    // A field has no width of its own; give it the story's.
                    match (probe.is_field(), width) {
                        (true, Some(width)) => {
                            div().w(px(*width)).child(element).into_any_element()
                        }
                        (true, None) => div().w(px(240.)).child(element).into_any_element(),
                        _ => element,
                    }
                })
                .collect()
        });

        {
            let measured = measured.clone();
            let (_ruler, cx) = cx.add_window_view(move |_, _| Ruler {
                theme,
                build: build_all,
                measured,
            });
            cx.run_until_parked();
        }

        let bounds = measured.borrow().clone();
        assert_eq!(
            bounds.len(),
            buildable.len(),
            "every specimen was measured under {name}"
        );

        for (specimen, bounds) in buildable.iter().zip(bounds) {
            let Some(want) = specimen.themes.get(name) else {
                continue;
            };
            let mut note = |field: &str, expected: String, actual: String| {
                let check = format!("{} · {field}", specimen.key);
                failing.insert(check.clone());
                detail
                    .entry(check)
                    .or_insert_with(|| format!("{name}: React {expected}, here {actual}"));
            };

            let width: f32 = bounds.size.width.into();
            let height: f32 = bounds.size.height.into();
            let colors = recipe(&specimen.probe, vars);
            measured_all
                .entry(specimen.key.clone())
                .or_default()
                .insert(
                    name.to_string(),
                    serde_json::json!({
                        "width": width,
                        "height": height,
                        "background": colors.map(|c| hex(c.surface)),
                        "color": colors.map(|c| hex(c.content)),
                        "border-color": colors.map(|c| hex(c.border)),
                    }),
                );
            // A width that depends on text is the text system's, not the
            // design's.
            let width_counts = specimen.probe.text.is_empty();
            if let Some(w) = want
                .width
                .filter(|w| width_counts && (w - width).abs() > 0.5)
            {
                note("width", format!("{w}"), format!("{width}"));
            }
            if !specimen.probe.height_is_content_driven()
                && let Some(h) = want.height.filter(|h| (h - height).abs() > 0.5)
            {
                note("height", format!("{h}"), format!("{height}"));
            }

            if let Some(colors) = colors {
                let pairs = [
                    ("background", &want.background, hex(colors.surface)),
                    ("color", &want.color, hex(colors.content)),
                    ("border-color", &want.border_color, hex(colors.border)),
                ];
                for (field, expected, actual) in pairs {
                    if !specimen.probe.declares.contains(field) {
                        continue;
                    }
                    if let Some(expected) = expected.as_ref().filter(|e| !same_colour(e, &actual)) {
                        note(field, expected.clone(), actual);
                    }
                }
            }
        }
    }

    std::fs::write(
        format!("{}/tests/parity/measured.json", env!("CARGO_MANIFEST_DIR")),
        serde_json::to_string_pretty(&serde_json::json!({
            "platform": "gpui",
            "specimens": measured_all,
        }))
        .unwrap(),
    )
    .unwrap();

    if recorded.is_none() {
        let mut lines = vec![
            "# Checks that fail against the React reference today, one per line:".to_string(),
            "# a specimen (class and axes) and the field that differs.".to_string(),
            "# Recorded by the first test run; held both ways from now on, so it".to_string(),
            "# can only shrink. Run with `-- --nocapture` to see the values.".to_string(),
        ];
        lines.extend(failing.iter().cloned());
        lines.push(String::new());
        std::fs::write(&drift_path, lines.join("\n")).unwrap();
        println!(
            "recorded {} checks into tests/parity/drift.txt",
            failing.len()
        );
    }
    let drift = recorded.unwrap_or_else(|| failing.clone());

    let mut problems: Vec<String> = failing
        .difference(&drift)
        .map(|check| format!("DRIFT  {check} — {}", detail[check]))
        .chain(
            drift
                .difference(&failing)
                .map(|check| format!("STALE  {check} — passes now, remove it from drift.txt")),
        )
        .collect();
    problems.sort();

    println!(
        "parity: {} specimens, {} built, {} checks drifting, no adapter for: {}",
        specimens.len(),
        buildable.len(),
        failing.len(),
        unsupported.iter().cloned().collect::<Vec<_>>().join(" ")
    );
    for problem in &problems {
        println!("{problem}");
    }
    assert!(
        problems.is_empty(),
        "{} checks are out of step with drift.txt; see above",
        problems.len()
    );
}
