import 'package:dazzui/dazzui.dart';

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
