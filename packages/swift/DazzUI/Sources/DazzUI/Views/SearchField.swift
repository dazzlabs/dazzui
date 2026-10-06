import FluentIcons
import SwiftUI

/// Inline search for a list or a table.
///
/// The box belongs to the wrapper rather than to the text field, because the
/// glyph and the trailing slot sit inside it — so the focus lift has to follow
/// the field's focus from out here rather than being drawn by the field
/// itself. The lift is the same move every text control makes: the recessed
/// card rises to the paper behind an accent border and a soft accent wash.
public struct SearchField<Hint: View>: View {
  public var value: String
  public var onValueChange: (String) -> Void
  public var placeholder: String
  public var size: TextFieldSize
  public var tint: Tint
  /// Focus on appearance, for a field opened by a shortcut.
  public var autoFocus: Bool
  /// Focuses the field each time it changes — a shortcut that jumps to a
  /// field already on screen (⌘K), as a ref's `focus()` does in React.
  public var focusRequest: Int
  /// Return in the field: the first match taken, a search run.
  public var onSubmit: (() -> Void)?
  /// Called on Escape once the field is already empty.
  public var onDismiss: (() -> Void)?
  /// Accessible name of the clear button.
  public var clearLabel: String
  public var accessibilityLabel: String?
  /// Trailing hint shown while the field is empty — usually a ``KeyCap``.
  @ViewBuilder public var hint: () -> Hint

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool
  @FocusState private var clearFocused: Bool

  public init(
    value: String,
    onValueChange: @escaping (String) -> Void,
    placeholder: String = "Search",
    size: TextFieldSize = .medium,
    tint: Tint = .primary,
    autoFocus: Bool = false,
    focusRequest: Int = 0,
    onSubmit: (() -> Void)? = nil,
    onDismiss: (() -> Void)? = nil,
    clearLabel: String = "Clear search",
    accessibilityLabel: String? = nil,
    @ViewBuilder hint: @escaping () -> Hint
  ) {
    self.value = value
    self.onValueChange = onValueChange
    self.placeholder = placeholder
    self.size = size
    self.tint = tint
    self.autoFocus = autoFocus
    self.focusRequest = focusRequest
    self.onSubmit = onSubmit
    self.onDismiss = onDismiss
    self.clearLabel = clearLabel
    self.accessibilityLabel = accessibilityLabel
    self.hint = hint
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)
    let style = FieldStyle(
      vars: vars,
      tint: tint,
      invalid: false,
      enabled: true,
      focused: focused
    )

    HStack(spacing: metrics.gap) {
      Icon(.search16Regular, size: metrics.iconSize)
        .foregroundStyle(vars.colorContentFaint)

      field

      if value.isEmpty {
        hint()
      } else {
        clear
      }
    }
    .font(FieldStyle.font(size: metrics.content.size, mono: false, vars: vars))
    .padding(.horizontal, vars.spacing3)
    .frame(height: metrics.size)
    .fieldChrome(style, cornerRadius: vars.radiusMedium, vars: vars)
    .onAppear { if autoFocus { focused = true } }
    .onChange(of: focusRequest) { _ in focused = true }
  }

  private var field: some View {
    let vars = theme.vars
    let text = Binding(get: { value }, set: onValueChange)

    return ZStack(alignment: .leading) {
      if value.isEmpty {
        Text(placeholder).foregroundStyle(vars.colorContentFaint)
      }
      SwiftUI.TextField("", text: text)
        .textFieldStyle(.plain)
        .lineLimit(1)
        .focused($focused)
        .onSubmit { onSubmit?() }
        .accessibilityLabel(accessibilityLabel ?? placeholder)
        #if os(macOS)
          // Escape clears first and dismisses only once there is nothing
          // left, so a stray press never closes a field mid-search.
          .onExitCommand {
            if value.isEmpty { onDismiss?() } else { onValueChange("") }
          }
        #endif
    }
    .frame(maxWidth: .infinity, alignment: .leading)
  }

  private var clear: some View {
    let vars = theme.vars

    return Pressable(
      action: { onValueChange("") },
      cornerRadius: vars.radiusNone
    ) { state in
      Icon(.dismiss12Regular, size: theme.vars.iconSmall)
        .foregroundStyle(state.hovered ? vars.colorContent : vars.colorContentFaint)
        .animation(vars.motion, value: state.hovered)
    }
    .accessibilityLabel(clearLabel)
    .focused($clearFocused)
    .focusRing(
      visible: clearFocused,
      color: vars.focusRing,
      cornerRadius: vars.radiusNone,
      width: vars.focusWidth
    )
  }
}

public extension SearchField where Hint == EmptyView {
  /// A search field with nothing in its trailing slot.
  init(
    value: String,
    onValueChange: @escaping (String) -> Void,
    placeholder: String = "Search",
    size: TextFieldSize = .medium,
    tint: Tint = .primary,
    autoFocus: Bool = false,
    onDismiss: (() -> Void)? = nil,
    clearLabel: String = "Clear search",
    accessibilityLabel: String? = nil
  ) {
    self.init(
      value: value,
      onValueChange: onValueChange,
      placeholder: placeholder,
      size: size,
      tint: tint,
      autoFocus: autoFocus,
      onDismiss: onDismiss,
      clearLabel: clearLabel,
      accessibilityLabel: accessibilityLabel,
      hint: { EmptyView() }
    )
  }
}

