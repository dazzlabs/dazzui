// ignore_for_file: annotate_overrides

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' as material;
import 'package:flutter/material.dart' show ThemeExtension;
import 'package:flutter/widgets.dart';

import '../generated/theme_variables.dart';
import '../painting/varianted_widget_state_color.dart';
import '../painting/widget_property.dart';
import '../widgets/alert_banner_theme.dart';
import '../widgets/badge_theme.dart';
import '../widgets/button_theme.dart';
import '../widgets/card_theme.dart';
import '../widgets/checkbox_theme.dart';
import '../widgets/kbd_theme.dart';
import '../widgets/sidebar_theme.dart';
import 'icon_library.dart';

export 'icon_library.dart';

/// A theme data.
class ThemeData extends ThemeExtension<ThemeData> with DiagnosticableTreeMixin {
  /// Creates a dark theme.
  static ThemeData dark({
    IconLibrary? iconLibrary,
  }) {
    return ThemeData(
      iconLibrary: iconLibrary ?? IconLibrary.material(),
      brightness: Brightness.dark,
    );
  }

  /// Creates a light theme.
  static ThemeData light({
    IconLibrary? iconLibrary,
  }) {
    return ThemeData(
      iconLibrary: iconLibrary ?? IconLibrary.material(),
      brightness: Brightness.dark,
    );
  }

  const ThemeData({
    this.vars = themeVariables,
    required this.brightness,
    required this.iconLibrary,
    this.alertBannerTheme = const AlertBannerThemeData(),
    this.badgeTheme = const BadgeThemeData(),
    this.buttonTheme = const ButtonThemeData(),
    this.cardTheme = const CardThemeData(),
    this.checkboxTheme = const CheckboxThemeData(),
    this.kbdTheme = const KbdThemeData(),
    this.sidebarTheme = const SidebarThemeData(),
  });

  /// The variables of the theme.
  final ThemeVariables vars;

  /// The brightness of the design theme.
  final Brightness brightness;

  /// The icon library of the design theme.
  final IconLibrary iconLibrary;

  /// The alert banner theme of the design theme.
  final AlertBannerThemeData alertBannerTheme;

  /// The badge theme of the design theme.
  final BadgeThemeData badgeTheme;

  /// The button theme of the design theme.
  final ButtonThemeData buttonTheme;

  /// The card theme of the design theme.
  final CardThemeData cardTheme;

  /// The checkbox theme of the design theme.
  final CheckboxThemeData checkboxTheme;

  /// The kbd theme of the design theme.
  final KbdThemeData kbdTheme;

  /// The sidebar theme of the design theme.
  final SidebarThemeData sidebarTheme;

  /// Creates a copy of this object but with the given fields replaced with the
  /// new values.
  @override
  ThemeData copyWith({
    ThemeVariables? vars,
    Brightness? brightness,
    IconLibrary? iconLibrary,
    AlertBannerThemeData? alertBannerTheme,
    BadgeThemeData? badgeTheme,
    ButtonThemeData? buttonTheme,
    CardThemeData? cardTheme,
    CheckboxThemeData? checkboxTheme,
    KbdThemeData? kbdTheme,
    SidebarThemeData? sidebarTheme,
  }) {
    return ThemeData(
      iconLibrary: iconLibrary ?? this.iconLibrary,
      brightness: brightness ?? this.brightness,
      alertBannerTheme: alertBannerTheme ?? this.alertBannerTheme,
      badgeTheme: badgeTheme ?? this.badgeTheme,
      buttonTheme: buttonTheme ?? this.buttonTheme,
      cardTheme: cardTheme ?? this.cardTheme,
      checkboxTheme: checkboxTheme ?? this.checkboxTheme,
      kbdTheme: kbdTheme ?? this.kbdTheme,
      sidebarTheme: sidebarTheme ?? this.sidebarTheme,
    );
  }

  /// Linearly interpolates between this [ThemeData] and another.
  @override
  ThemeData lerp(
    covariant ThemeExtension<ThemeData>? other,
    double t,
  ) {
    if (other is! ThemeData) return this;
    return ThemeData(
      brightness: t < 0.5 ? brightness : other.brightness,
      iconLibrary: t < 0.5 ? iconLibrary : other.iconLibrary,
      alertBannerTheme: t < 0.5 ? alertBannerTheme : other.alertBannerTheme,
      badgeTheme: t < 0.5 ? badgeTheme : other.badgeTheme,
      buttonTheme: t < 0.5 ? buttonTheme : other.buttonTheme,
      cardTheme: t < 0.5 ? cardTheme : other.cardTheme,
      checkboxTheme: t < 0.5 ? checkboxTheme : other.checkboxTheme,
      kbdTheme: t < 0.5 ? kbdTheme : other.kbdTheme,
      sidebarTheme: t < 0.5 ? sidebarTheme : other.sidebarTheme,
    );
  }

  @override
  int get hashCode => Object.hash(
    iconLibrary,
    brightness,
    alertBannerTheme,
    badgeTheme,
    buttonTheme,
    checkboxTheme,
    kbdTheme,
    sidebarTheme,
  );

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    return other is ThemeData &&
        other.iconLibrary == iconLibrary &&
        other.brightness == brightness &&
        other.alertBannerTheme == alertBannerTheme &&
        other.badgeTheme == badgeTheme &&
        other.buttonTheme == buttonTheme &&
        other.cardTheme == cardTheme &&
        other.checkboxTheme == checkboxTheme &&
        other.kbdTheme == kbdTheme &&
        other.sidebarTheme == sidebarTheme;
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('brightness', brightness))
      ..add(DiagnosticsProperty('iconLibrary', iconLibrary))
      ..add(DiagnosticsProperty('alertBannerTheme', alertBannerTheme))
      ..add(DiagnosticsProperty('badgeTheme', badgeTheme))
      ..add(DiagnosticsProperty('buttonTheme', buttonTheme))
      ..add(DiagnosticsProperty('cardTheme', cardTheme))
      ..add(DiagnosticsProperty('checkboxTheme', checkboxTheme))
      ..add(DiagnosticsProperty('kbdTheme', kbdTheme))
      ..add(DiagnosticsProperty('sidebarTheme', sidebarTheme));
  }
}

/// A extension that provides the design theme data.
extension ThemeDataBuildContextProps on BuildContext {
  /// The design theme data.
  ThemeData get themeData => Theme.of(this);
}

/// A inherited widget that provides the design theme data.
class Theme extends InheritedTheme {
  const Theme({
    super.key,
    required this.data,
    required super.child,
  });

  final ThemeData data;

  @override
  bool updateShouldNotify(Theme oldWidget) {
    return data != oldWidget.data;
  }

  @override
  Widget wrap(BuildContext context, Widget child) {
    return Theme(
      data: data,
      child: child,
    );
  }

  static ThemeData of(BuildContext context) {
    final theme = context.dependOnInheritedWidgetOfExactType<Theme>();
    return theme?.data ?? material.Theme.of(context).extension<ThemeData>()!;
  }
}

extension ThemeVariablesX on ThemeVariables {
  /// The control seed color.
  WidgetProperty<ColorSwatch<int>> get controlColor {
    return TintedWidgetProperty<ColorSwatch<int>>(
      primary: colorPrimary,
      neutral: colorNeutral,
      success: colorSuccess,
      info: colorInfo,
      warning: colorWarning,
      danger: colorDanger,
    );
  }

  WidgetProperty<Color> get controlColorSurface {
    return VariantedWidgetStateColor(
      normal: controlColorNormalSurface,
      filled: controlColorFilledSurface,
      tinted: controlColorTintedSurface,
      outlined: controlColorOutlinedSurface,
      plain: controlColorPlainSurface,
    );
  }

  WidgetProperty<Color> get controlColorContent {
    return VariantedWidgetStateColor(
      normal: controlColorNormalContent,
      filled: controlColorFilledContent,
      tinted: controlColorTintedContent,
      outlined: controlColorOutlinedContent,
      plain: controlColorPlainContent,
    );
  }

  WidgetProperty<Color> get controlColorBorder {
    return VariantedWidgetStateColor(
      normal: controlColorNormalBorder,
      filled: controlColorFilledBorder,
      tinted: controlColorTintedBorder,
      outlined: controlColorOutlinedBorder,
      plain: controlColorPlainBorder,
    );
  }
}
