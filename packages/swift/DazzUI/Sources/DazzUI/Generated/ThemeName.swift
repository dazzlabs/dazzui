import SwiftUI

// Design tokens generated from Terrazzo
// DO NOT EDIT - This file is auto-generated

/// Every theme the token graph carries, named as `data-theme` names it.
public enum ThemeName: String, CaseIterable, Sendable {
  case studioLight = "studio-light"
  case studioDark = "studio-dark"
  case brightLight = "bright-light"
  case brightDark = "bright-dark"
  case frostLight = "frost-light"
  case frostDark = "frost-dark"
  case graphiteLight = "graphite-light"
  case graphiteDark = "graphite-dark"
  case emberLight = "ember-light"
  case emberDark = "ember-dark"
  case nocturneLight = "nocturne-light"
  case nocturneDark = "nocturne-dark"
  case omarchyTokyoNightLight = "omarchy-tokyo-night-light"
  case omarchyTokyoNightDark = "omarchy-tokyo-night-dark"
  case omarchyCatppuccinLight = "omarchy-catppuccin-light"
  case omarchyCatppuccinDark = "omarchy-catppuccin-dark"
  case omarchyCatppuccinLatteLight = "omarchy-catppuccin-latte-light"
  case omarchyCatppuccinLatteDark = "omarchy-catppuccin-latte-dark"
  case omarchyEtherealLight = "omarchy-ethereal-light"
  case omarchyEtherealDark = "omarchy-ethereal-dark"
  case omarchyEverforestLight = "omarchy-everforest-light"
  case omarchyEverforestDark = "omarchy-everforest-dark"
  case omarchyFlexokiLightLight = "omarchy-flexoki-light-light"
  case omarchyFlexokiLightDark = "omarchy-flexoki-light-dark"
  case omarchyGruvboxLight = "omarchy-gruvbox-light"
  case omarchyGruvboxDark = "omarchy-gruvbox-dark"
  case omarchyHackermanLight = "omarchy-hackerman-light"
  case omarchyHackermanDark = "omarchy-hackerman-dark"
  case omarchyKanagawaLight = "omarchy-kanagawa-light"
  case omarchyKanagawaDark = "omarchy-kanagawa-dark"
  case omarchyLastHorizonLight = "omarchy-last-horizon-light"
  case omarchyLastHorizonDark = "omarchy-last-horizon-dark"
  case omarchyLumonLight = "omarchy-lumon-light"
  case omarchyLumonDark = "omarchy-lumon-dark"
  case omarchyLupineLight = "omarchy-lupine-light"
  case omarchyLupineDark = "omarchy-lupine-dark"
  case omarchyMatteBlackLight = "omarchy-matte-black-light"
  case omarchyMatteBlackDark = "omarchy-matte-black-dark"
  case omarchyMiasmaLight = "omarchy-miasma-light"
  case omarchyMiasmaDark = "omarchy-miasma-dark"
  case omarchyNordLight = "omarchy-nord-light"
  case omarchyNordDark = "omarchy-nord-dark"
  case omarchyOsakaJadeLight = "omarchy-osaka-jade-light"
  case omarchyOsakaJadeDark = "omarchy-osaka-jade-dark"
  case omarchyRetro82Light = "omarchy-retro-82-light"
  case omarchyRetro82Dark = "omarchy-retro-82-dark"
  case omarchyRistrettoLight = "omarchy-ristretto-light"
  case omarchyRistrettoDark = "omarchy-ristretto-dark"
  case omarchyRosePineLight = "omarchy-rose-pine-light"
  case omarchyRosePineDark = "omarchy-rose-pine-dark"
  case omarchySolitudeLight = "omarchy-solitude-light"
  case omarchySolitudeDark = "omarchy-solitude-dark"
  case omarchyVantablackLight = "omarchy-vantablack-light"
  case omarchyVantablackDark = "omarchy-vantablack-dark"
  case omarchyWhiteLight = "omarchy-white-light"
  case omarchyWhiteDark = "omarchy-white-dark"
  case macos27Light = "macos27-light"
  case macos27Dark = "macos27-dark"
  case macos15Light = "macos15-light"
  case macos15Dark = "macos15-dark"
  case windows11Light = "windows11-light"
  case windows11Dark = "windows11-dark"
  case ubuntuLight = "ubuntu-light"
  case ubuntuDark = "ubuntu-dark"
  case debianLight = "debian-light"
  case debianDark = "debian-dark"
  case fedoraLight = "fedora-light"
  case fedoraDark = "fedora-dark"
  case kdeLight = "kde-light"
  case kdeDark = "kde-dark"
  case spotifyLight = "spotify-light"
  case spotifyDark = "spotify-dark"

  public var variables: ThemeVariables {
    switch self {
    case .studioLight: return .studioLight
    case .studioDark: return .studioDark
    case .brightLight: return .brightLight
    case .brightDark: return .brightDark
    case .frostLight: return .frostLight
    case .frostDark: return .frostDark
    case .graphiteLight: return .graphiteLight
    case .graphiteDark: return .graphiteDark
    case .emberLight: return .emberLight
    case .emberDark: return .emberDark
    case .nocturneLight: return .nocturneLight
    case .nocturneDark: return .nocturneDark
    case .omarchyTokyoNightLight: return .omarchyTokyoNightLight
    case .omarchyTokyoNightDark: return .omarchyTokyoNightDark
    case .omarchyCatppuccinLight: return .omarchyCatppuccinLight
    case .omarchyCatppuccinDark: return .omarchyCatppuccinDark
    case .omarchyCatppuccinLatteLight: return .omarchyCatppuccinLatteLight
    case .omarchyCatppuccinLatteDark: return .omarchyCatppuccinLatteDark
    case .omarchyEtherealLight: return .omarchyEtherealLight
    case .omarchyEtherealDark: return .omarchyEtherealDark
    case .omarchyEverforestLight: return .omarchyEverforestLight
    case .omarchyEverforestDark: return .omarchyEverforestDark
    case .omarchyFlexokiLightLight: return .omarchyFlexokiLightLight
    case .omarchyFlexokiLightDark: return .omarchyFlexokiLightDark
    case .omarchyGruvboxLight: return .omarchyGruvboxLight
    case .omarchyGruvboxDark: return .omarchyGruvboxDark
    case .omarchyHackermanLight: return .omarchyHackermanLight
    case .omarchyHackermanDark: return .omarchyHackermanDark
    case .omarchyKanagawaLight: return .omarchyKanagawaLight
    case .omarchyKanagawaDark: return .omarchyKanagawaDark
    case .omarchyLastHorizonLight: return .omarchyLastHorizonLight
    case .omarchyLastHorizonDark: return .omarchyLastHorizonDark
    case .omarchyLumonLight: return .omarchyLumonLight
    case .omarchyLumonDark: return .omarchyLumonDark
    case .omarchyLupineLight: return .omarchyLupineLight
    case .omarchyLupineDark: return .omarchyLupineDark
    case .omarchyMatteBlackLight: return .omarchyMatteBlackLight
    case .omarchyMatteBlackDark: return .omarchyMatteBlackDark
    case .omarchyMiasmaLight: return .omarchyMiasmaLight
    case .omarchyMiasmaDark: return .omarchyMiasmaDark
    case .omarchyNordLight: return .omarchyNordLight
    case .omarchyNordDark: return .omarchyNordDark
    case .omarchyOsakaJadeLight: return .omarchyOsakaJadeLight
    case .omarchyOsakaJadeDark: return .omarchyOsakaJadeDark
    case .omarchyRetro82Light: return .omarchyRetro82Light
    case .omarchyRetro82Dark: return .omarchyRetro82Dark
    case .omarchyRistrettoLight: return .omarchyRistrettoLight
    case .omarchyRistrettoDark: return .omarchyRistrettoDark
    case .omarchyRosePineLight: return .omarchyRosePineLight
    case .omarchyRosePineDark: return .omarchyRosePineDark
    case .omarchySolitudeLight: return .omarchySolitudeLight
    case .omarchySolitudeDark: return .omarchySolitudeDark
    case .omarchyVantablackLight: return .omarchyVantablackLight
    case .omarchyVantablackDark: return .omarchyVantablackDark
    case .omarchyWhiteLight: return .omarchyWhiteLight
    case .omarchyWhiteDark: return .omarchyWhiteDark
    case .macos27Light: return .macos27Light
    case .macos27Dark: return .macos27Dark
    case .macos15Light: return .macos15Light
    case .macos15Dark: return .macos15Dark
    case .windows11Light: return .windows11Light
    case .windows11Dark: return .windows11Dark
    case .ubuntuLight: return .ubuntuLight
    case .ubuntuDark: return .ubuntuDark
    case .debianLight: return .debianLight
    case .debianDark: return .debianDark
    case .fedoraLight: return .fedoraLight
    case .fedoraDark: return .fedoraDark
    case .kdeLight: return .kdeLight
    case .kdeDark: return .kdeDark
    case .spotifyLight: return .spotifyLight
    case .spotifyDark: return .spotifyDark
    }
  }

  /// A theme's half is in its name, as it is in the stylesheet.
  public var colorScheme: ColorScheme {
    rawValue.hasSuffix("-dark") ? .dark : .light
  }
}
