//! Lowers a page to WinUI 3, in XAML and C#.
//!
//! A page becomes three files: the XAML, its code-behind — a constructor
//! that takes the view model and one handler per event, each forwarding an
//! action — and the view model with the action records derived from the
//! Rust enum. DazzUI controls come from `xmlns:dazz="using:DazzUI"`.

use crate::{CompiledPage, EnumDef, Fields, RustType};
use crate::ir::{
    ActionPayload, ActionRef, Component, Expr, GeneratedFile, UiNode, Value, page_stem, pascal_case,
};

pub fn generate(compiled: &CompiledPage, namespace: &str) -> Vec<GeneratedFile> {
    let page = &compiled.page;
    let stem = page_stem(&page.name);
    let mut handlers = Vec::new();
    let xaml = xaml(compiled, namespace, &mut handlers);
    vec![
        GeneratedFile::new(format!("{}.xaml", page.name), xaml),
        GeneratedFile::new(format!("{}.xaml.cs", page.name), code_behind(compiled, namespace, &handlers)),
        GeneratedFile::new(format!("{stem}ViewModel.cs"), view_model(compiled, namespace)),
    ]
}

/// An event handler in the code-behind: its name, the event args it takes,
/// and the action it sends.
struct Handler {
    name: String,
    args: &'static str,
    action: ActionRef,
}

fn xaml(compiled: &CompiledPage, namespace: &str, handlers: &mut Vec<Handler>) -> String {
    let page = &compiled.page;
    let mut w = Writer::default();
    w.line(&format!("<!-- AUTO-GENERATED FILE. DO NOT EDIT. Source: {} -->", compiled.file));
    w.line("<Page");
    w.indent(|w| {
        w.line(&format!("x:Class=\"{namespace}.{}\"", page.name));
        w.line("xmlns=\"http://schemas.microsoft.com/winfx/2006/xaml/presentation\"");
        w.line("xmlns:x=\"http://schemas.microsoft.com/winfx/2006/xaml\"");
        w.line("xmlns:dazz=\"using:DazzUI\">");
        node(w, &page.root, handlers);
    });
    w.line("</Page>");
    w.finish()
}

fn node(w: &mut Writer, node: &UiNode, handlers: &mut Vec<Handler>) {
    let UiNode::Component(c) = node;
    w.line(&format!("<!-- source: {} -->", c.source.short()));
    match c.id.as_str() {
        "VStack" => {
            w.line(&format!("<StackPanel Spacing=\"{}\">", int(c, "spacing")));
            w.indent(|w| c.children.iter().for_each(|child| self::node(w, child, handlers)));
            w.line("</StackPanel>");
        }
        "Text" => w.line(&format!("<TextBlock Text={}/>", attr(&string(c, "text")))),
        "Button" => {
            // DazzUI's C# Button defaults to `Filled` where React and SwiftUI
            // default to `Recessed`. The semantic default is `Recessed`, so
            // it is written out rather than left to the control.
            let mut tag = format!("<dazz:Button Content={} Variant=\"Recessed\"", attr(&string(c, "title")));
            if let Some(action) = c.event("click") {
                let name = handler(handlers, action, "Click", "RoutedEventArgs");
                tag += &format!(" Click=\"{name}\"");
            }
            w.line(&(tag + "/>"));
        }
        other => unreachable!("`{other}` passed the registry but has no WinUI lowering"),
    }
}

/// The handler for `action` on `event`, `OnSaveClick`, added once however
/// many controls send the same action.
fn handler(handlers: &mut Vec<Handler>, action: &ActionRef, event: &str, args: &'static str) -> String {
    let name = format!("On{}{event}", action.variant);
    if !handlers.iter().any(|h| h.name == name) {
        handlers.push(Handler { name: name.clone(), args, action: action.clone() });
    }
    name
}

fn string(c: &Component, prop: &str) -> String {
    match c.prop(prop) {
        Some(Expr::Literal(Value::String(s))) => s.clone(),
        other => unreachable!("`{prop}` on `{}` is {other:?}", c.id.as_str()),
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

fn header(source: &str) -> String {
    format!("// AUTO-GENERATED FILE. DO NOT EDIT. Source: {source}\n")
}

fn code_behind(compiled: &CompiledPage, namespace: &str, handlers: &[Handler]) -> String {
    let page = &compiled.page;
    let model = format!("{}ViewModel", page_stem(&page.name));
    let action = &compiled.actions.name;
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line("using Microsoft.UI.Xaml;");
    w.line("using Microsoft.UI.Xaml.Controls;");
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
        });
        w.line("}");
        for h in handlers {
            let ActionPayload::None = h.action.payload;
            w.line("");
            w.line(&format!(
                "private void {}(object sender, {} e) => ViewModel.Send(new {action}.{}());",
                h.name, h.args, h.action.variant
            ));
        }
    });
    w.line("}");
    w.finish()
}

fn view_model(compiled: &CompiledPage, namespace: &str) -> String {
    let stem = page_stem(&compiled.page.name);
    let action = &compiled.actions.name;
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line("using System;");
    w.line("");
    w.line(&format!("namespace {namespace};"));
    w.line("");
    action_records(&mut w, &compiled.actions);
    w.line("");
    w.line("/// <summary>Holds what the page reads and forwards what it sends. Until the");
    w.line("/// bridge to Rust is generated, the host supplies where actions go.</summary>");
    w.line(&format!("public sealed class {stem}ViewModel"));
    w.line("{");
    w.indent(|w| {
        w.line(&format!("private readonly Action<{action}> sink;"));
        w.line("");
        w.line(&format!("public {stem}ViewModel(Action<{action}> send) => sink = send;"));
        w.line("");
        w.line(&format!("public void Send({action} action) => sink(action);"));
    });
    w.line("}");
    w.finish()
}

fn action_records(w: &mut Writer, def: &EnumDef) {
    w.line(&format!("public abstract record {}", def.name));
    w.line("{");
    w.indent(|w| {
        w.line(&format!("private {}() {{ }}", def.name));
        for v in &def.variants {
            let params = match &v.fields {
                Fields::Unit => String::new(),
                Fields::Tuple(types) if types.len() == 1 => format!("{} Value", cs_type(&types[0])),
                Fields::Tuple(types) => types
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| format!("{} Item{}", cs_type(ty), i + 1))
                    .collect::<Vec<_>>()
                    .join(", "),
                Fields::Named(fields) => fields
                    .iter()
                    .map(|(name, ty)| format!("{} {}", cs_type(ty), pascal_case(name)))
                    .collect::<Vec<_>>()
                    .join(", "),
            };
            w.line("");
            w.line(&format!("public sealed record {}({params}) : {};", v.name, def.name));
        }
    });
    w.line("}");
}

fn cs_type(ty: &RustType) -> String {
    match ty {
        RustType::Bool => "bool".into(),
        RustType::String => "string".into(),
        RustType::Int => "long".into(),
        RustType::Float => "double".into(),
        RustType::Other(name) => name.clone(),
    }
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
