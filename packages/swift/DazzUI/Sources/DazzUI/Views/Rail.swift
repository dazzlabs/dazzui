import SwiftUI

/// The second navigation column, between the sidebar and the content: the
/// groups, books or pages inside whatever the sidebar selected. Lighter than
/// the sidebar, because it sits closer to the content.
public struct Rail<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    GeometryReader { proxy in
      ScrollView(.vertical) {
        VStack(alignment: .leading, spacing: vars.frameNavGap) {
          content()
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        // The column's inset is one value on every side. A group carries its
        // own leading step and CSS zeroes it on the first one with
        // `:first-child`. SwiftUI has no such selector, so the column hands
        // that much of its top padding over instead: a first group puts back
        // exactly what was given up — landing on the same 10 as the sides —
        // and every later group adds the step again on top of the gap.
        .padding(.horizontal, vars.spacing25)
        .padding(.top, vars.spacing25 - railGroupStep(vars))
        .padding(.bottom, vars.spacing25)
        // A run that does not fill the column still has to let a trailing
        // `NavAction` sink to the foot, which needs a height to sink through.
        .frame(minHeight: proxy.size.height, alignment: .top)
      }
    }
    .frame(width: vars.frameRailWidth)
    .background(vars.colorSurface)
    .rule(vars.colorBorder, edge: .trailing, width: hairline)
  }
}

/// A run of rail rows, optionally named.
///
/// The first run usually goes unlabelled: it is what the pane is already
/// called, and naming it twice says nothing. A second run is the one that
/// needs the label, because a gap alone leaves the reader to guess what the
/// rows below it have in common.
public struct RailGroup<Label: View, Content: View>: View {
  public var label: (() -> Label)?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(labelBuilder: label, content: content)
  }

  init(labelBuilder: (() -> Label)?, content: @escaping () -> Content) {
    self.label = labelBuilder
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.frameNavGap) {
      if let label {
        SectionLabel { label() }
          .padding(.vertical, vars.spacing1)
          .padding(.horizontal, vars.spacing25)
      }
      content()
    }
    .frame(maxWidth: .infinity, alignment: .leading)
    .padding(.top, railGroupStep(vars))
  }
}

public extension RailGroup where Label == Text {
  /// A group whose label is a plain string.
  init(_ title: String, @ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: { Text(title) }, content: content)
  }
}

public extension RailGroup where Label == EmptyView {
  /// An unnamed run of rows.
  init(@ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: nil, content: content)
  }
}

/// The step a rail group sets itself off by, over and above the column's gap.
private func railGroupStep(_ vars: ThemeVariables) -> CGFloat {
  vars.spacing15
}
