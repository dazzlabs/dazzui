import SwiftUI

/// The interaction state a ``Pressable`` hands its builder.
public struct PressableState: Sendable {
  public var hovered: Bool
  public var pressed: Bool
  public var enabled: Bool

  public init(hovered: Bool = false, pressed: Bool = false, enabled: Bool = true) {
    self.hovered = hovered
    self.pressed = pressed
    self.enabled = enabled
  }

  /// What the shared state machine resolves recipes for.
  public var controlState: ControlState {
    ControlState(hovered: hovered, pressed: pressed, enabled: enabled)
  }
}

/// The substrate every clickable component is built on.
///
/// It gives a view what the browser gives a `<button>` for free — pointer
/// cursor, hover state, press state, keyboard activation — which is the half
/// of `data-control` that CSS gets from pseudo-classes rather than from
/// tokens.
public struct Pressable<Content: View>: View {
  public var action: (() -> Void)?
  public var enabled: Bool
  public var cornerRadius: CGFloat
  public var help: String?
  @ViewBuilder public var content: (PressableState) -> Content

  @State private var hovered = false

  public init(
    action: (() -> Void)? = nil,
    enabled: Bool = true,
    cornerRadius: CGFloat = 0,
    help: String? = nil,
    @ViewBuilder content: @escaping (PressableState) -> Content
  ) {
    self.action = action
    self.enabled = enabled
    self.cornerRadius = cornerRadius
    self.help = help
    self.content = content
  }

  private var interactive: Bool { enabled && action != nil }

  public var body: some View {
    SwiftUI.Button(action: { action?() }) { EmptyView() }
      .buttonStyle(
        PressableButtonStyle(
          hovered: hovered && interactive,
          enabled: enabled,
          cornerRadius: cornerRadius,
          content: content
        )
      )
      .disabled(!interactive)
      .onHover { hovered = $0 }
      .help(help ?? "")
  }
}

/// Renders the content with the press state folded in, so no `onChange`
/// round-trip is needed to observe it.
private struct PressableButtonStyle<Content: View>: ButtonStyle {
  let hovered: Bool
  let enabled: Bool
  let cornerRadius: CGFloat
  @ViewBuilder let content: (PressableState) -> Content

  func makeBody(configuration: Configuration) -> some View {
    content(
      PressableState(hovered: hovered, pressed: configuration.isPressed, enabled: enabled)
    )
    .contentShape(RoundedRectangle(cornerRadius: cornerRadius, style: .continuous))
  }
}

/// Hover tracking without the button semantics — for a row that only changes
/// colour under the pointer.
public struct HoverRegion<Content: View>: View {
  public var enabled: Bool
  @ViewBuilder public var content: (Bool) -> Content

  @State private var hovered = false

  public init(enabled: Bool = true, @ViewBuilder content: @escaping (Bool) -> Content) {
    self.enabled = enabled
    self.content = content
  }

  public var body: some View {
    content(hovered && enabled)
      .onHover { hovered = $0 }
  }
}

public extension View {
  /// The keyboard focus ring: `:focus-visible { outline: 3px solid
  /// var(--focus-ring); outline-offset: 0 }`.
  ///
  /// It hugs the control with no gap, the way macOS draws it, and it is the
  /// system accent in every tint — it never follows `data-tint`.
  func focusRing(
    visible: Bool,
    color: Color,
    cornerRadius: CGFloat = 0,
    width: CGFloat = 3
  ) -> some View {
    overlay {
      if visible {
        // A stroke is centred on its path, so insetting by half the width
        // puts the whole ring outside the box — CSS's `outline-offset: 0`.
        RoundedRectangle(cornerRadius: cornerRadius + width, style: .continuous)
          .strokeBorder(color, lineWidth: width)
          .padding(-width)
      }
    }
  }

  /// A border on every edge, following the box's corner radius.
  func border(_ color: Color, width: CGFloat, cornerRadius: CGFloat) -> some View {
    overlay(
      RoundedRectangle(cornerRadius: cornerRadius, style: .continuous)
        .strokeBorder(color, lineWidth: width)
    )
  }

  /// A rule pinned to one edge — the separator under a toolbar, over a
  /// footer, or between rows.
  func rule(_ color: Color, edge: Edge, width: CGFloat) -> some View {
    overlay(alignment: edge.alignment) {
      Rectangle()
        .fill(color)
        .frame(
          width: edge == .leading || edge == .trailing ? width : nil,
          height: edge == .top || edge == .bottom ? width : nil
        )
    }
  }
}

public extension Edge {
  var alignment: Alignment {
    switch self {
    case .top: return .top
    case .bottom: return .bottom
    case .leading: return .leading
    case .trailing: return .trailing
    }
  }
}
