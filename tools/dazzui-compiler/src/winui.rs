//! Lowers a page to WinUI 3, in XAML and C#.
//!
//! A page becomes two files: the XAML, and its code-behind — a `State` to
//! show, a `Model` whose methods the events call, and one handler per
//! event. The model is held only as the interface of the methods the page
//! calls, so a host may hand it the Rust model or one of its own. A Rust
//! model becomes that interface and a store, thin over the UniFFI object
//! whose C# uniffi-bindgen-cs writes beside it. DazzUI controls come from
//! `xmlns:dazz="using:DazzUI"`.
//!
//! State is read with `{x:Bind …, Mode=OneWay}` and never `TwoWay`: a
//! change goes to the model, and the value comes back in the next
//! snapshot, so the page's copy of the state is never changed on its own.
//!
//! Row, Col and Text are drawn as the WinUI example draws the React
//! example's stage components. A spacing step and an ink are the theme's,
//! not XAML resources, so the code-behind sets them as each element loads,
//! which works inside a `DataTemplate` too.
//!
//! An `if` sets `Visibility` on each node of its branches rather than
//! `x:Load`, which works inside a `DataTemplate` too and needs no `x:Name`.
//! A `for` is an `ItemsRepeater` laid out as the stack around it is, so its
//! elements sit in that stack as SwiftUI's `ForEach` puts them.

use crate::Registry;
use crate::ir::{
    Argument, Component, ComponentSpec, Condition, Expr, GeneratedFile, MethodCall, Source, UiNode,
    Value, model_stem, pascal_case,
};
use crate::{CompiledModel, CompiledPage, Options};

pub fn generate(compiled: &CompiledPage, options: &Options) -> Vec<GeneratedFile> {
    let page = &compiled.page;
    let mut cx = Cx {
        page: page.name.clone(),
        registry: compiled.registry.clone(),
        handlers: Vec::new(),
        usings: Vec::new(),
        looks: Vec::new(),
        negates: false,
        visibilities: Vec::new(),
        conditions: Vec::new(),
        element: None,
        align: None,
    };
    let xaml = xaml(compiled, &options.namespace, &mut cx);
    vec![
        GeneratedFile::new(format!("{}.xaml", page.name), xaml),
        GeneratedFile::new(
            format!("{}.xaml.cs", page.name),
            code_behind(compiled, &options.namespace, &cx),
        ),
    ]
}

/// What the XAML needs from its code-behind, and where the writer is.
struct Cx {
    page: String,
    registry: Registry,
    handlers: Vec<Handler>,
    /// Namespaces the handlers' code needs.
    usings: Vec<String>,
    /// What the code-behind sets on an element as it loads, each with the
    /// handler that does.
    looks: Vec<Look>,
    /// Whether `Not(…)` is bound to.
    negates: bool,
    /// The negations of each visibility function, `Visible1(a, b)`, by
    /// index: `[false, true]` is shown when `a && !b`.
    visibilities: Vec<Vec<bool>>,
    /// The `if`s around the node being written, innermost last.
    conditions: Vec<Condition>,
    /// Inside a `for`: the struct its elements are, which paths are
    /// relative to.
    element: Option<String>,
    /// The alignment a Row or Col gives the child being written.
    align: Option<String>,
}

/// The stack a node is written into: a `for` lays its elements out as it.
#[derive(Clone)]
struct Stack {
    horizontal: bool,
    gap: String,
}

/// An event handler in the code-behind: its name, the event args it takes,
/// and the call it makes.
struct Handler {
    name: String,
    args: String,
    call: String,
}

/// What a `Loaded` handler sets from the theme.
#[derive(Clone, PartialEq)]
enum Look {
    /// A StackPanel's spacing.
    Gap(String),
    /// An ItemsRepeater's StackLayout's spacing.
    RepeaterGap(String),
    /// A Text's ink and weight.
    Text {
        tint: Option<String>,
        secondary: bool,
        strong: bool,
    },
}

/// `Loaded="OnGap3Loaded"`: the handler that sets `look`, written once.
fn loaded(cx: &mut Cx, look: Look) -> String {
    let index = match cx.looks.iter().position(|l| *l == look) {
        Some(i) => i,
        None => {
            cx.looks.push(look);
            cx.looks.len() - 1
        }
    };
    format!(" Loaded=\"{}\"", look_handler(&cx.looks, index))
}

fn look_handler(looks: &[Look], index: usize) -> String {
    let step = |s: &str| match s {
        "px" => "Px".to_string(),
        s => s.replace('-', "_"),
    };
    match &looks[index] {
        Look::Gap(s) => format!("OnGap{}Loaded", step(s)),
        Look::RepeaterGap(s) => format!("OnRepeaterGap{}Loaded", step(s)),
        Look::Text { .. } => {
            let n = looks[..=index]
                .iter()
                .filter(|l| matches!(l, Look::Text { .. }))
                .count();
            format!("OnText{n}Loaded")
        }
    }
}

fn xaml(compiled: &CompiledPage, namespace: &str, cx: &mut Cx) -> String {
    let page = &compiled.page;
    let mut w = Writer::default();
    w.line(&format!(
        "<!-- AUTO-GENERATED FILE. DO NOT EDIT. Source: {} -->",
        compiled.file
    ));
    w.line("<Page");
    w.indent(|w| {
        w.line(&format!("x:Class=\"{namespace}.{}\"", page.name));
        w.line("xmlns=\"http://schemas.microsoft.com/winfx/2006/xaml/presentation\"");
        w.line("xmlns:x=\"http://schemas.microsoft.com/winfx/2006/xaml\"");
        w.line(&format!("xmlns:local=\"using:{namespace}\""));
        w.line("xmlns:dazz=\"using:DazzUI\">");
        let column = Stack {
            horizontal: false,
            gap: "0".into(),
        };
        node(w, &page.root, &column, cx);
    });
    w.line("</Page>");
    w.finish()
}

fn node(w: &mut Writer, node: &UiNode, stack: &Stack, cx: &mut Cx) {
    let c = match node {
        UiNode::Component(c) => c,
        UiNode::If {
            condition,
            then_branch,
            else_branch,
            source,
        } => {
            w.line(&format!("<!-- source: {} (if) -->", source.short()));
            let align = cx.align.clone();
            cx.conditions.push(condition.clone());
            for n in then_branch {
                cx.align = align.clone();
                self::node(w, n, stack, cx);
            }
            cx.conditions.pop();
            if !else_branch.is_empty() {
                w.line(&format!("<!-- source: {} (else) -->", source.short()));
                cx.conditions.push(Condition {
                    negated: !condition.negated,
                    value: condition.value.clone(),
                });
                for n in else_branch {
                    cx.align = align.clone();
                    self::node(w, n, stack, cx);
                }
                cx.conditions.pop();
            }
            return;
        }
        UiNode::ForEach {
            collection,
            element,
            children,
            source,
            ..
        } => {
            w.line(&format!("<!-- source: {} -->", source.short()));
            let orientation = if stack.horizontal {
                "Horizontal"
            } else {
                "Vertical"
            };
            let align = cx.align.take().unwrap_or_default();
            let gap = loaded(cx, Look::RepeaterGap(stack.gap.clone()));
            w.line(&format!(
                "<ItemsRepeater ItemsSource={}{gap}{align}{}>",
                bind(cx, &Expr::State(collection.clone())),
                visibility(cx)
            ));
            w.indent(|w| {
                w.line("<ItemsRepeater.Layout>");
                w.indent(|w| w.line(&format!("<StackLayout Orientation=\"{orientation}\"/>")));
                w.line("</ItemsRepeater.Layout>");
                w.line("<ItemsRepeater.ItemTemplate>");
                w.indent(|w| {
                    w.line(&format!("<DataTemplate x:DataType=\"local:{element}\">"));
                    // Paths inside are the element's, and the conditions
                    // around the loop are on the repeater already.
                    let conditions = std::mem::take(&mut cx.conditions);
                    cx.element = Some(element.clone());
                    w.indent(|w| {
                        // A template has one root.
                        if let [only @ UiNode::Component(_)] = children.as_slice() {
                            self::node(w, only, stack, cx);
                        } else {
                            let gap = loaded(cx, Look::Gap(stack.gap.clone()));
                            w.line(&format!("<StackPanel Orientation=\"{orientation}\"{gap}>"));
                            w.indent(|w| children.iter().for_each(|n| self::node(w, n, stack, cx)));
                            w.line("</StackPanel>");
                        }
                    });
                    cx.element = None;
                    cx.conditions = conditions;
                    w.line("</DataTemplate>");
                });
                w.line("</ItemsRepeater.ItemTemplate>");
            });
            w.line("</ItemsRepeater>");
            return;
        }
    };
    w.line(&format!("<!-- source: {} -->", c.source.short()));
    let attrs = cx.align.take().unwrap_or_default() + &visibility(cx);
    // A mapping wins, so that an app's registry can replace even the
    // components this backend lowers itself.
    if let Some(spec) = cx
        .registry
        .get(c.id.as_str())
        .filter(|s| s.winui.is_some())
        .cloned()
    {
        return mapped(w, c, &spec, &attrs, cx);
    }
    match c.id.as_str() {
        kind @ ("Row" | "Col") => {
            let horizontal = kind == "Row";
            let gap = match c.prop("gap") {
                Some(Expr::Literal(Value::String(step))) => step.clone(),
                other => unreachable!("`gap` on `{kind}` is {other:?}"),
            };
            let inner = Stack { horizontal, gap };
            // A flex row centres its items across, and a column stretches
            // them; a StackPanel stretches both ways.
            let align = match (horizontal, case(c.prop("align"))) {
                (true, "Start") => Some(" VerticalAlignment=\"Top\""),
                (true, "Center") => Some(" VerticalAlignment=\"Center\""),
                (true, "End") => Some(" VerticalAlignment=\"Bottom\""),
                (false, "Start") => Some(" HorizontalAlignment=\"Left\""),
                (false, "Center") => Some(" HorizontalAlignment=\"Center\""),
                (false, "End") => Some(" HorizontalAlignment=\"Right\""),
                _ => None,
            };
            let orientation = if horizontal {
                " Orientation=\"Horizontal\""
            } else {
                ""
            };
            let gap = loaded(cx, Look::Gap(inner.gap.clone()));
            w.line(&format!("<StackPanel{orientation}{gap}{attrs}>"));
            // The conditions are on the panel; its children need not repeat them.
            let conditions = std::mem::take(&mut cx.conditions);
            w.indent(|w| {
                for child in &c.children {
                    cx.align = align.map(str::to_owned);
                    self::node(w, child, &inner, cx);
                }
            });
            cx.align = None;
            cx.conditions = conditions;
            w.line("</StackPanel>");
        }
        "Text" => {
            let tint = c.prop("tint").map(|t| case(Some(t)).to_lowercase());
            let look = Look::Text {
                secondary: case(c.prop("tone")) == "Secondary",
                strong: c.prop("strong") == Some(&Expr::Literal(Value::Bool(true))),
                tint,
            };
            let styled = if look
                == (Look::Text {
                    tint: None,
                    secondary: false,
                    strong: false,
                }) {
                String::new()
            } else {
                loaded(cx, look)
            };
            let tag = format!("<TextBlock TextWrapping=\"Wrap\"{styled}{attrs}");
            text_block(w, tag, &c.text, cx);
        }
        other => unreachable!("`{other}` passed the registry but has no WinUI lowering"),
    }
}

/// A TextBlock of `runs`: one in `Text`, several as `Run`s.
fn text_block(w: &mut Writer, tag: String, runs: &[Expr], cx: &Cx) {
    match runs {
        [one] => w.line(&format!("{tag} Text={}/>", value(cx, one))),
        runs => {
            w.line(&format!("{tag}>"));
            w.indent(|w| {
                for run in runs {
                    w.line(&format!("<Run Text={}/>", value(cx, run)));
                }
            });
            w.line("</TextBlock>");
        }
    }
}

/// ` Visibility=…` for the `if`s around the node, if any.
fn visibility(cx: &mut Cx) -> String {
    if cx.conditions.is_empty() {
        return String::new();
    }
    let negations: Vec<bool> = cx.conditions.iter().map(|c| c.negated).collect();
    let index = match cx.visibilities.iter().position(|v| *v == negations) {
        Some(i) => i,
        None => {
            cx.visibilities.push(negations);
            cx.visibilities.len() - 1
        }
    };
    let args: Vec<String> = cx.conditions.iter().map(|c| path(cx, &c.value)).collect();
    format!(
        " Visibility=\"{{x:Bind local:{}.Visible{}({}){}}}\"",
        cx.page,
        index + 1,
        args.join(", "),
        mode(cx)
    )
}

/// A component its registry mapping lowers: an element, its attributes and
/// events, and its slots and children in the properties the mapping names.
fn mapped(w: &mut Writer, c: &Component, spec: &ComponentSpec, attrs: &str, cx: &mut Cx) {
    let m = spec.winui.as_ref().expect("a WinUI mapping");
    let mut tag = format!("<{}", m.element);
    for attr in &m.attrs {
        if let Source::Prop {
            prop,
            cases,
            invert,
            omit,
        } = &attr.source
            && let Some(e) = c.prop(prop).filter(|e| !e.is_omitted(omit.as_ref()))
        {
            tag += &format!(" {}={}", attr.attr, attr_value(cx, e, cases, *invert));
        }
    }
    // A plain Text child — one run, nothing set on it — is an attribute,
    // `Content="Save"`; any other child goes in the children's property.
    // A component's own text, as an app's replacement for `Text` takes it,
    // is an attribute too.
    let mut children: &[UiNode] = &c.children;
    if let Some(text_attr) = m.children.as_ref().and_then(|ch| ch.text.as_deref()) {
        if let [UiNode::Component(child)] = children
            && plain_text(child)
        {
            tag += &format!(" {text_attr}={}", value(cx, &child.text[0]));
            children = &[];
        } else if let [run] = c.text.as_slice() {
            tag += &format!(" {text_attr}={}", value(cx, run));
        }
    }
    for event in &m.events {
        if let Some(call) = c.event(&event.name) {
            if let Some(using) = &event.using
                && !cx.usings.contains(using)
            {
                cx.usings.push(using.clone());
            }
            let name = handler(cx, call, &event.event, &event.args, event.value.as_deref());
            tag += &format!(" {}=\"{name}\"", event.event);
        }
    }
    tag += attrs;

    let slots: Vec<(&str, &UiNode)> = m
        .slots
        .iter()
        .filter_map(|(slot, property)| Some((property.as_str(), c.slot(slot)?)))
        .collect();
    if slots.is_empty() && children.is_empty() {
        return w.line(&(tag + "/>"));
    }
    w.line(&(tag + ">"));
    // The conditions are on the element; what it holds need not repeat them.
    let conditions = std::mem::take(&mut cx.conditions);
    let column = Stack {
        horizontal: false,
        gap: "0".into(),
    };
    w.indent(|w| {
        for (property, node) in slots {
            w.line(&format!("<{}.{property}>", m.element));
            w.indent(|w| self::node(w, node, &column, cx));
            w.line(&format!("</{}.{property}>", m.element));
        }
        if !children.is_empty() {
            let children = |w: &mut Writer, cx: &mut Cx| {
                children
                    .iter()
                    .for_each(|n| self::node(w, n, &column, cx))
            };
            let wrapped = |w: &mut Writer, cx: &mut Cx| match m
                .children
                .as_ref()
                .and_then(|ch| ch.wrap.as_ref())
            {
                Some((wrap, property)) => {
                    w.line(&format!("<{wrap}>"));
                    w.indent(|w| match property {
                        Some(p) => {
                            w.line(&format!("<{wrap}.{p}>"));
                            w.indent(|w| children(w, cx));
                            w.line(&format!("</{wrap}.{p}>"));
                        }
                        None => children(w, cx),
                    });
                    w.line(&format!("</{wrap}>"));
                }
                None => children(w, cx),
            };
            match m.children.as_ref().and_then(|ch| ch.property.as_ref()) {
                Some(p) => {
                    w.line(&format!("<{}.{p}>", m.element));
                    w.indent(|w| wrapped(w, cx));
                    w.line(&format!("</{}.{p}>", m.element));
                }
                None => wrapped(w, cx),
            }
        }
    });
    cx.conditions = conditions;
    w.line(&format!("</{}>", m.element));
}

/// A `Text` with one run and its default look, which an attribute can hold.
fn plain_text(c: &Component) -> bool {
    c.id.as_str() == "Text"
        && c.text.len() == 1
        && c.prop("tint").is_none()
        && c.prop("strong") != Some(&Expr::Literal(Value::Bool(true)))
        && !matches!(c.prop("tone"), Some(Expr::EnumCase { case, .. }) if case == "Secondary")
}

/// A quoted attribute value for `e`: a case's platform code, a flipped
/// literal, `Not(…)` of a path, or `e` itself.
fn attr_value(cx: &mut Cx, e: &Expr, cases: &[(String, String)], invert: bool) -> String {
    match e {
        Expr::EnumCase { ty, case } => {
            let code = cases
                .iter()
                .find(|(c, _)| c == case)
                .map(|(_, code)| code.clone())
                .unwrap_or_else(|| panic!("the registry maps no WinUI code for `{ty}::{case}`"));
            attr(&code)
        }
        Expr::Literal(Value::Bool(b)) if invert => value(cx, &Expr::Literal(Value::Bool(!b))),
        e @ (Expr::State(_) | Expr::Local(_)) if invert => {
            cx.negates = true;
            format!(
                "\"{{x:Bind local:{}.Not({}){}}}\"",
                cx.page,
                path(cx, e),
                mode(cx)
            )
        }
        other => value(cx, other),
    }
}

/// The handler for `call` on `event`, `OnSaveClick`: one per distinct call,
/// however many controls make it.
fn handler(
    cx: &mut Cx,
    call: &MethodCall,
    event: &str,
    args: &str,
    event_value: Option<&str>,
) -> String {
    let argument = match &call.argument {
        Argument::None => String::new(),
        Argument::EventValue => event_value.expect("an event with a value").to_string(),
        Argument::Expr(e) => cs_expr(cx, e),
    };
    let code = format!("Model?.{}({argument})", pascal_case(&call.method));
    if let Some(h) = cx
        .handlers
        .iter()
        .find(|h| h.call == code && h.args == args)
    {
        return h.name.clone();
    }
    let base = format!("On{}{event}", pascal_case(&call.method));
    let mut name = base.clone();
    let mut n = 1;
    while cx.handlers.iter().any(|h| h.name == name) {
        n += 1;
        name = format!("{base}{n}");
    }
    cx.handlers.push(Handler {
        name: name.clone(),
        args: args.into(),
        call: code,
    });
    name
}

/// The case an enum prop holds; `""` for none.
fn case(expr: Option<&Expr>) -> &str {
    match expr {
        Some(Expr::EnumCase { case, .. }) => case,
        None => "",
        Some(other) => unreachable!("expected an enum case, found {other:?}"),
    }
}

/// A quoted attribute value: a literal, or a binding.
fn value(cx: &Cx, e: &Expr) -> String {
    match e {
        Expr::Literal(Value::String(s)) => attr(s),
        Expr::Literal(Value::Bool(b)) => format!("\"{}\"", if *b { "True" } else { "False" }),
        Expr::Literal(Value::Int(n)) => format!("\"{n}\""),
        Expr::State(_) | Expr::Local(_) => bind(cx, e),
        Expr::EnumCase { ty, case } => {
            unreachable!("`{ty}::{case}` is lowered by the prop that takes it")
        }
    }
}

/// `"{x:Bind State.Flag, Mode=OneWay}"`, or `"{x:Bind Name}"`
/// inside a template, whose elements are replaced rather than changed.
fn bind(cx: &Cx, e: &Expr) -> String {
    format!("\"{{x:Bind {}{}}}\"", path(cx, e), mode(cx))
}

fn mode(cx: &Cx) -> &'static str {
    if cx.element.is_some() {
        ""
    } else {
        ", Mode=OneWay"
    }
}

/// The x:Bind path of a state or element value: `State.Flag`,
/// or `Owner.Name` inside a template.
fn path(cx: &Cx, e: &Expr) -> String {
    let pascal = |segments: &[String]| {
        segments
            .iter()
            .map(|s| pascal_case(s))
            .collect::<Vec<_>>()
            .join(".")
    };
    match e {
        Expr::State(p) => format!("State.{}", pascal(&p.segments)),
        Expr::Local(p) => {
            debug_assert!(cx.element.is_some(), "a loop item outside its loop");
            pascal(&p.segments)
        }
        other => unreachable!("not a path: {other:?}"),
    }
}

/// A C# expression for `e`, in a handler.
fn cs_expr(cx: &Cx, e: &Expr) -> String {
    match e {
        Expr::Literal(Value::String(s)) => cs_string(s),
        Expr::Literal(Value::Bool(b)) => b.to_string(),
        Expr::Literal(Value::Int(n)) => n.to_string(),
        Expr::State(_) => path(cx, e),
        // An ItemsRepeater does not set a DataContext; the element a
        // control was made for is found from the repeater it sits in.
        Expr::Local(_) => {
            let element = cx.element.as_deref().expect("a loop item inside its loop");
            format!("ItemOf<{element}>(sender).{}", path(cx, e))
        }
        Expr::EnumCase { ty, case } => {
            unreachable!("`{ty}::{case}` is lowered by the prop that takes it")
        }
    }
}

/// A quoted XAML attribute value holding `text` literally.
fn attr(text: &str) -> String {
    let mut out = String::from("\"");
    // A leading `{` would open a markup extension; `{}` escapes it.
    if text.starts_with('{') {
        out += "{}";
    }
    for ch in text.chars() {
        match ch {
            '&' => out += "&amp;",
            '<' => out += "&lt;",
            '>' => out += "&gt;",
            '"' => out += "&quot;",
            '\n' => out += "&#10;",
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn cs_string(s: &str) -> String {
    let mut out = String::from("\"");
    for ch in s.chars() {
        match ch {
            '\\' => out += "\\\\",
            '"' => out += "\\\"",
            '\n' => out += "\\n",
            '\r' => out += "\\r",
            '\t' => out += "\\t",
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn header(source: &str) -> String {
    format!("// AUTO-GENERATED FILE. DO NOT EDIT. Source: {source}\n")
}

fn code_behind(compiled: &CompiledPage, namespace: &str, cx: &Cx) -> String {
    let page = &compiled.page;
    let state = &page.state_type;
    let actions = format!("I{}Actions", model_stem(&page.model_type));
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    if !cx.looks.is_empty() {
        w.line("using DazzUI;");
    }
    let items = cx.handlers.iter().any(|h| h.call.contains("ItemOf<"));
    if items {
        w.line("using System;");
    }
    w.line("using Microsoft.UI.Xaml;");
    w.line("using Microsoft.UI.Xaml.Controls;");
    if items {
        w.line("using Microsoft.UI.Xaml.Media;");
    }
    for using in &cx.usings {
        w.line(&format!("using {using};"));
    }
    w.line("");
    w.line(&format!("namespace {namespace};"));
    w.line("");
    w.line(&format!("public sealed partial class {} : Page", page.name));
    w.line("{");
    w.indent(|w| {
        w.line(&format!(
            "public static readonly DependencyProperty StateProperty = DependencyProperty.Register(nameof(State), typeof({state}), typeof({}), new PropertyMetadata(null));",
            page.name
        ));
        w.line("");
        w.line("/// <summary>What the page shows: set a new snapshot to show it.</summary>");
        w.line(&format!("public {state} State"));
        w.line("{");
        w.indent(|w| {
            w.line(&format!("get => ({state})GetValue(StateProperty);"));
            w.line("set => SetValue(StateProperty, value);");
        });
        w.line("}");
        w.line("");
        w.line("/// <summary>Whose methods the page's events call.</summary>");
        w.line(&format!("public {actions}? Model {{ get; set; }}"));
        w.line("");
        w.line(&format!("public {}()", page.name));
        w.line("{");
        w.indent(|w| w.line("InitializeComponent();"));
        w.line("}");
        for h in &cx.handlers {
            w.line("");
            w.line(&format!(
                "private void {}(object sender, {} e) => {};",
                h.name, h.args, h.call
            ));
        }
        if items {
            w.line("");
            w.line("/// <summary>The element of the ItemsRepeater around <paramref name=\"sender\"/> it was made for.</summary>");
            w.line("private static T ItemOf<T>(object sender)");
            w.line("{");
            w.indent(|w| {
                w.line("var child = (DependencyObject)sender;");
                w.line("for (var parent = VisualTreeHelper.GetParent(child); parent is not null; child = parent, parent = VisualTreeHelper.GetParent(parent))");
                w.line("{");
                w.indent(|w| {
                    w.line("if (parent is ItemsRepeater repeater && child is UIElement element)");
                    w.indent(|w| w.line("return (T)repeater.ItemsSourceView.GetAt(repeater.GetElementIndex(element));"));
                });
                w.line("}");
                w.line("throw new InvalidOperationException(\"not inside an ItemsRepeater\");");
            });
            w.line("}");
        }
        if cx.negates {
            w.line("");
            w.line("public static bool Not(bool value) => !value;");
        }
        for (i, negations) in cx.visibilities.iter().enumerate() {
            let params: Vec<String> = (0..negations.len()).map(|j| format!("bool c{j}")).collect();
            let test: Vec<String> = negations
                .iter()
                .enumerate()
                .map(|(j, n)| format!("{}c{j}", if *n { "!" } else { "" }))
                .collect();
            w.line("");
            w.line(&format!(
                "public static Visibility Visible{}({}) => {} ? Visibility.Visible : Visibility.Collapsed;",
                i + 1,
                params.join(", "),
                test.join(" && ")
            ));
        }
        for (i, look) in cx.looks.iter().enumerate() {
            w.line("");
            let name = look_handler(&cx.looks, i);
            let spacing = |step: &str| match step {
                "px" => "spacing.px".to_string(),
                step => format!("spacing.{}", step.replace('-', ".")),
            };
            match look {
                Look::Gap(step) => w.line(&format!(
                    "private void {name}(object sender, RoutedEventArgs e) => ((StackPanel)sender).Spacing = Appearance.GetTheme(this).Number(\"{}\");",
                    spacing(step)
                )),
                Look::RepeaterGap(step) => w.line(&format!(
                    "private void {name}(object sender, RoutedEventArgs e) => ((StackLayout)((ItemsRepeater)sender).Layout).Spacing = Appearance.GetTheme(this).Number(\"{}\");",
                    spacing(step)
                )),
                Look::Text {
                    tint,
                    secondary,
                    strong,
                } => {
                    w.line(&format!("private void {name}(object sender, RoutedEventArgs e)"));
                    w.line("{");
                    w.indent(|w| {
                        w.line("var text = (TextBlock)sender;");
                        w.line("var theme = Appearance.GetTheme(this);");
                        match tint {
                            Some(tint) => w.line(&format!(
                                "text.Foreground = theme.Brush(\"color.{tint}.500\");"
                            )),
                            None if *secondary => w.line(
                                "text.Foreground = theme.Brush(\"color.content-secondary\");",
                            ),
                            None => {}
                        }
                        if *strong {
                            w.line("text.FontWeight = new Windows.UI.Text.FontWeight { Weight = (ushort)theme.Token(\"labelStrong\").GetProperty(\"fontWeight\").GetInt32() };");
                        }
                    });
                    w.line("}");
                }
            }
        }
    });
    w.line("}");
    w.finish()
}

/// A Rust page model's C#: `SettingsActions.cs`, the interface of the
/// methods a page calls, which a host may implement itself; and
/// `SettingsStore.cs`, its value observable over the UniFFI object.
pub fn model(model: &CompiledModel, options: &Options) -> Vec<GeneratedFile> {
    vec![actions(model, options), store(model, options)]
}

fn actions(model: &CompiledModel, options: &Options) -> GeneratedFile {
    let bridge = &model.bridge;
    let mut w = Writer::default();
    w.raw(&header(&model.file));
    w.line("");
    w.line(&format!("namespace {};", options.namespace));
    w.line("");
    w.line(&format!(
        "/// <summary>The methods of <c>{}</c> a page calls. The Rust model's store",
        bridge.model
    ));
    w.line("/// hands them on; a host, or a test, may implement them itself.</summary>");
    w.line(&format!("public interface I{}", bridge.actions()));
    w.line("{");
    w.indent(|w| {
        for m in &bridge.methods {
            w.line(&format!("void {};", signature(m)));
        }
    });
    w.line("}");
    GeneratedFile::new(format!("{}.cs", bridge.actions()), w.finish())
}

/// `SetOn(bool on)`
fn signature(m: &crate::bridge::BridgeMethod) -> String {
    let param = m
        .param
        .as_ref()
        .map(|(name, ty)| format!("{} {name}", ty.csharp()))
        .unwrap_or_default();
    format!("{}({param})", pascal_case(&m.name))
}

fn store(model: &CompiledModel, options: &Options) -> GeneratedFile {
    let bridge = &model.bridge;
    let (state, object, listener, store, actions) = (
        &bridge.state,
        &bridge.model,
        bridge.listener(),
        bridge.store(),
        format!("I{}", bridge.actions()),
    );
    let mut w = Writer::default();
    w.raw(&header(&model.file));
    w.line("");
    w.line("using System;");
    w.line("using System.ComponentModel;");
    w.line("using Microsoft.UI.Dispatching;");
    w.line("");
    w.line(&format!("namespace {};", options.namespace));
    w.line("");
    w.line(&format!(
        "/// <summary>The latest value of a <c>{object}</c>, observable, and its methods:"
    ));
    w.line("/// what a page takes, as a <c>ListenableBuilder</c> hands a widget. Whatever");
    w.line("/// thread the model changes on, the value is read again on the thread that");
    w.line("/// made the store.</summary>");
    w.line(&format!(
        "public sealed class {store} : INotifyPropertyChanged, IDisposable"
    ));
    w.line("{");
    w.indent(|w| {
        w.line(
            "private readonly DispatcherQueue? dispatcher = DispatcherQueue.GetForCurrentThread();",
        );
        w.line("private readonly ulong listener;");
        w.line("");
        w.line(&format!("public {object} Model {{ get; }}"));
        w.line("");
        w.line("/// <summary>The model's methods, as a page calls them.</summary>");
        w.line(&format!("public {actions} Actions {{ get; }}"));
        w.line("");
        w.line(&format!("public {state} State {{ get; private set; }}"));
        w.line("");
        w.line("public event PropertyChangedEventHandler? PropertyChanged;");
        w.line("");
        w.line(&format!("public {store}() : this(new {object}()) {{ }}"));
        w.line("");
        w.line(&format!("public {store}({object} model)"));
        w.line("{");
        w.indent(|w| {
            w.line("Model = model;");
            w.line("Actions = new Forward(model);");
            w.line("State = model.Value();");
            w.line(&format!(
                "listener = model.AddListener(new Listener(new WeakReference<{store}>(this)));"
            ));
        });
        w.line("}");
        w.line("");
        w.line("public void Dispose()");
        w.line("{");
        w.indent(|w| {
            w.line("Model.RemoveListener(listener);");
            w.line("Model.Dispose();");
        });
        w.line("}");
        w.line("");
        w.line("private void Refresh()");
        w.line("{");
        w.indent(|w| {
            w.line("State = Model.Value();");
            w.line("PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(nameof(State)));");
        });
        w.line("}");
        w.line("");
        w.line(&format!(
            "private sealed class Forward({object} model) : {actions}"
        ));
        w.line("{");
        w.indent(|w| {
            for m in &bridge.methods {
                let argument = m.param.as_ref().map(|(n, _)| n.as_str()).unwrap_or("");
                w.line(&format!(
                    "public void {} => model.{}({argument});",
                    signature(m),
                    pascal_case(&m.name)
                ));
            }
        });
        w.line("}");
        w.line("");
        w.line("/// <summary>Rust calls this back on the thread that changed the value. On the");
        w.line("/// store's own, it reads it at once, so a model that changes as a method runs");
        w.line("/// is shown changed when the call returns; from any other, it reads it there");
        w.line("/// next. It holds the store weakly: Rust holds it.</summary>");
        w.line(&format!(
            "private sealed class Listener(WeakReference<{store}> store) : {listener}"
        ));
        w.line("{");
        w.indent(|w| {
            w.line("public void Changed()");
            w.line("{");
            w.indent(|w| {
                w.line("if (!store.TryGetTarget(out var s)) return;");
                w.line("if (s.dispatcher is null || s.dispatcher.HasThreadAccess) s.Refresh();");
                w.line("else s.dispatcher.TryEnqueue(s.Refresh);");
            });
            w.line("}");
        });
        w.line("}");
    });
    w.line("}");
    GeneratedFile::new(format!("{store}.cs"), w.finish())
}

#[derive(Default)]
struct Writer {
    out: String,
    depth: usize,
}

impl Writer {
    fn line(&mut self, text: &str) {
        if !text.is_empty() {
            self.out += &"    ".repeat(self.depth);
            self.out += text;
        }
        self.out.push('\n');
    }

    fn raw(&mut self, text: &str) {
        self.out += text;
    }

    fn indent(&mut self, body: impl FnOnce(&mut Self)) {
        self.depth += 1;
        body(self);
        self.depth -= 1;
    }

    fn finish(self) -> String {
        self.out
    }
}
