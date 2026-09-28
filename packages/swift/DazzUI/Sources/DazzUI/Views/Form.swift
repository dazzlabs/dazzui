import SwiftUI

/// The run of fields, and the one thing above them that knows about all of
/// them: ``errors``.
///
/// That is what it is for. A stack of ``FormField``s already lays out; what it
/// cannot do is take what the server said back — `["email": "Already taken"]`
/// — and put each message under the field it belongs to. A field inside a form
/// needs only its name.
///
/// It also owns the rhythm between the fields, which is the other thing no
/// single field can decide. It draws nothing: a form is not a surface —
/// whatever it is inside is.
public struct Form<Content: View>: View {
  /// Validation messages returned from outside — a server, a form action —
  /// keyed by the name a ``FormField`` carries.
  public var errors: [String: String]
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    errors: [String: String] = [:],
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.errors = errors
    self.content = content
  }

  public var body: some View {
    VStack(alignment: .leading, spacing: theme.vars.spacing4) {
      content()
    }
    .environment(\.formErrors, errors)
  }
}

/// A run of fields that are one question between them — a date's three boxes,
/// an address, a set of radios.
///
/// The legend is the group's accessible name, which is the whole point: it is
/// announced before each field inside, so "Street" is heard as "Billing
/// address, Street". A heading in a `Text` above the fields looks the same and
/// says none of that.
///
/// It is not the way to head a *section* of unrelated fields — that is a
/// ``SectionLabel``, or a ``PreferenceSection``. A fieldset claims the fields
/// inside it belong together.
public struct Fieldset<Content: View>: View {
  /// Names the group. It is the fieldset's accessible name as well.
  public var legend: String?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(legend: String? = nil, @ViewBuilder content: @escaping () -> Content) {
    self.legend = legend
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing3) {
      if let legend {
        // The section label's voice: it names a group rather than labelling a
        // control, so it takes the caption face and not the field label's.
        Text(legend)
          .font(.system(size: vars.captionSmall.size, weight: vars.labelSmall.weight))
          .foregroundStyle(vars.colorContentSubtle)
      }
      content()
    }
    .accessibilityElement(children: .contain)
    .accessibilityLabel(legend ?? "")
  }
}

private struct FormErrorsKey: EnvironmentKey {
  static let defaultValue: [String: String] = [:]
}

public extension EnvironmentValues {
  /// What a named ``FormField`` reads to find its own message.
  var formErrors: [String: String] {
    get { self[FormErrorsKey.self] }
    set { self[FormErrorsKey.self] = newValue }
  }
}
