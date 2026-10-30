import SwiftUI

/// What the window draws over all of itself: a dialog, a drawer, the
/// toasts. A view deep in the hierarchy cannot draw past its own frame, so
/// it hands the layer to the window instead, the way the web portals one to
/// the document's body.
@MainActor
public final class OverlayStack: ObservableObject {
  public struct Layer: Identifiable {
    public let id: UUID
    let content: AnyView
  }

  @Published public private(set) var layers: [Layer] = []

  public init() {}

  /// Shows a layer, or replaces the one already shown under this id.
  public func show(_ id: UUID, _ content: AnyView) {
    let layer = Layer(id: id, content: content)
    if let index = layers.firstIndex(where: { $0.id == id }) {
      layers[index] = layer
    } else {
      layers.append(layer)
    }
  }

  public func hide(_ id: UUID) {
    layers.removeAll { $0.id == id }
  }
}

private struct OverlayStackKey: EnvironmentKey {
  static let defaultValue: OverlayStack? = nil
}

/// Closes the dialog or drawer the view is drawn in.
public struct CloseOverlayAction {
  let action: () -> Void

  public init(_ action: @escaping () -> Void) {
    self.action = action
  }

  public func callAsFunction() { action() }
}

private struct CloseOverlayKey: EnvironmentKey {
  static let defaultValue: CloseOverlayAction? = nil
}

public extension EnvironmentValues {
  /// The window's overlay stack, when it has one.
  var overlayStack: OverlayStack? {
    get { self[OverlayStackKey.self] }
    set { self[OverlayStackKey.self] = newValue }
  }

  /// Closes the dialog or drawer this view is drawn in; nil outside one.
  var closeOverlay: CloseOverlayAction? {
    get { self[CloseOverlayKey.self] }
    set { self[CloseOverlayKey.self] = newValue }
  }
}

public extension View {
  /// Makes this view — the window's content — the one dialogs, drawers and
  /// toasts are drawn over.
  func overlayHost() -> some View {
    modifier(OverlayHost())
  }

  /// Presents a dialog over the window, on the scrim, while `isPresented`
  /// holds. A press on the scrim or Escape dismisses it unless
  /// `dismissible` is false; a view inside can close it through
  /// `\.closeOverlay`.
  func dialog<Panel: View>(
    isPresented: Binding<Bool>,
    dismissible: Bool = true,
    @ViewBuilder content: @escaping () -> Panel
  ) -> some View {
    modifier(
      Presenting(isPresented: isPresented, dismissible: dismissible) { close in
        DialogScrim {
          ZStack {
            Color.clear
              .contentShape(Rectangle())
              .onTapGesture { if dismissible { close() } }
            content()
          }
        }
      }
    )
  }

  /// Presents a drawer over the window, from `side`, on a scrim that
  /// dismisses it.
  func drawer<Panel: View>(
    isPresented: Binding<Bool>,
    dismissible: Bool = true,
    @ViewBuilder content: @escaping () -> Panel
  ) -> some View {
    modifier(
      Presenting(isPresented: isPresented, dismissible: dismissible) { close in
        DrawerLayer(dismissible: dismissible, close: close, content: content)
      }
    )
  }
}

private struct OverlayHost: ViewModifier {
  @StateObject private var stack = OverlayStack()

  func body(content: Content) -> some View {
    content
      .environment(\.overlayStack, stack)
      .overlay {
        ZStack {
          ForEach(stack.layers) { layer in layer.content }
        }
      }
  }
}

private struct DrawerLayer<Panel: View>: View {
  let dismissible: Bool
  let close: () -> Void
  let content: () -> Panel

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars
    ZStack {
      vars.dialogScrimColor
        .opacity(vars.dialogScrimAlpha)
        .ignoresSafeArea()
        .contentShape(Rectangle())
        .onTapGesture { if dismissible { close() } }
      content()
    }
  }
}

/// Compare the theme's resolved sizing inputs together, then pass the new
/// snapshot into onChange. Reading the modifier's captured environment there
/// can otherwise replay the previous render's values into the portal.
private struct OverlayScope: Equatable {
  let theme: Theme
  let presentation: MenuPresentation

  static func == (lhs: Self, rhs: Self) -> Bool {
    lhs.presentation == rhs.presentation && lhs.theme.name == rhs.theme.name
      && lhs.theme.sizing == rhs.theme.sizing
      && lhs.theme.vars.bodyMedium.size == rhs.theme.vars.bodyMedium.size
      && ControlSize.allCases.allSatisfy {
        lhs.theme.vars.metrics($0).size == rhs.theme.vars.metrics($0).size
          && lhs.theme.vars.metrics($0).content.size == rhs.theme.vars.metrics($0).content.size
      }
  }
}

private struct Presenting<Layer: View>: ViewModifier {
  @Binding var isPresented: Bool
  let dismissible: Bool
  let layer: (_ close: @escaping () -> Void) -> Layer

  @Environment(\.overlayStack) private var stack
  @Environment(\.theme) private var theme
  @Environment(\.menuPresentation) private var presentation
  @State private var id = UUID()

  func body(content: Content) -> some View {
    content
      .onAppear { sync(isPresented) }
      .onChange(of: isPresented) { sync($0) }
      .onChange(of: OverlayScope(theme: theme, presentation: presentation)) { scope in
        if isPresented { sync(true, scope: scope) }
      }
      .onDisappear { stack?.hide(id) }
  }

  private func sync(_ shown: Bool, scope: OverlayScope? = nil) {
    let scope = scope ?? OverlayScope(theme: theme, presentation: presentation)
    let resolved = scope.theme
    guard let stack else { return }
    guard shown else {
      stack.hide(id)
      return
    }
    let binding = $isPresented
    let close = { binding.wrappedValue = false }
    let drawn = layer(close)
      .environment(\.theme, resolved)
      .font(resolved.vars.bodyMedium.font)
      .environment(\.colorScheme, resolved.name.colorScheme)
      .menuPresentation(scope.presentation)
      .environment(\.closeOverlay, CloseOverlayAction(close))
    #if os(macOS)
      stack.show(id, AnyView(drawn.onExitCommand { if dismissible { close() } }))
    #else
      stack.show(id, AnyView(drawn))
    #endif
  }
}
