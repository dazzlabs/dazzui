import SwiftUI

/// The one-time code a reader is copying out of a message.
///
/// It is drawn as a run of cells but it is one field underneath, and that is
/// deliberate: a paste of the whole code fills every cell at once, the
/// platform's own code autofill has somewhere to put it, and the caret only
/// ever has one place to be. A run of separate inputs gets all three wrong and
/// has to reinvent the moves between them.
///
/// Each cell is the text controls' box at one character wide, so a code sits
/// on the same rhythm as the fields around it.
public struct OtpField: View {
  @Binding public var value: String
  public var length: Int
  /// Breaks the run where a printed code breaks: `3` on a six-digit code makes
  /// it two groups of three, which is how it is read aloud.
  public var groupAfter: Int?
  /// For a code that is a secret rather than a one-time token — a PIN.
  public var mask: Bool
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    value: Binding<String>,
    length: Int,
    groupAfter: Int? = nil,
    mask: Bool = false,
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    enabled: Bool = true
  ) {
    self._value = value
    self.length = length
    self.groupAfter = groupAfter
    self.mask = mask
    self.size = size
    self.state = state
    self.tint = tint
    self.enabled = enabled
  }

  public var body: some View {
    let vars = theme.vars

    HStack(spacing: vars.spacing2) {
      ForEach(0 ..< length, id: \.self) { index in
        cell(at: index)
          // The break between two groups of digits is a step of air rather
          // than a rule, which is what a printed code does.
          .padding(.leading, breaks(at: index) ? vars.spacing3 - vars.spacing2 : 0)
      }
    }
    .overlay {
      // One field over the run, drawn in nothing: it holds the value and takes
      // the paste, and is what a press focuses. The cells beneath are a
      // picture of it.
      SwiftUI.TextField("", text: $value)
        .textFieldStyle(.plain)
        .focused($focused)
        .disabled(!enabled)
        .foregroundStyle(Color.clear)
        .tint(Color.clear)
        .onChange(of: value) { next in
          let digits = String(next.filter(\.isNumber).prefix(length))
          if digits != next { value = digits }
        }
    }
    .contentShape(Rectangle())
    .onTapGesture { focused = true }
  }

  private func cell(at index: Int) -> some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)
    let characters = Array(value)
    let filled = index < characters.count
    let here = focused && enabled && index == min(characters.count, length - 1)

    return Text(filled ? (mask ? "•" : String(characters[index])) : " ")
      // The mono face and tabular figures: a code is digits being compared,
      // and a 1 narrower than a 7 makes the run look mis-typed.
      .font(.system(size: metrics.content.size, weight: vars.bodySmall.weight, design: .monospaced))
      .frame(width: metrics.size, height: metrics.size)
      .fieldChrome(
        FieldStyle(
          vars: vars,
          tint: tintFor(state, tint),
          invalid: state == .error,
          enabled: enabled,
          // A filled cell lifts to the paper and keeps it, so the run says how
          // far through the code the reader is without the caret having to be
          // the only thing that does. Only the cell the caret is in glows.
          focused: here || filled
        ),
        cornerRadius: vars.radiusMedium,
        vars: vars
      )
  }

  private func breaks(at index: Int) -> Bool {
    guard let groupAfter, index > 0 else { return false }
    return index % groupAfter == 0
  }
}
