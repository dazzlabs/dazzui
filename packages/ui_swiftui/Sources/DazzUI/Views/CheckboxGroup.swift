import SwiftUI

/// Several checkboxes that are one answer, and the parent that turns them all
/// on or off.
///
/// The parent is a slot of its own rather than a flag on a child, and that is
/// what makes the indent honest: the group knows whether it has one, so it
/// knows whether the rest are *under* it. What the parent shows —
/// ``Checkbox/indeterminate`` when only some are checked — and what pressing
/// it does are the host's, which already holds the set.
///
/// The gap is tighter than a form's run between fields: these are the options
/// of a single question, and air between them would read as separate
/// questions.
public struct CheckboxGroup<Parent: View, Content: View>: View {
  @ViewBuilder public var parent: () -> Parent
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    @ViewBuilder parent: @escaping () -> Parent,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.parent = parent
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing25) {
      parent()
      VStack(alignment: .leading, spacing: vars.spacing25) {
        content()
      }
      // Where the group has a parent, its children are indented to where the
      // parent's label starts — the box plus the row's gap. The indent is what
      // says these are under it; the wiring alone is invisible. The medium box
      // is the measure because a parent and its children are one list, and a
      // mixed-size list is not a case this indent has to serve.
      .padding(.leading, Parent.self == EmptyView.self ? 0 : indent)
    }
    .accessibilityElement(children: .contain)
  }

  private var indent: CGFloat {
    theme.vars.checkboxMediumBox + theme.vars.spacing25
  }
}

public extension CheckboxGroup where Parent == EmptyView {
  /// A group with no parent: several boxes that are one answer, and nothing
  /// above them counting.
  init(@ViewBuilder content: @escaping () -> Content) {
    self.init(parent: { EmptyView() }, content: content)
  }
}
