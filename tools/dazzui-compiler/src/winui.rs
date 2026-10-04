//! Lowers a page to WinUI 3, in XAML and C#.
//!
//! A page becomes four files: the XAML; its code-behind — a constructor
//! that takes the view model, and one handler per event, each forwarding an
//! action; the view model, with the state record and action records derived
//! from the Rust types; and the C# side of the bridge. DazzUI controls come
//! from `xmlns:dazz="using:DazzUI"`.
//!
//! State is read with `{x:Bind …, Mode=OneWay}` and never `TwoWay`: a
//! change goes to Rust as an action, and the value comes back in the next
//! snapshot, so the C# copy of the state is never changed on its own.

use crate::bridge::{Bridge, Scalar};
use crate::ir::{
    ActionPayload, ActionRef, Component, Expr, GeneratedFile, UiNode, Value, page_stem, pascal_case,
};
use crate::rust_types::{FloatType, IntType};
use crate::{CompiledPage, Options};

pub fn generate(compiled: &CompiledPage, options: &Options) -> Vec<GeneratedFile> {
    let page = &compiled.page;
    let stem = page_stem(&page.name);
    let mut cx = Cx::default();
    let xaml = xaml(compiled, &options.namespace, &mut cx);
    vec![
        GeneratedFile::new(format!("{}.xaml", page.name), xaml),
        GeneratedFile::new(format!("{}.xaml.cs", page.name), code_behind(compiled, &options.namespace, &cx)),
        GeneratedFile::new(format!("{stem}ViewModel.cs"), view_model(compiled, &options.namespace)),
        GeneratedFile::new(format!("{}Bridge.cs", compiled.bridge.stem), bridge_glue(compiled, options)),
    ]
}

/// What the XAML needs from its code-behind.
#[derive(Default)]
struct Cx {
    handlers: Vec<Handler>,
    /// Named texts and the type token each is set in once loaded.
    styled: Vec<(String, &'static str)>,
    /// Whether `Not(…)` is bound to.
    negates: bool,
}

/// An event handler in the code-behind: its name, the event args it takes,
/// the action it sends, and how the event's own value is read.
struct Handler {
    name: String,
    args: &'static str,
    action: ActionRef,
    event_value: Option<&'static str>,
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
        w.line("xmlns:dazz=\"using:DazzUI\">");
        node(w, &page.root, cx);
    });
    w.line("</Page>");
    w.finish()
}

fn node(w: &mut Writer, node: &UiNode, cx: &mut Cx) {
    let UiNode::Component(c) = node;
    w.line(&format!("<!-- source: {} -->", c.source.short()));
    match c.id.as_str() {
        stack @ ("VStack" | "HStack") => {
            let orientation = if stack == "HStack" { " Orientation=\"Horizontal\"" } else { "" };
            w.line(&format!("<StackPanel{orientation} Spacing=\"{}\">", int(c, "spacing")));
            w.indent(|w| c.children.iter().for_each(|child| self::node(w, child, cx)));
            w.line("</StackPanel>");
        }
        "Text" => {
            let mut tag = format!("<TextBlock Text={}", value(c.prop("text").expect("text is required")));
            if let Some(style) = c.prop("style") {
                // A type token is not a XAML resource, so the code-behind
                // sets the face once the page knows its theme.
                let name = format!("Text{}", cx.styled.len() + 1);
                tag += &format!(" x:Name=\"{name}\"");
                cx.styled.push((name, text_token(case(style))));
            }
            w.line(&(tag + "/>"));
        }
        "Button" => {
            // DazzUI's C# Button defaults to `Filled` where React and SwiftUI
            // default to `Recessed`. The semantic default is `Recessed`, so
            // the variant is always written rather than left to the control.
            let variant = match case(c.prop("style").expect("style has a default")) {
                "Primary" => "Filled",
                _ => "Recessed",
            };
            let title = value(c.prop("title").expect("title is required"));
            let mut tag = format!("<dazz:Button Content={title} Variant=\"{variant}\"");
            tag += &enabled(c, cx);
            if let Some(action) = c.event("click") {
                let name = handler(cx, action, "Click", "RoutedEventArgs", None);
                tag += &format!(" Click=\"{name}\"");
            }
            w.line(&(tag + "/>"));
        }
        "Toggle" => {
            let (checked, change) = match c.binding("value") {
                Some(b) => (bind(&b.path.segments), Some(&b.on_change)),
                None => (value(c.prop("value").expect("value is required")), c.event("change")),
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
            w.line(&(tag + "/>"));
        }
        other => unreachable!("`{other}` passed the registry but has no WinUI lowering"),
    }
}

/// ` IsEnabled=…` for a `disabled` prop, unless it is the literal `false`
/// that both the registry and the control default to.
fn enabled(c: &Component, cx: &mut Cx) -> String {
    match c.prop("disabled").expect("disabled has a default") {
        Expr::Literal(Value::Bool(false)) => String::new(),
        Expr::Literal(Value::Bool(true)) => " IsEnabled=\"False\"".into(),
        Expr::State(path) => {
            cx.negates = true;
            format!(" IsEnabled=\"{{x:Bind Not({}), Mode=OneWay}}\"", state(&path.segments))
        }
        other => unreachable!("`disabled` is {other:?}"),
    }
}

/// The handler for `action` on `event`, `OnSaveClick`: one per distinct
/// action, however many controls send it.
fn handler(
    cx: &mut Cx,
    action: &ActionRef,
    event: &str,
    args: &'static str,
    event_value: Option<&'static str>,
) -> String {
    if let Some(h) = cx.handlers.iter().find(|h| &h.action == action && h.event_value == event_value) {
        return h.name.clone();
    }
    let base = format!("On{}{event}", action.variant);
    let mut name = base.clone();
    let mut n = 1;
    while cx.handlers.iter().any(|h| h.name == name) {
        n += 1;
        name = format!("{base}{n}");
    }
    cx.handlers.push(Handler { name: name.clone(), args, action: action.clone(), event_value });
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

/// A quoted attribute value: a literal, or a one-way binding to the state.
fn value(e: &Expr) -> String {
    match e {
        Expr::Literal(Value::String(s)) => attr(s),
        Expr::Literal(Value::Bool(b)) => format!("\"{}\"", if *b { "True" } else { "False" }),
        Expr::Literal(Value::Int(n)) => format!("\"{n}\""),
        Expr::State(path) => bind(&path.segments),
        Expr::EnumCase { ty, case } => unreachable!("`{ty}::{case}` is lowered by the prop that takes it"),
    }
}

fn bind(segments: &[String]) -> String {
    format!("\"{{x:Bind {}, Mode=OneWay}}\"", state(segments))
}

/// `ViewModel.State.LaunchAtStartup`
fn state(segments: &[String]) -> String {
    let path: Vec<String> = segments.iter().map(|s| pascal_case(s)).collect();
    format!("ViewModel.State.{}", path.join("."))
}

/// A C# expression for `e`, in the code-behind.
fn cs_expr(e: &Expr) -> String {
    match e {
        Expr::Literal(Value::String(s)) => cs_string(s),
        Expr::Literal(Value::Bool(b)) => b.to_string(),
        Expr::Literal(Value::Int(n)) => n.to_string(),
        Expr::State(path) => state(&path.segments),
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
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    if !cx.styled.is_empty() {
        w.line("using DazzUI;");
    }
    w.line("using Microsoft.UI.Xaml;");
    w.line("using Microsoft.UI.Xaml.Controls;");
    if cx.handlers.iter().any(|h| h.event_value.is_some()) {
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
            if !cx.styled.is_empty() {
                w.line("Loaded += (_, _) =>");
                w.line("{");
                w.indent(|w| {
                    for (name, token) in &cx.styled {
                        w.line(&format!("SetFace({name}, \"{token}\");"));
                    }
                });
                w.line("};");
            }
        });
        w.line("}");
        for h in &cx.handlers {
            let payload = match &h.action.payload {
                ActionPayload::None => String::new(),
                ActionPayload::EventValue => h.event_value.expect("an event with a value").to_string(),
                ActionPayload::Expr(e) => cs_expr(e),
            };
            w.line("");
            w.line(&format!(
                "private void {}(object sender, {} e) => ViewModel.Send(new {action}.{}({payload}));",
                h.name, h.args, h.action.variant
            ));
        }
        if cx.negates {
            w.line("");
            w.line("public static bool Not(bool value) => !value;");
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

/// The type a DTO holds a scalar in: blittable, so a `bool` is a byte.
fn dto_type(ty: Scalar) -> &'static str {
    match ty {
        Scalar::Bool => "byte",
        Scalar::Str => "NuiStr",
        other => cs_type(other),
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
    w.line("using System.ComponentModel;");
    w.line("using System.Runtime.CompilerServices;");
    w.line("using System.Runtime.InteropServices;");
    w.line("");
    w.line(&format!("namespace {namespace};"));
    w.line("");
    w.line(&format!("/// <summary><c>{}</c>, as Rust last handed it over.</summary>", bridge.state));
    let params: Vec<String> =
        bridge.fields.iter().map(|f| format!("{} {}", cs_type(f.ty), Bridge::property(&f.name))).collect();
    w.line(&format!("public sealed record {}({});", bridge.state, params.join(", ")));
    w.line("");
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
        w.line("");
        w.line("[StructLayout(LayoutKind.Sequential)]");
        w.line("internal struct StateDto");
        w.line("{");
        w.indent(|w| {
            for field in &bridge.fields {
                w.line(&format!("public {} {};", dto_type(field.ty), Bridge::property(&field.name)));
            }
        });
        w.line("}");
        if bridge.has_payloads() {
            w.line("");
            w.line("[StructLayout(LayoutKind.Explicit)]");
            w.line("internal struct ActionPayload");
            w.line("{");
            w.indent(|w| {
                for v in &bridge.variants {
                    if let Some(ty) = v.payload {
                        w.line(&format!("[FieldOffset(0)] public {} {};", dto_type(ty), v.name));
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
            w.line("try");
            w.line("{");
            w.indent(|w| {
                let values: Vec<String> = bridge
                    .fields
                    .iter()
                    .map(|field| {
                        let p = Bridge::property(&field.name);
                        match field.ty {
                            Scalar::Bool => format!("dto.{p} != 0"),
                            Scalar::Str => format!("Decode(dto.{p})"),
                            _ => format!("dto.{p}"),
                        }
                    })
                    .collect();
                w.line(&format!("return new {}({});", bridge.state, values.join(", ")));
            });
            w.line("}");
            w.line(&format!("finally {{ {}(dto); }}", f("state_free")));
        });
        w.line("}");
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
