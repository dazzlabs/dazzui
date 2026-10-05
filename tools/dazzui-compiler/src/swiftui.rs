//! Lowers a page to SwiftUI.
//!
//! A page becomes a view of a state and a `send` for its actions, which
//! holds nothing and knows nothing of where either comes from: a host feeds
//! it from whatever it likes. A Rust page model becomes a store — a thin
//! `ObservableObject` over the UniFFI object, whose Swift (the state, the
//! actions, the source) UniFFI writes beside it — which is one such host.
//! SwiftUI's own layout views stay SwiftUI's; a DazzUI component is always
//! written `DazzUI.Button`, because DazzUI's names shadow SwiftUI's.

use crate::Registry;
use crate::ir::{
    ActionPayload, ActionRef, Component, ComponentSpec, Expr, GeneratedFile, Source,
    SwiftUIMapping, Trailing, UiNode, Value, camel_case,
};
use crate::{CompiledModel, CompiledPage};

pub fn generate(compiled: &CompiledPage) -> Vec<GeneratedFile> {
    vec![GeneratedFile::new(
        format!("{}.swift", compiled.page.name),
        view(compiled),
    )]
}

fn header(source: &str) -> String {
    format!("// AUTO-GENERATED FILE. DO NOT EDIT. Source: {source}\n")
}

fn view(compiled: &CompiledPage) -> String {
    let page = &compiled.page;
    let (state, action) = (&page.state_type, &page.action_type);
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line("import DazzUI");
    w.line("import SwiftUI");
    w.line("");
    w.line(&format!("public struct {}: View {{", page.name));
    w.indent(|w| {
        w.line(&format!("public let state: {state}"));
        w.line(&format!("public let send: @MainActor ({action}) -> Void"));
        if uses_theme(&page.root) {
            w.line("@Environment(\\.theme) private var theme");
        }
        w.line("");
        w.line(&format!(
            "public init(state: {state}, send: @escaping @MainActor ({action}) -> Void) {{"
        ));
        w.indent(|w| {
            w.line("self.state = state");
            w.line("self.send = send");
        });
        w.line("}");
        w.line("");
        w.line("public var body: some View {");
        w.indent(|w| node(w, &compiled.registry, &page.root));
        w.line("}");
    });
    w.line("}");
    w.finish()
}

/// Whether anything reads the theme's tokens: a styled text does.
fn uses_theme(node: &UiNode) -> bool {
    match node {
        UiNode::Component(c) => {
            (c.id.as_str() == "Text" && c.prop("style").is_some())
                || c.children.iter().any(uses_theme)
        }
        UiNode::If {
            then_branch,
            else_branch,
            ..
        } => then_branch.iter().chain(else_branch).any(uses_theme),
        UiNode::ForEach { children, .. } => children.iter().any(uses_theme),
    }
}

fn node(w: &mut Writer, r: &Registry, node: &UiNode) {
    let c = match node {
        UiNode::Component(c) => c,
        UiNode::If {
            condition,
            then_branch,
            else_branch,
            source,
        } => {
            w.line(&format!("// source: {}", source.short()));
            let not = if condition.negated { "!" } else { "" };
            w.line(&format!("if {not}{} {{", expr(&condition.value)));
            w.indent(|w| then_branch.iter().for_each(|n| self::node(w, r, n)));
            if else_branch.is_empty() {
                w.line("}");
            } else {
                w.line("} else {");
                w.indent(|w| else_branch.iter().for_each(|n| self::node(w, r, n)));
                w.line("}");
            }
            return;
        }
        UiNode::ForEach {
            collection,
            item,
            key,
            children,
            source,
            ..
        } => {
            w.line(&format!("// source: {}", source.short()));
            let key: Vec<String> = key.segments.iter().map(|s| camel_case(s)).collect();
            w.line(&format!(
                "ForEach({}, id: \\.{}) {{ {item} in",
                state(&collection.segments),
                key.join(".")
            ));
            w.indent(|w| children.iter().for_each(|n| self::node(w, r, n)));
            w.line("}");
            return;
        }
    };
    w.line(&format!("// source: {}", c.source.short()));
    // A mapping wins, so that an app's registry can replace even the
    // components this backend lowers itself.
    if let Some((spec, m)) = r
        .get(c.id.as_str())
        .and_then(|spec| spec.swiftui.as_ref().map(|m| (spec, m)))
    {
        return mapped(w, r, c, spec, m);
    }
    match c.id.as_str() {
        stack @ ("VStack" | "HStack") => {
            w.line(&format!("{stack}(spacing: {}) {{", int(c, "spacing")));
            w.indent(|w| c.children.iter().for_each(|child| self::node(w, r, child)));
            w.line("}");
        }
        "Text" => {
            w.line(&format!(
                "Text({})",
                expr(c.prop("text").expect("text is required"))
            ));
            if let Some(style) = c.prop("style") {
                let token = text_token(case(style));
                w.indent(|w| {
                    w.line(&format!(".font(theme.vars.{token}.font)"));
                    w.line(&format!(".lineSpacing(theme.vars.{token}.lineSpacing)"));
                });
            }
        }
        other => unreachable!("`{other}` passed the registry but has no SwiftUI lowering"),
    }
}

/// A piece of a call's argument list: written inline, or a closure that
/// holds a node.
enum Piece<'a> {
    Inline(String),
    Closure(&'a str, &'a UiNode),
}

/// A component its registry mapping lowers: `Call(arg: value, …) { … }`.
fn mapped(w: &mut Writer, r: &Registry, c: &Component, spec: &ComponentSpec, m: &SwiftUIMapping) {
    let mut pieces = Vec::new();
    for arg in &m.args {
        let piece = match &arg.source {
            Source::Prop {
                prop,
                cases,
                invert,
                omit,
            } => c
                .prop_value(prop)
                .filter(|e| !e.is_omitted(omit.as_ref()))
                .map(|e| Piece::Inline(format!("{}: {}", arg.arg, render(&e, cases, *invert)))),
            Source::Event(event) => c
                .event_action(spec, event)
                .map(|a| Piece::Inline(format!("{}: {}", arg.arg, closure(a)))),
            Source::Slot(slot) => c.slot(slot).map(|node| Piece::Closure(&arg.arg, node)),
        };
        match (piece, &arg.fallback) {
            (Some(piece), _) => pieces.push(piece),
            (None, Some(code)) => pieces.push(Piece::Inline(format!("{}: {code}", arg.arg))),
            (None, None) => {}
        }
    }

    let mut line = m.call.clone();
    if !pieces.is_empty() || m.trailing.is_none() {
        line.push('(');
        for (i, piece) in pieces.iter().enumerate() {
            if i > 0 {
                line += ", ";
            }
            match piece {
                Piece::Inline(code) => line += code,
                Piece::Closure(arg, node) => {
                    w.line(&format!("{line}{arg}: {{"));
                    w.indent(|w| self::node(w, r, node));
                    line = "}".into();
                }
            }
        }
        line.push(')');
    }
    match &m.trailing {
        None => w.line(&line),
        Some(trailing) => {
            w.line(&format!("{line} {{"));
            w.indent(|w| match trailing {
                Trailing::Children => c.children.iter().for_each(|child| self::node(w, r, child)),
                Trailing::Prop { prop, view } => {
                    if let Some(e) = c.prop_value(prop) {
                        w.line(&format!("{view}({})", expr(&e)));
                    }
                }
                Trailing::Slot(slot) => {
                    if let Some(node) = c.slot(slot) {
                        self::node(w, r, node);
                    }
                }
            });
            w.line("}");
        }
    }
}

fn render(e: &Expr, cases: &[(String, String)], invert: bool) -> String {
    match e {
        Expr::EnumCase { ty, case } => cases
            .iter()
            .find(|(c, _)| c == case)
            .map(|(_, code)| code.clone())
            .unwrap_or_else(|| panic!("the registry maps no SwiftUI code for `{ty}::{case}`")),
        Expr::Literal(Value::Bool(b)) if invert => (!b).to_string(),
        other if invert => format!("!{}", expr(other)),
        other => expr(other),
    }
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

/// `{ send(.save) }`; an event's own value is `$0`.
fn closure(action: &ActionRef) -> String {
    let case = camel_case(&action.variant);
    let payload = match &action.payload {
        ActionPayload::None => String::new(),
        ActionPayload::EventValue => "($0)".into(),
        ActionPayload::Expr(e) => format!("({})", expr(e)),
    };
    format!("{{ send(.{case}{payload}) }}")
}

fn expr(e: &Expr) -> String {
    match e {
        Expr::Literal(Value::String(s)) => swift_string(s),
        Expr::Literal(Value::Bool(b)) => b.to_string(),
        Expr::Literal(Value::Int(n)) => n.to_string(),
        Expr::State(path) => state(&path.segments),
        Expr::Local(path) => {
            let segments: Vec<String> = path.segments.iter().map(|s| camel_case(s)).collect();
            format!("{}.{}", path.local, segments.join("."))
        }
        Expr::EnumCase { ty, case } => {
            unreachable!("`{ty}::{case}` is lowered by the prop that takes it")
        }
    }
}

fn state(segments: &[String]) -> String {
    let path: Vec<String> = segments.iter().map(|s| camel_case(s)).collect();
    format!("state.{}", path.join("."))
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

/// `SettingsStore.swift`: the state of a Rust page model, observable, and
/// its `send`, for a host to hand a page.
pub fn store(model: &CompiledModel) -> GeneratedFile {
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
    w.line("import Combine");
    w.line("import Foundation");
    w.line("");
    w.line(&format!(
        "/// The latest snapshot of the model `{}` makes, and a way to send it",
        bridge.function
    ));
    w.line("/// actions: what a page takes. Whatever thread the model changes on, the");
    w.line("/// snapshot is read again on the main one.");
    w.line("@MainActor");
    w.line(&format!("public final class {store}: ObservableObject {{"));
    w.indent(|w| {
        w.line(&format!(
            "@Published public private(set) var state: {state}"
        ));
        w.line(&format!("private let source: {source}"));
        w.line("");
        w.line(&format!("public init(source: {source} = {source}()) {{"));
        w.indent(|w| {
            w.line("self.source = source");
            w.line("state = source.state()");
            w.line("source.subscribe(listener: Listener(store: self))");
        });
        w.line("}");
        w.line("");
        w.line(&format!("public func send(_ action: {action}) {{"));
        w.indent(|w| w.line("source.send(action: action)"));
        w.line("}");
        w.line("");
        w.line("fileprivate func refresh() {");
        w.indent(|w| w.line("state = source.state()"));
        w.line("}");
    });
    w.line("}");
    w.line("");
    w.line("/// Rust calls this back on the thread that changed the state. On the main");
    w.line("/// one, the store reads it at once, so a model that changes as it takes an");
    w.line("/// action is shown changed when `send` returns; from any other, it reads it");
    w.line("/// there next. It holds the store weakly: Rust holds it.");
    w.line(&format!(
        "private final class Listener: {listener}, @unchecked Sendable {{"
    ));
    w.indent(|w| {
        w.line(&format!("weak var store: {store}?"));
        w.line("");
        w.line(&format!("init(store: {store}) {{"));
        w.indent(|w| w.line("self.store = store"));
        w.line("}");
        w.line("");
        w.line("func stateChanged() {");
        w.indent(|w| {
            w.line("if Thread.isMainThread {");
            w.indent(|w| w.line("MainActor.assumeIsolated { store?.refresh() }"));
            w.line("} else {");
            w.indent(|w| {
                w.line("DispatchQueue.main.async { [weak self] in");
                w.indent(|w| w.line("MainActor.assumeIsolated { self?.store?.refresh() }"));
                w.line("}");
            });
            w.line("}");
        });
        w.line("}");
    });
    w.line("}");
    GeneratedFile::new(format!("{store}.swift"), w.finish())
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
