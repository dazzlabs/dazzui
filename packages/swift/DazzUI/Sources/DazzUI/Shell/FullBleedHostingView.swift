#if os(macOS)
  import AppKit
  import SwiftUI

  /// An `NSHostingView` that reports zero safe-area insets.
  ///
  /// Windows using `.fullSizeContentView` with a hidden, transparent titlebar
  /// draw their own top chrome strip in SwiftUI. AppKit still reports the
  /// titlebar band as a top safe-area inset, and on macOS 26+ that causes any
  /// scroll view touching the top edge to allocate an `NSScrollPocket` backdrop
  /// that offsets content and swallows clicks on toolbar controls. Zeroing
  /// `safeAreaInsets` opts out of automatic titlebar insetting so custom
  /// titlebar controls remain clickable.
  public final class FullBleedHostingView<Content: View>: NSHostingView<Content> {
    public override var safeAreaInsets: NSEdgeInsets {
      NSEdgeInsets(top: 0, left: 0, bottom: 0, right: 0)
    }
  }
#endif
