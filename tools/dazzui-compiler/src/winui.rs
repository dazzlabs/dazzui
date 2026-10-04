//! Lowers a page to WinUI 3, in XAML and C#.
//!
//! A page becomes four files: the XAML; its code-behind — a constructor
//! that takes the view model, and one handler per event, each forwarding an
//! action; the view model, with the state and struct records and action
//! records derived from the Rust types; and the C# side of the bridge.
//! DazzUI controls come from `xmlns:dazz="using:DazzUI"`.
//!
//! State is read with `{x:Bind …, Mode=OneWay}` and never `TwoWay`: a
//! change goes to Rust as an action, and the value comes back in the next
//! snapshot, so the C# copy of the state is never changed on its own.
//!
//! An `if` sets `Visibility` on each node of its branches rather than
//! `x:Load`, which works inside a `DataTemplate` too and needs no `x:Name`.
//! A `for` is an `ItemsRepeater` laid out as the stack around it is, so its
//! elements sit in that stack as SwiftUI's `ForEach` puts them.

use crate::bridge::{Bridge, BridgeStruct, FieldType, Scalar};
use crate::ir::{
    ActionPayload, ActionRef, Component, Condition, Expr, GeneratedFile, UiNode, Value, page_stem, pascal_case,
};
use crate::rust_types::{FloatType, IntType};
use crate::{CompiledPage, Options};

pub fn generate(compiled: &CompiledPage, options: &Options) -> Vec<GeneratedFile> {
    let page = &compiled.page;
    let stem = page_stem(&page.name);
    let mut cx = Cx { page: page.name.clone(), ..Cx::default() };
    let xaml = xaml(compiled, &options.namespace, &mut cx);
    vec![
        GeneratedFile::new(format!("{}.xaml", page.name), xaml),
        GeneratedFile::new(format!("{}.xaml.cs", page.name), code_behind(compiled, &options.namespace, &cx)),
        GeneratedFile::new(format!("{stem}ViewModel.cs"), view_model(compiled, &options.namespace)),
        GeneratedFile::new(format!("{}Bridge.cs", compiled.bridge.stem), bridge_glue(compiled, options)),
    ]
}

/// What the XAML needs from its code-behind, and where the writer is.
#[derive(Default)]
struct Cx {
    page: String,
    handlers: Vec<Handler>,
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
    args: &'static str,
    action: ActionRef,
    payload: String,
}

fn xaml(compiled: &CompiledPage, namespace: &str, cx: &mut Cx) -> String {
    let page = &compiled.page;
    let mut w = Writer::default();
    w.line(&format!("<!-- AUTO-GENERATED FILE. DO NOT EDIT. Source: {} -->", compiled.file));
    w.line("<Page");
    w.indent(|w| {
        w.line(&format!("x:Class=\"{namespace}.{}\"", page.name));
        w.line("xmlns=\"http://schemas.microsoft.com/winfx/2006/xaml/presentation\"");
        w.line("xmlns:x=\"http://schemas.microsoft.com/winfx/2006/xaml\"");
        w.line(&format!("xmlns:local=\"using:{namespace}\""));
        w.line("xmlns:dazz=\"using:DazzUI\">");
        node(w, &page.root, Stack { horizontal: false, spacing: 0 }, cx);
    });
    w.line("</Page>");
    w.finish()
}

fn node(w: &mut Writer, node: &UiNode, stack: Stack, cx: &mut Cx) {
    let c = match node {
        UiNode::Component(c) => c,
        UiNode::If { condition, then_branch, else_branch, source } => {
            w.line(&format!("<!-- source: {} (if) -->", source.short()));
            cx.conditions.push(condition.clone());
            then_branch.iter().for_each(|n| self::node(w, n, stack, cx));
            cx.conditions.pop();
            if !else_branch.is_empty() {
                w.line(&format!("<!-- source: {} (else) -->", source.short()));
                cx.conditions.push(Condition { negated: !condition.negated, value: condition.value.clone() });
                else_branch.iter().for_each(|n| self::node(w, n, stack, cx));
                cx.conditions.pop();
            }
            return;
        }
        UiNode::ForEach { collection, element, children, source, .. } => {
            w.line(&format!("<!-- source: {} -->", source.short()));
            let orientation = if stack.horizontal { "Horizontal" } else { "Vertical" };
            w.line(&format!(
                "<ItemsRepeater ItemsSource={}{}>",
                bind(cx, &Expr::State(collection.clone())),
                visibility(cx)
            ));
            w.indent(|w| {
                w.line("<ItemsRepeater.Layout>");
                w.indent(|w| w.line(&format!("<StackLayout Orientation=\"{orientation}\" Spacing=\"{}\"/>", stack.spacing)));
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
                            w.line(&format!("<StackPanel Orientation=\"{orientation}\" Spacing=\"{}\">", stack.spacing));
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
    match c.id.as_str() {
        kind @ ("VStack" | "HStack") => {
            let inner = Stack { horizontal: kind == "HStack", spacing: int(c, "spacing") };
            let orientation = if inner.horizontal { " Orientation=\"Horizontal\"" } else { "" };
            w.line(&format!("<StackPanel{orientation} Spacing=\"{}\"{visible}>", inner.spacing));
            // The conditions are on the panel; its children need not repeat them.
            let conditions = std::mem::take(&mut cx.conditions);
            w.indent(|w| c.children.iter().for_each(|child| self::node(w, child, inner, cx)));
            cx.conditions = conditions;
            w.line("</StackPanel>");
        }
        "Text" => {
            let mut tag = format!("<TextBlock Text={}", value(cx, c.prop("text").expect("text is required")));
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
                    let name = format!("Text{}", cx.styled.iter().filter(|(n, _)| !n.is_empty()).count() + 1);
                    tag += &format!(" x:Name=\"{name}\"");
                    cx.styled.push((name, token));
                }
            }
            w.line(&(tag + &visible + "/>"));
        }
        "Button" => {
            // DazzUI's C# Button defaults to `Filled` where React and SwiftUI
            // default to `Recessed`. The semantic default is `Recessed`, so
            // the variant is always written rather than left to the control.
            let variant = match case(c.prop("style").expect("style has a default")) {
                "Primary" => "Filled",
                _ => "Recessed",
            };
            let title = value(cx, c.prop("title").expect("title is required"));
            let mut tag = format!("<dazz:Button Content={title} Variant=\"{variant}\"");
            tag += &enabled(c, cx);
            if let Some(action) = c.event("click") {
                let name = handler(cx, action, "Click", "RoutedEventArgs", None);
                tag += &format!(" Click=\"{name}\"");
            }
            w.line(&(tag + &visible + "/>"));
        }
        "Toggle" => {
            let (checked, change) = match c.binding("value") {
                Some(b) => (bind(cx, &Expr::State(b.path.clone())), Some(&b.on_change)),
                None => (value(cx, c.prop("value").expect("value is required")), c.event("change")),
            };
            let mut tag = format!("<dazz:Switch IsChecked={checked}");
            tag += &enabled(c, cx);
            if let Some(action) = change {
                // A ToggleButton has flipped itself by the time it raises
                // Click; its new state is what the action carries.
                let value = Some("((ToggleButton)sender).IsChecked == true");
                let name = handler(cx, action, "Click", "RoutedEventArgs", value);
                tag += &format!(" Click=\"{name}\"");
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
    format!(" Visibility=\"{{x:Bind local:{}.Visible{}({}){}}}\"", cx.page, index + 1, args.join(", "), mode(cx))
}

/// ` IsEnabled=…` for a `disabled` prop, unless it is the literal `false`
/// that both the registry and the control default to.
fn enabled(c: &Component, cx: &mut Cx) -> String {
    match c.prop("disabled").expect("disabled has a default") {
        Expr::Literal(Value::Bool(false)) => String::new(),
        Expr::Literal(Value::Bool(true)) => " IsEnabled=\"False\"".into(),
        e @ (Expr::State(_) | Expr::Local(_)) => {
            cx.negates = true;
            format!(" IsEnabled=\"{{x:Bind local:{}.Not({}){}}}\"", cx.page, path(cx, e), mode(cx))
        }
        other => unreachable!("`disabled` is {other:?}"),
    }
}

/// The handler for `action` on `event`, `OnSaveClick`: one per distinct
/// action and value, however many controls send it.
fn handler(cx: &mut Cx, action: &ActionRef, event: &str, args: &'static str, event_value: Option<&str>) -> String {
    let payload = match &action.payload {
        ActionPayload::None => String::new(),
        ActionPayload::EventValue => event_value.expect("an event with a value").to_string(),
        ActionPayload::Expr(e) => cs_expr(cx, e),
    };
    if let Some(h) = cx.handlers.iter().find(|h| &h.action == action && h.payload == payload) {
        return h.name.clone();
    }
    let base = format!("On{}{event}", action.variant);
    let mut name = base.clone();
    let mut n = 1;
    while cx.handlers.iter().any(|h| h.name == name) {
        n += 1;
        name = format!("{base}{n}");
    }
    cx.handlers.push(Handler { name: name.clone(), args, action: action.clone(), payload });
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
        Expr::EnumCase { ty, case } => unreachable!("`{ty}::{case}` is lowered by the prop that takes it"),
    }
}

/// `"{x:Bind ViewModel.State.Flag, Mode=OneWay}"`, or `"{x:Bind Name}"`
/// inside a template, whose elements are replaced rather than changed.
fn bind(cx: &Cx, e: &Expr) -> String {
    format!("\"{{x:Bind {}{}}}\"", path(cx, e), mode(cx))
}

fn mode(cx: &Cx) -> &'static str {
    if cx.element.is_some() { "" } else { ", Mode=OneWay" }
}

/// The x:Bind path of a state or element value: `ViewModel.State.Flag`,
/// or `Owner.Name` inside a template.
fn path(cx: &Cx, e: &Expr) -> String {
    let pascal = |segments: &[String]| segments.iter().map(|s| pascal_case(s)).collect::<Vec<_>>().join(".");
    match e {
        Expr::State(p) => format!("ViewModel.State.{}", pascal(&p.segments)),
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
        // The element a control in a template was made for is its
        // DataContext.
        Expr::Local(_) => {
            let element = cx.element.as_deref().expect("a loop item inside its loop");
            format!("(({element})((FrameworkElement)sender).DataContext).{}", path(cx, e))
        }
        Expr::EnumCase { ty, case } => unreachable!("`{ty}::{case}` is lowered by the prop that takes it"),
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
    let model = format!("{}ViewModel", page_stem(&page.name));
    let action = &compiled.bridge.action;
    let named: Vec<&(String, &str)> = cx.styled.iter().filter(|(n, _)| !n.is_empty()).collect();
    let in_templates = cx.styled.iter().any(|(n, _)| n.is_empty());
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    if !cx.styled.is_empty() {
        w.line("using DazzUI;");
    }
    w.line("using Microsoft.UI.Xaml;");
    w.line("using Microsoft.UI.Xaml.Controls;");
    if cx.handlers.iter().any(|h| h.payload.contains("ToggleButton")) {
        w.line("using Microsoft.UI.Xaml.Controls.Primitives;");
    }
    w.line("");
    w.line(&format!("namespace {namespace};"));
    w.line("");
    w.line(&format!("public sealed partial class {} : Page", page.name));
    w.line("{");
    w.indent(|w| {
        w.line(&format!("public {model} ViewModel {{ get; }}"));
        w.line("");
        w.line(&format!("public {}({model} viewModel)", page.name));
        w.line("{");
        w.indent(|w| {
            w.line("ViewModel = viewModel;");
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
        for h in &cx.handlers {
            w.line("");
            w.line(&format!(
                "private void {}(object sender, {} e) => ViewModel.Send(new {action}.{}({}));",
                h.name, h.args, h.action.variant, h.payload
            ));
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

fn cs_type(ty: Scalar) -> &'static str {
    match ty {
        Scalar::Bool => "bool",
        Scalar::Int(IntType::I8) => "sbyte",
        Scalar::Int(IntType::I16) => "short",
        Scalar::Int(IntType::I32) => "int",
        Scalar::Int(IntType::I64) => "long",
        Scalar::Int(IntType::U8) => "byte",
        Scalar::Int(IntType::U16) => "ushort",
        Scalar::Int(IntType::U32) => "uint",
        Scalar::Int(IntType::U64) => "ulong",
        Scalar::Int(IntType::Isize) => "nint",
        Scalar::Int(IntType::Usize) => "nuint",
        Scalar::Float(FloatType::F32) => "float",
        Scalar::Float(FloatType::F64) => "double",
        Scalar::Str => "string",
    }
}

fn field_type(ty: &FieldType) -> String {
    match ty {
        FieldType::Scalar(scalar) => cs_type(*scalar).into(),
        FieldType::Struct(name) => name.clone(),
        FieldType::List(name) => format!("IReadOnlyList<{name}>"),
    }
}

/// The C# name of a struct's DTO inside the native class: `StateDto` for
/// the state, `PluginDto` for another.
fn dto(bridge: &Bridge, name: &str) -> String {
    if name == bridge.state { "StateDto".into() } else { format!("{name}Dto") }
}

/// The type a DTO holds a field in: blittable, so a `bool` is a byte.
fn dto_type(bridge: &Bridge, ty: &FieldType) -> String {
    match ty {
        FieldType::Scalar(Scalar::Bool) => "byte".into(),
        FieldType::Scalar(Scalar::Str) => "NuiStr".into(),
        FieldType::Scalar(other) => cs_type(*other).into(),
        FieldType::Struct(name) => dto(bridge, name),
        FieldType::List(name) => format!("{name}List"),
    }
}

fn view_model(compiled: &CompiledPage, namespace: &str) -> String {
    let stem = page_stem(&compiled.page.name);
    let bridge = &compiled.bridge;
    let native = native_class(bridge);
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line("using System;");
    w.line("using System.Collections.Generic;");
    w.line("using System.ComponentModel;");
    w.line("using System.Runtime.CompilerServices;");
    w.line("using System.Runtime.InteropServices;");
    w.line("");
    w.line(&format!("namespace {namespace};"));
    w.line("");
    for st in &bridge.structs {
        record(&mut w, bridge, st);
        w.line("");
    }
    action_records(&mut w, bridge);
    w.line("");
    w.line("/// <summary>Holds the latest snapshot of the Rust state and sends actions to");
    w.line(&format!(
        "/// Rust's <c>{}</c>, which runs on the calling thread before <c>Send</c> returns.</summary>",
        bridge.update
    ));
    w.line(&format!("public sealed unsafe class {stem}ViewModel : INotifyPropertyChanged, IDisposable"));
    w.line("{");
    w.indent(|w| {
        w.line("private IntPtr app;");
        w.line("private GCHandle self;");
        w.line("");
        w.line(&format!("public {} State {{ get; private set; }}", bridge.state));
        w.line("");
        w.line("public event PropertyChangedEventHandler? PropertyChanged;");
        w.line("");
        w.line(&format!("public {stem}ViewModel()"));
        w.line("{");
        w.indent(|w| {
            w.line(&format!("app = {native}.{}();", bridge.function("create")));
            w.line("self = GCHandle.Alloc(this, GCHandleType.Weak);");
            w.line(&format!("{native}.{}(app, &OnStateChanged, GCHandle.ToIntPtr(self));", bridge.function("subscribe")));
            w.line(&format!("State = {native}.ReadState(app);"));
        });
        w.line("}");
        w.line("");
        w.line(&format!("public void Send({} action) => {native}.Send(app, action);", bridge.action));
        w.line("");
        w.line("public void Dispose()");
        w.line("{");
        w.indent(|w| {
            w.line("if (app == IntPtr.Zero) return;");
            w.line(&format!("{native}.{}(app);", bridge.function("destroy")));
            w.line("app = IntPtr.Zero;");
            w.line("self.Free();");
        });
        w.line("}");
        w.line("");
        w.line("/// <summary>Rust calls this back on the thread that sent the action.</summary>");
        w.line("[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]");
        w.line("private static void OnStateChanged(IntPtr context)");
        w.line("{");
        w.indent(|w| {
            w.line(&format!("if (GCHandle.FromIntPtr(context).Target is not {stem}ViewModel model) return;"));
            w.line(&format!("model.State = {native}.ReadState(model.app);"));
            w.line("model.PropertyChanged?.Invoke(model, new PropertyChangedEventArgs(nameof(State)));");
        });
        w.line("}");
    });
    w.line("}");
    w.finish()
}

fn record(w: &mut Writer, bridge: &Bridge, st: &BridgeStruct) {
    if st.name == bridge.state {
        w.line(&format!("/// <summary><c>{}</c>, as Rust last handed it over.</summary>", st.name));
    } else {
        w.line(&format!("/// <summary><c>{}</c>, from the Rust app core.</summary>", st.name));
    }
    let params: Vec<String> =
        st.fields.iter().map(|f| format!("{} {}", field_type(&f.ty), Bridge::property(&f.name))).collect();
    w.line(&format!("public sealed record {}({});", st.name, params.join(", ")));
}

fn action_records(w: &mut Writer, bridge: &Bridge) {
    w.line(&format!("public abstract record {}", bridge.action));
    w.line("{");
    w.indent(|w| {
        w.line(&format!("private {}() {{ }}", bridge.action));
        for v in &bridge.variants {
            let params = v.payload.map(|ty| format!("{} Value", cs_type(ty))).unwrap_or_default();
            w.line("");
            w.line(&format!("public sealed record {}({params}) : {};", v.name, bridge.action));
        }
    });
    w.line("}");
}

/// `SettingsNative`: the class the bridge's imports are on.
fn native_class(bridge: &Bridge) -> String {
    format!("{}Native", bridge.stem)
}

fn bridge_glue(compiled: &CompiledPage, options: &Options) -> String {
    let bridge = &compiled.bridge;
    let native = native_class(bridge);
    let f = |name: &str| bridge.function(name);
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line("using System;");
    w.line("using System.Collections.Generic;");
    w.line("using System.Runtime.InteropServices;");
    w.line("using System.Text;");
    w.line("");
    w.line(&format!("namespace {};", options.namespace));
    w.line("");
    w.line(&format!("/// <summary>The C ABI of <c>{}</c>, from the Rust app core.</summary>", options.library));
    w.line(&format!("internal static unsafe partial class {native}"));
    w.line("{");
    w.indent(|w| {
        w.line(&format!("private const string Library = \"{}\";", options.library));
        w.line("");
        w.line("[StructLayout(LayoutKind.Sequential)]");
        w.line("internal struct NuiStr { public byte* Ptr; public nuint Len; }");
        for st in &bridge.structs {
            w.line("");
            w.line("[StructLayout(LayoutKind.Sequential)]");
            w.line(&format!("internal struct {}", dto(bridge, &st.name)));
            w.line("{");
            w.indent(|w| {
                for field in &st.fields {
                    w.line(&format!("public {} {};", dto_type(bridge, &field.ty), Bridge::property(&field.name)));
                }
            });
            w.line("}");
            if bridge.is_listed(&st.name) {
                w.line("");
                w.line("[StructLayout(LayoutKind.Sequential)]");
                w.line(&format!("internal struct {}List {{ public {}* Ptr; public nuint Len; }}", st.name, dto(bridge, &st.name)));
            }
        }
        if bridge.has_payloads() {
            w.line("");
            w.line("[StructLayout(LayoutKind.Explicit)]");
            w.line("internal struct ActionPayload");
            w.line("{");
            w.indent(|w| {
                for v in &bridge.variants {
                    if let Some(ty) = v.payload {
                        w.line(&format!("[FieldOffset(0)] public {} {};", dto_type(bridge, &FieldType::Scalar(ty)), v.name));
                    }
                }
            });
            w.line("}");
        }
        w.line("");
        w.line("[StructLayout(LayoutKind.Sequential)]");
        w.line("internal struct ActionDto");
        w.line("{");
        w.indent(|w| {
            w.line("public uint Tag;");
            if bridge.has_payloads() {
                w.line("public ActionPayload Payload;");
            }
        });
        w.line("}");
        w.line("");
        for (ret, name, params) in [
            ("IntPtr", f("create"), "".to_string()),
            ("void", f("destroy"), "IntPtr app".to_string()),
            ("StateDto", f("get_state"), "IntPtr app".to_string()),
            ("void", f("state_free"), "StateDto state".to_string()),
            ("void", f("send"), "IntPtr app, ActionDto action".to_string()),
            (
                "void",
                f("subscribe"),
                "IntPtr app, delegate* unmanaged[Cdecl]<IntPtr, void> callback, IntPtr context".to_string(),
            ),
        ] {
            w.line(&format!("[LibraryImport(Library)] internal static partial {ret} {name}({params});"));
        }
        w.line("");
        w.line(&format!("internal static {} ReadState(IntPtr app)", bridge.state));
        w.line("{");
        w.indent(|w| {
            w.line(&format!("var dto = {}(app);", f("get_state")));
            w.line("try { return Read(dto); }");
            w.line(&format!("finally {{ {}(dto); }}", f("state_free")));
        });
        w.line("}");
        for st in &bridge.structs {
            let values: Vec<String> = st
                .fields
                .iter()
                .map(|field| {
                    let p = Bridge::property(&field.name);
                    match &field.ty {
                        FieldType::Scalar(Scalar::Bool) => format!("dto.{p} != 0"),
                        FieldType::Scalar(Scalar::Str) => format!("Decode(dto.{p})"),
                        FieldType::Scalar(_) => format!("dto.{p}"),
                        FieldType::Struct(_) => format!("Read(dto.{p})"),
                        FieldType::List(_) => format!("ReadList(dto.{p})"),
                    }
                })
                .collect();
            w.line("");
            w.line(&format!(
                "private static {} Read({} dto) => new({});",
                st.name,
                dto(bridge, &st.name),
                values.join(", ")
            ));
            if bridge.is_listed(&st.name) {
                w.line("");
                w.line(&format!("private static IReadOnlyList<{0}> ReadList({0}List list)", st.name));
                w.line("{");
                w.indent(|w| {
                    w.line(&format!("var items = new List<{}>(checked((int)list.Len));", st.name));
                    w.line("for (nuint i = 0; i < list.Len; i++) items.Add(Read(list.Ptr[i]));");
                    w.line("return items;");
                });
                w.line("}");
            }
        }
        w.line("");
        w.line(&format!("internal static void Send(IntPtr app, {} action)", bridge.action));
        w.line("{");
        w.indent(|w| {
            w.line("switch (action)");
            w.line("{");
            w.indent(|w| {
                for v in &bridge.variants {
                    let record = format!("{}.{}", bridge.action, v.name);
                    match v.payload {
                        None => {
                            w.line(&format!("case {record}:"));
                            w.indent(|w| {
                                w.line(&format!("{}(app, new ActionDto {{ Tag = {} }});", f("send"), v.tag));
                                w.line("break;");
                            });
                        }
                        Some(Scalar::Str) => {
                            w.line(&format!("case {record} a:"));
                            w.line("{");
                            w.indent(|w| {
                                w.line("var bytes = Encoding.UTF8.GetBytes(a.Value);");
                                w.line("fixed (byte* ptr = bytes)");
                                w.line("{");
                                w.indent(|w| {
                                    w.line(&format!("var dto = new ActionDto {{ Tag = {} }};", v.tag));
                                    w.line(&format!(
                                        "dto.Payload.{} = new NuiStr {{ Ptr = ptr, Len = (nuint)bytes.Length }};",
                                        v.name
                                    ));
                                    w.line(&format!("{}(app, dto);", f("send")));
                                });
                                w.line("}");
                                w.line("break;");
                            });
                            w.line("}");
                        }
                        Some(ty) => {
                            let value = if ty == Scalar::Bool { "a.Value ? (byte)1 : (byte)0" } else { "a.Value" };
                            w.line(&format!("case {record} a:"));
                            w.line("{");
                            w.indent(|w| {
                                w.line(&format!("var dto = new ActionDto {{ Tag = {} }};", v.tag));
                                w.line(&format!("dto.Payload.{} = {value};", v.name));
                                w.line(&format!("{}(app, dto);", f("send")));
                                w.line("break;");
                            });
                            w.line("}");
                        }
                    }
                }
            });
            w.line("}");
        });
        w.line("}");
        if bridge.has_strings() {
            w.line("");
            w.line("private static string Decode(NuiStr text) =>");
            w.indent(|w| w.line("text.Len == 0 ? \"\" : Encoding.UTF8.GetString(text.Ptr, checked((int)text.Len));"));
        }
    });
    w.line("}");
    w.finish()
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
