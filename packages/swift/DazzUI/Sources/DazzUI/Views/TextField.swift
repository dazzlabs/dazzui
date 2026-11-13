import FluentIcons

/// The text controls' own three sizes, each of which binds one of the shared
/// `[data-size]` control profiles.
import SwiftUI

/// Whether the control is reporting a problem.

/// An erroring control is re-tinted rather than given a second palette, so its
/// border, wash, ink and focus glow all move together.

// MARK: - The shared box

/// The colours the text controls' shared box paints in one state.
///
/// The box is a recessed card — one step below the paper — that lifts to the
/// paper on focus behind an accent border and a soft accent wash. That
/// card→paper transition *is* the focus signal; a field that stays white can
/// only signal with its ring.

// MARK: - TextField

/// A text control: a line of text, or a box of it.
///
/// `multiline` is the axis rather than a view of its own, because the box, the
/// states and the tinting are one design either way. What it changes: the box
/// takes the container corner rather than the control corner — a theme that
/// makes controls pills would otherwise turn a growing box into a lozenge —
/// the leading opens up, because this is the one text control whose text
/// wraps, and the height becomes a floor rather than the control height.
///
/// Named `TextField` rather than prefixed, and reached as
/// `DazzUI.TextField` where `SwiftUI.TextField` is also in scope.

// MARK: - Select

/// One entry in a ``Select``.

/// Options set under a heading, the way `<optgroup>` groups them.

/// What a ``Select``'s list holds: an option, or a group of them under a
/// heading. React's `options` takes the two mixed, in the order they are
/// listed — a flat "Unassigned" above a "People" and an "Agents" group.

/// How wide a ``Select``'s box is.

/// A dropdown in the shape of a ``TextField``.
///
/// A Select is a TextField with a chevron, so it shares the box outright: the
/// same height, the same corner, the same border recipe, the same focus lift.
/// Its list is the ``MenuSheet``, the panel a ``Menu`` and a ``Combobox``
/// open, with a tick on the chosen row and, where the options are grouped,
/// each group under its heading.
///
/// The trigger is drawn here rather than handed to `SwiftUI.Menu` as a label.
/// On macOS that menu flattens its label into a native pop-up button — a title
/// and an image — so the box's height and padding are dropped and the chevron
/// is moved to the leading edge, whatever the label's own layout says.
///
/// It fills the width offered, as the stylesheet's `width: 100%` has it; a
/// select that should be as wide as what it says, or as its longest choice,
/// asks with `width`.

// MARK: - FieldValue

/// A value in the shape of an input, for a row that reports rather than edits.
///
/// Not a disabled input: nothing here was ever editable, so nothing is greyed
/// out to say it stopped being.

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
public enum TextFieldState: String, CaseIterable, Sendable {
  case `default`
  case error
}
public func tintFor(_ state: TextFieldState, _ tint: Tint) -> Tint {
  state == .error ? .danger : tint
}
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

  /// A face the caller chose, as a host's class sets `font-family`,
  /// `font-size` and `font-weight` on the field; mono still takes the code
  /// face, in the chosen size and weight.
  static func font(_ style: TypeStyle, mono: Bool) -> Font {
    mono
      ? .system(size: style.size, weight: style.weight, design: .monospaced)
      : style.font.weight(style.weight)
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
      .clipShape(RoundedRectangle(cornerRadius: cornerRadius, style: .circular))
      .modifier(FieldBorder(color: style.border, cornerRadius: cornerRadius))
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
private struct FieldBorder: ViewModifier {
  let color: Color
  let cornerRadius: CGFloat
  @Environment(\.hairline) private var hairline

  func body(content: Content) -> some View {
    content.border(color, width: hairline, cornerRadius: cornerRadius)
  }
}
public struct TextField: View {
  @Binding public var text: String
  public var placeholder: String
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  /// Set the mono face — for a key, an ID, or a URL.
  public var mono: Bool
  public var enabled: Bool
  /// Draw as a box that grows rather than as a single line.
  public var multiline: Bool
  /// Hide what is typed — React's `type="password"`. A secure field is one
  /// line, so it is not also `multiline`.
  public var secure: Bool
  /// The face, size and weight the text is set in — what a host's class
  /// sets on React's field, a task's title in `titleSmall`, say. `nil` is
  /// the field's own: the size profile's, at body weight. The line height
  /// stays the field's, so a single line keeps the control height.
  public var typeStyle: TypeStyle?
  /// How many lines a multiline field opens at — the textarea's `rows`, two
  /// unless set, as a textarea's are. The box is never shorter than its
  /// floor, `spacing-20`.
  public var rows: Int

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @FocusState private var focused: Bool

  public init(
    text: Binding<String>,
    placeholder: String = "",
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    mono: Bool = false,
    enabled: Bool = true,
    multiline: Bool = false,
    secure: Bool = false,
    typeStyle: TypeStyle? = nil,
    rows: Int = 2
  ) {
    self._text = text
    self.placeholder = placeholder
    self.size = size
    self.state = state
    self.tint = tint
    self.mono = mono
    self.enabled = enabled
    self.multiline = multiline
    self.secure = secure
    self.typeStyle = typeStyle
    self.rows = rows
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)
    let ramp = tintFor(state, tint)

    let editor = ZStack(alignment: multiline ? .topLeading : .leading) {
      #if !os(Windows) && !os(Linux)
        if text.isEmpty {
          Text(placeholder).foregroundStyle(vars.colorContentFaint)
        }
      #endif
      Group {
        if secure && !multiline {
          SwiftUI.SecureField(nativePlaceholder, text: $text)
        } else {
          SwiftUI.TextField(
            nativePlaceholder,
            text: $text,
            axis: multiline ? .vertical : .horizontal
          )
        }
      }
      .textFieldStyle(.plain)
      .focused($focused)
      .disabled(!enabled)
    }
    .font(
      typeStyle.map { FieldStyle.font($0, mono: mono) }
        ?? FieldStyle.font(size: metrics.content.size, mono: mono, vars: vars)
    )

    return Group {
      if multiline {
        let line = proseLine(metrics: metrics)
        // The textarea's height: `rows` lines, its padding and its border,
        // and never under the floor its `min-height` sets.
        let height = max(
          vars.spacing20,
          line.lineHeight * CGFloat(max(1, rows)) + vars.spacing25 * 2 + hairline * 2
        )
        editor
          .lineBox(line)
          .frame(maxWidth: .infinity, alignment: .topLeading)
          .padding(.vertical, vars.spacing25)
          .padding(.horizontal, vars.spacing3)
          .frame(minHeight: height, alignment: .topLeading)
      } else {
        editor
          .lineLimit(1)
          .padding(.horizontal, vars.spacing3)
          // Single-line controls take the shared control height, so a form can
          // mix them with buttons without the rows stepping. It wins over the
          // profile's block padding, which is what the stylesheet's `height`
          // does too.
          .frame(height: metrics.size)
      }
    }
    // The whole box is the field's, as a textarea's is: a press anywhere in it
    // — below a multiline field's last line, in its padding — focuses the
    // text. Behind the text, so a press on the text itself places the caret.
    .background {
      Color.clear
        .contentShape(Rectangle())
        .onTapGesture { if enabled { focused = true } }
    }
    .fieldChrome(
      FieldStyle(
        vars: vars,
        tint: ramp,
        invalid: state == .error,
        enabled: enabled,
        focused: focused
      ),
      // A growing box takes the container corner; a single line takes the
      // field corner, one step above the buttons' in every drawn family.
      cornerRadius: multiline ? vars.radiusLarge : vars.controlFieldRadius,
      vars: vars
    )
  }

  private var nativePlaceholder: String {
    #if os(Windows) || os(Linux)
      placeholder
    #else
      ""
    #endif
  }

  /// `line-height: 1.6` — prose leading, stated as a multiple in the
  /// stylesheet rather than held as a token.
  private static var proseLeading: CGFloat { 1.6 }

  /// The face a multiline field's lines are set in, in its prose line box.
  private func proseLine(metrics: ControlMetrics) -> TypeStyle {
    let vars = theme.vars
    let face =
      typeStyle
      ?? TypeStyle(
        family: vars.bodySmall.family,
        size: metrics.content.size,
        weight: vars.bodySmall.weight,
        lineHeight: 0
      )
    return TypeStyle(
      family: face.family,
      size: face.size,
      weight: face.weight,
      lineHeight: face.size * Self.proseLeading
    )
  }
}
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
public struct SelectGroup<Value: Hashable> {
  public var label: String
  public var options: [SelectOption<Value>]

  public init(_ label: String, options: [SelectOption<Value>]) {
    self.label = label
    self.options = options
  }
}
public enum SelectEntry<Value: Hashable> {
  case option(SelectOption<Value>)
  case group(SelectGroup<Value>)

  /// A flat option.
  public static func option(value: Value, label: String, enabled: Bool = true) -> Self {
    .option(SelectOption(value: value, label: label, enabled: enabled))
  }

  /// Options under a heading.
  public static func group(_ label: String, options: [SelectOption<Value>]) -> Self {
    .group(SelectGroup(label, options: options))
  }

  fileprivate var options: [SelectOption<Value>] {
    switch self {
    case .option(let option): return [option]
    case .group(let group): return group.options
    }
  }
}
public enum SelectWidth: Sendable {
  /// The width offered, as the stylesheet's `width: 100%` makes it — the
  /// default, in a form or a column of a set width.
  case fill
  /// As wide as the value shown, as `width: auto` makes it — a select in a
  /// toolbar, beside the controls it filters.
  case value
  /// As wide as the longest option, so choosing a shorter one does not make
  /// it jump — the stylesheet's `min-width` set to the longest choice.
  case longestOption
}
public struct Select<Value: Hashable>: View {
  public var entries: [SelectEntry<Value>]
  @Binding public var selection: Value
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  public var enabled: Bool
  /// How wide the box is.
  public var width: SelectWidth
  /// What the field says while its selection matches no option — React's
  /// `placeholder`, in the faint ink a placeholder is.
  public var placeholder: String

  @Environment(\.theme) private var theme
  @Environment(\.menuPresentation) private var presentation
  @FocusState private var focused: Bool
  @State private var open = false

  /// A list of options and groups of them, in the order they are listed.
  public init(
    options: [SelectEntry<Value>],
    selection: Binding<Value>,
    placeholder: String = "Choose…",
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    enabled: Bool = true,
    width: SelectWidth = .fill
  ) {
    self.entries = options
    self._selection = selection
    self.placeholder = placeholder
    self.size = size
    self.state = state
    self.tint = tint
    self.enabled = enabled
    self.width = width
  }

  /// A flat list of options.
  public init(
    options: [SelectOption<Value>],
    selection: Binding<Value>,
    placeholder: String = "Choose…",
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    enabled: Bool = true,
    width: SelectWidth = .fill
  ) {
    self.init(
      options: options.map { SelectEntry.option($0) },
      selection: selection,
      placeholder: placeholder,
      size: size,
      state: state,
      tint: tint,
      enabled: enabled,
      width: width
    )
  }

  /// Every option, out of its group.
  public var options: [SelectOption<Value>] {
    entries.flatMap(\.options)
  }

  public var body: some View {
    let vars = theme.vars

    trigger
      // Modal, as Base UI's select is: the form behind waits for a choice.
      .floatingPanel(
        isPresented: Binding(get: { open && !presentation.usesNativeMenus }, set: { open = $0 }),
        gap: vars.spacing1, modal: true
      ) { anchor in
        AnchoredMenuSheet(width: anchor.width) { list }
      }
      .nativeMenu(isPresented: $open, entries: nativeEntries)
      .onChange(of: enabled) { if !$0 { open = false } }
  }

  private var trigger: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)
    let all = options

    return Pressable(
      action: { open.toggle() },
      enabled: enabled,
      cornerRadius: vars.controlFieldRadius
    ) { _ in
      // The value takes the room and the chevron sits at the trailing edge.
      HStack(spacing: vars.spacing2) {
        ZStack(alignment: .leading) {
          if width == .longestOption {
            // Every label, unseen, so the box is the longest one's width.
            ForEach(all) { option in
              Text(option.label).hidden()
            }
            Text(placeholder).hidden()
          }
          if let chosen = all.first(where: { $0.value == selection }) {
            Text(chosen.label)
          } else {
            Text(placeholder).foregroundStyle(vars.colorContentFaint)
          }
        }
        .lineLimit(1)
        .truncationMode(.tail)
        .frame(maxWidth: width == .fill ? .infinity : nil, alignment: .leading)
        Icon(.chevronDown12Regular, size: vars.iconSmall)
          .foregroundStyle(vars.colorContentSubtle)
      }
      .font(FieldStyle.font(size: metrics.content.size, mono: false, vars: vars))
      .padding(.horizontal, vars.spacing3)
      .frame(height: metrics.size)
      // The trigger of an open list is not idle, so it lifts while the list
      // is open as well as while it holds the focus.
      .fieldChrome(
        FieldStyle(
          vars: vars,
          tint: tintFor(state, tint),
          invalid: state == .error,
          enabled: enabled,
          focused: focused || open
        ),
        cornerRadius: vars.controlFieldRadius,
        vars: vars
      )
    }
    .focused($focused)
    .accessibilityValue(selectedLabel)
  }

  @ViewBuilder private var list: some View {
    let vars = theme.vars

    ForEach(Array(entries.enumerated()), id: \.offset) { index, entry in
      switch entry {
      case .option(let option):
        row(option)
      case .group(let group):
        VStack(spacing: 0) {
          MenuGroupLabel(title: group.label)
          ForEach(group.options) { option in row(option) }
        }
        // `.dz-popup__group + .dz-popup__group`: a group that follows a group
        // stands a step off it.
        .padding(.top, index > 0 && entries[index - 1].isGroup ? vars.spacing1 : 0)
      }
    }
  }

  private func row(_ option: SelectOption<Value>) -> some View {
    MenuRow(
      item: MenuItem(
        option.label,
        checked: option.value == selection,
        disabled: !option.enabled
      ),
      tint: .neutral
    ) {
      selection = option.value
      open = false
    }
  }

  internal var nativeEntries: [NativeMenuEntry] {
    entries.flatMap { entry in
      switch entry {
      case .option(let option): return [nativeEntry(option)]
      case .group(let group):
        return [.heading(group.label)] + group.options.map(nativeEntry)
      }
    }
  }

  private func nativeEntry(_ option: SelectOption<Value>) -> NativeMenuEntry {
    .item(
      MenuItem(option.label, checked: option.value == selection, disabled: !option.enabled) {
        selection = option.value
      })
  }

  private var selectedLabel: String {
    options.first { $0.value == selection }?.label ?? placeholder
  }
}
extension Select {
  /// The earlier spelling of `width`: `true` is `.fill`, and `false` the
  /// longest option's width this select used to take by default.
  @available(*, deprecated, message: "Use width: — .fill, .value or .longestOption")
  public init(
    options: [SelectOption<Value>],
    selection: Binding<Value>,
    placeholder: String = "Choose…",
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    enabled: Bool = true,
    fullWidth: Bool
  ) {
    self.init(
      options: options,
      selection: selection,
      placeholder: placeholder,
      size: size,
      state: state,
      tint: tint,
      enabled: enabled,
      width: fullWidth ? .fill : .longestOption
    )
  }
}
extension SelectEntry {
  fileprivate var isGroup: Bool {
    if case .group = self { return true }
    return false
  }
}
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
        cornerRadius: vars.controlFieldRadius,
        vars: vars
      )
  }
}
extension FieldValue where Content == Text {
  /// A reported value that is a plain string.
  public init(_ value: String, size: TextFieldSize = .medium, mono: Bool = false) {
    self.init(size: size, mono: mono, content: { Text(value) })
  }
}
