import SwiftUI

// Design tokens generated from Terrazzo
// DO NOT EDIT - This file is auto-generated

/// The category a ``ThemeFamily`` belongs to.
public enum ThemeGroup: String, CaseIterable, Sendable, Identifiable, Hashable {
  case drawn = "drawn"
  case desktops = "desktops"
  case applications = "applications"
  case omarchy = "omarchy"

  public var id: String { rawValue }

  public var displayName: String {
    switch self {
    case .drawn: return "DazzUI"
    case .desktops: return "Desktops"
    case .applications: return "Applications"
    case .omarchy: return "Omarchy"
    }
  }
}

/// Every paired theme family the token graph carries, each with a light and dark half.
public enum ThemeFamily: String, CaseIterable, Sendable, Identifiable, Hashable {
  case bright = "bright"
  case studio = "studio"
  case frost = "frost"
  case graphite = "graphite"
  case ember = "ember"
  case nocturne = "nocturne"
  case macos27 = "macos27"
  case macos15 = "macos15"
  case windows11 = "windows11"
  case ubuntu = "ubuntu"
  case debian = "debian"
  case fedora = "fedora"
  case kde = "kde"
  case spotify = "spotify"
  case omarchyTokyoNight = "omarchy-tokyo-night"
  case omarchyCatppuccin = "omarchy-catppuccin"
  case omarchyCatppuccinLatte = "omarchy-catppuccin-latte"
  case omarchyEthereal = "omarchy-ethereal"
  case omarchyEverforest = "omarchy-everforest"
  case omarchyFlexokiLight = "omarchy-flexoki-light"
  case omarchyGruvbox = "omarchy-gruvbox"
  case omarchyHackerman = "omarchy-hackerman"
  case omarchyKanagawa = "omarchy-kanagawa"
  case omarchyLastHorizon = "omarchy-last-horizon"
  case omarchyLumon = "omarchy-lumon"
  case omarchyLupine = "omarchy-lupine"
  case omarchyMatteBlack = "omarchy-matte-black"
  case omarchyMiasma = "omarchy-miasma"
  case omarchyNord = "omarchy-nord"
  case omarchyOsakaJade = "omarchy-osaka-jade"
  case omarchyRetro82 = "omarchy-retro-82"
  case omarchyRistretto = "omarchy-ristretto"
  case omarchyRosePine = "omarchy-rose-pine"
  case omarchySolitude = "omarchy-solitude"
  case omarchyVantablack = "omarchy-vantablack"
  case omarchyWhite = "omarchy-white"

  public var id: String { rawValue }

  /// The six drawn DazzUI families.
  public static let drawn: [ThemeFamily] = [
    .bright,
    .studio,
    .frost,
    .graphite,
    .ember,
    .nocturne,
  ]

  /// The imported families (desktops, applications, and Omarchy).
  public static let imported: [ThemeFamily] = allCases.filter { !$0.isDrawn }

  private static let drawnFamilies: [ThemeFamily] = [
    .bright,
    .studio,
    .frost,
    .graphite,
    .ember,
    .nocturne,
  ]

  private static let desktopsFamilies: [ThemeFamily] = [
    .macos27,
    .macos15,
    .windows11,
    .ubuntu,
    .debian,
    .fedora,
    .kde,
  ]

  private static let applicationsFamilies: [ThemeFamily] = [
    .spotify,
  ]

  private static let omarchyFamilies: [ThemeFamily] = [
    .omarchyTokyoNight,
    .omarchyCatppuccin,
    .omarchyCatppuccinLatte,
    .omarchyEthereal,
    .omarchyEverforest,
    .omarchyFlexokiLight,
    .omarchyGruvbox,
    .omarchyHackerman,
    .omarchyKanagawa,
    .omarchyLastHorizon,
    .omarchyLumon,
    .omarchyLupine,
    .omarchyMatteBlack,
    .omarchyMiasma,
    .omarchyNord,
    .omarchyOsakaJade,
    .omarchyRetro82,
    .omarchyRistretto,
    .omarchyRosePine,
    .omarchySolitude,
    .omarchyVantablack,
    .omarchyWhite,
  ]

  public static func families(in group: ThemeGroup) -> [ThemeFamily] {
    switch group {
    case .drawn: return drawnFamilies
    case .desktops: return desktopsFamilies
    case .applications: return applicationsFamilies
    case .omarchy: return omarchyFamilies
    }
  }

  public var group: ThemeGroup {
    switch self {
    case .bright: return .drawn
    case .studio: return .drawn
    case .frost: return .drawn
    case .graphite: return .drawn
    case .ember: return .drawn
    case .nocturne: return .drawn
    case .macos27: return .desktops
    case .macos15: return .desktops
    case .windows11: return .desktops
    case .ubuntu: return .desktops
    case .debian: return .desktops
    case .fedora: return .desktops
    case .kde: return .desktops
    case .spotify: return .applications
    case .omarchyTokyoNight: return .omarchy
    case .omarchyCatppuccin: return .omarchy
    case .omarchyCatppuccinLatte: return .omarchy
    case .omarchyEthereal: return .omarchy
    case .omarchyEverforest: return .omarchy
    case .omarchyFlexokiLight: return .omarchy
    case .omarchyGruvbox: return .omarchy
    case .omarchyHackerman: return .omarchy
    case .omarchyKanagawa: return .omarchy
    case .omarchyLastHorizon: return .omarchy
    case .omarchyLumon: return .omarchy
    case .omarchyLupine: return .omarchy
    case .omarchyMatteBlack: return .omarchy
    case .omarchyMiasma: return .omarchy
    case .omarchyNord: return .omarchy
    case .omarchyOsakaJade: return .omarchy
    case .omarchyRetro82: return .omarchy
    case .omarchyRistretto: return .omarchy
    case .omarchyRosePine: return .omarchy
    case .omarchySolitude: return .omarchy
    case .omarchyVantablack: return .omarchy
    case .omarchyWhite: return .omarchy
    }
  }

  public var isDrawn: Bool {
    group == .drawn
  }

  public var displayName: String {
    switch self {
    case .bright: return "Bright"
    case .studio: return "Studio"
    case .frost: return "Frost"
    case .graphite: return "Graphite"
    case .ember: return "Ember"
    case .nocturne: return "Nocturne"
    case .macos27: return "macOS 27"
    case .macos15: return "macOS 15"
    case .windows11: return "Windows 11"
    case .ubuntu: return "Ubuntu"
    case .debian: return "Debian (GNOME)"
    case .fedora: return "Fedora (GNOME)"
    case .kde: return "KDE Plasma"
    case .spotify: return "Spotify"
    case .omarchyTokyoNight: return "Tokyo Night"
    case .omarchyCatppuccin: return "Catppuccin"
    case .omarchyCatppuccinLatte: return "Catppuccin Latte"
    case .omarchyEthereal: return "Ethereal"
    case .omarchyEverforest: return "Everforest"
    case .omarchyFlexokiLight: return "Flexoki"
    case .omarchyGruvbox: return "Gruvbox"
    case .omarchyHackerman: return "Hackerman"
    case .omarchyKanagawa: return "Kanagawa"
    case .omarchyLastHorizon: return "Last Horizon"
    case .omarchyLumon: return "Lumon"
    case .omarchyLupine: return "Lupine"
    case .omarchyMatteBlack: return "Matte Black"
    case .omarchyMiasma: return "Miasma"
    case .omarchyNord: return "Nord"
    case .omarchyOsakaJade: return "Osaka Jade"
    case .omarchyRetro82: return "Retro 82"
    case .omarchyRistretto: return "Ristretto"
    case .omarchyRosePine: return "Rosé Pine"
    case .omarchySolitude: return "Solitude"
    case .omarchyVantablack: return "Vantablack"
    case .omarchyWhite: return "White"
    }
  }

  public var light: ThemeName {
    switch self {
    case .bright: return .brightLight
    case .studio: return .studioLight
    case .frost: return .frostLight
    case .graphite: return .graphiteLight
    case .ember: return .emberLight
    case .nocturne: return .nocturneLight
    case .macos27: return .macos27Light
    case .macos15: return .macos15Light
    case .windows11: return .windows11Light
    case .ubuntu: return .ubuntuLight
    case .debian: return .debianLight
    case .fedora: return .fedoraLight
    case .kde: return .kdeLight
    case .spotify: return .spotifyLight
    case .omarchyTokyoNight: return .omarchyTokyoNightLight
    case .omarchyCatppuccin: return .omarchyCatppuccinLight
    case .omarchyCatppuccinLatte: return .omarchyCatppuccinLatteLight
    case .omarchyEthereal: return .omarchyEtherealLight
    case .omarchyEverforest: return .omarchyEverforestLight
    case .omarchyFlexokiLight: return .omarchyFlexokiLightLight
    case .omarchyGruvbox: return .omarchyGruvboxLight
    case .omarchyHackerman: return .omarchyHackermanLight
    case .omarchyKanagawa: return .omarchyKanagawaLight
    case .omarchyLastHorizon: return .omarchyLastHorizonLight
    case .omarchyLumon: return .omarchyLumonLight
    case .omarchyLupine: return .omarchyLupineLight
    case .omarchyMatteBlack: return .omarchyMatteBlackLight
    case .omarchyMiasma: return .omarchyMiasmaLight
    case .omarchyNord: return .omarchyNordLight
    case .omarchyOsakaJade: return .omarchyOsakaJadeLight
    case .omarchyRetro82: return .omarchyRetro82Light
    case .omarchyRistretto: return .omarchyRistrettoLight
    case .omarchyRosePine: return .omarchyRosePineLight
    case .omarchySolitude: return .omarchySolitudeLight
    case .omarchyVantablack: return .omarchyVantablackLight
    case .omarchyWhite: return .omarchyWhiteLight
    }
  }

  public var dark: ThemeName {
    switch self {
    case .bright: return .brightDark
    case .studio: return .studioDark
    case .frost: return .frostDark
    case .graphite: return .graphiteDark
    case .ember: return .emberDark
    case .nocturne: return .nocturneDark
    case .macos27: return .macos27Dark
    case .macos15: return .macos15Dark
    case .windows11: return .windows11Dark
    case .ubuntu: return .ubuntuDark
    case .debian: return .debianDark
    case .fedora: return .fedoraDark
    case .kde: return .kdeDark
    case .spotify: return .spotifyDark
    case .omarchyTokyoNight: return .omarchyTokyoNightDark
    case .omarchyCatppuccin: return .omarchyCatppuccinDark
    case .omarchyCatppuccinLatte: return .omarchyCatppuccinLatteDark
    case .omarchyEthereal: return .omarchyEtherealDark
    case .omarchyEverforest: return .omarchyEverforestDark
    case .omarchyFlexokiLight: return .omarchyFlexokiLightDark
    case .omarchyGruvbox: return .omarchyGruvboxDark
    case .omarchyHackerman: return .omarchyHackermanDark
    case .omarchyKanagawa: return .omarchyKanagawaDark
    case .omarchyLastHorizon: return .omarchyLastHorizonDark
    case .omarchyLumon: return .omarchyLumonDark
    case .omarchyLupine: return .omarchyLupineDark
    case .omarchyMatteBlack: return .omarchyMatteBlackDark
    case .omarchyMiasma: return .omarchyMiasmaDark
    case .omarchyNord: return .omarchyNordDark
    case .omarchyOsakaJade: return .omarchyOsakaJadeDark
    case .omarchyRetro82: return .omarchyRetro82Dark
    case .omarchyRistretto: return .omarchyRistrettoDark
    case .omarchyRosePine: return .omarchyRosePineDark
    case .omarchySolitude: return .omarchySolitudeDark
    case .omarchyVantablack: return .omarchyVantablackDark
    case .omarchyWhite: return .omarchyWhiteDark
    }
  }

  public func themeName(dark: Bool) -> ThemeName {
    dark ? self.dark : self.light
  }

  public func themeName(for colorScheme: ColorScheme) -> ThemeName {
    themeName(dark: colorScheme == .dark)
  }
}

public extension ThemeName {
  /// The ``ThemeFamily`` this palette belongs to.
  var family: ThemeFamily {
    switch self {
    case .brightLight, .brightDark: return .bright
    case .studioLight, .studioDark: return .studio
    case .frostLight, .frostDark: return .frost
    case .graphiteLight, .graphiteDark: return .graphite
    case .emberLight, .emberDark: return .ember
    case .nocturneLight, .nocturneDark: return .nocturne
    case .macos27Light, .macos27Dark: return .macos27
    case .macos15Light, .macos15Dark: return .macos15
    case .windows11Light, .windows11Dark: return .windows11
    case .ubuntuLight, .ubuntuDark: return .ubuntu
    case .debianLight, .debianDark: return .debian
    case .fedoraLight, .fedoraDark: return .fedora
    case .kdeLight, .kdeDark: return .kde
    case .spotifyLight, .spotifyDark: return .spotify
    case .omarchyTokyoNightLight, .omarchyTokyoNightDark: return .omarchyTokyoNight
    case .omarchyCatppuccinLight, .omarchyCatppuccinDark: return .omarchyCatppuccin
    case .omarchyCatppuccinLatteLight, .omarchyCatppuccinLatteDark: return .omarchyCatppuccinLatte
    case .omarchyEtherealLight, .omarchyEtherealDark: return .omarchyEthereal
    case .omarchyEverforestLight, .omarchyEverforestDark: return .omarchyEverforest
    case .omarchyFlexokiLightLight, .omarchyFlexokiLightDark: return .omarchyFlexokiLight
    case .omarchyGruvboxLight, .omarchyGruvboxDark: return .omarchyGruvbox
    case .omarchyHackermanLight, .omarchyHackermanDark: return .omarchyHackerman
    case .omarchyKanagawaLight, .omarchyKanagawaDark: return .omarchyKanagawa
    case .omarchyLastHorizonLight, .omarchyLastHorizonDark: return .omarchyLastHorizon
    case .omarchyLumonLight, .omarchyLumonDark: return .omarchyLumon
    case .omarchyLupineLight, .omarchyLupineDark: return .omarchyLupine
    case .omarchyMatteBlackLight, .omarchyMatteBlackDark: return .omarchyMatteBlack
    case .omarchyMiasmaLight, .omarchyMiasmaDark: return .omarchyMiasma
    case .omarchyNordLight, .omarchyNordDark: return .omarchyNord
    case .omarchyOsakaJadeLight, .omarchyOsakaJadeDark: return .omarchyOsakaJade
    case .omarchyRetro82Light, .omarchyRetro82Dark: return .omarchyRetro82
    case .omarchyRistrettoLight, .omarchyRistrettoDark: return .omarchyRistretto
    case .omarchyRosePineLight, .omarchyRosePineDark: return .omarchyRosePine
    case .omarchySolitudeLight, .omarchySolitudeDark: return .omarchySolitude
    case .omarchyVantablackLight, .omarchyVantablackDark: return .omarchyVantablack
    case .omarchyWhiteLight, .omarchyWhiteDark: return .omarchyWhite
    }
  }
}
