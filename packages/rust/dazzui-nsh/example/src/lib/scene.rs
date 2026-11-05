mod extended;
use super::state::{Context, print, truthy};
use dazzui_nsh::{CardVariant, SelectItem, SelectValue, SliderValue, prelude::*};
use nsh_core::element::AnyElement;
use serde::Deserialize;
use serde_json::Value;
use std::rc::Rc;

#[derive(Clone, Deserialize)]
pub struct Story {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub preview: Scene,
    pub examples: Vec<Scene>,
}
#[derive(Clone, Deserialize, serde::Serialize)]
pub struct Scene {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub note: Option<String>,
    pub content: Value,
    #[serde(default)]
    pub state: Value,
    #[serde(default)]
    pub gaps: Vec<String>,
}
impl Scene {
    pub fn render(&self) -> AnyElement {
        if !self.gaps.is_empty() {
            return view()
                .gap(px(8.0))
                .child(text("Not mapped yet"))
                .child(text(self.gaps.join("\n")).text_size(px(11.0)))
                .into_element();
        }
        scene_root()
            .key(self.id.clone())
            .node(self.content.to_string())
            .state(self.state.to_string())
            .scene_id(self.id.clone())
            .into_element()
    }
}
#[derive(Clone)]
struct Props {
    json: Rc<Value>,
    cx: Context,
}
impl Props {
    fn new(json: Value, cx: Context) -> Self {
        Self {
            json: Rc::new(json),
            cx,
        }
    }
    fn raw(&self, k: &str) -> Value {
        self.json[k].clone()
    }
    fn val(&self, k: &str) -> Value {
        self.cx.resolve(&self.json[k], &[])
    }
    fn bool(&self, k: &str) -> impl Fn() -> bool + 'static {
        let v = self.raw(k);
        let cx = self.cx;
        move || truthy(&cx.resolve(&v, &[]))
    }
    fn optional_bool(&self, k: &str) -> impl Fn() -> Option<bool> + 'static {
        let v = self.json.get(k).cloned();
        let cx = self.cx;
        move || v.as_ref().map(|v| truthy(&cx.resolve(v, &[])))
    }
    fn string(&self, k: &str) -> impl Fn() -> SharedString + 'static {
        let v = self.raw(k);
        let cx = self.cx;
        move || print(&cx.resolve(&v, &[])).into()
    }
    fn optional_string(&self, k: &str) -> impl Fn() -> Option<SharedString> + 'static {
        let v = self.json.get(k).cloned();
        let cx = self.cx;
        move || v.as_ref().map(|v| print(&cx.resolve(v, &[])).into())
    }
    fn number(&self, k: &str, default: f64) -> impl Fn() -> f64 + 'static {
        let v = self.raw(k);
        let cx = self.cx;
        move || cx.resolve(&v, &[]).as_f64().unwrap_or(default)
    }
    fn size(&self, default: Size) -> impl Fn() -> Size + 'static {
        let v = self.raw("size");
        let cx = self.cx;
        move || Size::parse(&print(&cx.resolve(&v, &[]))).unwrap_or(default)
    }
    fn tint(&self) -> impl Fn() -> Tint + 'static {
        let v = self.raw("tint");
        let cx = self.cx;
        move || Tint::parse(&print(&cx.resolve(&v, &[]))).unwrap_or_default()
    }
    fn variant(&self, default: Variant) -> impl Fn() -> Variant + 'static {
        let v = self.raw("variant");
        let cx = self.cx;
        move || Variant::parse(&print(&cx.resolve(&v, &[]))).unwrap_or(default)
    }
    fn action(&self, k: &str) -> impl Fn(Value) + 'static {
        let v = self.raw(k);
        let cx = self.cx;
        move |arg| cx.run(&v, &[arg])
    }
}
fn children(v: &Value, cx: Context, id: &str) -> Vec<AnyElement> {
    v.as_array()
        .into_iter()
        .flatten()
        .map(|v| render(v.clone(), cx, id))
        .collect()
}
#[derive(Clone, PartialEq, Props)]
struct SceneRootProps {
    node: Prop<SharedString>,
    state: Prop<SharedString>,
    scene_id: Prop<SharedString>,
}
#[component]
fn SceneRoot(p: SceneRootProps) -> impl IntoElement {
    let state: Value = serde_json::from_str(&p.state.get_untracked()).expect("scene state");
    provide_context(Context::new(&state));
    render(
        serde_json::from_str(&p.node.get_untracked()).expect("scene node"),
        use_context::<Context>().unwrap(),
        &p.scene_id.get_untracked(),
    )
}
#[derive(Clone, PartialEq, Props)]
struct RenderNodeProps {
    node: Prop<SharedString>,
    scene_id: Prop<SharedString>,
}
#[component]
fn RenderNode(p: RenderNodeProps) -> impl IntoElement {
    let v: Value = serde_json::from_str(&p.node.get_untracked()).expect("scene node");
    let cx = use_context::<Context>().expect("scene state context");
    render_inner(v, cx, &p.scene_id.get_untracked())
}
// Parse each node in its own mounted scope. Eagerly interpreting a complete
// JSON tree makes its controls siblings of their providers in NSH's owner tree.
pub fn render(v: Value, _cx: Context, id: &str) -> AnyElement {
    let address = v["$at"].as_u64().map(|i| format!("{id}/{i}"));
    let mut element = RenderNode::element(RenderNodeProps {
        node: v.to_string().into(),
        scene_id: id.to_owned().into(),
    });
    if let Some(address) = address {
        element = element.test_id(&address);
        if let Some(captured) = use_context::<crate::capture::CaptureEnabled>() {
            let reference = element_ref();
            element = element.element_ref(&reference);
            captured.register(address, reference);
        }
    }
    element.into_element()
}

#[derive(Clone, Copy)]
pub(super) struct InlineText;

fn render_inner(v: Value, cx: Context, id: &str) -> AnyElement {
    let theme = Theme::current();
    if v.is_null() || v.is_boolean() {
        return view().into_element();
    }
    if v.get("$if").is_some() {
        let id = id.to_owned();
        return dynamic(move || render(cx.resolve(&v, &[]), cx, &id));
    }
    if let Some(name) = v.get("$icon").and_then(Value::as_str) {
        return icon(name).into_element();
    }
    let Some(kind) = v.get("$el").and_then(Value::as_str) else {
        return text(move || print(&cx.resolve(&v, &[]))).into_element();
    };
    let p = Props::new(v["props"].clone(), cx);
    let kids = || children(&v["children"], cx, id);
    match kind {
        "Row" | "Col" | "Frame" => {
            let row = kind == "Row";
            let frame = kind == "Frame";
            let props = p.clone();
            view()
                .style_with(move |s| {
                    let t = theme.tokens();
                    let gap = props.val("gap");
                    let gap = gap.as_str().unwrap_or("3");
                    let mut s = s.gap(px(spacing(t, gap))).max_w(relative(1.0));
                    s = if row {
                        s.flex_row()
                            .items_center()
                            .style_when(props.val("justify").as_str() != Some("between"), |s| {
                                s.flex_wrap()
                            })
                    } else {
                        s.flex_col()
                    };
                    s = match props.val("align").as_str() {
                        Some("start") => s.items_start(),
                        Some("end") => s.items_end(),
                        Some("center") => s.items_center(),
                        Some("stretch") => s.items_stretch(),
                        _ => s,
                    };
                    s = match props.val("justify").as_str() {
                        Some("between") => s.justify_between(),
                        Some("center") => s.justify_center(),
                        Some("end") => s.justify_end(),
                        _ => s,
                    };
                    if frame {
                        s = s
                            .flex_row()
                            .gap(px(0.0))
                            .overflow_hidden()
                            .w(px(576.0))
                            .h(px(256.0))
                            .border(px(t.stroke_hairline))
                            .border_color(t.color_border.hsla())
                            .rounded(px(t.radius_large))
                            .bg(t.color_surface.hsla());
                    }
                    if let Some(value) = props
                        .val("width")
                        .as_str()
                        .and_then(|v| v.strip_suffix('%'))
                        .and_then(|v| v.parse::<f32>().ok())
                    {
                        s = s.w(relative(value / 100.0));
                    }
                    if let Some(w) = length(&props.val("width")) {
                        s = s.w(px(w)).max_w(relative(1.0));
                    }
                    if let Some(h) = length(&props.val("height")) {
                        s = s.h(px(h));
                    }
                    s
                })
                .child(kids())
                .into_element()
        }
        "Text" => {
            provide_context(InlineText);
            let strong = p.bool("strong");
            let tone = p.string("tone");
            let tint = p.tint();
            let tinted = p.json.get("tint").is_some();
            view()
                .flex_row()
                .items_center()
                .style_with(move |s| {
                    let t = theme.tokens();
                    let s = s.style_when(strong(), |s| s.font_weight(t.label_strong.font_weight()));
                    if tinted {
                        s.text_color(tint().ramp(t).shade(500).hsla())
                    } else if tone().as_ref() == "secondary" {
                        s.text_color(t.color_content_secondary.hsla())
                    } else {
                        s
                    }
                })
                .child(
                    v["children"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .map(|node| {
                            if let Some(value) = node.as_str() {
                                text(value.to_owned())
                                    .min_w(px(0.0))
                                    .max_w(relative(1.0))
                                    .flex_shrink(1.0)
                                    .into_element()
                            } else {
                                render(node.clone(), cx, id)
                            }
                        })
                        .collect::<Vec<_>>(),
                )
                .into_element()
        }
        "Icon" => {
            let name = p.raw("icon");
            let name = name["$icon"].as_str().expect("compiled Icon glyph");
            let mut el = icon(name);
            if let Some(size) = p.val("size").as_f64() {
                el = el.size(move || px(size as f32));
            } else if let Some(size) = p.val("size").as_str() {
                let size = size.to_owned();
                el = el.size(move || {
                    px(match size.as_str() {
                        "small" => theme.tokens().icon_small,
                        "large" => theme.tokens().icon_large,
                        _ => theme.tokens().icon_medium,
                    })
                });
            }
            el.into_element()
        }
        "Button" | "IconButton" => {
            let action = p.action("onClick");
            let icon_only = kind == "IconButton";
            let nodes = v["children"].as_array();
            let is_icon = |v: &Value| v["$el"].as_str() == Some("Icon");
            let leading_icon = !icon_only && nodes.and_then(|v| v.first()).is_some_and(is_icon);
            let trailing_icon = !icon_only && nodes.and_then(|v| v.last()).is_some_and(is_icon);
            let default_size = if icon_only { Size::Tiny } else { Size::Small };
            let size = p.size(default_size);
            let plain_label = nodes.is_some_and(|children| {
                children
                    .iter()
                    .all(|v| v.is_string() || v.get("$state").is_some())
            });
            let labels = v["children"].clone();
            button()
                .size(p.size(default_size))
                .icon_only(icon_only)
                .leading_icon(leading_icon)
                .trailing_icon(trailing_icon)
                .tint(if icon_only && p.json.get("tint").is_none() {
                    Tint::Neutral
                } else {
                    p.tint()()
                })
                .variant(p.variant(if icon_only {
                    Variant::Plain
                } else {
                    Variant::Recessed
                }))
                .disabled(p.bool("disabled"))
                .full_width(p.bool("fullWidth"))
                .action(match p.val("type").as_str() {
                    Some("submit") => dazzui_nsh::ButtonAction::Submit,
                    Some("reset") => dazzui_nsh::ButtonAction::Reset,
                    _ => dazzui_nsh::ButtonAction::Button,
                })
                .when_some(optional_slot(&p, "shortcut", cx, id), |el, slot| {
                    el.shortcut(slot)
                })
                .accessibility_label(p.optional_string("label"))
                .style_with(move |s| {
                    if icon_only {
                        s.w(px(size().profile(theme.tokens()).size)).p(px(0.0))
                    } else {
                        s
                    }
                })
                .on_press(move || action(Value::Null))
                .label(move || {
                    plain_label.then(|| {
                        labels
                            .as_array()
                            .into_iter()
                            .flatten()
                            .map(|v| print(&cx.resolve(v, &[])))
                            .collect::<String>()
                            .into()
                    })
                })
                .child(if plain_label { vec![] } else { kids() })
                .into_element()
        }
        "Badge" => badge()
            .raised(p.val("variant").as_str() == Some("raised"))
            .size(p.size(Size::Medium))
            .tint(p.tint())
            .variant(p.variant(Variant::Tinted))
            .child(kids())
            .into_element(),
        "Card" => {
            let variant = p.string("variant");
            card()
                .size(p.size(Size::Medium))
                .variant(move || match variant().as_ref() {
                    "raised" => CardVariant::Raised,
                    "outlined" => CardVariant::Outlined,
                    "tinted" => CardVariant::Tinted,
                    _ => CardVariant::Sunken,
                })
                .child(kids())
                .into_element()
        }
        "Checkbox" => {
            let action = p.action("onCheckedChange");
            checkbox()
                .size(p.size(Size::Medium))
                .tint(p.tint())
                .checked(p.optional_bool("checked"))
                .default_checked(p.bool("defaultChecked"))
                .indeterminate(p.bool("indeterminate"))
                .when_some(optional_slot(&p, "note", cx, id), |el, slot| el.note(slot))
                .disabled(p.bool("disabled"))
                .on_change(move |checked: &bool| action(Value::Bool(*checked)))
                .child(kids())
                .into_element()
        }
        "Switch" => {
            let action = p.action("onCheckedChange");
            switch()
                .size(p.size(Size::Medium))
                .tint(p.tint())
                .checked(p.optional_bool("checked"))
                .default_checked(p.bool("defaultChecked"))
                .disabled(p.bool("disabled"))
                .accessibility_label(p.optional_string("aria-label"))
                .on_change(move |checked: &bool| action(Value::Bool(*checked)))
                .into_element()
        }
        "Toggle" => {
            let action = p.action("onPressedChange");
            toggle()
                .size(p.size(Size::Tiny))
                .icon_only(v["children"].as_array().is_some_and(|items| {
                    items.len() == 1
                        && (items[0]["$el"] == "Icon" || items[0].get("$icon").is_some())
                }))
                .tint(if p.json.get("tint").is_none() {
                    Tint::Neutral
                } else {
                    p.tint()()
                })
                .variant(p.variant(Variant::Plain))
                .pressed(p.optional_bool("pressed"))
                .default_pressed(p.bool("defaultPressed"))
                .disabled(p.bool("disabled"))
                .accessibility_label(p.optional_string("label"))
                .on_change(move |pressed: &bool| action(Value::Bool(*pressed)))
                .child(kids())
                .into_element()
        }
        "TextField" => {
            let action = p.action("onValueChange");
            let state = p.string("state");
            text_field()
                .size(p.size(Size::Medium))
                .tint(p.tint())
                .value(p.optional_string("value"))
                .default_value(p.string("defaultValue"))
                .placeholder(p.string("placeholder"))
                .disabled(p.bool("disabled"))
                .read_only(p.bool("readOnly"))
                .mono(p.bool("mono"))
                .multiline(p.bool("multiline"))
                .rows(p.number("rows", 3.0)() as u32)
                .masked(p.val("type").as_str() == Some("password"))
                .email(p.val("type").as_str() == Some("email"))
                .required(p.bool("required"))
                .error(move || state().as_ref() == "error")
                .accessibility_label(p.optional_string("aria-label"))
                .on_change(move |s: &SharedString| action(Value::String(s.to_string())))
                .into_element()
        }
        "Select" => {
            let action = p.action("onValueChange");
            let value = p.optional_string("value");
            let controlled = p.json.get("value").is_some();
            let items = extended::select_items(&p.val("options"));
            select()
                .size(p.size(Size::Medium))
                .tint(p.tint())
                .items(items)
                .value(move || {
                    if controlled {
                        SelectValue::Controlled(value())
                    } else {
                        SelectValue::Uncontrolled
                    }
                })
                .default_value(p.optional_string("defaultValue"))
                .placeholder(p.string("placeholder"))
                .disabled(p.bool("disabled"))
                .accessibility_label(p.optional_string("aria-label"))
                .on_change(move |s: &Option<SharedString>| {
                    action(
                        s.as_ref()
                            .map(|s| Value::String(s.to_string()))
                            .unwrap_or(Value::Null),
                    )
                })
                .into_element()
        }
        "Slider" => {
            let action = p.action("onValueChange");
            let own = p.clone();
            let val = p.clone();
            slider()
                .show_value(p.bool("showValue"))
                .size(p.size(Size::Medium))
                .tint(p.tint())
                .default_value(move || slider_value(&own.val("defaultValue")))
                .value(move || {
                    val.json
                        .get("value")
                        .map(|_| slider_value(&val.val("value")))
                })
                .min(p.number("min", 0.0))
                .max(p.number("max", 100.0))
                .step(p.number("step", 1.0))
                .disabled(p.bool("disabled"))
                .accessibility_label(p.optional_string("aria-label"))
                .on_change(move |v: &SliderValue| {
                    action(match v {
                        SliderValue::Single(n) => serde_json::json!(n),
                        SliderValue::Range(a, b) => serde_json::json!([a, b]),
                    })
                })
                .into_element()
        }
        "Progress" => {
            let val = p.number("value", 0.0);
            progress()
                .size(p.size(Size::Small))
                .tint(p.tint())
                .value(move || val() as f32)
                .indeterminate(p.raw("value").is_null())
                .gradient(p.bool("gradient"))
                .accessibility_label(p.optional_string("aria-label"))
                .into_element()
        }
        "KeyCap" => {
            let labels = v["children"].clone();
            let plain = labels.as_array().is_some_and(|children| {
                children
                    .iter()
                    .all(|v| v.is_string() || v.get("$state").is_some())
            });
            let variant = p.string("variant");
            key_cap()
                .size(p.size(Size::Small))
                .variant(move || match variant().as_ref() {
                    "key" => dazzui_nsh::KeyCapVariant::Key,
                    "strong" => dazzui_nsh::KeyCapVariant::Strong,
                    _ => dazzui_nsh::KeyCapVariant::Hint,
                })
                .value(move || {
                    plain.then(|| {
                        labels
                            .as_array()
                            .into_iter()
                            .flatten()
                            .map(|v| print(&cx.resolve(v, &[])))
                            .collect::<String>()
                            .into()
                    })
                })
                .child(if plain { vec![] } else { kids() })
                .into_element()
        }
        "SectionLabel" => {
            let tint = p.clone();
            section_label()
                .faint(p.bool("faint"))
                .tint(move || {
                    tint.json
                        .get("tint")
                        .and_then(|_| Tint::parse(&print(&tint.val("tint"))))
                })
                .child(kids())
                .into_element()
        }
        "EmptyState" => empty_state()
            .title(p.string("title"))
            .when_some(optional_slot(&p, "action", cx, id), |el, slot| {
                el.action(slot)
            })
            .child(kids())
            .into_element(),
        "Avatar" => {
            let shape = p.string("shape");
            let mut el = avatar()
                .name(p.string("name"))
                .size(p.size(Size::Medium))
                .tint(p.tint())
                .square(move || shape().as_ref() == "square");
            if !p.raw("fallback").is_null() {
                el = el.fallback(render(p.raw("fallback"), cx, id));
            }
            if let Some(src) = p.val("src").as_str() {
                el = el.src(nsh_core::element::ImageSrc::from(src.to_owned()));
            }
            el.into_element()
        }
        "Callout" => callout()
            .title(p.string("title"))
            .size(p.size(Size::Medium))
            .tint(p.tint())
            .when_some(optional_slot(&p, "icon", cx, id), |el, slot| el.icon(slot))
            .when_some(optional_slot(&p, "action", cx, id), |el, slot| {
                el.action(slot)
            })
            .child(kids())
            .into_element(),
        "Collapsible" => {
            let action = p.action("onOpenChange");
            collapsible()
                .title(p.string("title"))
                .when_some(optional_slot(&p, "meta", cx, id), |el, slot| el.meta(slot))
                .open(p.optional_bool("open"))
                .default_open(p.bool("defaultOpen"))
                .on_change(move |v: &bool| action(Value::Bool(*v)))
                .child(kids())
                .into_element()
        }
        "RadioGroup" | "SegmentedControl" => {
            let key = if kind == "RadioGroup" {
                "options"
            } else {
                "items"
            };
            let action = p.action("onValueChange");
            let items = extended::select_items(&p.val(key));
            if kind == "RadioGroup" {
                radio_group()
                    .options(items)
                    .value(p.optional_string("value"))
                    .default_value(p.optional_string("defaultValue"))
                    .size(p.size(Size::Medium))
                    .tint(p.tint())
                    .accessibility_label(p.optional_string("aria-label"))
                    .on_change(move |v: &SharedString| action(Value::String(v.to_string())))
                    .into_element()
            } else {
                segmented_control()
                    .items(items)
                    .value(p.optional_string("value"))
                    .default_value(p.optional_string("defaultValue"))
                    .size(p.size(Size::Medium))
                    .tint(p.tint())
                    .stretch(p.bool("stretch"))
                    .icon_only(p.bool("iconOnly"))
                    .leading_icon(
                        p.val("items")
                            .as_array()
                            .is_some_and(|items| items.iter().any(|i| i.get("icon").is_some())),
                    )
                    .active_style(if p.val("activeStyle").as_str() == Some("filled") {
                        dazzui_nsh::SegmentStyle::Filled
                    } else {
                        dazzui_nsh::SegmentStyle::Raised
                    })
                    .when_some(
                        p.val("items")
                            .as_array()
                            .is_some_and(|items| items.iter().any(|i| i.get("icon").is_some()))
                            .then_some(()),
                        |el, _| {
                            el.render_icon(extended::choice_icon_renderer(p.raw("items"), cx, id))
                        },
                    )
                    .accessibility_label(p.optional_string("aria-label"))
                    .on_change(move |v: &SharedString| action(Value::String(v.to_string())))
                    .into_element()
            }
        }
        "ActionBar" => {
            let align = p.string("align");
            action_bar()
                .align(move || match align().as_ref() {
                    "end" => dazzui_nsh::ActionAlign::End,
                    "between" => dazzui_nsh::ActionAlign::Between,
                    _ => dazzui_nsh::ActionAlign::Start,
                })
                .child(kids())
                .into_element()
        }
        "FormField" => {
            let state = p.string("state");
            form_field()
                .label(p.string("label"))
                .hint(p.string("hint"))
                .tint(p.tint())
                .error(move || state().as_ref() == "error")
                .child(kids())
                .into_element()
        }
        "Preferences" => preferences().child(kids()).into_element(),
        "PreferenceGroup" => preference_group()
            .title(p.string("title"))
            .description(p.optional_string("description"))
            .when_some(optional_slot(&p, "action", cx, id), |el, slot| {
                el.action(slot)
            })
            .child(kids())
            .into_element(),
        "PreferenceSection" => preference_section()
            .label(p.optional_string("label"))
            .footer(p.optional_string("footer"))
            .when_some(optional_slot(&p, "action", cx, id), |el, slot| {
                el.action(slot)
            })
            .child(kids())
            .into_element(),
        "PreferenceRow" => {
            let action = p.action("onOpen");
            preference_row()
                .title(p.string("title"))
                .subtitle(p.optional_string("subtitle"))
                .when_some(optional_slot(&p, "icon", cx, id), |el, slot| el.icon(slot))
                .interactive(p.json.get("onOpen").is_some())
                .tint(p.tint())
                .on_open(move || action(Value::Null))
                .child(kids())
                .into_element()
        }
        "Menu" => {
            let items = extended::menu_items(&p.raw("items"), cx, id);
            let trigger = p.raw("trigger");
            menu()
                .trigger(render(trigger, cx, id))
                .child(items)
                .into_element()
        }
        "Divider" => {
            let orientation = p.string("orientation");
            divider()
                .vertical(move || orientation().as_ref() == "vertical")
                .tone(match p.val("tone").as_str() {
                    Some("muted") => dazzui_nsh::DividerTone::Muted,
                    Some("strong") => dazzui_nsh::DividerTone::Strong,
                    _ => dazzui_nsh::DividerTone::Normal,
                })
                .into_element()
        }
        _ => extended::extend(kind, &p, &v, cx, id)
            .unwrap_or_else(|| panic!("supported scene {id} has unmapped element {kind}")),
    }
}

fn slider_value(v: &Value) -> SliderValue {
    if let Some(a) = v.as_array() {
        SliderValue::Range(a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.0))
    } else {
        SliderValue::Single(v.as_f64().unwrap_or(0.0))
    }
}
fn length(v: &Value) -> Option<f32> {
    v.as_f64().map(|n| n as f32).or_else(|| {
        let s = v.as_str()?;
        if let Some(n) = s.strip_suffix("rem") {
            n.parse::<f32>().ok().map(|n| n * 16.0)
        } else {
            s.trim_end_matches("px").parse().ok()
        }
    })
}
fn spacing(t: &dazzui_nsh::ThemeTokens, n: &str) -> f32 {
    match n {
        "0" => t.spacing_0,
        "0.5" => t.spacing_0_5,
        "1" => t.spacing_1,
        "1.5" => t.spacing_1_5,
        "2" => t.spacing_2,
        "2.5" => t.spacing_2_5,
        "3" => t.spacing_3,
        "4" => t.spacing_4,
        "5" => t.spacing_5,
        "6" => t.spacing_6,
        "8" => t.spacing_8,
        _ => panic!("unmapped stage spacing {n}"),
    }
}

fn optional_slot(p: &Props, name: &str, cx: Context, id: &str) -> Option<AnyElement> {
    let value = p.raw(name);
    if value.is_null() {
        None
    } else {
        Some(render(value, cx, id))
    }
}
