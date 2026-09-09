import SwiftUI

/// The label, the hint, and whatever control sits between them.
///
/// The wrapper draws no box of its own — the control it holds does that — so
/// it owns only the space between the three and the type of the two it prints
/// itself. Erroring, the label and the hint join the control on the danger
/// ramp: the wrapper is re-tinted rather than given error colours of its own.
///
/// It is `FormField` rather than `Field` because in this library the `*Field`
/// suffix already belongs to controls — ``SearchField`` is one — and a wrapper
/// sharing that suffix would read like their base rather than their container.
public struct FormField<Content: View>: View {
  public var label: String
  /// Help text under the control; carries the error tone when erroring.
  public var hint: String?
  /// Identifies the field to the ``Form`` above it. Given one, a message the
  /// form was handed for this name replaces the hint and marks the field
  /// invalid — which is the whole reason a form is a view rather than a stack.
  public var name: String?
  public var state: TextFieldState
  public var tint: Tint
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.formErrors) private var formErrors

  public init(
    label: String,
    hint: String? = nil,
    name: String? = nil,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.label = label
    self.hint = hint
    self.name = name
    self.state = state
    self.tint = tint
    self.content = content
  }

  /// The message the form is holding for this field, if any.
  private var error: String? {
    name.flatMap { formErrors[$0] }
  }

  private var reportedState: TextFieldState {
    error == nil ? state : .error
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing15) {
      Text(label)
        .font(.system(size: vars.labelSmall.size, weight: vars.labelSmall.weight))
        .foregroundStyle(ink)

      content()

      if let hint = error ?? hint {
        Text(hint)
          .font(.system(size: vars.labelQuiet.size, weight: vars.labelQuiet.weight))
          // Prose leading rather than the label face's solid line: a hint is a
          // sentence and wraps like one.
          .lineSpacing(vars.labelQuiet.size * (Self.proseLeading - 1))
          .fixedSize(horizontal: false, vertical: true)
          .foregroundStyle(ink)
      }
    }
    .frame(maxWidth: .infinity, alignment: .leading)
  }

  /// One ink for both lines: subtle while the field is fine, and the accent's
  /// text grade off the re-tinted ramp once it is not.
  private var ink: Color {
    guard reportedState == .error else { return theme.vars.colorContentSubtle }
    return theme.vars.ramp(tintFor(reportedState, tint))[700]
  }

  /// `line-height: 1.6` — prose leading, stated as a multiple in the
  /// stylesheet rather than held as a token.
  private static var proseLeading: CGFloat { 1.6 }
}
