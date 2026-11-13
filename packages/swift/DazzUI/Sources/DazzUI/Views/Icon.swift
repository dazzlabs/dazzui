import FluentIcons

/// Draws a glyph at a given size, inheriting the surrounding text colour.
///
/// The set is Fluent, the same one `packages/js/dazzui` pulls from
/// `@fluentui/react-icons` — its components name specific glyphs
/// (`Checkmark12Filled`, `ChevronDown12Regular`), so the port names the same
/// ones rather than substituting a system symbol at a different weight.
///
/// The glyph is drawn as a template image, so `foregroundStyle` tints it the
/// way `currentColor` does on the web, and the fixed-size artwork scales to
/// `size`.
import SwiftUI

public struct Icon: View {
  public var image: Image
  /// Explicit edge length; nil follows the enclosing slot or theme.
  public var size: CGFloat?

  /// A Fluent glyph, scaled to the current theme unless explicitly sized.
  public init(_ icon: FluentIcon, size: CGFloat? = nil) {
    self.image = Image(fluent: icon)
    self.size = size
  }

  /// Any other artwork the host wants to draw at icon size, which is the
  /// counterpart of React's `IconGlyph` accepting any component that renders
  /// an `<svg>`.
  public init(_ image: Image, size: CGFloat? = nil) {
    self.image = image
    self.size = size
  }

  @Environment(\.theme) private var theme
  @Environment(\.dazzIconSize) private var inheritedSize

  public var body: some View {
    let edge = size ?? inheritedSize ?? theme.vars.iconMedium
    image
      .resizable()
      .renderingMode(.template)
      .scaledToFit()
      .frame(width: edge, height: edge)
      .accessibilityHidden(true)
  }
}
