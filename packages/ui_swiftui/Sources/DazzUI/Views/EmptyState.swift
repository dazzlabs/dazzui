import SwiftUI

/// What a pane shows before it has content: one muted line and at most one
/// action.
///
/// The surrounding chrome already names the pane, so there is no heading, no
/// explainer copy, and no illustration.
public struct EmptyState<Action: View>: View {
  public var title: String
  /// The way out of the empty state — usually one button.
  @ViewBuilder public var action: () -> Action

  @Environment(\.theme) private var theme

  public init(title: String, @ViewBuilder action: @escaping () -> Action) {
    self.title = title
    self.action = action
  }

  public var body: some View {
    let vars = theme.vars

    VStack(spacing: vars.spacing3) {
      Text(title)
        .font(vars.bodyMedium.font)
        .lineSpacing(vars.bodyMedium.lineSpacing)
        .multilineTextAlignment(.center)
        .foregroundStyle(vars.colorContentSubtle)
      // A row of controls, so it takes the control gap rather than the block
      // gap.
      if Action.self != EmptyView.self {
        HStack(spacing: vars.controlMediumGap) { action() }
      }
    }
    .padding(.vertical, vars.spacing14)
    .padding(.horizontal, vars.spacing8)
    .frame(maxWidth: .infinity, maxHeight: .infinity)
  }
}

public extension EmptyState where Action == EmptyView {
  /// An empty state with no way out — a pane that fills itself.
  init(_ title: String) {
    self.init(title: title, action: { EmptyView() })
  }
}
