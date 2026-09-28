import 'package:dazzui/dazzui.dart';
import 'package:flutter/widgets.dart';

/// The six families the token graph carries, in the order the toolbar lists
/// them.
///
/// The toolbar picks a family and a brightness rather than a theme, which is
/// how the graph itself is shaped: a family is a mode, `dark` is a mode, and
/// a theme is the pair. Ten names in one segmented control would not fit a
/// titlebar, and would hide that structure besides.
enum PlaygroundFamily {
  studio,
  bright,
  frost,
  graphite,
  ember,
  nocturne;

  String get title {
    return switch (this) {
      PlaygroundFamily.studio => 'Studio',
      PlaygroundFamily.bright => 'Bright',
      PlaygroundFamily.frost => 'Frost',
      PlaygroundFamily.graphite => 'Graphite',
      PlaygroundFamily.ember => 'Ember',
      PlaygroundFamily.nocturne => 'Nocturne',
    };
  }
}

/// The twelve themes, as the SwiftUI playground names them.
enum PlaygroundTheme {
  studioLight(PlaygroundFamily.studio, false),
  studioDark(PlaygroundFamily.studio, true),
  brightLight(PlaygroundFamily.bright, false),
  brightDark(PlaygroundFamily.bright, true),
  frostLight(PlaygroundFamily.frost, false),
  frostDark(PlaygroundFamily.frost, true),
  graphiteLight(PlaygroundFamily.graphite, false),
  graphiteDark(PlaygroundFamily.graphite, true),
  emberLight(PlaygroundFamily.ember, false),
  emberDark(PlaygroundFamily.ember, true),
  nocturneLight(PlaygroundFamily.nocturne, false),
  nocturneDark(PlaygroundFamily.nocturne, true);

  const PlaygroundTheme(this.family, this.isDark);

  final PlaygroundFamily family;
  final bool isDark;

  /// The one theme with this family and this brightness.
  static PlaygroundTheme of(PlaygroundFamily family, {required bool dark}) {
    return PlaygroundTheme.values.firstWhere(
      (theme) => theme.family == family && theme.isDark == dark,
    );
  }

  String get title {
    return switch (this) {
      PlaygroundTheme.studioLight => 'Studio',
      PlaygroundTheme.studioDark => 'Studio Dark',
      PlaygroundTheme.brightLight => 'Bright',
      PlaygroundTheme.brightDark => 'Bright Dark',
      PlaygroundTheme.frostLight => 'Frost',
      PlaygroundTheme.frostDark => 'Frost Dark',
      PlaygroundTheme.graphiteLight => 'Graphite',
      PlaygroundTheme.graphiteDark => 'Graphite Dark',
      PlaygroundTheme.emberLight => 'Ember',
      PlaygroundTheme.emberDark => 'Ember Dark',
      PlaygroundTheme.nocturneLight => 'Nocturne',
      PlaygroundTheme.nocturneDark => 'Nocturne Dark',
    };
  }

  ThemeData get data {
    return switch (this) {
      PlaygroundTheme.studioLight => ThemeData.studioLight(),
      PlaygroundTheme.studioDark => ThemeData.studioDark(),
      PlaygroundTheme.brightLight => ThemeData.brightLight(),
      PlaygroundTheme.brightDark => ThemeData.brightDark(),
      PlaygroundTheme.frostLight => ThemeData.frostLight(),
      PlaygroundTheme.frostDark => ThemeData.frostDark(),
      PlaygroundTheme.graphiteLight => ThemeData.graphiteLight(),
      PlaygroundTheme.graphiteDark => ThemeData.graphiteDark(),
      PlaygroundTheme.emberLight => ThemeData.emberLight(),
      PlaygroundTheme.emberDark => ThemeData.emberDark(),
      PlaygroundTheme.nocturneLight => ThemeData.nocturneLight(),
      PlaygroundTheme.nocturneDark => ThemeData.nocturneDark(),
    };
  }
}

/// A theme *style* — a family — as the toolbar offers it: picked first,
/// then a brightness. `key` is the `data-theme` stem, so the theme is
/// `<key>-light` or `<key>-dark`.
///
/// [PlaygroundTheme] is the twelve drawn themes, which the Themes page and
/// the every-story test measure; these are every style the token graph
/// carries, the imported desktops and Omarchy's included — the list the
/// React playground's `THEME_STYLES` holds, in its order.
class PlaygroundStyle {
  const PlaygroundStyle(this.key, this.title);

  final String key;
  final String title;

  /// The generated theme's name in the given brightness.
  String theme({required bool dark}) => '$key-${dark ? 'dark' : 'light'}';
}

/// The styles, grouped the way the toolbar lists them.
class PlaygroundStyleGroup {
  const PlaygroundStyleGroup(this.title, this.styles);

  final String title;
  final List<PlaygroundStyle> styles;
}

const List<PlaygroundStyleGroup> playgroundStyleGroups = [
  PlaygroundStyleGroup('DazzUI', [
    PlaygroundStyle('studio', 'Studio'),
    PlaygroundStyle('bright', 'Bright'),
    PlaygroundStyle('frost', 'Frost'),
    PlaygroundStyle('graphite', 'Graphite'),
    PlaygroundStyle('ember', 'Ember'),
    PlaygroundStyle('nocturne', 'Nocturne'),
  ]),
  PlaygroundStyleGroup('Desktops', [
    PlaygroundStyle('macos27', 'macOS 27 Golden Gate'),
    PlaygroundStyle('macos15', 'macOS 15 Sequoia'),
    PlaygroundStyle('windows11', 'Windows 11'),
    PlaygroundStyle('ubuntu', 'Ubuntu'),
    PlaygroundStyle('debian', 'Debian (GNOME)'),
    PlaygroundStyle('fedora', 'Fedora (GNOME)'),
    PlaygroundStyle('kde', 'KDE Plasma'),
  ]),
  PlaygroundStyleGroup('Omarchy', [
    PlaygroundStyle('omarchy-tokyo-night', 'Tokyo Night'),
    PlaygroundStyle('omarchy-catppuccin', 'Catppuccin'),
    PlaygroundStyle('omarchy-catppuccin-latte', 'Catppuccin Latte'),
    PlaygroundStyle('omarchy-ethereal', 'Ethereal'),
    PlaygroundStyle('omarchy-everforest', 'Everforest'),
    PlaygroundStyle('omarchy-flexoki-light', 'Flexoki Light'),
    PlaygroundStyle('omarchy-gruvbox', 'Gruvbox'),
    PlaygroundStyle('omarchy-hackerman', 'Hackerman'),
    PlaygroundStyle('omarchy-kanagawa', 'Kanagawa'),
    PlaygroundStyle('omarchy-last-horizon', 'Last Horizon'),
    PlaygroundStyle('omarchy-lumon', 'Lumon'),
    PlaygroundStyle('omarchy-lupine', 'Lupine'),
    PlaygroundStyle('omarchy-matte-black', 'Matte Black'),
    PlaygroundStyle('omarchy-miasma', 'Miasma'),
    PlaygroundStyle('omarchy-nord', 'Nord'),
    PlaygroundStyle('omarchy-osaka-jade', 'Osaka Jade'),
    PlaygroundStyle('omarchy-retro-82', 'Retro 82'),
    PlaygroundStyle('omarchy-ristretto', 'Ristretto'),
    PlaygroundStyle('omarchy-rose-pine', 'Rosé Pine'),
    PlaygroundStyle('omarchy-solitude', 'Solitude'),
    PlaygroundStyle('omarchy-vantablack', 'Vantablack'),
    PlaygroundStyle('omarchy-white', 'White'),
  ]),
];

/// The theme a style and a brightness name, read off the generated
/// [themeVariablesByName] rather than a case per theme.
ThemeData playgroundThemeData(PlaygroundStyle style, {required bool dark}) {
  final ThemeVariables? vars = themeVariablesByName[style.theme(dark: dark)];
  if (vars == null) {
    throw StateError('No generated theme called ${style.theme(dark: dark)}.');
  }
  return ThemeData(
    vars: vars,
    brightness: dark ? Brightness.dark : Brightness.light,
  );
}
