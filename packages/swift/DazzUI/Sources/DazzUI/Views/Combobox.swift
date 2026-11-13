import FluentIcons

/// One choosable option: the value a caller gets back, and the label the query
/// is matched against.
import SwiftUI

/// A choice out of a list too long to scroll: the reader types to narrow it,
/// and picks one of what is left.
///
/// It is a ``Select`` with a query in front of it, and that is the only
/// question worth asking when choosing between them — a list of six is a
/// select, a list of six hundred is this. Both end in one of the options;
/// neither lets a reader enter something that is not on the list, which is
/// what separates them from an ``Autocomplete``.
///
/// The box is the ``TextField``'s and the list is the ``MenuSheet``, so a
/// combobox in a form sits on the same rhythm as everything around it and its
/// list is the same panel a ``Menu`` opens.

/// A text field that suggests as you type.
///
/// The difference from a ``Combobox`` is what the value is: a combobox ends in
/// one of its options and nothing else; an autocomplete's value is whatever
/// was typed, and the list is a shortcut to a likely answer rather than the
/// set of allowed ones. A branch name, a search query, a tag that may not
/// exist yet.
///
/// So there is no chevron and no clear: nothing here is opened or chosen, and
/// the field is emptied the way any text field is.

/// The clear and the chevron: the field's own furniture rather than buttons on
/// it — no box at rest, the subtle ink, and the neutral wash only under the
/// pointer. A pair of drawn buttons inside a field would read as two more
/// controls in the form.

public struct ComboboxOption<Value: Hashable>: Identifiable {
  public var id: Value { value }
  public var value: Value
  public var label: String
  /// The heading this option sits under. Options carrying one are grouped in
  /// the order the groups first appear; a list where none do is flat.
  public var group: String?
  public var disabled: Bool

  public init(_ value: Value, label: String, group: String? = nil, disabled: Bool = false) {
    self.value = value
    self.label = label
    self.group = group
    self.disabled = disabled
  }
}
public struct Combobox<Value: Hashable>: View {
  public var options: [ComboboxOption<Value>]
  @Binding public var value: Value?
  /// The query. It is a binding rather than private state so a host can clear
  /// it, and so the field behaves like every other text control here.
  @Binding public var query: String
  public var placeholder: String
  /// What the list says when the query matches nothing.
  public var emptyLabel: String
  /// Offers a control that clears the chosen option.
  public var clearable: Bool
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool
  @State private var open = false

  public init(
    options: [ComboboxOption<Value>],
    value: Binding<Value?>,
    query: Binding<String>,
    placeholder: String = "Search…",
    emptyLabel: String = "No matches",
    clearable: Bool = true,
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    enabled: Bool = true
  ) {
    self.options = options
    self._value = value
    self._query = query
    self.placeholder = placeholder
    self.emptyLabel = emptyLabel
    self.clearable = clearable
    self.size = size
    self.state = state
    self.tint = tint
    self.enabled = enabled
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)

    field(metrics: metrics)
      // Not modal, as Base UI's combobox is not: the field keeps the typing
      // while the list is open, and a press on the field is the field's.
      .floatingPanel(
        isPresented: Binding(get: { open && enabled }, set: { open = $0 }),
        gap: vars.spacing1,
        modal: false
      ) { anchor in
        // The list is at least as wide as the field it dropped from, the way
        // a select's is.
        AnchoredMenuSheet(width: anchor.width) { list }
      }
  }

  private func field(metrics: ControlMetrics) -> some View {
    let vars = theme.vars

    #if os(Windows)
      let nativePlaceholder = placeholder
    #else
      let nativePlaceholder = ""
    #endif
    return HStack(spacing: vars.spacing1) {
      ZStack(alignment: .leading) {
        #if !os(Windows)
          if query.isEmpty {
            Text(placeholder).foregroundStyle(vars.colorContentFaint)
          }
        #endif
        SwiftUI.TextField(nativePlaceholder, text: $query)
          .textFieldStyle(.plain)
          .focused($focused)
          .disabled(!enabled)
          .onChange(of: query) { query in
            // Selection updates query and closes the panel in one transaction.
            // Its deferred change callback must not reopen the selected row.
            if query != options.first(where: { $0.value == value })?.label { open = true }
          }
      }
      .lineLimit(1)
      .font(FieldStyle.font(size: metrics.content.size, mono: false, vars: vars))
      if clearable && value != nil {
        FieldAction(icon: .dismiss12Regular, help: "Clear") {
          value = nil
          query = ""
        }
      }
      FieldAction(
        icon: open ? .chevronUp12Regular : .chevronDown12Regular,
        help: "Open"
      ) {
        open.toggle()
        if open { focused = true }
      }
    }
    .padding(.leading, vars.spacing3)
    .padding(.trailing, vars.spacing1)
    .frame(height: metrics.size)
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

  @ViewBuilder private var list: some View {
    let kept = matches

    if kept.isEmpty {
      MenuEmpty(label: emptyLabel)
    } else {
      // The headings are drawn from what survived the query rather than from
      // the list as it was given: a group the query emptied is not a heading
      // over nothing.
      ForEach(Array(kept.enumerated()), id: \.element.id) { index, option in
        if let group = option.group, group != kept[safe: index - 1]?.group {
          MenuGroupLabel(title: group)
        }
        MenuRow(
          item: MenuItem(
            option.label,
            checked: option.value == value,
            disabled: option.disabled
          ),
          tint: .neutral
        ) {
          value = option.value
          query = option.label
          open = false
        }
      }
    }
  }

  private var matches: [ComboboxOption<Value>] {
    let needle = query.trimmingCharacters(in: .whitespaces).lowercased()
    guard !needle.isEmpty else { return options }
    return options.filter { option in
      // The chosen option's own label is not a query: a list that emptied
      // itself the moment it was reopened would look broken.
      option.label.lowercased() == needle || option.label.lowercased().contains(needle)
    }
  }
}
public struct Autocomplete: View {
  public var items: [String]
  @Binding public var text: String
  public var placeholder: String
  public var emptyLabel: String
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  /// Set the mono face — for a path, a branch name, a key.
  public var mono: Bool
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool
  @State private var open = false
  @State private var selectedText: String?

  public init(
    items: [String],
    text: Binding<String>,
    placeholder: String = "",
    emptyLabel: String = "No matches",
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    mono: Bool = false,
    enabled: Bool = true
  ) {
    self.items = items
    self._text = text
    self.placeholder = placeholder
    self.emptyLabel = emptyLabel
    self.size = size
    self.state = state
    self.tint = tint
    self.mono = mono
    self.enabled = enabled
  }

  public var body: some View {
    let vars = theme.vars

    TextField(
      text: $text,
      placeholder: placeholder,
      size: size,
      state: state,
      tint: tint,
      mono: mono,
      enabled: enabled
    )
    .focused($focused)
    .onChange(of: text) { if $0 != selectedText { open = true } }
    .floatingPanel(
      isPresented: Binding(get: { open && enabled }, set: { open = $0 }),
      gap: vars.spacing1,
      modal: false
    ) { anchor in
      AnchoredMenuSheet(width: anchor.width) {
        if matches.isEmpty {
          MenuEmpty(label: emptyLabel)
        } else {
          ForEach(matches, id: \.self) { item in
            MenuRow(item: MenuItem(item), tint: .neutral) {
              selectedText = item
              text = item
              open = false
            }
          }
        }
      }
    }
  }

  private var matches: [String] {
    let needle = text.trimmingCharacters(in: .whitespaces).lowercased()
    guard !needle.isEmpty else { return items }
    return items.filter { $0.lowercased().contains(needle) }
  }
}
private struct FieldAction: View {
  let icon: FluentIcon
  let help: String
  let action: () -> Void

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    Pressable(action: action, enabled: true, cornerRadius: vars.radiusSmall, help: help) { state in
      Icon(icon, size: vars.iconSmall)
        .foregroundStyle(state.hovered ? vars.colorContent : vars.colorContentSubtle)
        .frame(width: vars.spacing5, height: vars.spacing5)
        .background(state.hovered || state.pressed ? wash : .clear)
        .clipShape(RoundedRectangle(cornerRadius: vars.radiusSmall, style: .circular))
        .animation(vars.motion, value: state.controlState)
    }
  }

  private var wash: Color {
    theme.vars.resolve(theme.vars.descriptor(.normal, .surface), tint: .neutral, state: .hovered)
  }
}
extension Array {
  /// The element at an index that may be out of range — what a "same group as
  /// the row before" test needs at the top of the list.
  fileprivate subscript(safe index: Int) -> Element? {
    indices.contains(index) ? self[index] : nil
  }
}
