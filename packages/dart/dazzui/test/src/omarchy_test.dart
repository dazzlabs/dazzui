import 'dart:io';

import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

/// The run-time derivation is `ds/scripts/themes.mjs`'s, transcribed, so it
/// is held to what that script derived at build time: every built-in
/// Omarchy theme, read back from the three colours `ds/themes/omarchy.yaml`
/// gives it, has to land on the generated instance in both of its halves.
void main() {
  final themes = _readSources();

  test('the sources were read', () {
    expect(themes, hasLength(22));
  });

  for (final (name, palette) in themes) {
    for (final half in [palette, palette.opposite]) {
      final id = 'omarchy-$name-${half.brightness.name}';
      test('$id derives what the build derived', () {
        final expected = themeVariablesByName[id]!;
        final actual = half.themeVariables;
        expect(_colors(actual), _colors(expected));
        for (final step in const [
          50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950, //
        ]) {
          expect(
            actual.colorPrimary[step],
            expected.colorPrimary[step],
            reason: 'primary $step',
          );
        }
        expect(
          actual.colorPrimary.toARGB32(),
          expected.colorPrimary.toARGB32(),
        );
        expect(actual.radiusMedium, expected.radiusMedium);
        expect(_type(actual), _type(expected));
      });
    }
  }

  group('the shell style', () {
    test('at its default, is the one the build gave every Omarchy theme', () {
      // Laid over a theme that does not carry it, so each face has to be
      // moved to the step the build moved it to.
      expect(
        _type(const OmarchyShellStyle().applyTo(themeVariables)),
        _type(themeVariablesOmarchyTokyoNightDark),
      );
    });

    test('follows base-size as the shell does', () {
      final scale = OmarchyShellStyle.fromShellToml(['[font]\nbase-size = 15']);
      expect(
        [scale.caption, scale.bodySmall, scale.body, scale.subtitle],
        [12, 14, 15, 16],
      );
      final vars = scale.applyTo(themeVariables);
      expect(vars.bodyMediumFontSize, 15);
      expect(vars.bodyMediumLineHeight, 20);
      expect(vars.labelMediumFontSize, 15);
      expect(vars.labelMediumLineHeight, 15);
      expect(vars.captionSmallFontSize, 12);
      // The headlines are not the shell's to move.
      expect(vars.headlineSmall, themeVariables.headlineSmall);
    });

    test("takes a pinned step, and the user's file over the theme's", () {
      final scale = OmarchyShellStyle.fromShellToml([
        '[font]\nbase-size = 12\ntitle = 20 # big\n[spacing]\nscale = 2',
        '[font]\nbase-size = 14',
      ]);
      expect(scale.fontBaseSize, 14);
      expect(scale.title, 20);
      expect(scale.body, 14);
    });

    test('scales every length with base-size, as the shell does', () {
      final vars = OmarchyShellStyle.fromShellToml([
        '[font]\nbase-size = 16',
      ]).applyTo(themeVariablesOmarchyTokyoNightDark);
      // 16 / 12 of the shell's own: a 28px control is 37, its 10px inset 13.
      expect(vars.controlMediumSize, 37);
      expect(vars.controlMediumPaddingInline, 13);
      expect(vars.spacing2, 11);
      expect(vars.controlMediumGap, 11);
      expect(vars.iconMedium, 21);
      expect(vars.spacing0, 0);
      // Not lengths the shell scales.
      expect(vars.radiusFull, themeVariablesOmarchyTokyoNightDark.radiusFull);
      expect(vars.strokeControl, 1.5);
    });

    test('takes [spacing] scale, scale-with-font and pinned controls', () {
      final style = OmarchyShellStyle.fromShellToml([
        '[font]\nbase-size = 16\n'
            '[spacing]\nscale = 1.5\nscale-with-font = false\n'
            'control-height = 40\n',
      ]);
      expect(style.lengthScale, 1.5);
      final vars = style.applyTo(themeVariablesOmarchyTokyoNightDark);
      expect(vars.controlMediumSize, 40);
      expect(vars.controlMediumPaddingInline, 15);
      expect(vars.spacing4, 24);
    });

    test('at its default, moves no length', () {
      final vars = const OmarchyShellStyle().applyTo(
        themeVariablesOmarchyTokyoNightDark,
      );
      expect(vars.controlMediumSize, 28);
      expect(vars.controlMediumPaddingInline, 10);
      expect(vars.spacing3, 12);
      expect(vars.frameSidebarWidth, 172);
    });

    test('without a [font] section, is the default', () {
      expect(
        OmarchyShellStyle.fromShellToml(['[bar]\nsize-horizontal = 26']),
        const OmarchyShellStyle(),
      );
    });
  });

  group('colors.toml', () {
    test('reads the three colours and the mode', () {
      final palette = OmarchyPalette.tryParse('''
mode = "dark"
accent = "#b59790"
background = "#0c0b0c"
foreground = "#FAFCFB"
hyprland_active_border = "rgba(8a8588ee) rgba(e2dddcee)"
''', name: 'last-horizon')!;
      expect(palette.background, const Color(0xFF0C0B0C));
      expect(palette.foreground, const Color(0xFFFAFCFB));
      expect(palette.accent, const Color(0xFFB59790));
      expect(palette.brightness, Brightness.dark);
      expect(palette.title, 'Last Horizon');
    });

    test('without a mode, takes the one it is given, else the paper', () {
      const toml =
          'background = "#eff1f5"\nforeground = "#4c4f69"\n'
          'accent = "#1e66f5"';
      expect(
        OmarchyPalette.tryParse(toml)!.brightness,
        Brightness.light,
      );
      expect(
        OmarchyPalette.tryParse(toml, brightness: Brightness.dark)!.brightness,
        Brightness.dark,
      );
    });

    test('is null without all three colours', () {
      expect(OmarchyPalette.tryParse('background = "#000000"'), isNull);
    });
  });

  group('the current theme', () {
    late Directory current;

    setUp(() {
      current = Directory.systemTemp.createTempSync('omarchy');
      Directory('${current.path}/theme').createSync();
    });

    tearDown(() => current.deleteSync(recursive: true));

    void wear(String name, String background) {
      File('${current.path}/theme/colors.toml').writeAsStringSync(
        'background = "$background"\nforeground = "#cdd6f4"\n'
        'accent = "#89b4fa"\n',
      );
      File('${current.path}/theme.name').writeAsStringSync('$name\n');
    }

    test('is read off Omarchy\'s current directory', () async {
      wear('catppuccin', '#1e1e2e');
      final palette = await readOmarchyPalette(directory: current.path);
      expect(palette?.name, 'catppuccin');
      expect(palette?.background, const Color(0xFF1E1E2E));
      expect(palette?.shellStyle, const OmarchyShellStyle());
    });

    test(
      "takes its shell style from the theme's shell.toml and the user's",
      () async {
        wear('catppuccin', '#1e1e2e');
        File(
          '${current.path}/theme/shell.toml',
        ).writeAsStringSync('[font]\nbase-size = 13\ncaption = 9\n');
        final config = Directory('${current.path}/config')..createSync();
        File(
          '${config.path}/shell.toml',
        ).writeAsStringSync('[font]\nbase-size = 16\n');
        final palette = await readOmarchyPalette(
          directory: current.path,
          config: config.path,
        );
        expect(
          palette?.shellStyle,
          const OmarchyShellStyle(
            fontBaseSize: 16,
            fontOverrides: {'caption': 9},
          ),
        );
        expect(palette?.themeVariables.bodyMediumFontSize, 16);
      },
    );

    test("is followed when the user's shell.toml changes", () async {
      wear('catppuccin', '#1e1e2e');
      final config = Directory('${current.path}/config')..createSync();
      final seen = <int?>[];
      final subscription = watchOmarchyPalette(
        directory: current.path,
        config: config.path,
      ).listen((palette) => seen.add(palette?.shellStyle.fontBaseSize));
      await Future<void>.delayed(const Duration(milliseconds: 300));
      File(
        '${config.path}/shell.toml',
      ).writeAsStringSync('[font]\nbase-size = 14\n');
      await Future<void>.delayed(const Duration(milliseconds: 600));
      await subscription.cancel();
      expect(seen, [12, 14]);
    });

    test('is null where there is none', () async {
      expect(
        await readOmarchyPalette(directory: '${current.path}/missing'),
        isNull,
      );
    });

    test('is followed when the theme changes', () async {
      wear('catppuccin', '#1e1e2e');
      final seen = <String?>[];
      final subscription = watchOmarchyPalette(
        directory: current.path,
      ).listen((palette) => seen.add(palette?.name));
      await Future<void>.delayed(const Duration(milliseconds: 300));
      // As `omarchy-theme-set` does it: the theme directory swapped whole,
      // then the name written.
      Directory('${current.path}/theme').deleteSync(recursive: true);
      Directory('${current.path}/theme').createSync();
      wear('tokyo-night', '#1a1b26');
      await Future<void>.delayed(const Duration(milliseconds: 600));
      await subscription.cancel();
      expect(seen, ['catppuccin', 'tokyo-night']);
    });
  });
}

List<double> _type(ThemeVariables v) => [
  v.titleSmallFontSize,
  v.titleSmallLineHeight,
  v.titleMediumFontSize,
  v.titleMediumLineHeight,
  v.titleLargeFontSize,
  v.titleLargeLineHeight,
  v.bodySmallFontSize,
  v.bodySmallLineHeight,
  v.bodyMediumFontSize,
  v.bodyMediumLineHeight,
  v.bodyLargeFontSize,
  v.bodyLargeLineHeight,
  v.labelQuietFontSize,
  v.labelQuietLineHeight,
  v.labelStrongFontSize,
  v.labelStrongLineHeight,
  v.labelSmallFontSize,
  v.labelSmallLineHeight,
  v.labelMediumFontSize,
  v.labelMediumLineHeight,
  v.labelLargeFontSize,
  v.labelLargeLineHeight,
  v.captionSmallFontSize,
  v.captionSmallLineHeight,
  v.captionMediumFontSize,
  v.captionMediumLineHeight,
  v.captionLargeFontSize,
  v.captionLargeLineHeight,
];

List<Color> _colors(ThemeVariables v) => [
  v.colorCanvas,
  v.colorSurface,
  v.colorSurfaceMuted,
  v.colorSurfaceSunken,
  v.colorSurfaceSubtle,
  v.colorSurfaceInset,
  v.colorSurfaceRaised,
  v.colorSurfaceOverlay,
  v.colorSurfaceChrome,
  v.colorSurfaceColumn,
  v.colorContent,
  v.colorContentSecondary,
  v.colorContentNav,
  v.colorContentMuted,
  v.colorContentSubtle,
  v.colorContentFaint,
  v.colorBorder,
  v.colorBorderStrong,
  v.colorBorderMuted,
  v.colorOnAccent,
  v.progressGradientFrom,
  v.progressGradientTo,
];

/// `ds/themes/omarchy.yaml`'s themes, each as the palette it ships in. The
/// file is flat enough to read line by line.
List<(String, OmarchyPalette)> _readSources() {
  final source = File('../../../ds/themes/omarchy.yaml').readAsLinesSync();
  final themes = <(String, OmarchyPalette)>[];
  String? name;
  var fields = <String, String>{};
  void flush() {
    if (name == null) return;
    final palette = OmarchyPalette.tryParse(
      fields.entries.map((e) => '${e.key} = "${e.value}"').join('\n'),
      name: name,
    )!;
    themes.add((name, palette));
  }

  var inThemes = false;
  for (final line in source) {
    if (line.startsWith('themes:')) {
      inThemes = true;
      continue;
    }
    if (!inThemes) continue;
    final head = RegExp(r'^  omarchy-([a-z0-9-]+):$').firstMatch(line);
    if (head != null) {
      flush();
      name = head[1];
      fields = {};
      continue;
    }
    final field = RegExp(r'^    ([a-z]+): "?([^"]+)"?$').firstMatch(line);
    if (field != null) {
      fields[field[1] == 'scheme' ? 'mode' : field[1]!] = field[2]!;
    }
  }
  flush();
  return themes;
}
