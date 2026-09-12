import SwiftUI

// Design tokens generated from Terrazzo
// DO NOT EDIT - This file is auto-generated
public enum ThemeFamily: String, CaseIterable, Sendable, Identifiable {
  case studio
  case bright
  case frost
  case graphite
  case ember
  case nocturne

  public var id: String { rawValue }

  public func palette(for colorScheme: ColorScheme) -> Palette {
    switch self {
    case .studio: return colorScheme == .dark ? .studioDark : .studioLight
    case .bright: return colorScheme == .dark ? .brightDark : .brightLight
    case .frost: return colorScheme == .dark ? .frostDark : .frostLight
    case .graphite: return colorScheme == .dark ? .graphiteDark : .graphiteLight
    case .ember: return colorScheme == .dark ? .emberDark : .emberLight
    case .nocturne: return colorScheme == .dark ? .nocturneDark : .nocturneLight
    }
  }
}
