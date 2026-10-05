//! Lowers a page to WinUI 3, in XAML and C#.
//!
//! A page becomes two files: the XAML, and its code-behind — a `State` to
//! show, a `Send` for actions, and one handler per event, each sending one.
//! It holds nothing else, and knows nothing of where either comes from. A
//! Rust page model becomes a store, thin over the UniFFI object whose C# —
//! the state, the actions, the source — uniffi-bindgen-cs writes beside it,
//! which is one way for a host to feed a page.
//! DazzUI controls come from `xmlns:dazz="using:DazzUI"`.
//!
//! State is read with `{x:Bind …, Mode=OneWay}` and never `TwoWay`: a
//! change goes out as an action, and the value comes back in the next
//! snapshot, so the page's copy of the state is never changed on its own.
//!
//! An `if` sets `Visibility` on each node of its branches rather than
//! `x:Load`, which works inside a `DataTemplate` too and needs no `x:Name`.
//! A `for` is an `ItemsRepeater` laid out as the stack around it is, so its
//! elements sit in that stack as SwiftUI's `ForEach` puts them.

use crate::Registry;
use crate::ir::{
    ActionPayload, ActionRef, Component, ComponentSpec, Condition, Expr, GeneratedFile, Source,
    UiNode, Value, pascal_case,
};
use crate::rust_types::Fields;
use crate::{CompiledModel, CompiledPage, Options};

pub fn generate(compiled: &CompiledPage, options: &Options) -> Vec<GeneratedFile> {
    let page = &compiled.page;
    let mut cx = Cx {
        page: page.name.clone(),
        registry: compiled.registry.clone(),
        handlers: Vec::new(),
        usings: Vec::new(),
        styled: Vec::new(),
        negates: false,
        visibilities: Vec::new(),
        conditions: Vec::new(),
        element: None,
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
    /// Named texts and the type token each is set in once loaded.
    styled: Vec<(String, &'static str)>,
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
}

/// The stack a node is written into: a `for` lays its elements out as it.
#[derive(Clone, Copy)]
struct Stack {
    horizontal: bool,
    spacing: i64,
}

/// An event handler in the code-behind: its name, the event args it takes,
/// the action it sends, and the C# that builds the action's value.
struct Handler {
    name: String,
    args: String,
    action: ActionRef,
    payload: String,
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
        node(
            w,
            &page.root,
            Stack {
                horizontal: false,
                spacing: 0,
            },
            cx,
        );
    });
    w.line("</Page>");
    w.finish()
}

fn node(w: &mut Writer, node: &UiNode, stack: Stack, cx: &mut Cx) {
    let c = match node {
        UiNode::Component(c) => c,
        UiNode::If {
            condition,
            then_branch,
            else_branch,
            source,
        } => {
            w.line(&format!("<!-- source: {} (if) -->", source.short()));
            cx.conditions.push(condition.clone());
            then_branch.iter().for_each(|n| self::node(w, n, stack, cx));
            cx.conditions.pop();
            if !else_branch.is_empty() {
                w.line(&format!("<!-- source: {} (else) -->", source.short()));
                cx.conditions.push(Condition {
                    negated: !condition.negated,
                    value: condition.value.clone(),
                });
                else_branch.iter().for_each(|n| self::node(w, n, stack, cx));
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
            w.line(&format!(
                "<ItemsRepeater ItemsSource={}{}>",
                bind(cx, &Expr::State(collection.clone())),
                visibility(cx)
            ));
            w.indent(|w| {
                w.line("<ItemsRepeater.Layout>");
                w.indent(|w| {
                    w.line(&format!(
                        "<StackLayout Orientation=\"{orientation}\" Spacing=\"{}\"/>",
                        stack.spacing
                    ))
                });
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
                            w.line(&format!(
                                "<StackPanel Orientation=\"{orientation}\" Spacing=\"{}\">",
                                stack.spacing
                            ));
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
    let visible = visibility(cx);
    // A mapping wins, so that an app's registry can replace even the
    // components this backend lowers itself.
    if let Some(spec) = cx
        .registry
        .get(c.id.as_str())
        .filter(|s| s.winui.is_some())
        .cloned()
    {
        return mapped(w, c, &spec, &visible, cx);
    }
    match c.id.as_str() {
        kind @ ("VStack" | "HStack") => {
            let inner = Stack {
                horizontal: kind == "HStack",
                spacing: int(c, "spacing"),
            };
            let orientation = if inner.horizontal {
                " Orientation=\"Horizontal\""
            } else {
                ""
            };
            w.line(&format!(
                "<StackPanel{orientation} Spacing=\"{}\"{visible}>",
                inner.spacing
            ));
            // The conditions are on the panel; its children need not repeat them.
            let conditions = std::mem::take(&mut cx.conditions);
            w.indent(|w| {
                c.children
                    .iter()
                    .for_each(|child| self::node(w, child, inner, cx))
            });
            cx.conditions = conditions;
            w.line("</StackPanel>");
        }
        "Text" => {
            let mut tag = format!(
                "<TextBlock Text={}",
                value(cx, c.prop("text").expect("text is required"))
            );
            if let Some(style) = c.prop("style") {
                let token = text_token(case(style));
                if cx.element.is_some() {
                    // A name inside a template is not a field of the page;
                    // the face is set as the text loads instead.
                    tag += &format!(" Tag=\"{token}\" Loaded=\"OnStyledTextLoaded\"");
                    cx.styled.push((String::new(), token));
                } else {
                    // A type token is not a XAML resource, so the
                    // code-behind sets the face once the page knows its theme.
                    let name = format!(
                        "Text{}",
                        cx.styled.iter().filter(|(n, _)| !n.is_empty()).count() + 1
                    );
                    tag += &format!(" x:Name=\"{name}\"");
                    cx.styled.push((name, token));
                }
            }
            w.line(&(tag + &visible + "/>"));
        }
        other => unreachable!("`{other}` passed the registry but has no WinUI lowering"),
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
fn mapped(w: &mut Writer, c: &Component, spec: &ComponentSpec, visible: &str, cx: &mut Cx) {
    let m = spec.winui.as_ref().expect("a WinUI mapping");
    let mut tag = format!("<{}", m.element);
    for attr in &m.attrs {
        if let Source::Prop {
            prop,
            cases,
            invert,
            omit,
        } = &attr.source
            && let Some(e) = c.prop_value(prop).filter(|e| !e.is_omitted(omit.as_ref()))
        {
            tag += &format!(" {}={}", attr.attr, attr_value(cx, &e, cases, *invert));
        }
    }
    for event in &m.events {
        if let Some(action) = c.event_action(spec, &event.name) {
            if let Some(using) = &event.using
                && !cx.usings.contains(using)
            {
                cx.usings.push(using.clone());
            }
            let name = handler(
                cx,
                action,
                &event.event,
                &event.args,
                event.value.as_deref(),
            );
            tag += &format!(" {}=\"{name}\"", event.event);
        }
    }
    tag += visible;

    let slots: Vec<(&str, &UiNode)> = m
        .slots
        .iter()
        .filter_map(|(slot, property)| Some((property.as_str(), c.slot(slot)?)))
        .collect();
    if slots.is_empty() && c.children.is_empty() {
        return w.line(&(tag + "/>"));
    }
    w.line(&(tag + ">"));
    // The conditions are on the element; what it holds need not repeat them.
    let conditions = std::mem::take(&mut cx.conditions);
    let column = Stack {
        horizontal: false,
        spacing: 0,
    };
    w.indent(|w| {
        for (property, node) in slots {
            w.line(&format!("<{}.{property}>", m.element));
            w.indent(|w| self::node(w, node, column, cx));
            w.line(&format!("</{}.{property}>", m.element));
        }
        if !c.children.is_empty() {
            let children = |w: &mut Writer, cx: &mut Cx| {
                c.children.iter().for_each(|n| self::node(w, n, column, cx))
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

/// The handler for `action` on `event`, `OnSaveClick`: one per distinct
/// action and value, however many controls send it.
fn handler(
    cx: &mut Cx,
    action: &ActionRef,
    event: &str,
    args: &str,
    event_value: Option<&str>,
) -> String {
    let payload = match &action.payload {
        ActionPayload::None => String::new(),
        ActionPayload::EventValue => event_value.expect("an event with a value").to_string(),
        ActionPayload::Expr(e) => cs_expr(cx, e),
    };
    if let Some(h) = cx
        .handlers
        .iter()
        .find(|h| &h.action == action && h.payload == payload)
    {
        return h.name.clone();
    }
    let base = format!("On{}{event}", action.variant);
    let mut name = base.clone();
    let mut n = 1;
    while cx.handlers.iter().any(|h| h.name == name) {
        n += 1;
        name = format!("{base}{n}");
    }
    cx.handlers.push(Handler {
        name: name.clone(),
        args: args.into(),
        action: action.clone(),
        payload,
    });
    name
}

fn text_token(case: &str) -> &'static str {
    match case {
        "Title" => "titleMedium",
        "Body" => "bodyMedium",
        "Caption" => "captionMedium",
        other => unreachable!("TextStyle has no case `{other}`"),
    }
}

fn case(expr: &Expr) -> &str {
    match expr {
        Expr::EnumCase { case, .. } => case,
        other => unreachable!("expected an enum case, found {other:?}"),
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

fn int(c: &Component, prop: &str) -> i64 {
    match c.prop(prop) {
        Some(Expr::Literal(Value::Int(n))) => *n,
        other => unreachable!("`{prop}` on `{}` is {other:?}", c.id.as_str()),
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
    let (state, action) = (&page.state_type, &page.action_type);
    let named: Vec<&(String, &str)> = cx.styled.iter().filter(|(n, _)| !n.is_empty()).collect();
    let in_templates = cx.styled.iter().any(|(n, _)| n.is_empty());
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    if !cx.styled.is_empty() {
        w.line("using DazzUI;");
    }
    let items = cx.handlers.iter().any(|h| h.payload.contains("ItemOf<"));
    w.line("using System;");
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
        w.line("/// <summary>Where the page's actions go.</summary>");
        w.line(&format!("public Action<{action}>? Send {{ get; set; }}"));
        w.line("");
        w.line(&format!("public {}()", page.name));
        w.line("{");
        w.indent(|w| {
            w.line("InitializeComponent();");
            if !named.is_empty() {
                w.line("Loaded += (_, _) =>");
                w.line("{");
                w.indent(|w| {
                    for (name, token) in &named {
                        w.line(&format!("SetFace({name}, \"{token}\");"));
                    }
                });
                w.line("};");
            }
        });
        w.line("}");
        // uniffi-bindgen-cs writes an action with no values as a C# enum,
        // and one with any as records.
        let flat = compiled
            .actions
            .variants
            .iter()
            .all(|v| v.fields == Fields::Unit);
        for h in &cx.handlers {
            w.line("");
            let action = if flat {
                format!("{action}.{}", h.action.variant)
            } else {
                format!("new {action}.{}({})", h.action.variant, h.payload)
            };
            w.line(&format!("private void {}(object sender, {} e) => Send?.Invoke({action});", h.name, h.args));
        }
        if cx.handlers.iter().any(|h| h.payload.contains("ItemOf<")) {
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
            let test: Vec<String> =
                negations.iter().enumerate().map(|(j, n)| format!("{}c{j}", if *n { "!" } else { "" })).collect();
            w.line("");
            w.line(&format!(
                "public static Visibility Visible{}({}) => {} ? Visibility.Visible : Visibility.Collapsed;",
                i + 1,
                params.join(", "),
                test.join(" && ")
            ));
        }
        if in_templates {
            w.line("");
            w.line("private void OnStyledTextLoaded(object sender, RoutedEventArgs e)");
            w.line("{");
            w.indent(|w| w.line("if (sender is TextBlock text && text.Tag is string token) SetFace(text, token);"));
            w.line("}");
        }
        if !cx.styled.is_empty() {
            w.line("");
            w.line("private void SetFace(TextBlock text, string token)");
            w.line("{");
            w.indent(|w| {
                w.line("var face = Appearance.GetTheme(this).Text(\"\", token);");
                w.line("text.FontSize = face.FontSize;");
                w.line("text.FontWeight = face.FontWeight;");
                w.line("text.LineHeight = face.LineHeight;");
                w.line("text.LineStackingStrategy = face.LineStackingStrategy;");
            });
            w.line("}");
        }
    });
    w.line("}");
    w.finish()
}

/// `SettingsStore.cs`: the state of a Rust page model, observable, and its
/// `Send`, for a host to hand a page.
pub fn store(model: &CompiledModel, options: &Options) -> GeneratedFile {
    let bridge = &model.bridge;
    let (state, action, source, listener, store) = (
        &bridge.state,
        &bridge.action,
        bridge.source(),
        bridge.listener(),
        bridge.store(),
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
        "/// <summary>The latest snapshot of the model <c>{}</c> makes, and a way to",
        bridge.function
    ));
    w.line("/// send it actions: what a page takes. Whatever thread the model changes on,");
    w.line("/// the snapshot is read again on the thread that made the store.</summary>");
    w.line(&format!(
        "public sealed class {store} : INotifyPropertyChanged, IDisposable"
    ));
    w.line("{");
    w.indent(|w| {
        w.line(&format!("private readonly {source} source;"));
        w.line(
            "private readonly DispatcherQueue? dispatcher = DispatcherQueue.GetForCurrentThread();",
        );
        w.line("");
        w.line(&format!("public {state} State {{ get; private set; }}"));
        w.line("");
        w.line("public event PropertyChangedEventHandler? PropertyChanged;");
        w.line("");
        w.line(&format!("public {store}() : this(new {source}()) {{ }}"));
        w.line("");
        w.line(&format!("public {store}({source} source)"));
        w.line("{");
        w.indent(|w| {
            w.line("this.source = source;");
            w.line("State = source.State();");
            w.line(&format!(
                "source.Subscribe(new Listener(new WeakReference<{store}>(this)));"
            ));
        });
        w.line("}");
        w.line("");
        w.line(&format!(
            "public void Send({action} action) => source.Send(action);"
        ));
        w.line("");
        w.line("public void Dispose() => source.Dispose();");
        w.line("");
        w.line("private void Refresh()");
        w.line("{");
        w.indent(|w| {
            w.line("State = source.State();");
            w.line("PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(nameof(State)));");
        });
        w.line("}");
        w.line("");
        w.line("/// <summary>Rust calls this back on the thread that changed the state. On the");
        w.line("/// store's own, it reads it at once, so a model that changes as it takes an");
        w.line("/// action is shown changed when <c>Send</c> returns; from any other, it reads it");
        w.line("/// there next. It holds the store weakly: Rust holds it.</summary>");
        w.line(&format!(
            "private sealed class Listener(WeakReference<{store}> store) : {listener}"
        ));
        w.line("{");
        w.indent(|w| {
            w.line("public void StateChanged()");
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
    fn raw(&mut self, text: &str) {
        self.out += text;
    }

    fn line(&mut self, text: &str) {
        if !text.is_empty() {
            self.out += &"    ".repeat(self.depth);
            self.out += text;
        }
        self.out.push('\n');
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
