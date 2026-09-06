import DazzUI
import SwiftUI

/// Documentation presentation — the counterpart of the Storybook's
/// `components/gallery.tsx`, and used by the same one story per component:
/// `Overview`. Every other story is the component alone and reaches for
/// ``Stage`` instead.
///
/// It is drawn with the design system's own tokens, so a theme switch carries
/// the page furniture with it too.

/// The measure the Storybook sets its prose to — 48rem. A line length is a
/// typographic decision rather than a step off the spacing scale, so it is
/// stated once here instead of being composed out of one.
let proseMeasure: CGFloat = 768

/// The name column a specimen and a bar both hang their label in: the
/// Storybook's 6rem, composed out of the two steps that make it rather than
/// written down.
func nameColumn(_ vars: ThemeVariables) -> CGFloat {
  vars.spacing20 + vars.spacing4
}

/// A length, printed the way the token graph states it.
func px(_ value: CGFloat) -> String {
  value == value.rounded()
    ? "\(Int(value))px"
    : String(format: "%gpx", Double(value))
}

/// An axis case, printed the way a story labels it.
func label<Value: RawRepresentable>(_ value: Value) -> String
where Value.RawValue == String {
  value.rawValue.prefix(1).uppercased() + value.rawValue.dropFirst()
}

/// A documented page: what the component is, then the sections that argue for
/// each of its axes.
struct Page<Content: View>: View {
  let title: String
  var description: String?
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(
    _ title: String,
    description: String? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.title = title
    self.description = description
    self.content = content
  }

  var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing10) {
      VStack(alignment: .leading, spacing: vars.spacing2) {
        Text(title)
          .font(vars.headlineMedium.font)
          .foregroundStyle(vars.colorContent)

        if let description {
          Prose(description)
        }
      }

      content()
    }
    .frame(maxWidth: .infinity, alignment: .leading)
    .padding(vars.spacing8)
  }
}

/// One axis of a component, named and argued for.
struct Section<Content: View>: View {
  let title: String
  var description: String?
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  init(
    _ title: String,
    description: String? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.title = title
    self.description = description
    self.content = content
  }

  var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing4) {
      VStack(alignment: .leading, spacing: vars.spacing2) {
        Text(title)
          .font(vars.titleMedium.font)
          .foregroundStyle(vars.colorContent)

        if let description {
          Prose(description)
        }
      }
      .frame(maxWidth: .infinity, alignment: .leading)
      .padding(.bottom, vars.spacing3)
      .rule(vars.colorBorder, edge: .bottom, width: hairline)

      content()
    }
  }
}

/// A paragraph, held to the measure.
struct Prose: View {
  let text: String

  @Environment(\.theme) private var theme

  init(_ text: String) {
    self.text = text
  }

  var body: some View {
    let vars = theme.vars

    Text(text)
      .font(vars.bodySmall.font)
      .lineSpacing(vars.bodySmall.lineSpacing)
      .foregroundStyle(vars.colorContentMuted)
      .fixedSize(horizontal: false, vertical: true)
      .frame(maxWidth: proseMeasure, alignment: .leading)
  }
}

/// A run of specimens.
struct Stack<Content: View>: View {
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  var body: some View {
    VStack(alignment: .leading, spacing: theme.vars.spacing3) {
      content()
    }
  }
}

/// One demo row: what the variation is called, and the live component.
struct Specimen<Content: View>: View {
  let label: String
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(_ label: String, @ViewBuilder content: @escaping () -> Content) {
    self.label = label
    self.content = content
  }

  var body: some View {
    let vars = theme.vars

    HStack(alignment: .center, spacing: vars.spacing4) {
      Text(label)
        .font(.system(size: vars.captionMedium.size, design: .monospaced))
        .foregroundStyle(vars.colorContentMuted)
        // The Storybook's specimen grid: a 6rem name column, then the stage.
        .frame(width: nameColumn(vars), alignment: .leading)

      Row { content() }
        .frame(maxWidth: .infinity, alignment: .leading)
    }
    .frame(maxWidth: .infinity, alignment: .leading)
  }
}

/// A grid of tiles, for the token pages where a shelf would run off the edge.
struct Grid<Content: View>: View {
  var min: CGFloat = 256
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(min: CGFloat = 256, @ViewBuilder content: @escaping () -> Content) {
    self.min = min
    self.content = content
  }

  var body: some View {
    LazyVGrid(
      columns: [GridItem(.adaptive(minimum: min), spacing: theme.vars.spacing4)],
      alignment: .leading,
      spacing: theme.vars.spacing4
    ) {
      content()
    }
  }
}

/// One row of a component's token table.
struct TokenRow: Identifiable {
  let name: String
  let value: String

  var id: String { name }

  init(_ name: String, _ value: String) {
    self.name = name
    self.value = value
  }

  /// A length, printed the way the token graph states it.
  init(_ name: String, _ value: CGFloat) {
    self.init(name, px(value))
  }
}

/// The tokens a component owns, and what they resolve to in this theme.
///
/// It appears only where the component genuinely owns tokens — the files in
/// `packages/ds/tokens/component/`. Most components own none: they read the
/// shared vocabulary directly, and a table that would come back empty is not
/// worth a section.
struct TokenTable: View {
  let rows: [TokenRow]

  @Environment(\.theme) private var theme

  init(_ rows: [TokenRow]) {
    self.rows = rows
  }

  var body: some View {
    DazzUI.Table {
      TableHead {
        TableCell(head: true) { Text("Token") }
        TableCell(head: true, align: .end) { Text("Resolved") }
      }

      ForEach(rows) { row in
        TableRow {
          TableCell {
            Text(row.name)
              .font(.system(size: theme.vars.captionMedium.size, design: .monospaced))
          }
          TableCell(align: .end) {
            Text(row.value)
              .font(.system(size: theme.vars.captionSmall.size, design: .monospaced))
              .foregroundStyle(theme.vars.colorContentMuted)
          }
        }
      }
    }
  }
}

/// A colour role: the swatch, its name, and what it is for.
struct Swatch: View {
  let name: String
  let color: Color
  var note: String?

  @Environment(\.theme) private var theme

  init(_ name: String, _ color: Color, note: String? = nil) {
    self.name = name
    self.color = color
    self.note = note
  }

  var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing2) {
      RoundedRectangle(cornerRadius: vars.radiusMedium, style: .continuous)
        .fill(color)
        .frame(height: vars.spacing14)
        .border(vars.colorBorder, width: vars.spacingPx, cornerRadius: vars.radiusMedium)

      Text(name)
        .font(.system(size: vars.captionMedium.size, design: .monospaced))
        .foregroundStyle(vars.colorContent)

      if let note {
        Text(note)
          .font(vars.captionSmall.font)
          .foregroundStyle(vars.colorContentMuted)
          .fixedSize(horizontal: false, vertical: true)
      }
    }
  }
}

/// The eleven steps of one ramp, in the active theme.
struct Ramp: View {
  let name: String
  let swatch: ColorSwatch

  @Environment(\.theme) private var theme

  static let shades = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950]

  init(_ name: String, _ swatch: ColorSwatch) {
    self.name = name
    self.swatch = swatch
  }

  var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing2) {
      Text(name)
        .font(vars.labelMedium.font)
        .foregroundStyle(vars.colorContent)

      Row(spacing: vars.spacing2) {
        ForEach(Self.shades, id: \.self) { shade in
          VStack(spacing: vars.spacing1) {
            RoundedRectangle(cornerRadius: vars.radiusSmall, style: .continuous)
              .fill(swatch[shade])
              .frame(width: vars.spacing12, height: vars.spacing10)
              .border(
                vars.colorBorder,
                width: vars.spacingPx,
                cornerRadius: vars.radiusSmall
              )

            Text(String(shade))
              .font(.system(size: vars.captionSmall.size, design: .monospaced))
              .foregroundStyle(vars.colorContentMuted)
          }
        }
      }
    }
  }
}

/// A framed preview with its name under it — a radius, a shadow, a face.
struct Tile<Content: View>: View {
  let label: String
  var note: String?
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(
    _ label: String,
    note: String? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.label = label
    self.note = note
    self.content = content
  }

  var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing3) {
      ZStack {
        content()
      }
      .frame(maxWidth: .infinity, minHeight: vars.spacing20)

      VStack(alignment: .leading, spacing: vars.spacing05) {
        Text(label)
          .font(.system(size: vars.captionMedium.size, design: .monospaced))
          .foregroundStyle(vars.colorContent)

        if let note {
          Text(note)
            .font(vars.captionSmall.font)
            .foregroundStyle(vars.colorContentMuted)
        }
      }
    }
    .padding(vars.spacing4)
    .background(vars.colorSurface)
    .clipShape(RoundedRectangle(cornerRadius: vars.radiusMedium, style: .continuous))
    .border(vars.colorBorder, width: vars.spacingPx, cornerRadius: vars.radiusMedium)
  }
}

/// A length, drawn to scale beside its name — the spacing scale's page.
struct Bar: View {
  let label: String
  let length: CGFloat

  @Environment(\.theme) private var theme

  init(_ label: String, _ length: CGFloat) {
    self.label = label
    self.length = length
  }

  var body: some View {
    let vars = theme.vars

    HStack(spacing: vars.spacing4) {
      Text(label)
        .font(.system(size: vars.captionMedium.size, design: .monospaced))
        .foregroundStyle(vars.colorContent)
        .frame(width: nameColumn(vars), alignment: .leading)

      RoundedRectangle(cornerRadius: vars.radiusTiny, style: .continuous)
        .fill(theme.vars.ramp(.primary)[500])
        .frame(width: max(length, vars.spacingPx), height: vars.spacing3)

      Text(px(length))
        .font(.system(size: vars.captionSmall.size, design: .monospaced))
        .foregroundStyle(vars.colorContentMuted)
    }
  }
}
