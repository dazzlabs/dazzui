//! Lowers a page to SwiftUI.
//!
//! A page becomes a view of a state that calls a model's methods, as a
//! Flutter widget reads a `ValueListenable` and calls its model: it holds
//! `state` and `model`, the latter only as the protocol of the methods it
//! calls, so a host may hand it the Rust model or one of its own. A Rust
//! model becomes that protocol and a store — a thin `ObservableObject` over
//! the UniFFI object, whose Swift (the state, the object) UniFFI writes
//! beside it.
//!
//! Row, Col and Text are drawn as the SwiftUI example draws the React
//! example's stage components, with the theme's spacing and inks. A DazzUI
//! component is always written `DazzUI.Button`, because DazzUI's names
//! shadow SwiftUI's.

use crate::Registry;
use crate::ir::{
    Argument, Component, Expr, GeneratedFile, MethodCall, Source, SwiftUIMapping, Trailing, UiNode,
    Value, camel_case, model_stem,
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
    let state = &page.state_type;
    let actions = format!("{}Actions", model_stem(&page.model_type));
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line("import DazzUI");
    w.line("import SwiftUI");
    w.line("");
    w.line(&format!("public struct {}: View {{", page.name));
    w.indent(|w| {
        w.line(&format!("public let state: {state}"));
        w.line(&format!("public let model: any {actions}"));
        if uses_theme(&page.root) {
            w.line("@Environment(\\.theme) private var theme");
        }
        w.line("");
        w.line(&format!(
            "public init(state: {state}, model: any {actions}) {{"
        ));
        w.indent(|w| {
            w.line("self.state = state");
            w.line("self.model = model");
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

/// Whether anything reads the theme's tokens: a stage component does.
fn uses_theme(node: &UiNode) -> bool {
    match node {
        UiNode::Component(c) => {
            matches!(c.id.as_str(), "Row" | "Col" | "Text")
                || c.children.iter().any(uses_theme)
                || c.slots.iter().any(|s| uses_theme(&s.node))
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
    if let Some(m) = r.get(c.id.as_str()).and_then(|spec| spec.swiftui.as_ref()) {
        return mapped(w, r, c, m);
    }
    match c.id.as_str() {
        "Row" => {
            // A flex row centres its items across by default; HStack does too.
            let align = match case(c.prop("align")) {
                "Start" | "Stretch" => "alignment: .top, ",
                "End" => "alignment: .bottom, ",
                _ => "",
            };
            w.line(&format!("HStack({align}spacing: {}) {{", spacing(c)));
            w.indent(|w| c.children.iter().for_each(|child| self::node(w, r, child)));
            w.line("}");
        }
        "Col" => {
            let (align, stretch) = match case(c.prop("align")) {
                "Center" => (".center", false),
                "End" => (".trailing", false),
                "Start" => (".leading", false),
                _ => (".leading", true),
            };
            w.line(&format!(
                "VStack(alignment: {align}, spacing: {}) {{",
                spacing(c)
            ));
            w.indent(|w| c.children.iter().for_each(|child| self::node(w, r, child)));
            w.line("}");
            // A stretched column is as wide as the room, as a block is on the web.
            if stretch {
                w.indent(|w| w.line(".frame(maxWidth: .infinity, alignment: .leading)"));
            }
        }
        "Text" => {
            w.line(&text(&c.text));
            w.indent(|w| {
                if c.prop("strong") == Some(&Expr::Literal(Value::Bool(true))) {
                    w.line(".fontWeight(theme.vars.labelStrong.weight)");
                }
                match c.prop("tint") {
                    Some(tint) => w.line(&format!(
                        ".foregroundStyle(theme.vars.ramp(.{})[500])",
                        case(Some(tint)).to_lowercase()
                    )),
                    None if case(c.prop("tone")) == "Secondary" => {
                        w.line(".foregroundStyle(theme.vars.colorContentSecondary)")
                    }
                    None => {}
                }
            });
        }
        other => unreachable!("`{other}` passed the registry but has no SwiftUI lowering"),
    }
}

/// `theme.vars.spacing3` for `gap: "3"`; `spacing05` for `"0-5"`.
fn spacing(c: &Component) -> String {
    let Some(Expr::Literal(Value::String(step))) = c.prop("gap") else {
        unreachable!("`gap` on `{}` has a default", c.id.as_str())
    };
    let name = match step.as_str() {
        "px" => "Px".to_string(),
        step => step.replace('-', ""),
    };
    format!("theme.vars.spacing{name}")
}

/// `Text("Save")`, or runs joined: `Text("Saved ") + Text(state.when)`.
fn text(runs: &[Expr]) -> String {
    runs.iter()
        .map(|run| format!("Text({})", expr(run)))
        .collect::<Vec<_>>()
        .join(" + ")
}

/// A piece of a call's argument list: written inline, or a closure that
/// holds nodes.
enum Piece<'a> {
    Inline(String),
    Closure(&'a str, Vec<&'a UiNode>),
}

/// A component its registry mapping lowers: `Call(arg: value, …) { … }`.
fn mapped(w: &mut Writer, r: &Registry, c: &Component, m: &SwiftUIMapping) {
    let mut pieces = Vec::new();
    for arg in &m.args {
        let piece = match &arg.source {
            Source::Prop {
                prop,
                cases,
                invert,
                omit,
            } => c
                .prop(prop)
                .filter(|e| !e.is_omitted(omit.as_ref()))
                .map(|e| Piece::Inline(format!("{}: {}", arg.arg, render(e, cases, *invert)))),
            Source::Event(event) => c
                .event(event)
                .map(|call| Piece::Inline(format!("{}: {}", arg.arg, closure(call)))),
            Source::Slot(slot) => c
                .slot(slot)
                .map(|node| Piece::Closure(&arg.arg, vec![node])),
            Source::Children => Some(Piece::Closure(&arg.arg, c.children.iter().collect())),
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
                Piece::Closure(arg, nodes) => {
                    w.line(&format!("{line}{arg}: {{"));
                    w.indent(|w| {
                        if nodes.is_empty() {
                            w.line("EmptyView()");
                        }
                        nodes.iter().for_each(|node| self::node(w, r, node));
                    });
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
                Trailing::Text => w.line(&text(&c.text)),
                Trailing::Prop { prop, view } => {
                    if let Some(e) = c.prop(prop) {
                        w.line(&format!("{view}({})", expr(e)));
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

/// The case an enum prop holds; `""` for none.
fn case(expr: Option<&Expr>) -> &str {
    match expr {
        Some(Expr::EnumCase { case, .. }) => case,
        None => "",
        Some(other) => unreachable!("expected an enum case, found {other:?}"),
    }
}

/// `{ model.save() }`; an event's own value is `$0`, labelled as UniFFI
/// labels the method's parameter: `{ model.setOn(on: $0) }`.
fn closure(call: &MethodCall) -> String {
    let method = camel_case(&call.method);
    let label = call
        .param
        .as_deref()
        .map(|p| format!("{}: ", camel_case(p)))
        .unwrap_or_default();
    let argument = match &call.argument {
        Argument::None => String::new(),
        Argument::EventValue => format!("{label}$0"),
        Argument::Expr(e) => format!("{label}{}", expr(e)),
    };
    format!("{{ model.{method}({argument}) }}")
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

/// A Rust page model's Swift: `SettingsActions.swift`, the protocol of the
/// methods a page calls, which a host may implement itself; and
/// `SettingsStore.swift`, its value observable over the UniFFI object, which
/// has those methods.
pub fn model(model: &CompiledModel) -> Vec<GeneratedFile> {
    vec![actions(model), store(model)]
}

fn actions(model: &CompiledModel) -> GeneratedFile {
    let bridge = &model.bridge;
    let mut w = Writer::default();
    w.raw(&header(&model.file));
    w.line("");
    w.line(&format!(
        "/// The methods of `{}` a page calls. The Rust model's native object has",
        bridge.model
    ));
    w.line("/// them; a host, or a test, may implement them itself.");
    w.line(&format!("public protocol {} {{", bridge.actions()));
    w.indent(|w| {
        for m in &bridge.methods {
            let param = m
                .param
                .as_ref()
                .map(|(name, ty)| format!("{}: {}", camel_case(name), ty.swift()))
                .unwrap_or_default();
            w.line(&format!("func {}({param})", camel_case(&m.name)));
        }
    });
    w.line("}");
    GeneratedFile::new(format!("{}.swift", bridge.actions()), w.finish())
}

fn store(model: &CompiledModel) -> GeneratedFile {
    let bridge = &model.bridge;
    let (state, object, listener, store) = (
        &bridge.state,
        &bridge.model,
        bridge.listener(),
        bridge.store(),
    );
    let mut w = Writer::default();
    w.raw(&header(&model.file));
    w.line("");
    w.line("import Combine");
    w.line("import Foundation");
    w.line("");
    w.line(&format!("extension {object}: {} {{}}", bridge.actions()));
    w.line("");
    w.line(&format!(
        "/// The latest value of a `{object}`, observable, and the model itself: what"
    ));
    w.line("/// a page takes, as `ListenableBuilder` hands a widget. Whatever thread the");
    w.line("/// model changes on, the value is read again on the main one.");
    w.line("@MainActor");
    w.line(&format!("public final class {store}: ObservableObject {{"));
    w.indent(|w| {
        w.line(&format!(
            "@Published public private(set) var state: {state}"
        ));
        w.line(&format!("public let model: {object}"));
        w.line("");
        w.line(&format!("public init(model: {object} = {object}()) {{"));
        w.indent(|w| {
            w.line("self.model = model");
            w.line("state = model.value()");
            w.line("_ = model.addListener(listener: Listener(store: self))");
        });
        w.line("}");
        w.line("");
        w.line("fileprivate func refresh() {");
        w.indent(|w| w.line("state = model.value()"));
        w.line("}");
    });
    w.line("}");
    w.line("");
    w.line("/// Rust calls this back on the thread that changed the value. On the main");
    w.line("/// one, the store reads it at once, so a model that changes as a method");
    w.line("/// runs is shown changed when the call returns; from any other, it reads it");
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
        w.line("func changed() {");
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
