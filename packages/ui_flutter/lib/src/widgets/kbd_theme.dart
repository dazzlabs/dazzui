import 'dart:ui' show lerpDouble;

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show ThemeExtension;
import 'package:flutter/widgets.dart';

import '../painting/widget_property.dart';
import '../theme/theme.dart';

/// Overrides the default properties values for descendant [Kbd] widgets.
///
/// Descendant widgets obtain the current [KbdThemeData] object
/// using [KbdTheme.of]. Instances of [KbdThemeData] can
/// be customized with [KbdThemeData.copyWith].
///
/// Typically a [KbdThemeData] is specified as part of the
/// overall [Theme] with [ThemeData.kbdTheme].
///
/// All [KbdThemeData] properties are `null` by default.
/// When null, the [Kbd] will use the values from [ThemeData]
/// if they exist, otherwise it will provide its own defaults.
///
/// See also:
///
///  * [ThemeData], which describes the overall theme information for the
///    application.
@immutable
class KbdThemeData extends ThemeExtension<KbdThemeData> with Diagnosticable {
  /// Creates the set of color, style, and size properties used to configure [Kbd].
  const KbdThemeData({
    this.textColor,
    this.smallSize,
    this.largeSize,
    this.textStyle,
    this.padding,
  });

  /// Overrides the default value for [Kbd.textColor].
  final Color? textColor;

  /// Overrides the default value for [Kbd.smallSize].
  final double? smallSize;

  /// Overrides the default value for [Kbd.largeSize].
  final double? largeSize;

  /// Overrides the default value for [Kbd.textStyle].
  final TextStyle? textStyle;

  /// Overrides the default value for [Kbd.padding].
  final SizedWidgetProperty<EdgeInsetsGeometry>? padding;

  /// Creates a copy of this object but with the given fields replaced with the
  /// new values.
  @override
  KbdThemeData copyWith({
    Color? backgroundColor,
    Color? textColor,
    double? smallSize,
    double? largeSize,
    TextStyle? textStyle,
    SizedWidgetProperty<EdgeInsetsGeometry>? padding,
  }) {
    return KbdThemeData(
      textColor: textColor ?? this.textColor,
      smallSize: smallSize ?? this.smallSize,
      largeSize: largeSize ?? this.largeSize,
      textStyle: textStyle ?? this.textStyle,
      padding: padding ?? this.padding,
    );
  }

  /// Linearly interpolates between this [KbdThemeData] and another.
  @override
  KbdThemeData lerp(covariant ThemeExtension<KbdThemeData>? other, double t) {
    if (other is! KbdThemeData) return this;
    return KbdThemeData(
      textColor: Color.lerp(textColor, other.textColor, t),
      smallSize: lerpDouble(smallSize, other.smallSize, t),
      largeSize: lerpDouble(largeSize, other.largeSize, t),
      textStyle: TextStyle.lerp(textStyle, other.textStyle, t),
      padding: t < 0.5 ? padding : other.padding,
    );
  }

  @override
  int get hashCode => Object.hash(
    textColor,
    smallSize,
    largeSize,
    textStyle,
    padding,
  );

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    return other is KbdThemeData &&
        other.textColor == textColor &&
        other.smallSize == smallSize &&
        other.largeSize == largeSize &&
        other.textStyle == textStyle &&
        other.padding == padding;
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties.add(ColorProperty('textColor', textColor, defaultValue: null));
    properties.add(DoubleProperty('smallSize', smallSize, defaultValue: null));
    properties.add(DoubleProperty('largeSize', largeSize, defaultValue: null));
    properties.add(
      DiagnosticsProperty<TextStyle>(
        'textStyle',
        textStyle,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<SizedWidgetProperty<EdgeInsetsGeometry>>(
        'padding',
        padding,
        defaultValue: null,
      ),
    );
  }
}

/// An inherited widget that overrides the default color style, and size
/// parameters for [Kbd]s in this widget's subtree.
///
/// Values specified here override the defaults for [Kbd] properties which
/// are not given an explicit non-null value.
class KbdTheme extends InheritedTheme {
  /// Creates a theme that overrides the default properties for [Kbd]s
  /// in this widget's subtree.
  const KbdTheme({super.key, required this.data, required super.child});

  /// Specifies the default property overrides for descendant [Kbd] widgets.
  final KbdThemeData data;

  /// Retrieves the [KbdThemeData] from the closest ancestor [KbdTheme].
  ///
  /// If there is no enclosing [KbdTheme] widget, then
  /// [ThemeData.kbdTheme] is used.
  ///
  /// Typical usage is as follows:
  ///
  /// ```dart
  /// KbdThemeData theme = KbdTheme.of(context);
  /// ```
  static KbdThemeData of(BuildContext context) {
    final KbdTheme? kbdTheme = context
        .dependOnInheritedWidgetOfExactType<KbdTheme>();
    return kbdTheme?.data ?? context.themeData.kbdTheme;
  }

  @override
  Widget wrap(BuildContext context, Widget child) {
    return KbdTheme(data: data, child: child);
  }

  @override
  bool updateShouldNotify(KbdTheme oldWidget) => data != oldWidget.data;
}
