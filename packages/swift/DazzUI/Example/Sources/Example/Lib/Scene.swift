import DazzUI
import FluentIcons
import SwiftUI

/// A scene's slots: the values its `useState`s hold.
@MainActor
final class SceneState: ObservableObject {
  @Published private(set) var slots: [String: Value]

  init(_ slots: [String: Value]) {
    self.slots = slots
  }

  func write(_ slot: String, _ value: Value) {
    slots[slot] = value
  }
}

/// Draws one ``Scene``: its slots, held here, and its tree.
struct SceneView: View {
  let scene: Scene
  @StateObject private var state: SceneState

  init(scene: Scene) {
    self.scene = scene
    _state = StateObject(wrappedValue: SceneState(scene.state))
  }

  var body: some View {
    NodeView(scene.content).environmentObject(state)
  }
}

private struct TriggerPressKey: EnvironmentKey {
  static let defaultValue: (() -> Void)? = nil
}

private struct SceneLineBoxKey: EnvironmentKey {
  static let defaultValue = false
}

extension EnvironmentValues {
  var sceneLineBox: Bool {
    get { self[SceneLineBoxKey.self] }
    set { self[SceneLineBoxKey.self] = newValue }
  }

  /// The press a trigger is given — the button a dialog, a drawer or a
  /// popover opens from never declares its own.
  var triggerPress: (() -> Void)? {
    get { self[TriggerPressKey.self] }
    set { self[TriggerPressKey.self] = newValue }
  }
}

/// One node of a scene, drawn. Every component is a view of its own, so it
/// reads the environment where it actually stands — the press its trigger
/// was given, the toaster its provider holds.
struct NodeView: View {
  let node: Value

  init(_ node: Value) {
    self.node = node
  }

  @EnvironmentObject private var scene: SceneState
  @Environment(\.toaster) private var toaster
  @Environment(\.triggerPress) private var press
  @Environment(\.theme) private var theme
  @Environment(\.sceneLineBox) private var lineBox

  var body: some View {
    let context = Node.Context(scene: scene, toaster: toaster, press: press)
    switch node {
    case .string(let text):
      textRun(text)
    case .object(let object) where object["$icon"] != nil:
      Icon(glyph(node))
    case .object(let object) where object["$el"] != nil:
      let parsed = Node(node, context)
      if let build = components[parsed.component] {
        build(parsed)
          .environment(\.sceneLineBox, false)
          .specimen(object["$at"]?.number.map { Int($0) })
      } else {
        fatalError("No SwiftUI mapping for <\(parsed.component)>")
      }
    case .object where isLive(node):
      textRun(context.resolve(node).text)
    default:
      EmptyView()
    }
  }

  @ViewBuilder private func textRun(_ text: String) -> some View {
    if lineBox {
      Text(text).lineBox(theme.vars.bodyMedium)
    } else {
      Text(text)
    }
  }
}

/// Whether a value is only known while the scene runs.
func isLive(_ value: Value) -> Bool {
  guard case .object(let object) = value else { return false }
  return ["$state", "$eq", "$not", "$if", "$concat", "$arg"].contains {
    object[$0] != nil
  }
}

/// The glyph an `$icon` names.
func glyph(_ value: Value) -> FluentIcon {
  guard let name = value["$icon"]?.string, let glyph = glyphs[name] else {
    fatalError("No glyph for \(value)")
  }
  return glyph
}

/// A component in the tree, read the way its builder needs it.
struct Node {
  struct Context {
    let scene: SceneState
    let toaster: Toaster?
    let press: (() -> Void)?

    /// A prop's value now: a slot read, a test worked out, an argument taken.
    @MainActor
    func resolve(_ value: Value, _ args: [Value] = []) -> Value {
      guard case .object(let object) = value, isLive(value) else { return value }
      if let slot = object["$state"]?.string { return scene.slots[slot] ?? .null }
      if let index = object["$arg"]?.number {
        let at = Int(index)
        return at < args.count ? args[at] : .null
      }
      if let pair = object["$eq"]?.array, pair.count == 2 {
        return .bool(resolve(pair[0], args) == resolve(pair[1], args))
      }
      if let test = object["$not"] { return .bool(!resolve(test, args).truthy) }
      if let branches = object["$if"]?.array, branches.count == 3 {
        return resolve(branches[0], args).truthy
          ? resolve(branches[1], args) : resolve(branches[2], args)
      }
      let parts = object["$concat"]?.array ?? []
      return .string(parts.map { resolve($0, args).text }.joined())
    }

    /// Makes the writes an event declares: slots set, toasts raised.
    @MainActor
    func run(_ action: Value, _ args: [Value] = []) {
      for step in action["$do"]?.array ?? [] {
        if let slot = step["set"]?.string {
          scene.write(slot, resolve(step["to"] ?? .null, args))
        } else if let toast = step["toast"] {
          let act = toast["action"]
          toaster?.add(
            ToastOptions(
              title: resolve(toast["title"] ?? .null, args).text,
              tint: Tint(rawValue: resolve(toast["tint"] ?? .null, args).string ?? "")
                ?? .neutral,
              action: act.map { act in
                ToastAction(act["label"]?.text ?? "") {
                  run(act["onClick"] ?? .null)
                }
              }
            )
          )
        }
      }
    }
  }

  let component: String
  let props: [String: Value]
  let children: [Value]
  let context: Context

  init(_ json: Value, _ context: Context) {
    component = json["$el"]?.string ?? ""
    props = json["props"]?.object ?? [:]
    children = json["children"]?.array ?? []
    self.context = context
  }

  private init(_ node: Node, children: [Value]) {
    component = node.component
    props = node.props
    self.children = children
    context = node.context
  }

  /// The same component with some of its children taken out.
  func with(children: [Value]) -> Node { Node(self, children: children) }

  func has(_ name: String) -> Bool { props[name] != nil }

  /// A prop's current value.
  @MainActor
  subscript(_ name: String) -> Value {
    context.resolve(props[name] ?? .null)
  }

  @MainActor
  func string(_ name: String) -> String? {
    let value = self[name]
    return value == .null ? nil : value.text
  }

  @MainActor
  func flag(_ name: String) -> Bool { self[name].truthy }

  @MainActor
  func number(_ name: String) -> Double? { self[name].number }

  /// A string prop read as one of a kit enum's cases, or the fallback.
  @MainActor
  func option<E: RawRepresentable>(_ name: String, _ fallback: E) -> E
  where E.RawValue == String {
    string(name).flatMap(E.init(rawValue:)) ?? fallback
  }

  /// A prop that holds a component, drawn.
  func element(_ name: String) -> NodeView? {
    props[name].map(NodeView.init)
  }

  /// The children, drawn.
  var kids: some View {
    ForEach(Array(children.enumerated()), id: \.offset) { NodeView($0.element) }
  }

  /// The children as one string — the label of a button, the body of a
  /// callout.
  @MainActor
  var text: String {
    children.map { $0.string ?? context.resolve($0).text }.joined()
  }

  /// An event with no argument, or nil when there is none.
  @MainActor
  func action(_ name: String) -> (() -> Void)? {
    guard let value = props[name] else { return nil }
    return { context.run(value) }
  }

  /// An event that passes what changed.
  @MainActor
  func handler(_ name: String) -> ((Value) -> Void)? {
    guard let value = props[name] else { return nil }
    return { context.run(value, [$0]) }
  }

  /// What a press does: a trigger's own, the story's event, or nothing — a
  /// React button with no handler is still a button.
  @MainActor
  func press(_ event: String = "onClick") -> () -> Void {
    context.press ?? action(event) ?? {}
  }
}

/// Local state for a prop a story leaves to the component — a
/// `defaultValue`, a `defaultChecked` — since the kit's views are
/// controlled.
struct Held<T, Content: View>: View {
  @State private var value: T
  let content: (Binding<T>) -> Content

  init(_ initial: T, @ViewBuilder content: @escaping (Binding<T>) -> Content) {
    _value = State(initialValue: initial)
    self.content = content
  }

  var body: some View { content($value) }
}

/// A value a story either binds (`value` and its change event) or leaves to
/// the component (`default…`). Bound, it reads and writes the scene's slot;
/// left, it is held here.
@MainActor
func bound<T, Content: View>(
  _ node: Node,
  value: String = "value",
  onChange: String? = nil,
  fallback: String? = nil,
  initial: T,
  read: @escaping (Value) -> T,
  write: @escaping (T) -> Value,
  @ViewBuilder content: @escaping (Binding<T>) -> Content
) -> AnyView {
  if node.has(value) {
    let binding = Binding<T>(
      get: { read(node[value]) },
      set: { next in
        if let onChange { node.handler(onChange)?(write(next)) }
      }
    )
    return AnyView(content(binding))
  }
  let start = fallback.map { node[$0] } ?? .null
  return AnyView(
    Held(start == .null ? initial : read(start)) { held in
      content(
        Binding(
          get: { held.wrappedValue },
          set: { next in
            held.wrappedValue = next
            if let onChange { node.handler(onChange)?(write(next)) }
          }
        )
      )
    }
  )
}
