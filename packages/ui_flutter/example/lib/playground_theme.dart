import 'package:dazzui/dazzui.dart';

/// The four themes, as the SwiftUI playground names them.
enum PlaygroundTheme {
  studioLight,
  studioDark,
  brightLight,
  brightDark;

  String get title {
    return switch (this) {
      PlaygroundTheme.studioLight => 'Studio',
      PlaygroundTheme.studioDark => 'Studio Dark',
      PlaygroundTheme.brightLight => 'Bright',
      PlaygroundTheme.brightDark => 'Bright Dark',
    };
  }

  ThemeData get data {
    return switch (this) {
      PlaygroundTheme.studioLight => ThemeData.studioLight(),
      PlaygroundTheme.studioDark => ThemeData.studioDark(),
      PlaygroundTheme.brightLight => ThemeData.brightLight(),
      PlaygroundTheme.brightDark => ThemeData.brightDark(),
    };
  }
}
