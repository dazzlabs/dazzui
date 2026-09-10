import FluentIcons
import SwiftUI

/// A run of code inside a sentence — a flag, a path, an identifier.
///
/// A wash and a face, not a box: the chip corner and the half-step inset keep
/// it separable from the prose around it without breaking the line's rhythm,
/// which is what a border at this size always does.
public struct Code: View {
  public var code: String

  @Environment(\.theme) private var theme

  public init(_ code: String) {
    self.code = code
  }

  public var body: some View {
    let vars = theme.vars

    Text(code)
      // The code face, the one place a view reaches a `base.*` primitive —
      // exactly as a `mono` TextField does. `base.font.code` has no semantic
      // token in front of it, because the code face is not a role.
      //
      // A step under the prose it sits in: a monospace face at the same
      // nominal size always reads larger, and the run would ride above the
      // line.
      .font(.system(size: vars.captionSmall.size, design: .monospaced))
      .foregroundStyle(vars.colorContent)
      .padding(.vertical, vars.spacing05)
      .padding(.horizontal, vars.spacing1)
      .background(vars.colorSurfaceSunken)
      .clipShape(RoundedRectangle(cornerRadius: vars.radiusTiny, style: .circular))
  }
}

/// How long the tick stands after a copy.
///
/// A file-scope constant rather than a static on the view: `CodeBlock` is
/// generic over its actions, and a generic type cannot carry a stored static.
private let codeBlockCopiedFor: Double = 1.6

/// A block of code, read rather than edited.
///
/// The gutter is a column of its own rather than text in the same run, so a
/// selection that crosses ten lines takes ten lines of code and not ten line
/// numbers — the single thing a reader does with one of these, and the thing a
/// numbered run gets wrong. It also sits outside the scroller, so it stays put
/// while the code slides under it.
///
/// A head is drawn only where there is something to put in it. A block with no
/// title, no language and nothing to press is the code alone, because a bar
/// holding one label says less than the space it takes.
public struct CodeBlock<Actions: View>: View {
  public var code: String
  /// What the block is — a path, a filename, a shell's name.
  public var title: String?
  /// Named at the trailing edge of the head, and nothing more: this package
  /// ships no highlighter, because a grammar per language is a dependency a
  /// design system has no opinion about.
  public var language: String?
  public var lineNumbers: Bool
  /// Wraps long lines instead of scrolling them. A wrapped line of code has
  /// lost what its indentation was carrying, so scrolling is the default.
  public var wrap: Bool
  public var copyable: Bool
  @ViewBuilder public var actions: () -> Actions

  @Environment(\.theme) private var theme
  @State private var copied = false

  public init(
    code: String,
    title: String? = nil,
    language: String? = nil,
    lineNumbers: Bool = false,
    wrap: Bool = false,
    copyable: Bool = false,
    @ViewBuilder actions: @escaping () -> Actions
  ) {
    self.code = code
    self.title = title
    self.language = language
    self.lineNumbers = lineNumbers
    self.wrap = wrap
    self.copyable = copyable
    self.actions = actions
  }

  private var body_: String {
    var trimmed = code
    while trimmed.hasSuffix("\n") { trimmed.removeLast() }
    return trimmed
  }

  private var lines: [String] {
    body_.components(separatedBy: "\n")
  }

  private var hasHead: Bool {
    title != nil || language != nil || copyable || Actions.self != EmptyView.self
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: 0) {
      if hasHead { head }
      HStack(alignment: .top, spacing: 0) {
        if lineNumbers { gutter }
        listing
      }
      .padding(.vertical, vars.spacing25)
    }
    .background(
      // The card's materials, not the popup's: this sits *in* the page rather
      // than over it, so it takes the muted surface and the ordinary border
      // rather than the overlay's edge and shadow.
      vars.colorSurfaceMuted
    )
    .clipShape(RoundedRectangle(cornerRadius: vars.radiusMedium, style: .circular))
    .border(vars.colorBorder, width: vars.strokeHairline, cornerRadius: vars.radiusMedium)
  }

  private var head: some View {
    let vars = theme.vars

    return HStack(spacing: vars.spacing2) {
      if let title {
        Text(title)
          // Set in the code face too: the title is nearly always a path, and a
          // proportional face beside a monospace body reads as a different
          // document rather than as its label.
          .font(.system(size: vars.captionSmall.size, design: .monospaced))
          .foregroundStyle(vars.colorContentSecondary)
          .lineLimit(1)
          .truncationMode(.tail)
      }
      if let language {
        Text(language.uppercased())
          .font(.system(size: vars.labelSmall.size, weight: vars.labelMedium.weight))
          .foregroundStyle(vars.colorContentFaint)
      }
      // Pushed to the trailing edge whichever of the two labels before it are
      // drawn, which is why this is a spacer rather than an alignment on the
      // row.
      Spacer(minLength: 0)
      actions()
      if copyable {
        IconButton(
          label: copied ? "Copied" : "Copy",
          icon: copied ? .checkmark16Regular : .copy16Regular,
          action: copy
        )
      }
    }
    .padding(.vertical, vars.spacing1)
    .padding(.horizontal, vars.spacing3)
    .frame(minHeight: vars.spacing8)
    .background(
      // Chrome, and it reads as chrome: the titlebar's step and a rule under
      // it.
      vars.colorSurfaceChrome
    )
    .overlay(alignment: .bottom) {
      Rectangle()
        .fill(vars.colorBorder)
        .frame(height: vars.strokeHairline)
    }
  }

  private var gutter: some View {
    let vars = theme.vars

    return VStack(alignment: .trailing, spacing: 0) {
      ForEach(Array(lines.indices), id: \.self) { index in
        Text("\(index + 1)")
          .frame(height: vars.bodySmall.lineHeight)
      }
    }
    .font(.system(size: vars.captionSmall.size, design: .monospaced))
    .foregroundStyle(vars.colorContentFaint)
    // Right-aligned so the ones column stays a column past line 9, and wide
    // enough for four digits before it starts pushing the code across.
    .frame(width: vars.spacing10, alignment: .trailing)
    .padding(.trailing, vars.spacing3)
    // Never part of a selection: the whole point of the split column.
    .accessibilityHidden(true)
  }

  private var listing: some View {
    let vars = theme.vars

    return VStack(alignment: .leading, spacing: 0) {
      ForEach(Array(lines.enumerated()), id: \.offset) { _, line in
        Text(line)
          .fixedSize(horizontal: !wrap, vertical: false)
          .frame(
            maxWidth: .infinity,
            minHeight: vars.bodySmall.lineHeight,
            alignment: .leading
          )
      }
    }
    .font(.system(size: vars.captionSmall.size, design: .monospaced))
    .foregroundStyle(vars.colorContent)
    .padding(.leading, lineNumbers ? 0 : vars.spacing3)
    .padding(.trailing, vars.spacing3)
    .frame(maxWidth: .infinity, alignment: .leading)
  }

  private func copy() {
    #if canImport(AppKit)
      NSPasteboard.general.clearContents()
      NSPasteboard.general.setString(body_, forType: .string)
    #endif
    copied = true
    // The tick outlives the copy by a second and a half, so the block may be
    // gone by the time it should be cleared — which is harmless here, since
    // the state goes with the view.
    DispatchQueue.main.asyncAfter(deadline: .now() + codeBlockCopiedFor) {
      copied = false
    }
  }
}

public extension CodeBlock where Actions == EmptyView {
  /// A block with nothing in its head but its own labels.
  init(
    code: String,
    title: String? = nil,
    language: String? = nil,
    lineNumbers: Bool = false,
    wrap: Bool = false,
    copyable: Bool = false
  ) {
    self.init(
      code: code,
      title: title,
      language: language,
      lineNumbers: lineNumbers,
      wrap: wrap,
      copyable: copyable,
      actions: { EmptyView() }
    )
  }
}
