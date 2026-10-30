#if canImport(AppKit)
  import AppKit
#endif
import SwiftUI

/// The menu a right-click opens, over the thing that was clicked.
///
/// It is the same menu a button opens — the panel and the rows are
/// ``MenuSheet`` and ``MenuRow``, so this shares the drawing rather than
/// resembling it. What is its own is where it opens: at the pointer rather
/// than against a trigger's edge.
///
/// The secondary click is caught here so the popup follows the subtree's
/// `menuPresentation`: DazzUI's sheet by default, or AppKit's menu in native
/// mode. Every affordance in a context menu must also be reachable
/// somewhere a pointer can see: a right-click is a shortcut, never the only
/// door.
public struct ContextMenu<Content: View>: View {
  public var items: [MenuItem]
  /// Which colour ramp the row wash resolves against.
  public var tint: Tint
  /// The region that opens the menu. It is drawn as given — the catcher sits
  /// behind it — so a row, a card or a canvas keeps its own layout.
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.menuPresentation) private var presentation
  @State private var open = false
  @State private var at: CGPoint = .zero

  public init(
    items: [MenuItem],
    tint: Tint = .neutral,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.items = items
    self.tint = tint
    self.content = content
  }

  public var body: some View {
    content()
      .background(SecondaryClickCatcher { point in
        at = point
        open = true
      })
      .overlay(alignment: .topLeading) {
        if open && !presentation.usesNativeMenus {
          MenuSheet {
            ForEach(items) { item in
              MenuRow(item: item, tint: tint) { open = false }
            }
          }
          // At the pointer, not against an edge: the offset is the press's own
          // position inside this region.
          .offset(x: at.x, y: at.y)
        }
      }
      .nativeMenu(isPresented: $open, entries: items.map(NativeMenuEntry.item), point: at)
      .background {
        if open && !presentation.usesNativeMenus {
          SwiftUI.Button("") { open = false }
            .keyboardShortcut(.cancelAction)
            .opacity(0)
            .frame(width: 0, height: 0)
        }
      }
  }
}

#if canImport(AppKit)
  /// Catches the secondary click SwiftUI has no gesture for.
  ///
  /// It draws nothing and sits behind the region, so what it changes is only
  /// which events reach the view: a right-click reports its position in the
  /// region's own coordinates, and everything else passes through.
  private struct SecondaryClickCatcher: NSViewRepresentable {
    let onSecondaryClick: (CGPoint) -> Void

    func makeNSView(context: Context) -> CatcherView {
      let view = CatcherView()
      view.onSecondaryClick = onSecondaryClick
      return view
    }

    func updateNSView(_ view: CatcherView, context: Context) {
      view.onSecondaryClick = onSecondaryClick
    }

    final class CatcherView: NSView {
      var onSecondaryClick: ((CGPoint) -> Void)?

      override func rightMouseDown(with event: NSEvent) {
        let point = convert(event.locationInWindow, from: nil)
        // AppKit's origin is the bottom-left; SwiftUI's is the top-left.
        onSecondaryClick?(CGPoint(x: point.x, y: bounds.height - point.y))
      }

      override func hitTest(_ point: NSPoint) -> NSView? {
        // Only the secondary button is ours; a primary press belongs to
        // whatever the region actually holds.
        NSApp.currentEvent?.type == .rightMouseDown ? self : nil
      }
    }
  }
#else
  /// Nothing to catch off AppKit: the menu is opened by the host instead.
  private struct SecondaryClickCatcher: View {
    let onSecondaryClick: (CGPoint) -> Void

    var body: some View { Color.clear }
  }
#endif
