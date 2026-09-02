import FluentIcons
import SwiftUI

/// The text controls' own three sizes, each of which binds one of the shared
/// `[data-size]` control profiles.
public enum TextFieldSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large

  /// The shared size profile the box's geometry and type come off.
  public var controlSize: ControlSize {
    switch self {
    case .small: return .small
    case .medium: return .medium
    case .large: return .large
    }
  }
}

/// Whether the control is reporting a problem.
public enum TextFieldState: String, CaseIterable, Sendable {
  case `default`
  case error
}

/// An erroring control is re-tinted rather than given a second palette, so its
/// border, wash, ink and focus glow all move together.
public func tintFor(_ state: TextFieldState, _ tint: Tint) -> Tint {
  state == .error ? .danger : tint
}

// MARK: - The shared box

/// The colours the text controls' shared box paints in one state.
///
/// The box is a recessed card — one step below the paper — that lifts to the
/// paper on focus behind an accent border and a soft accent wash. That
/// card→paper transition *is* the focus signal; a field that stays white can
/// only signal with its ring.
struct FieldStyle: Equatable {
  let surface: Color
  let border: Color
  let content: Color
  let glow: Color
  let glowing: Bool

  init(vars: ThemeVariables, tint: Tint, invalid: Bool, enabled: Bool, focused: Bool) {
    let ramp = vars.ramp(tint)

    // The stylesheet writes base, `:focus`, `[aria-invalid]` and `:disabled`
    // at equal specificity in that order, so the later rule wins: disabled
    // outranks the error tone, which outranks the focused lift.
    if !enabled {
      self.surface = vars.resolve(
        vars.descriptor(.normal, .surface),
        tint: tint,
        state: .disabled
      )
      self.content = vars.resolve(
        vars.descriptor(.normal, .content),
        tint: tint,
        state: .disabled
      )
    } else if invalid {
      self.surface = ramp[50]
      self.content = ramp[800]
    } else {
      self.surface = focused ? vars.colorSurface : vars.colorSurfaceMuted
      self.content = vars.colorContent
    }

    self.border = invalid || (focused && enabled) ? ramp[600] : vars.colorBorderStrong
    // The error rule replaces no box-shadow, so a focused field glows whatever
    // tone it is in — and erroring, it is already on the danger ramp.
    self.glow = ramp[600].opacity(vars.focusGlowAlpha)
    self.glowing = focused && enabled
  }

  /// Entered text is body weight rather than the profile's label weight — a
  /// query typed in semibold reads as shouting — and takes the code face
  /// where the value is a key, an ID, or a URL.
  static func font(size: CGFloat, mono: Bool, vars: ThemeVariables) -> Font {
    .system(size: size, weight: vars.bodySmall.weight, design: mono ? .monospaced : .default)
  }
}

extension View {
  /// The box the text controls share, and share deliberately: a Select is an
  /// TextField with a chevron and a FieldValue is one that was never editable, so
  /// the three drifting apart would be the bug rather than the feature.
  func fieldChrome(
    _ style: FieldStyle,
    cornerRadius: CGFloat,
    vars: ThemeVariables
  ) -> some View {
    self
      .foregroundStyle(style.content)
      .background(style.surface)
      .clipShape(RoundedRectangle(cornerRadius: cornerRadius, style: .continuous))
      .border(style.border, width: vars.strokeHairline, cornerRadius: cornerRadius)
      // A wash rather than the system ring: the field's focus signal is its
      // own tint, which is what makes an erroring field glow danger.
      .focusRing(
        visible: style.glowing,
        color: style.glow,
        cornerRadius: cornerRadius,
        width: vars.focusWidth
      )
      .animation(vars.motion, value: style)
  }
}

// MARK: - TextField

/// A single-line text control.
///
/// Named `TextField` rather than prefixed, and reached as
/// `DazzUI.TextField` where `SwiftUI.TextField` is also in scope.
public struct TextField: View {
  @Binding public var text: String
  public var placeholder: String
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  /// Set the mono face — for a key, an ID, or a URL.
  public var mono: Bool
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    text: Binding<String>,
    placeholder: String = "",
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    mono: Bool = false,
    enabled: Bool = true
  ) {
    self._text = text
    self.placeholder = placeholder
    self.size = size
    self.state = state
    self.tint = tint
    self.mono = mono
    self.enabled = enabled
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)
    let ramp = tintFor(state, tint)

    ZStack(alignment: .leading) {
      if text.isEmpty {
        Text(placeholder).foregroundStyle(vars.colorContentFaint)
      }
      SwiftUI.TextField("", text: $text)
        .textFieldStyle(.plain)
        .focused($focused)
        .disabled(!enabled)
    }
    .font(FieldStyle.font(size: metrics.content.size, mono: mono, vars: vars))
    .lineLimit(1)
    .padding(.horizontal, vars.spacing3)
    // Single-line controls take the shared control height, so a form can mix
    // them with buttons without the rows stepping. It wins over the profile's
    // block padding, which is what the stylesheet's `height` does too.
    .frame(height: metrics.size)
    // The box takes the regular container corner, one step above the buttons'.
    .fieldChrome(
      FieldStyle(
        vars: vars,
        tint: ramp,
        invalid: state == .error,
        enabled: enabled,
        focused: focused
      ),
      cornerRadius: vars.radiusMedium,
      vars: vars
    )
  }
}

// MARK: - Textarea

/// A growing text control.
///
/// It takes the container corner rather than the control corner — a theme that
/// makes controls pills would otherwise turn a multi-line box into a lozenge —
/// a floor rather than a height, and real leading: its text wraps, and the
/// control line-height would set it solid.
public struct Textarea: View {
  @Binding public var text: String
  public var placeholder: String
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  /// Set the mono face — for a key, an ID, or a URL.
  public var mono: Bool
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    text: Binding<String>,
    placeholder: String = "",
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    mono: Bool = false,
    enabled: Bool = true
  ) {
    self._text = text
    self.placeholder = placeholder
    self.size = size
    self.state = state
    self.tint = tint
    self.mono = mono
    self.enabled = enabled
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)
    let ramp = tintFor(state, tint)

    ZStack(alignment: .topLeading) {
      if text.isEmpty {
        Text(placeholder).foregroundStyle(vars.colorContentFaint)
      }
      SwiftUI.TextField("", text: $text, axis: .vertical)
        .textFieldStyle(.plain)
        .focused($focused)
        .disabled(!enabled)
    }
    .font(FieldStyle.font(size: metrics.content.size, mono: mono, vars: vars))
    .lineSpacing(metrics.content.size * (Self.proseLeading - 1))
    .frame(maxWidth: .infinity, alignment: .topLeading)
    .padding(.vertical, vars.spacing25)
    .padding(.horizontal, vars.spacing3)
    .frame(minHeight: vars.spacing20, alignment: .topLeading)
    .fieldChrome(
      FieldStyle(
        vars: vars,
        tint: ramp,
        invalid: state == .error,
        enabled: enabled,
        focused: focused
      ),
      cornerRadius: vars.radiusLarge,
      vars: vars
    )
  }

  /// `line-height: 1.6` — prose leading, stated as a multiple in the
  /// stylesheet rather than held as a token.
  private static var proseLeading: CGFloat { 1.6 }
}

// MARK: - Select

/// One entry in a ``Select``.
public struct SelectOption<Value: Hashable>: Identifiable {
  public var value: Value
  public var label: String
  public var enabled: Bool

  public init(value: Value, label: String, enabled: Bool = true) {
    self.value = value
    self.label = label
    self.enabled = enabled
  }

  public var id: Value { value }
}

/// A dropdown in the shape of an ``TextField``.
///
/// A Select is a TextField with a chevron, so it shares the box outright: the
/// same height, the same corner, the same border recipe, the same focus lift.
public struct Select<Value: Hashable>: View {
  public var options: [SelectOption<Value>]
  @Binding public var selection: Value
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    options: [SelectOption<Value>],
    selection: Binding<Value>,
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    enabled: Bool = true
  ) {
    self.options = options
    self._selection = selection
    self.size = size
    self.state = state
    self.tint = tint
    self.enabled = enabled
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)
    let ramp = tintFor(state, tint)

    // `SwiftUI.Menu`, because this package's own `Menu` is the popover list
    // rather than the platform's dropdown.
    SwiftUI.Menu {
      ForEach(options) { option in
        SwiftUI.Button(option.label) { selection = option.value }
          .disabled(!option.enabled)
      }
    } label: {
      HStack(spacing: vars.spacing0) {
        Text(selectedLabel)
          .lineLimit(1)
        // Laid out rather than absolutely placed: the gap the stylesheet buys
        // with `padding-inline-end` is what the spacer holds open here.
        Spacer(minLength: vars.spacing25)
        Icon(.chevronDown12Regular, size: vars.spacing3)
          .foregroundStyle(vars.colorContentSubtle)
      }
      .font(FieldStyle.font(size: metrics.content.size, mono: false, vars: vars))
      .padding(.leading, vars.spacing3)
      .padding(.trailing, vars.spacing25)
      .frame(height: metrics.size)
      .contentShape(Rectangle())
    }
    .modifier(BareMenu())
    .focused($focused)
    .disabled(!enabled)
    .fieldChrome(
      FieldStyle(
        vars: vars,
        tint: ramp,
        invalid: state == .error,
        enabled: enabled,
        focused: focused
      ),
      cornerRadius: vars.radiusMedium,
      vars: vars
    )
  }

  private var selectedLabel: String {
    options.first { $0.value == selection }?.label ?? ""
  }
}

/// Strips the platform's own menu chrome, so the shared field box is the only
/// box drawn and the chevron the only indicator.
private struct BareMenu: ViewModifier {
  func body(content: Content) -> some View {
    #if os(macOS)
      content.menuStyle(.borderlessButton).menuIndicator(.hidden)
    #else
      content.menuStyle(.button).buttonStyle(.plain)
    #endif
  }
}


// MARK: - FieldValue

/// A value in the shape of an input, for a row that reports rather than edits.
///
/// Not a disabled input: nothing here was ever editable, so nothing is greyed
/// out to say it stopped being.
public struct FieldValue<Content: View>: View {
  public var size: TextFieldSize
  /// Set the mono face — for a key, an ID, or a URL.
  public var mono: Bool
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    size: TextFieldSize = .medium,
    mono: Bool = false,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.size = size
    self.mono = mono
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)

    content()
      .font(FieldStyle.font(size: metrics.content.size, mono: mono, vars: vars))
      .lineLimit(1)
      .frame(maxWidth: .infinity, alignment: .leading)
      .padding(.horizontal, vars.spacing3)
      .frame(height: metrics.size)
      .fieldChrome(
        FieldStyle(vars: vars, tint: .primary, invalid: false, enabled: true, focused: false),
        cornerRadius: vars.radiusMedium,
        vars: vars
      )
  }
}

public extension FieldValue where Content == Text {
  /// A reported value that is a plain string.
  init(_ value: String, size: TextFieldSize = .medium, mono: Bool = false) {
    self.init(size: size, mono: mono, content: { Text(value) })
  }
}
