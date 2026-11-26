import FluentIcons
import SwiftUI

/// Which edge a ``ToastViewport``'s stack hangs from.
public enum ToastPlacement: String, CaseIterable, Sendable {
  case bottom
  case top
}

/// A transient receipt for something that happened out of view.
///
/// One line, no wrapping — anything needing a second sentence belongs in an
/// ``Callout`` in the flow instead. Lifetime belongs to the host, so the
/// system stays stateless, but the contract worth keeping is: a few seconds
/// for a plain notice, longer when it carries an action, and a failure stays
/// until it is dismissed.
public struct Toast<Leading: View, Message: View, Trailing: View>: View {
  /// Which colour ramp the leading glyph is drawn from.
  public var tint: Tint
  /// Leading slot — a glyph, or a ``Spinner`` for work still in flight.
  @ViewBuilder public var leading: () -> Leading
  @ViewBuilder public var message: () -> Message
  /// Trailing affordance — undo, retry.
  @ViewBuilder public var trailing: () -> Trailing
  /// Renders the dismiss button; wire it to remove the toast.
  public var onDismiss: (() -> Void)?
  /// Accessible name of the dismiss button.
  public var dismissLabel: String

  @Environment(\.theme) private var theme
  @Environment(\.accessibilityReduceMotion) private var reduceMotion
  @State private var landed = false

  public init(
    tint: Tint = .neutral,
    onDismiss: (() -> Void)? = nil,
    dismissLabel: String = "Dismiss",
    @ViewBuilder leading: @escaping () -> Leading,
    @ViewBuilder message: @escaping () -> Message,
    @ViewBuilder trailing: @escaping () -> Trailing
  ) {
    self.tint = tint
    self.onDismiss = onDismiss
    self.dismissLabel = dismissLabel
    self.leading = leading
    self.message = message
    self.trailing = trailing
  }

  public var body: some View {
    let vars = theme.vars
    let hairline = vars.strokeHairline

    HStack(spacing: vars.spacing25) {
      leading()
        .environment(\.dazzIconSize, vars.iconMedium)
        .foregroundStyle(vars.ramp(tint)[600])
      message()
        .font(vars.labelMedium.font)
        .lineLimit(1)
        .truncationMode(.tail)
        // Match the single CSS line box — the message sets its own 1.4
        // leading rather than the label face's — before its padding.
        .frame(height: vars.labelMedium.size * messageLeading)
        .foregroundStyle(vars.colorContent)
        .padding(.vertical, vars.spacing05)
      trailing()
      if let onDismiss {
        ToastDismiss(label: dismissLabel, action: onDismiss)
      }
    }
    // CSS's border participates in the box; our stroke is an overlay, so its
    // width is reserved in the inset instead — the same correction Button and
    // Badge make.
    .padding(.vertical, vars.spacing15 + hairline)
    // Asymmetric on purpose: the text gets breathing room, the dismiss button
    // sits tight in the corner it lives in.
    .padding(.leading, vars.spacing3 + hairline)
    .padding(.trailing, vars.spacing15 + hairline)
    // Both bounds are on the padded box, the way `box-sizing: border-box`
    // states them: the minimum height counts the padding rather than adding
    // to it, and the maximum width caps the whole toast rather than its text.
    .frame(minHeight: vars.spacing9)
    .frame(maxWidth: vars.toastMaxWidth)
    // `width: max-content` with that cap: the toast is as wide as what it
    // holds, so the stack needs no magic width, and only a message past the
    // cap truncates. A `maxWidth` frame alone would take whatever width it
    // was offered, which in a viewport is all of it.
    .fixedSize(horizontal: true, vertical: false)
    .background(vars.colorSurfaceOverlay)
    .clipShape(RoundedRectangle(cornerRadius: vars.radiusBig, style: .circular))
    .border(vars.colorBorderStrong, width: hairline, cornerRadius: vars.radiusBig)
    .tokenShadow(vars.shadowMd)
    .opacity(landed ? 1 : 0)
    .scaleEffect(landed ? 1 : entryScale)
    .offset(y: landed ? 0 : entryRise)
    .animation(entry, value: landed)
    .onAppear { landed = true }
    .accessibilityElement(children: .combine)
  }

  /// The arrival: a short overshooting curve, skipped outright when the reader
  /// has asked for less motion.
  private var entry: Animation? {
    reduceMotion ? nil : .timingCurve(0.21, 0.9, 0.35, 1.05, duration: 0.26)
  }

  private var entryScale: CGFloat { 0.98 }

  private var entryRise: CGFloat { theme.vars.spacing25 }

  /// `line-height: 1.4` on the message: the one place the toast overrides the
  /// label face's own leading, because a single line in a 36pt box has to sit
  /// on the centre of it.
  private var messageLeading: CGFloat { 1.4 }
}

public extension Toast where Leading == EmptyView, Trailing == EmptyView {
  /// A toast that is only its message.
  init(
    tint: Tint = .neutral,
    onDismiss: (() -> Void)? = nil,
    dismissLabel: String = "Dismiss",
    @ViewBuilder message: @escaping () -> Message
  ) {
    self.init(
      tint: tint,
      onDismiss: onDismiss,
      dismissLabel: dismissLabel,
      leading: { EmptyView() },
      message: message,
      trailing: { EmptyView() }
    )
  }
}

public extension Toast
where Leading == EmptyView, Message == Text, Trailing == EmptyView {
  /// A toast whose message is a plain string.
  init(
    _ text: String,
    tint: Tint = .neutral,
    onDismiss: (() -> Void)? = nil,
    dismissLabel: String = "Dismiss"
  ) {
    self.init(
      tint: tint,
      onDismiss: onDismiss,
      dismissLabel: dismissLabel,
      message: { Text(text) }
    )
  }
}

/// The dismiss glyph: quiet ink that takes the neutral row wash under the
/// pointer, rather than a control chip that would compete with the action
/// beside it.
private struct ToastDismiss: View {
  let label: String
  let action: () -> Void

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  var body: some View {
    let vars = theme.vars

    Pressable(action: action, cornerRadius: vars.radiusSmall) { state in
      Icon(.dismiss12Regular, size: vars.iconSmall)
        .foregroundStyle(
          state.hovered || state.pressed ? vars.colorContent : vars.colorContentSubtle
        )
        .frame(width: vars.controlTinySize, height: vars.controlTinySize)
        .background(
          state.hovered || state.pressed
            ? vars.controlColor(.normal, .surface, tint: .neutral, state: .hovered)
            : .clear
        )
        .clipShape(RoundedRectangle(cornerRadius: vars.radiusSmall, style: .circular))
        .animation(vars.motion, value: state.controlState)
    }
    .accessibilityLabel(label)
    .focused($focused)
    .focusRing(
      visible: focused,
      color: vars.focusRing,
      cornerRadius: vars.radiusSmall,
      width: vars.focusWidth
    )
  }
}

/// Where a window's toasts land.
///
/// The stack hangs from one edge and is transparent to the pointer between its
/// toasts, so the page underneath stays usable while one is on screen. Order
/// is the host's: SwiftUI has no `column-reverse` for arbitrary builder
/// content, so a top-placed viewport shows what it is given in the order it is
/// given.
public struct ToastViewport<Content: View>: View {
  public var placement: ToastPlacement
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    placement: ToastPlacement = .bottom,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.placement = placement
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(spacing: vars.spacing2) {
      content()
    }
    .padding(vars.spacing4)
    .frame(
      maxWidth: .infinity,
      maxHeight: .infinity,
      alignment: placement == .bottom ? .bottom : .top
    )
  }
}

/// What a toast offers to do about the thing it reports.
public struct ToastAction {
  public let label: String
  public let action: () -> Void

  public init(_ label: String, action: @escaping () -> Void) {
    self.label = label
    self.action = action
  }
}

/// A toast to raise: what happened, and how it is shown.
///
/// Unless `timeout` says otherwise a danger toast stays until dismissed, one
/// with an action stays long enough to be acted on, and any other leaves
/// after five seconds — React's timings.
public struct ToastOptions {
  public var title: String
  public var tint: Tint
  public var action: ToastAction?
  public var dismissable: Bool
  public var timeout: TimeInterval?
  /// Optional leading Fluent icon.
  public var icon: FluentIcon?
  /// Shows a ``Spinner`` in the leading slot while work is in flight.
  public var loading: Bool
  /// Optional custom leading view (takes precedence over `icon` when provided).
  public var leading: AnyView?

  public init(
    title: String,
    tint: Tint = .neutral,
    action: ToastAction? = nil,
    dismissable: Bool = true,
    timeout: TimeInterval? = nil,
    icon: FluentIcon? = nil,
    loading: Bool = false,
    leading: (any View)? = nil
  ) {
    self.title = title
    self.tint = tint
    self.action = action
    self.dismissable = dismissable
    self.timeout = timeout
    self.icon = icon
    self.loading = loading
    self.leading = leading.map { AnyView($0) }
  }

  var resolvedTimeout: TimeInterval {
    timeout ?? (tint == .danger || loading ? 0 : action != nil ? 8 : 5)
  }
}

/// The toasts raised under a ``ToastProvider``, oldest first.
///
/// Auto-dismiss timers pause while the pointer hovers a toast and resume with
/// the remaining duration when the pointer leaves.
@MainActor
public final class Toaster: ObservableObject {
  public struct Raised: Identifiable {
    public let id: Int
    public let options: ToastOptions
  }

  private struct TimerState {
    var remaining: TimeInterval
    var startedAt: Date?
    var paused: Bool
    var task: Task<Void, Never>?
  }

  @Published public private(set) var toasts: [Raised] = []
  private var next = 0
  private var timers: [Int: TimerState] = [:]

  public init() {}

  /// Raises a toast and returns its id, for ``close(_:)``.
  @discardableResult
  public func add(_ options: ToastOptions) -> Int {
    let id = next
    next += 1
    toasts.append(Raised(id: id, options: options))
    let timeout = options.resolvedTimeout
    if timeout.isFinite && timeout > 0 {
      startTimer(id: id, remaining: timeout)
    }
    return id
  }

  /// Pauses the auto-dismiss timer for `id` (for example while the user hovers
  /// the toast to read it or click its action).
  public func pause(_ id: Int) {
    guard var state = timers[id], !state.paused else { return }
    state.task?.cancel()
    state.task = nil
    if let startedAt = state.startedAt {
      let elapsed = Date().timeIntervalSince(startedAt)
      state.remaining = max(0, state.remaining - elapsed)
    }
    state.startedAt = nil
    state.paused = true
    timers[id] = state
  }

  /// Resumes a paused auto-dismiss timer with its remaining duration.
  public func resume(_ id: Int) {
    guard let state = timers[id], state.paused else { return }
    guard state.remaining > 0 else {
      close(id)
      return
    }
    startTimer(id: id, remaining: state.remaining)
  }

  /// Whether the auto-dismiss timer for `id` is currently paused.
  public func isPaused(_ id: Int) -> Bool {
    timers[id]?.paused == true
  }

  /// Dismisses one toast, or every toast when `id` is nil.
  public func close(_ id: Int? = nil) {
    if let id {
      timers[id]?.task?.cancel()
      timers.removeValue(forKey: id)
      toasts.removeAll { $0.id == id }
    } else {
      for state in timers.values {
        state.task?.cancel()
      }
      timers.removeAll()
      toasts.removeAll()
    }
  }

  private func startTimer(id: Int, remaining: TimeInterval) {
    let task = Task { @MainActor [weak self] in
      try? await Task.sleep(for: .seconds(remaining))
      guard !Task.isCancelled else { return }
      self?.close(id)
    }
    timers[id] = TimerState(
      remaining: remaining,
      startedAt: Date(),
      paused: false,
      task: task
    )
  }
}

private struct ToasterKey: EnvironmentKey {
  static let defaultValue: Toaster? = nil
}

public extension EnvironmentValues {
  /// The toaster the nearest ``ToastProvider`` holds.
  var toaster: Toaster? {
    get { self[ToasterKey.self] }
    set { self[ToasterKey.self] = newValue }
  }
}

/// Holds a ``Toaster`` without publishing `toasts` mutations to ``ToastProvider``,
/// so raising or dismissing a toast only invalidates ``ToasterStack`` instead of
/// the entire subtree under the provider.
@MainActor
private final class ToasterHolder: ObservableObject {
  let toaster = Toaster()
}

/// Where toasts are raised from: anything inside reaches `\.toaster`, and a
/// ``ToasterViewport`` inside shows what was raised.
public struct ToastProvider<Content: View>: View {
  @ViewBuilder public var content: () -> Content
  @StateObject private var holder = ToasterHolder()

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    content().environment(\.toaster, holder.toaster)
  }
}

/// The provider's toasts, stacked across the window's bottom edge — or its
/// top, newest nearest the edge. It draws nothing where it stands: the stack
/// is the window's, handed to its overlay.
public struct ToasterViewport: View {
  public var placement: ToastPlacement

  @Environment(\.toaster) private var toaster
  @Environment(\.overlayStack) private var stack
  @Environment(\.theme) private var theme
  @State private var id = UUID()

  public init(placement: ToastPlacement = .bottom) {
    self.placement = placement
  }

  public var body: some View {
    Group {
      if let toaster {
        ToasterStack(toaster: toaster, placement: placement, stack: stack, id: id)
      }
    }
    .onDisappear { stack?.hide(id) }
  }
}

private struct ToasterStack: View {
  @ObservedObject var toaster: Toaster
  let placement: ToastPlacement
  let stack: OverlayStack?
  let id: UUID

  @Environment(\.theme) private var theme

  var body: some View {
    Color.clear
      .frame(width: 0, height: 0)
      .onAppear { publish() }
      .onChange(of: toaster.toasts.map(\.id)) { _ in publish() }
      .onChange(of: theme) { _ in publish() }
  }

  private func publish() {
    guard let stack else { return }
    let toasts = placement == .top ? Array(toaster.toasts.reversed()) : toaster.toasts
    stack.show(
      id,
      AnyView(
        ToastViewport(placement: placement) {
          ForEach(toasts) { raised in
            Toast(
              tint: raised.options.tint,
              onDismiss: raised.options.dismissable ? { toaster.close(raised.id) } : nil,
              leading: {
                if raised.options.loading {
                  Spinner(size: .small, tint: raised.options.tint)
                } else if let leading = raised.options.leading {
                  leading
                } else if let icon = raised.options.icon {
                  Icon(icon)
                }
              },
              message: { Text(raised.options.title) },
              trailing: {
                if let action = raised.options.action {
                  Button(size: .small, variant: .plain, tint: raised.options.tint) {
                    action.action()
                    toaster.close(raised.id)
                  } label: {
                    Text(action.label)
                  }
                }
              }
            )
            .onHover { hovering in
              if hovering {
                toaster.pause(raised.id)
              } else {
                toaster.resume(raised.id)
              }
            }
          }
        }
        .allowsHitTesting(!toasts.isEmpty)
        .theme(theme)
      )
    )
  }
}
