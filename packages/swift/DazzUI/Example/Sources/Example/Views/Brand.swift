import DazzUI
import SwiftUI

/// The name, and which of its implementations this is. Every port has an
/// example of its own, and a window of one looks like a window of the next.
struct Brand: View {
  let action: () -> Void
  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars
    SwiftUI.Button(action: action) {
      HStack(spacing: vars.spacing2) {
        RoundedRectangle(cornerRadius: vars.radiusSmall)
          .fill(vars.colorPrimary[500])
          .frame(width: vars.spacing5, height: vars.spacing5)
        Text("DazzUI")
          .font(vars.titleSmall.font)
          .foregroundStyle(vars.colorContent)
        Badge(size: .small, variant: .tinted) { Text("SwiftUI") }
      }
      .fixedSize()
    }
    .buttonStyle(.plain)
    .accessibilityLabel("DazzUI")
  }
}
