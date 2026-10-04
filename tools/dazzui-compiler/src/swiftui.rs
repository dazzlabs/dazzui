//! Lowers a page to SwiftUI.
//!
//! A page becomes the view; the view model it reads from and sends actions
//! through, with the state struct and action enum derived from the Rust
//! ones; the Swift side of the bridge; and the C module that bridge imports.
//! SwiftUI's own layout views stay SwiftUI's; a DazzUI component is always
//! written `DazzUI.Button`, because DazzUI's names shadow SwiftUI's.

use crate::CompiledPage;
use crate::bridge::{Bridge, BridgeStruct, FieldType, Scalar};
use crate::ir::{
    ActionPayload, ActionRef, Component, Expr, GeneratedFile, UiNode, Value, camel_case, page_stem,
};
use crate::rust_types::{FloatType, IntType};

pub fn generate(compiled: &CompiledPage) -> Vec<GeneratedFile> {
    let page = &compiled.page;
    let stem = page_stem(&page.name);
    let bridge = &compiled.bridge;
    let module = c_module(bridge);
    vec![
        GeneratedFile::new(format!("{}.swift", page.name), view(compiled)),
        GeneratedFile::new(format!("{stem}ViewModel.swift"), view_model(compiled)),
        GeneratedFile::new(format!("{}Bridge.swift", bridge.stem), bridge_glue(compiled)),
        GeneratedFile::new(format!("{module}/{}.h", bridge.prefix), bridge.c_header()),
        GeneratedFile::new(
            format!("{module}/module.modulemap"),
            format!("module {module} {{\n  header \"{}.h\"\n  export *\n}}\n", bridge.prefix),
        ),
    ]
}

/// `CSettings`: the Clang module the bridge's header is imported as.
pub fn c_module(bridge: &Bridge) -> String {
    format!("C{}", bridge.stem)
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
        if uses_theme(&page.root) {
            w.line("@Environment(\\.theme) private var theme");
        }
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

/// Whether anything reads the theme's tokens: a styled text does.
fn uses_theme(node: &UiNode) -> bool {
    match node {
        UiNode::Component(c) => {
            (c.id.as_str() == "Text" && c.prop("style").is_some()) || c.children.iter().any(uses_theme)
        }
        UiNode::If { then_branch, else_branch, .. } => then_branch.iter().chain(else_branch).any(uses_theme),
        UiNode::ForEach { children, .. } => children.iter().any(uses_theme),
    }
}

fn node(w: &mut Writer, node: &UiNode) {
    let c = match node {
        UiNode::Component(c) => c,
        UiNode::If { condition, then_branch, else_branch, source } => {
            w.line(&format!("// source: {}", source.short()));
            let not = if condition.negated { "!" } else { "" };
            w.line(&format!("if {not}{} {{", expr(&condition.value)));
            w.indent(|w| then_branch.iter().for_each(|n| self::node(w, n)));
            if else_branch.is_empty() {
                w.line("}");
            } else {
                w.line("} else {");
                w.indent(|w| else_branch.iter().for_each(|n| self::node(w, n)));
                w.line("}");
            }
            return;
        }
        UiNode::ForEach { collection, item, key, children, source, .. } => {
            w.line(&format!("// source: {}", source.short()));
            let key: Vec<String> = key.segments.iter().map(|s| camel_case(s)).collect();
            w.line(&format!("ForEach({}, id: \\.{}) {{ {item} in", state(&collection.segments), key.join(".")));
            w.indent(|w| children.iter().for_each(|n| self::node(w, n)));
            w.line("}");
            return;
        }
    };
    w.line(&format!("// source: {}", c.source.short()));
    match c.id.as_str() {
        stack @ ("VStack" | "HStack") => {
            w.line(&format!("{stack}(spacing: {}) {{", int(c, "spacing")));
            w.indent(|w| c.children.iter().for_each(|child| self::node(w, child)));
            w.line("}");
        }
        "Text" => {
            w.line(&format!("Text({})", expr(c.prop("text").expect("text is required"))));
            if let Some(style) = c.prop("style") {
                let token = text_token(case(style));
                w.indent(|w| {
                    w.line(&format!(".font(theme.vars.{token}.font)"));
                    w.line(&format!(".lineSpacing(theme.vars.{token}.lineSpacing)"));
                });
            }
        }
        "Button" => {
            let mut args = Vec::new();
            // DazzUI's Button is recessed unless told otherwise, as the
            // semantic default is, so only the other case is written.
            if case(c.prop("style").expect("style has a default")) == "Primary" {
                args.push("variant: .filled".to_string());
            }
            args.extend(enabled(c));
            let action = c.event("click").map(closure).unwrap_or_else(|| "{}".into());
            args.push(format!("action: {action}"));
            w.line(&format!("DazzUI.Button({}) {{", args.join(", ")));
            w.indent(|w| w.line(&format!("Text({})", expr(c.prop("title").expect("title is required")))));
            w.line("}");
        }
        "Toggle" => {
            // DazzUI's Switch takes its value and a change callback rather
            // than a `Binding`: the change goes to Rust, and the value comes
            // back in the next snapshot.
            let (value, change) = match c.binding("value") {
                Some(b) => (state(&b.path.segments), Some(&b.on_change)),
                None => (expr(c.prop("value").expect("value is required")), c.event("change")),
            };
            let mut args = vec![format!("checked: {value}")];
            args.extend(change.map(|a| format!("onCheckedChange: {}", closure(a))));
            args.extend(enabled(c));
            w.line(&format!("DazzUI.Switch({})", args.join(", ")));
        }
        other => unreachable!("`{other}` passed the registry but has no SwiftUI lowering"),
    }
}

/// `enabled: …` for a `disabled` prop, unless it is the literal `false`
/// that both the registry and DazzUI default to.
fn enabled(c: &Component) -> Option<String> {
    match c.prop("disabled").expect("disabled has a default") {
        Expr::Literal(Value::Bool(false)) => None,
        Expr::Literal(Value::Bool(true)) => Some("enabled: false".into()),
        other => Some(format!("enabled: !{}", expr(other))),
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

/// `{ model.send(.save) }`; an event's own value is `$0`.
fn closure(action: &ActionRef) -> String {
    let case = camel_case(&action.variant);
    let payload = match &action.payload {
        ActionPayload::None => String::new(),
        ActionPayload::EventValue => "($0)".into(),
        ActionPayload::Expr(e) => format!("({})", expr(e)),
    };
    format!("{{ model.send(.{case}{payload}) }}")
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
        Expr::EnumCase { ty, case } => unreachable!("`{ty}::{case}` is lowered by the prop that takes it"),
    }
}

fn state(segments: &[String]) -> String {
    let path: Vec<String> = segments.iter().map(|s| camel_case(s)).collect();
    format!("model.state.{}", path.join("."))
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

/// The Swift type a scalar is, which is also what Swift imports its C type
/// as.
fn swift_type(ty: Scalar) -> &'static str {
    match ty {
        Scalar::Bool => "Bool",
        Scalar::Int(IntType::I8) => "Int8",
        Scalar::Int(IntType::I16) => "Int16",
        Scalar::Int(IntType::I32) => "Int32",
        Scalar::Int(IntType::I64) => "Int64",
        Scalar::Int(IntType::U8) => "UInt8",
        Scalar::Int(IntType::U16) => "UInt16",
        Scalar::Int(IntType::U32) => "UInt32",
        Scalar::Int(IntType::U64) => "UInt64",
        Scalar::Int(IntType::Isize) => "Int",
        Scalar::Int(IntType::Usize) => "UInt",
        Scalar::Float(FloatType::F32) => "Float",
        Scalar::Float(FloatType::F64) => "Double",
        Scalar::Str => "String",
    }
}

/// The Swift type a field is.
fn field_type(ty: &FieldType) -> String {
    match ty {
        FieldType::Scalar(scalar) => swift_type(*scalar).into(),
        FieldType::Struct(name) => name.clone(),
        FieldType::List(name) => format!("[{name}]"),
    }
}

fn swift_struct(w: &mut Writer, bridge: &Bridge, st: &BridgeStruct) {
    if st.name == bridge.state {
        w.line(&format!("/// `{}`, as Rust last handed it over.", st.name));
    } else {
        w.line(&format!("/// `{}`, from the Rust app core.", st.name));
    }
    w.line(&format!("public struct {}: Equatable {{", st.name));
    w.indent(|w| {
        for field in &st.fields {
            w.line(&format!("public let {}: {}", camel_case(&field.name), field_type(&field.ty)));
        }
    });
    w.line("}");
}

fn view_model(compiled: &CompiledPage) -> String {
    let stem = page_stem(&compiled.page.name);
    let bridge = &compiled.bridge;
    let f = |name: &str| bridge.function(name);
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line(&format!("import {}", c_module(bridge)));
    w.line("import Combine");
    w.line("");
    for st in &bridge.structs {
        swift_struct(&mut w, bridge, st);
        w.line("");
    }
    w.line(&format!("public enum {}: Equatable {{", bridge.action));
    w.indent(|w| {
        for v in &bridge.variants {
            let payload = v.payload.map(|ty| format!("({})", swift_type(ty))).unwrap_or_default();
            w.line(&format!("case {}{payload}", camel_case(&v.name)));
        }
    });
    w.line("}");
    w.line("");
    w.line("/// Holds the latest snapshot of the Rust state and sends actions to");
    w.line(&format!("/// Rust's `{}`, which runs on the calling thread before `send` returns.", bridge.update));
    w.line("@MainActor");
    w.line(&format!("public final class {stem}ViewModel: ObservableObject {{"));
    w.indent(|w| {
        w.line(&format!("@Published public private(set) var state: {}", bridge.state));
        w.line("private let app: OpaquePointer");
        w.line("");
        w.line("public init() {");
        w.indent(|w| {
            w.line(&format!("let app: OpaquePointer = {}()", f("create")));
            w.line("self.app = app");
            w.line(&format!("state = {}(reading: app)", bridge.state));
            w.line(&format!(
                "{}(app, {}StateChanged, Unmanaged.passUnretained(self).toOpaque())",
                f("subscribe"),
                bridge.prefix
            ));
        });
        w.line("}");
        w.line("");
        w.line("deinit {");
        w.indent(|w| w.line(&format!("{}(app)", f("destroy"))));
        w.line("}");
        w.line("");
        w.line(&format!("public func send(_ action: {}) {{", bridge.action));
        w.indent(|w| w.line(&format!("action.withDto {{ {}(app, $0) }}", f("send"))));
        w.line("}");
        w.line("");
        w.line("fileprivate func refresh() {");
        w.indent(|w| w.line(&format!("state = {}(reading: app)", bridge.state)));
        w.line("}");
    });
    w.line("}");
    w.line("");
    w.line("/// Rust calls this back on the thread that sent the action, which for a");
    w.line("/// view model is the main one.");
    w.line(&format!(
        "private let {}StateChanged: @convention(c) (UnsafeMutableRawPointer?) -> Void = {{ context in",
        bridge.prefix
    ));
    w.indent(|w| {
        w.line("guard let context else { return }");
        w.line(&format!("let model = Unmanaged<{stem}ViewModel>.fromOpaque(context).takeUnretainedValue()"));
        w.line("MainActor.assumeIsolated { model.refresh() }");
    });
    w.line("}");
    w.finish()
}

fn bridge_glue(compiled: &CompiledPage) -> String {
    let bridge = &compiled.bridge;
    let stem = &bridge.stem;
    let mut w = Writer::default();
    w.raw(&header(&compiled.file));
    w.line("");
    w.line(&format!("import {}", c_module(bridge)));
    w.line("");
    w.line(&format!("extension {} {{", bridge.state));
    w.indent(|w| {
        w.line("/// Reads a snapshot from Rust and hands its memory back.");
        w.line("init(reading app: OpaquePointer) {");
        w.indent(|w| {
            w.line(&format!("let dto = {}(app)", bridge.function("get_state")));
            w.line(&format!("defer {{ {}(dto) }}", bridge.function("state_free")));
            w.line("self.init(dto: dto)");
        });
        w.line("}");
    });
    w.line("}");
    w.line("");
    for st in &bridge.structs {
        w.line(&format!("extension {} {{", st.name));
        w.indent(|w| {
            w.line(&format!("init(dto: {}) {{", bridge.dto(&st.name)));
            w.indent(|w| {
                w.line("self.init(");
                w.indent(|w| {
                    let n = st.fields.len();
                    for (i, field) in st.fields.iter().enumerate() {
                        let value = match &field.ty {
                            FieldType::Scalar(Scalar::Str) => format!("decode(dto.{})", field.name),
                            FieldType::Scalar(_) => format!("dto.{}", field.name),
                            FieldType::Struct(t) => format!("{t}(dto: dto.{})", field.name),
                            FieldType::List(t) => format!(
                                "UnsafeBufferPointer(start: dto.{0}.ptr, count: dto.{0}.len).map({t}.init(dto:))",
                                field.name
                            ),
                        };
                        let comma = if i + 1 < n { "," } else { "" };
                        w.line(&format!("{}: {value}{comma}", camel_case(&field.name)));
                    }
                });
                w.line(")");
            });
            w.line("}");
        });
        w.line("}");
        w.line("");
    }
    w.line(&format!("extension {} {{", bridge.action));
    w.indent(|w| {
        w.line("/// Lays the action out as Rust reads it, for the length of `body`.");
        w.line(&format!("func withDto(_ body: ({stem}ActionDto) -> Void) {{"));
        w.indent(|w| {
            w.line("switch self {");
            for v in &bridge.variants {
                let case = camel_case(&v.name);
                let dto = |payload: Option<String>| match (bridge.has_payloads(), payload) {
                    (false, _) => format!("{stem}ActionDto(tag: {})", v.tag),
                    (true, None) => format!("{stem}ActionDto(tag: {}, payload: {stem}ActionPayload())", v.tag),
                    (true, Some(p)) => format!(
                        "{stem}ActionDto(tag: {}, payload: {stem}ActionPayload({}: {p}))",
                        v.tag,
                        v.member()
                    ),
                };
                match v.payload {
                    None => {
                        w.line(&format!("case .{case}:"));
                        w.indent(|w| w.line(&format!("body({})", dto(None))));
                    }
                    Some(Scalar::Str) => {
                        w.line(&format!("case .{case}(var value):"));
                        w.indent(|w| {
                            w.line("value.withUTF8 { bytes in");
                            w.indent(|w| {
                                w.line(&format!(
                                    "body({})",
                                    dto(Some("NuiStr(ptr: bytes.baseAddress, len: bytes.count)".into()))
                                ))
                            });
                            w.line("}");
                        });
                    }
                    Some(_) => {
                        w.line(&format!("case .{case}(let value):"));
                        w.indent(|w| w.line(&format!("body({})", dto(Some("value".into())))));
                    }
                }
            }
            w.line("}");
        });
        w.line("}");
    });
    w.line("}");
    if bridge.has_strings() {
        w.line("");
        w.line("private func decode(_ text: NuiStr) -> String {");
        w.indent(|w| {
            w.line("guard let ptr = text.ptr, text.len > 0 else { return \"\" }");
            w.line("return String(decoding: UnsafeBufferPointer(start: ptr, count: text.len), as: UTF8.self)");
        });
        w.line("}");
    }
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
