import SwiftUI

/// One name on the bar, and the rows it opens.
public struct MenubarMenu: Identifiable {
  public var id: String { label }
  public var label: String
  public var items: [MenuItem]
  public var disabled: Bool

  public init(_ label: String, items: [MenuItem], disabled: Bool = false) {
    self.label = label
    self.items = items
    self.disabled = disabled
  }
}

/// The application menu bar: several menus that behave as one.
///
/// That is the whole reason it exists rather than a row of ``Menu``s. Once one
/// of them is open, moving the pointer along the bar opens the next without a
/// second click — the behaviour every desktop menu bar has and none of them
/// documents.
///
/// Each panel is a ``MenuSheet`` of ``MenuRow``s, so the drawing is the same
/// everywhere; the bar adds only its triggers.
public struct Menubar: View {
  public var menus: [MenubarMenu]
  public var tint: Tint

  @Environment(\.theme) private var theme
  @Environment(\.menuPresentation) private var presentation
  @State private var open: String?
  @StateObject private var nativeBar = NativeMenuBar()

  public init(menus: [MenubarMenu], tint: Tint = .neutral) {
    self.menus = menus
    self.tint = tint
  }

  public var body: some View {
    let vars = theme.vars

    HStack(spacing: vars.spacing05) {
      ForEach(menus) { menu in
        MenubarTrigger(
          menu: menu,
          open: open == menu.id,
          // Once one is open the bar is armed: crossing another name opens it,
          // which is what makes this a bar rather than a row of separate
          // menus.
          armed: open != nil,
          tint: tint,
          toggle: { open = open == menu.id ? nil : menu.id },
          enter: { if open != nil { open = menu.id } }
        )
        .nativeMenu(
          isPresented: Binding(get: { open == menu.id }, set: { if !$0 && open == menu.id { open = nil } }),
          entries: menu.disabled ? [] : menu.items.map(NativeMenuEntry.item),
          bar: nativeBar, menuID: menu.id, switchMenu: { open = menu.id }
        )
        .overlay(alignment: .bottomLeading) {
          if open == menu.id && !presentation.usesNativeMenus {
            MenuSheet {
              ForEach(menu.items) { item in
                MenuRow(item: item, tint: tint) { open = nil }
              }
            }
            .alignmentGuide(.bottom) { _ in 0 }
            .offset(y: vars.spacing1)
          }
        }
      }
    }
    .background {
      if open != nil && !presentation.usesNativeMenus {
        SwiftUI.Button("") { open = nil }
          .keyboardShortcut(.cancelAction)
          .opacity(0)
          .frame(width: 0, height: 0)
      }
    }
  }
}

/// A name on the bar.
///
/// It is chrome rather than a control, so it has no box at rest: the name is
/// the affordance, and the wash arrives only under the pointer or while its
/// own menu is open. An open trigger keeps the wash — a bar where the open
/// menu's name looked idle would leave the panel belonging to nothing.
private struct MenubarTrigger: View {
  let menu: MenubarMenu
  let open: Bool
  let armed: Bool
  let tint: Tint
  let toggle: () -> Void
  let enter: () -> Void

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(.small)

    Pressable(action: toggle, enabled: !menu.disabled, cornerRadius: vars.radiusSmall) { state in
      Text(menu.label)
        .font(.system(size: metrics.content.size, weight: metrics.content.weight))
        .foregroundStyle(ink)
        .padding(.horizontal, vars.spacing2)
        .frame(height: metrics.size)
        .background(washed(state: state) ? wash : .clear)
        .clipShape(RoundedRectangle(cornerRadius: vars.radiusSmall, style: .circular))
        .animation(vars.motion, value: state.controlState)
    }
    .onHover { hovered in
      if hovered && armed && !menu.disabled { enter() }
    }
  }

  private func washed(state: PressableState) -> Bool {
    !menu.disabled && (open || state.hovered || state.pressed)
  }

  private var wash: Color {
    theme.vars.resolve(theme.vars.descriptor(.normal, .surface), tint: tint, state: .hovered)
  }

  private var ink: Color {
    menu.disabled
      ? theme.vars.resolve(
        theme.vars.descriptor(.normal, .content),
        tint: tint,
        state: .disabled
      )
      : theme.vars.colorContent
  }
}
