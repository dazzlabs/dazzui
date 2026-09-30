import FluentIcons
import SwiftUI

/// When the rule under the label is drawn.
///
/// `hover` is the default, because a page of prose with a rule under every link
/// reads as struck through. `always` is for a link that has to be findable
/// without a pointer — a list of references, a footer. `none` is for one whose
/// surroundings already say it is a link: a breadcrumb, a row that is entirely
/// a target.
public enum LinkUnderline: String, CaseIterable, Sendable {
  case hover
  case always
  case none
}

/// Text that goes somewhere.
///
/// It is the one interactive thing in this package that is genuinely inline,
/// which is why it exists beside `Button(variant: .plain)` rather than inside
/// it: a button keeps its box at every variant, and a box is exactly what a
/// link in a sentence must not have. So this takes the shared state machine for
/// its colour — `plain`'s content grade, a step darker on hover — and none of
/// its geometry.
public struct Link: View {
  public var title: String
  public var tint: Tint
  public var underline: LinkUnderline
  /// Goes somewhere outside this window, and draws the arrow that says so
  /// before the reader commits.
  public var external: Bool
  public var enabled: Bool
  public var action: (() -> Void)?

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    _ title: String,
    tint: Tint = .primary,
    underline: LinkUnderline = .hover,
    external: Bool = false,
    enabled: Bool = true,
    action: (() -> Void)? = nil
  ) {
    self.title = title
    self.tint = tint
    self.underline = underline
    self.external = external
    self.enabled = enabled
    self.action = action
  }

  public var body: some View {
    let vars = theme.vars

    Pressable(action: action ?? {}, enabled: enabled, cornerRadius: vars.radiusTiny) { state in
      // The `plain` recipe's content grade, and only its content: the wash that
      // recipe also carries would ragged the line it sits on, and the grade
      // change is what a link has always used to answer the pointer.
      let content = vars.resolve(
        vars.descriptor(.plain, .content),
        tint: tint,
        state: state.controlState
      )

      HStack(spacing: vars.spacing05) {
        Text(title)
          .underline(ruled(state: state))
        if external {
          Icon(.arrowUpRight12Regular, size: vars.iconSmall)
        }
      }
      .foregroundStyle(content)
      .animation(vars.motion, value: state.controlState)
    }
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: vars.radiusTiny,
      width: vars.focusWidth
    )
    .accessibilityAddTraits(.isLink)
  }

  private func ruled(state: PressableState) -> Bool {
    switch underline {
    case .always: return true
    case .hover: return state.hovered && state.enabled
    case .none: return false
    }
  }
}
