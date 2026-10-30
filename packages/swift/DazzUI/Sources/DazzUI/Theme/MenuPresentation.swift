import SwiftUI

/// How ordinary menus are presented, independently from theme and sizing.
public enum MenuPresentation: String, CaseIterable, Sendable {
  case custom
  case native

  internal var usesNativeMenus: Bool {
    #if canImport(AppKit)
      self == .native
    #else
      false
    #endif
  }
}

private struct MenuPresentationKey: EnvironmentKey {
  static let defaultValue = MenuPresentation.custom
}

extension EnvironmentValues {
  public var menuPresentation: MenuPresentation {
    get { self[MenuPresentationKey.self] }
    set { self[MenuPresentationKey.self] = newValue }
  }
}

extension View {
  /// Scopes menu presentation to this subtree. The default is custom;
  /// platforms without AppKit retain custom menus. Search suggestions remain
  /// custom in either mode.
  public func menuPresentation(_ presentation: MenuPresentation) -> some View {
    environment(\.menuPresentation, presentation)
  }
}
