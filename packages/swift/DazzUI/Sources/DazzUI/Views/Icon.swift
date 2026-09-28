import FluentIcons
import SwiftUI

/// Draws a glyph at a given size, inheriting the surrounding text colour.
///
/// The set is Fluent, the same one `packages/js/dazzui-react` pulls from
/// `@fluentui/react-icons` — its components name specific glyphs
/// (`Checkmark12Filled`, `ChevronDown12Regular`), so the port names the same
/// ones rather than substituting a system symbol at a different weight.
///
/// The glyph is drawn as a template image, so `foregroundStyle` tints it the
/// way `currentColor` does on the web, and the fixed-size artwork scales to
/// `size`.
public struct Icon: View {
  public var image: Image
  /// Rendered edge length. Fixed-size glyphs are scaled to fit.
  public var size: CGFloat

  /// A Fluent glyph. The set carries its size in the name — `checkmark12Filled`
  /// is the 12pt cut — so `size` normally repeats it rather than rescaling.
  public init(_ icon: FluentIcon, size: CGFloat = 20) {
    self.image = Image(fluent: icon)
    self.size = size
  }

  /// Any other artwork the host wants to draw at icon size, which is the
  /// counterpart of React's `IconGlyph` accepting any component that renders
  /// an `<svg>`.
  public init(_ image: Image, size: CGFloat = 20) {
    self.image = image
    self.size = size
  }

  public var body: some View {
    image
      .resizable()
      .renderingMode(.template)
      .scaledToFit()
      .frame(width: size, height: size)
      .accessibilityHidden(true)
  }
}
