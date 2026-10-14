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
  case omarchyTokyoNight
  case omarchyCatppuccin
  case omarchyCatppuccinLatte
  case omarchyEthereal
  case omarchyEverforest
  case omarchyFlexokiLight
  case omarchyGruvbox
  case omarchyHackerman
  case omarchyKanagawa
  case omarchyLastHorizon
  case omarchyLumon
  case omarchyLupine
  case omarchyMatteBlack
  case omarchyMiasma
  case omarchyNord
  case omarchyOsakaJade
  case omarchyRetro82
  case omarchyRistretto
  case omarchyRosePine
  case omarchySolitude
  case omarchyVantablack
  case omarchyWhite
  case macos27
  case macos15
  case windows11
  case ubuntu
  case debian
  case fedora
  case kde
  case spotify

  public var id: String { rawValue }

  public func palette(for colorScheme: ColorScheme) -> Palette {
    switch self {
    case .studio: return colorScheme == .dark ? .studioDark : .studioLight
    case .bright: return colorScheme == .dark ? .brightDark : .brightLight
    case .frost: return colorScheme == .dark ? .frostDark : .frostLight
    case .graphite: return colorScheme == .dark ? .graphiteDark : .graphiteLight
    case .ember: return colorScheme == .dark ? .emberDark : .emberLight
    case .nocturne: return colorScheme == .dark ? .nocturneDark : .nocturneLight
    case .omarchyTokyoNight: return colorScheme == .dark ? .omarchyTokyoNightDark : .omarchyTokyoNightLight
    case .omarchyCatppuccin: return colorScheme == .dark ? .omarchyCatppuccinDark : .omarchyCatppuccinLight
    case .omarchyCatppuccinLatte: return colorScheme == .dark ? .omarchyCatppuccinLatteDark : .omarchyCatppuccinLatteLight
    case .omarchyEthereal: return colorScheme == .dark ? .omarchyEtherealDark : .omarchyEtherealLight
    case .omarchyEverforest: return colorScheme == .dark ? .omarchyEverforestDark : .omarchyEverforestLight
    case .omarchyFlexokiLight: return colorScheme == .dark ? .omarchyFlexokiLightDark : .omarchyFlexokiLightLight
    case .omarchyGruvbox: return colorScheme == .dark ? .omarchyGruvboxDark : .omarchyGruvboxLight
    case .omarchyHackerman: return colorScheme == .dark ? .omarchyHackermanDark : .omarchyHackermanLight
    case .omarchyKanagawa: return colorScheme == .dark ? .omarchyKanagawaDark : .omarchyKanagawaLight
    case .omarchyLastHorizon: return colorScheme == .dark ? .omarchyLastHorizonDark : .omarchyLastHorizonLight
    case .omarchyLumon: return colorScheme == .dark ? .omarchyLumonDark : .omarchyLumonLight
    case .omarchyLupine: return colorScheme == .dark ? .omarchyLupineDark : .omarchyLupineLight
    case .omarchyMatteBlack: return colorScheme == .dark ? .omarchyMatteBlackDark : .omarchyMatteBlackLight
    case .omarchyMiasma: return colorScheme == .dark ? .omarchyMiasmaDark : .omarchyMiasmaLight
    case .omarchyNord: return colorScheme == .dark ? .omarchyNordDark : .omarchyNordLight
    case .omarchyOsakaJade: return colorScheme == .dark ? .omarchyOsakaJadeDark : .omarchyOsakaJadeLight
    case .omarchyRetro82: return colorScheme == .dark ? .omarchyRetro82Dark : .omarchyRetro82Light
    case .omarchyRistretto: return colorScheme == .dark ? .omarchyRistrettoDark : .omarchyRistrettoLight
    case .omarchyRosePine: return colorScheme == .dark ? .omarchyRosePineDark : .omarchyRosePineLight
    case .omarchySolitude: return colorScheme == .dark ? .omarchySolitudeDark : .omarchySolitudeLight
    case .omarchyVantablack: return colorScheme == .dark ? .omarchyVantablackDark : .omarchyVantablackLight
    case .omarchyWhite: return colorScheme == .dark ? .omarchyWhiteDark : .omarchyWhiteLight
    case .macos27: return colorScheme == .dark ? .macos27Dark : .macos27Light
    case .macos15: return colorScheme == .dark ? .macos15Dark : .macos15Light
    case .windows11: return colorScheme == .dark ? .windows11Dark : .windows11Light
    case .ubuntu: return colorScheme == .dark ? .ubuntuDark : .ubuntuLight
    case .debian: return colorScheme == .dark ? .debianDark : .debianLight
    case .fedora: return colorScheme == .dark ? .fedoraDark : .fedoraLight
    case .kde: return colorScheme == .dark ? .kdeDark : .kdeLight
    case .spotify: return colorScheme == .dark ? .spotifyDark : .spotifyLight
    }
  }
}
