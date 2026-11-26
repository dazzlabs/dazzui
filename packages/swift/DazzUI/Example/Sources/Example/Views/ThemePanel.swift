import DazzUI
import SwiftUI

/// The theme, and nothing else: which half of it, then which family.
struct ThemePanel: View {
  @Binding var theme: ThemeChoice
  @Binding var sizing: ThemeSizing
  @Binding var menuPresentation: MenuPresentation
  var omarchy: OmarchyPalette?
  var followingOmarchy = false
  var onFollowOmarchy: ((Bool) -> Void)?

  @Environment(\.theme) private var current

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
        Rectangle().fill(vars.colorBorder).frame(height: vars.strokeHairline)
      }
      ScrollView {
        let columns = Array(
          repeating: GridItem(.flexible(), spacing: vars.spacing2),
          count: 3
        )
        LazyVStack(alignment: .leading, spacing: 0) {
          #if os(macOS)
            DazzUI.Toggle(
              pressed: sizing == .system, size: .small,
              accessibilityName: "Follow system sizing",
              action: {
                sizing = sizing == .system ? .design : .system
              }
            ) { Text("Follow system sizing") }
            .padding(.horizontal, vars.spacing4)
            .padding(.bottom, vars.spacing2)
          #endif
          #if os(macOS) || ((os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY)
            DazzUI.Toggle(
              pressed: menuPresentation == .native, size: .small,
              accessibilityName: "Use native menus",
              action: {
                menuPresentation = menuPresentation == .native ? .custom : .native
              }
            ) { Text("Use native menus") }
            .padding(.horizontal, vars.spacing4)
            .padding(.bottom, vars.spacing2)
          #endif
          if let omarchy {
            VStack(alignment: .leading, spacing: vars.spacing2) {
              SectionLabel { Text("This machine") }
                .font(vars.labelSmall.font)
                .frame(height: vars.spacing5)
              LazyVGrid(columns: columns, spacing: vars.spacing2) {
                tile(
                  palette: ThemePreviewPalette(vars: half(omarchy, dark: theme.dark).themeVariables),
                  title: omarchy.title ?? "Omarchy",
                  selected: followingOmarchy,
                  action: { onFollowOmarchy?(true) }
                )
              }
            }
            .padding(.vertical, vars.spacing2)
            .padding(.horizontal, vars.spacing4)
          }
          ForEach(themeFamilyGroups, id: \.title) { group in
            VStack(alignment: .leading, spacing: vars.spacing2) {
              SectionLabel { Text(group.title) }
                .font(vars.labelSmall.font)
                .frame(height: vars.spacing5)
              LazyVGrid(columns: columns, spacing: vars.spacing2) {
                ForEach(group.families, id: \.self) { family in
                  familyTile(family)
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

  /// The Omarchy theme in the half asked for: as it ships, or its opposite.
  private func half(_ palette: OmarchyPalette, dark: Bool) -> OmarchyPalette {
    (palette.colorScheme == .dark) == dark ? palette : palette.opposite
  }

  private func familyTile(_ family: ThemeFamily) -> some View {
    var choice = theme
    choice.family = family.value
    return tile(
      palette: ThemePreviewPalette.forTheme(choice.name),
      title: family.title,
      selected: !followingOmarchy && family.value == theme.family,
      action: {
        onFollowOmarchy?(false)
        theme.family = family.value
      }
    )
  }

  private func tile(
    palette: ThemePreviewPalette,
    title: String,
    selected: Bool,
    action: @escaping () -> Void
  ) -> some View {
    let vars = current.vars
    return OptionCard(
      selected: selected,
      padding: EdgeInsets(),
      cornerRadius: 8,
      action: action,
      title: {
        VStack(spacing: 0) {
          ThemePreview(palette: palette, height: vars.spacing10, framed: false)
          Text(title)
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
    .accessibilityLabel(title)
  }
}
