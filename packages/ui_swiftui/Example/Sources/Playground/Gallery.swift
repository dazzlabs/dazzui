import DazzUI
import SwiftUI

/// The playground's own presentation — a heading and a frame around a row of
/// specimens. It draws with the design's tokens like everything else, so a
/// theme switch carries the page furniture with it.
struct Specimen<Content: View>: View {
  let title: String
  var note: String?
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(_ title: String, note: String? = nil, @ViewBuilder content: @escaping () -> Content) {
    self.title = title
    self.note = note
    self.content = content
  }

  var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing2) {
      HStack(spacing: vars.spacing2) {
        Text(title)
          .font(vars.labelLarge.font)
          .foregroundStyle(vars.colorContent)

        if let note {
          Text(note)
            .font(vars.captionSmall.font)
            .foregroundStyle(vars.colorContentSubtle)
        }
      }

      content()
    }
  }
}

/// A page heading: what this page is, and the one sentence saying why the
/// components on it belong together.
struct PageTitle: View {
  let title: String
  let subtitle: String

  @Environment(\.theme) private var theme

  init(_ title: String, _ subtitle: String) {
    self.title = title
    self.subtitle = subtitle
  }

  var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing1) {
      Text(title)
        .font(vars.headlineSmall.font)
        .foregroundStyle(vars.colorContent)

      Text(subtitle)
        .font(vars.bodyMedium.font)
        .lineSpacing(vars.bodyMedium.lineSpacing)
        .foregroundStyle(vars.colorContentMuted)
    }
  }
}

/// A row of specimens that wraps rather than clipping, which is what a
/// gallery of variants wants at any window width.
struct Row<Content: View>: View {
  var spacing: CGFloat?
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(spacing: CGFloat? = nil, @ViewBuilder content: @escaping () -> Content) {
    self.spacing = spacing
    self.content = content
  }

  var body: some View {
    HStack(alignment: .center, spacing: spacing ?? theme.vars.spacing2) {
      content()
    }
  }
}
