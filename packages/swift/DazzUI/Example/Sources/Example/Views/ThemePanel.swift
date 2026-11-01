import DazzUI
import SwiftUI

/// The theme, and nothing else: which half of it, then which family.
struct ThemePanel: View {
  @Binding var theme: ThemeChoice
  @Binding var sizing: ThemeSizing
  @Binding var menuPresentation: MenuPresentation

  @Environment(\.theme) private var current
  @Environment(\.hairline) private var hairline

  var body: some View {
    let vars = current.vars
    VStack(spacing: 0) {
      HStack {
        Text("Theme").fontWeight(vars.labelStrong.weight)
        Spacer()
        Tooltip {
          Text(theme.dark ? "Switch to light" : "Switch to dark")
        } content: {
          DazzUI.Toggle(
            pressed: theme.dark,
            glyphOnly: true,
            accessibilityName: theme.dark ? "Switch to light" : "Switch to dark",
            action: { theme.dark.toggle() }
          ) {
            Icon(theme.dark ? .weatherMoon20Regular : .weatherSunny20Regular)
          }
        }
      }
      .padding(.horizontal, vars.spacing4)
      .frame(height: vars.frameTitlebarSize)
      .overlay(alignment: .bottom) {
        Rectangle().fill(vars.colorBorder).frame(height: hairline)
      }
      ScrollView {
        VStack(alignment: .leading, spacing: 0) {
          DazzUI.Toggle(pressed: sizing == .system, size: .small,
            accessibilityName: "Follow system sizing", action: {
            sizing = sizing == .system ? .design : .system
          }) { Text("Follow system sizing") }
          .padding(.horizontal, vars.spacing4)
          .padding(.bottom, vars.spacing2)
          DazzUI.Toggle(pressed: menuPresentation == .native, size: .small,
            accessibilityName: "Use native menus", action: {
            menuPresentation = menuPresentation == .native ? .custom : .native
          }) { Text("Use native menus") }
          .padding(.horizontal, vars.spacing4)
          .padding(.bottom, vars.spacing2)
          ForEach(themeFamilyGroups, id: \.title) { group in
            VStack(alignment: .leading, spacing: vars.spacing2) {
              SectionLabel { Text(group.title) }
                .font(vars.labelSmall.font)
                .frame(height: vars.spacing5)
              Grid(horizontalSpacing: vars.spacing2, verticalSpacing: vars.spacing2) {
                ForEach(rows(group.families), id: \.first) { row in
                  GridRow {
                    ForEach(0..<3, id: \.self) { index in
                      if index < row.count {
                        tile(row[index])
                      } else {
                        Color.clear.gridCellUnsizedAxes([.vertical])
                      }
                    }
                  }
                }
              }
            }
            .padding(.vertical, vars.spacing2)
            .padding(.horizontal, vars.spacing4)
          }
        }
        .padding(.vertical, vars.spacing2)
      }
    }
    .frame(maxHeight: .infinity)
  }

  private func rows(_ families: [ThemeFamily]) -> [[ThemeFamily]] {
    stride(from: 0, to: families.count, by: 3).map {
      Array(families[$0..<min($0 + 3, families.count)])
    }
  }

  private func tile(_ family: ThemeFamily) -> some View {
    let vars = current.vars
    var choice = theme
    choice.family = family.value
    return OptionCard(
      selected: family.value == theme.family,
      padding: EdgeInsets(),
      cornerRadius: 8,
      action: { theme.family = family.value },
      title: {
        VStack(spacing: 0) {
          ThemePreview(theme: choice, height: vars.spacing10, framed: false)
          Text(family.title)
            .font(vars.labelSmall.font)
            .lineLimit(1)
            .truncationMode(.tail)
            .padding(.horizontal, vars.spacing15)
            .frame(maxWidth: .infinity)
            .frame(height: vars.spacing6)
        }
      },
      description: { EmptyView() }
    )
    .accessibilityLabel(family.title)
  }
}
