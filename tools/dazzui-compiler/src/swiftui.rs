//! Lowers a page to SwiftUI.
//!
//! A page becomes two files: the view, and the view model it reads from and
//! sends actions through, whose action enum is derived from the Rust one.
//! SwiftUI's own layout views stay SwiftUI's; a DazzUI component is always
//! written `DazzUI.Button`, because DazzUI's names shadow SwiftUI's.

use crate::{CompiledPage, EnumDef, Fields, RustType};
use crate::ir::{
    ActionPayload, ActionRef, Component, Expr, GeneratedFile, UiNode, Value, camel_case, page_stem,
};

pub fn generate(compiled: &CompiledPage) -> Vec<GeneratedFile> {
    let page = &compiled.page;
    let stem = page_stem(&page.name);
    vec![
        GeneratedFile::new(format!("{}.swift", page.name), view(compiled)),
        GeneratedFile::new(format!("{stem}ViewModel.swift"), view_model(compiled)),
    ]
}

fn header(source: &str) -> String {
    format!("// AUTO-GENERATED FILE. DO NOT EDIT. Source: {source}\n")
}

fn view(compiled: &CompiledPage) -> String {
    let page = &compiled.page;
    let model = format!("{}ViewModel", page_stem(&page.name));
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line("import DazzUI");
    w.line("import SwiftUI");
    w.line("");
    w.line(&format!("public struct {}: View {{", page.name));
    w.indent(|w| {
        w.line(&format!("@ObservedObject public var model: {model}"));
        w.line("");
        w.line(&format!("public init(model: {model}) {{"));
        w.indent(|w| w.line("self.model = model"));
        w.line("}");
        w.line("");
        w.line("public var body: some View {");
        w.indent(|w| node(w, &page.root));
        w.line("}");
    });
    w.line("}");
    w.finish()
}

fn node(w: &mut Writer, node: &UiNode) {
    let UiNode::Component(c) = node;
    w.line(&format!("// source: {}", c.source.short()));
    match c.id.as_str() {
        "VStack" => {
            w.line(&format!("VStack(spacing: {}) {{", int(c, "spacing")));
            w.indent(|w| c.children.iter().for_each(|child| self::node(w, child)));
            w.line("}");
        }
        "Text" => w.line(&format!("Text({})", string(c, "text"))),
        "Button" => {
            let action = c.event("click").map(send).unwrap_or_else(|| "{}".into());
            w.line(&format!("DazzUI.Button(action: {action}) {{"));
            w.indent(|w| w.line(&format!("Text({})", string(c, "title"))));
            w.line("}");
        }
        other => unreachable!("`{other}` passed the registry but has no SwiftUI lowering"),
    }
}

fn send(action: &ActionRef) -> String {
    let ActionPayload::None = action.payload;
    format!("{{ model.send(.{}) }}", camel_case(&action.variant))
}

fn string(c: &Component, prop: &str) -> String {
    match c.prop(prop) {
        Some(Expr::Literal(Value::String(s))) => swift_string(s),
        other => unreachable!("`{prop}` on `{}` is {other:?}", c.id.as_str()),
    }
}

fn int(c: &Component, prop: &str) -> i64 {
    match c.prop(prop) {
        Some(Expr::Literal(Value::Int(n))) => *n,
        other => unreachable!("`{prop}` on `{}` is {other:?}", c.id.as_str()),
    }
}

fn swift_string(s: &str) -> String {
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

fn view_model(compiled: &CompiledPage) -> String {
    let stem = page_stem(&compiled.page.name);
    let action = &compiled.actions.name;
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line("import Combine");
    w.line("");
    action_enum(&mut w, &compiled.actions);
    w.line("");
    w.line("/// Holds what the page reads and forwards what it sends. Until the");
    w.line("/// bridge to Rust is generated, the host supplies where actions go.");
    w.line("@MainActor");
    w.line(&format!("public final class {stem}ViewModel: ObservableObject {{"));
    w.indent(|w| {
        w.line(&format!("private let sink: ({action}) -> Void"));
        w.line("");
        w.line(&format!("public init(send: @escaping ({action}) -> Void) {{"));
        w.indent(|w| w.line("self.sink = send"));
        w.line("}");
        w.line("");
        w.line(&format!("public func send(_ action: {action}) {{"));
        w.indent(|w| w.line("sink(action)"));
        w.line("}");
    });
    w.line("}");
    w.finish()
}

fn action_enum(w: &mut Writer, def: &EnumDef) {
    w.line(&format!("public enum {}: Equatable {{", def.name));
    w.indent(|w| {
        for v in &def.variants {
            let case = camel_case(&v.name);
            let payload = match &v.fields {
                Fields::Unit => String::new(),
                Fields::Tuple(types) => {
                    format!("({})", types.iter().map(swift_type).collect::<Vec<_>>().join(", "))
                }
                Fields::Named(fields) => format!(
                    "({})",
                    fields
                        .iter()
                        .map(|(name, ty)| format!("{}: {}", camel_case(name), swift_type(ty)))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            };
            w.line(&format!("case {case}{payload}"));
        }
    });
    w.line("}");
}

fn swift_type(ty: &RustType) -> String {
    match ty {
        RustType::Bool => "Bool".into(),
        RustType::String => "String".into(),
        RustType::Int => "Int".into(),
        RustType::Float => "Double".into(),
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
            self.out += &"  ".repeat(self.depth);
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
