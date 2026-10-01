import 'package:flutter/widgets.dart';

import '../generated/theme_variables.dart';
import '../theme/theme.dart';
import 'derive.dart';

/// An Omarchy theme, as the three colours DazzUI reads it off: the
/// background, the foreground and the accent of its `colors.toml`, in the
/// one appearance it ships in.
///
/// The generated `omarchy-*` themes are the built-in ones, derived at build
/// time; this is the same derivation at run time, so a machine wearing a
/// theme DazzUI has never seen — one the user installed or wrote — is drawn
/// in it too. Read the machine's current one with `readOmarchyPalette`, or
/// follow it with `watchOmarchyPalette` or the [OmarchyTheme] widget.
@immutable
class OmarchyPalette {
  const OmarchyPalette({
    required this.background,
    required this.foreground,
    required this.accent,
    required this.brightness,
    this.name,
  });

  /// A palette out of a `colors.toml`.
  ///
  /// Only `background`, `foreground` and `accent` are read, with `mode`
  /// where the file states one (`"light"` or `"dark"`). A file without it —
  /// Omarchy's older ones marked a light theme with a `light.mode` file
  /// beside it instead — takes [brightness] when given, else the
  /// background's own lightness. Returns null when one of the three colours
  /// is missing or is not a hex colour.
  static OmarchyPalette? tryParse(
    String colorsToml, {
    String? name,
    Brightness? brightness,
  }) {
    final values = <String, String>{};
    for (final raw in colorsToml.split('\n')) {
      final line = raw.trim();
      if (line.isEmpty || line.startsWith('#') || line.startsWith('[')) {
        continue;
      }
      final match = _entry.firstMatch(line);
      if (match != null) values[match[1]!] = match[2] ?? match[3]!;
    }
    final background = _hex(values['background']);
    final foreground = _hex(values['foreground']);
    final accent = _hex(values['accent']);
    if (background == null || foreground == null || accent == null) {
      return null;
    }
    final mode = switch (values['mode']?.toLowerCase()) {
      'light' => Brightness.light,
      'dark' => Brightness.dark,
      _ => null,
    };
    return OmarchyPalette(
      background: background,
      foreground: foreground,
      accent: accent,
      brightness:
          mode ??
          brightness ??
          (background.computeLuminance() > 0.5
              ? Brightness.light
              : Brightness.dark),
      name: name,
    );
  }

  final Color background;
  final Color foreground;
  final Color accent;

  /// The appearance the theme ships in.
  final Brightness brightness;

  /// The theme's name as Omarchy stores it (`tokyo-night`), when known.
  final String? name;

  /// The same theme in the other appearance, derived from these three
  /// colours the way the generated themes' other halves are. DazzUI's
  /// reading of the theme, not Omarchy's.
  OmarchyPalette get opposite {
    final other = oppositeOmarchyColors(
      background: background,
      foreground: foreground,
      accent: accent,
      dark: brightness == Brightness.dark,
    );
    return OmarchyPalette(
      background: other.background,
      foreground: other.foreground,
      accent: other.accent,
      brightness: brightness == Brightness.dark
          ? Brightness.light
          : Brightness.dark,
      name: name,
    );
  }

  /// Every token, with this palette's colours derived into it and Omarchy's
  /// square geometry.
  ThemeVariables get themeVariables => deriveOmarchyThemeVariables(
    background: background,
    foreground: foreground,
    accent: accent,
    dark: brightness == Brightness.dark,
  );

  ThemeData get themeData =>
      ThemeData(vars: themeVariables, brightness: brightness);

  /// The name the way `omarchy theme current` prints it: `Tokyo Night`.
  String? get title => name
      ?.split('-')
      .where((word) => word.isNotEmpty)
      .map((word) => word[0].toUpperCase() + word.substring(1))
      .join(' ');

  @override
  bool operator ==(Object other) =>
      other is OmarchyPalette &&
      other.background == background &&
      other.foreground == foreground &&
      other.accent == accent &&
      other.brightness == brightness &&
      other.name == name;

  @override
  int get hashCode =>
      Object.hash(background, foreground, accent, brightness, name);

  @override
  String toString() =>
      'OmarchyPalette(${name ?? 'unnamed'}, ${brightness.name}, '
      'background: $background, foreground: $foreground, accent: $accent)';
}

final _entry = RegExp(
  r'''^([A-Za-z0-9_-]+)\s*=\s*(?:"([^"]*)"|'([^']*)')''',
);

Color? _hex(String? value) {
  final match = RegExp(
    r'^#?([0-9a-fA-F]{6})([0-9a-fA-F]{2})?$',
  ).firstMatch(value?.trim() ?? '');
  if (match == null) return null;
  final rgb = int.parse(match[1]!, radix: 16);
  final alpha = match[2] == null ? 0xFF : int.parse(match[2]!, radix: 16);
  return Color((alpha << 24) | rgb);
}
