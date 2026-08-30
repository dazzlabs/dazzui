import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show ThemeExtension;
import 'package:flutter/widgets.dart';

import '../painting/widget_property.dart';
import '../theme/theme.dart';

/// Overrides the default properties values for descendant [AlertBanner] widgets.
///
/// Descendant widgets obtain the current [AlertBannerThemeData] object
/// using [AlertBannerTheme.of]. Instances of [AlertBannerThemeData] can
/// be customized with [AlertBannerThemeData.copyWith].
///
/// Typically a [AlertBannerThemeData] is specified as part of the
/// overall [Theme] with [ThemeData.alertBannerTheme].
///
/// All [AlertBannerThemeData] properties are `null` by default.
/// When null, the [AlertBanner] will use the values from [ThemeData]
/// if they exist, otherwise it will provide its own defaults.
///
/// See also:
///
///  * [ThemeData], which describes the overall theme information for the
///    application.
@immutable
class AlertBannerThemeData extends ThemeExtension<AlertBannerThemeData>
    with Diagnosticable {
  /// Creates the set of color, style, and size properties used to configure [AlertBanner].
  const AlertBannerThemeData({
    this.color,
    this.minSize,
    this.margin,
    this.padding,
    this.surfaceColor,
    this.contentColor,
    this.borderColor,
    this.borderRadius,
    this.titleStyle,
    this.descriptionStyle,
  });

  final WidgetProperty<ColorSwatch<int>>? color;

  final WidgetProperty<Size>? minSize;

  final WidgetProperty<EdgeInsets>? margin;

  final WidgetProperty<EdgeInsets>? padding;

  final WidgetProperty<Color>? surfaceColor;

  final WidgetProperty<Color>? contentColor;

  final WidgetProperty<Color>? borderColor;

  final WidgetProperty<BorderRadius>? borderRadius;

  final WidgetProperty<TextStyle>? titleStyle;

  final WidgetProperty<TextStyle>? descriptionStyle;

  /// Creates a copy of this object but with the given fields replaced with the
  /// new values.
  @override
  AlertBannerThemeData copyWith({
    WidgetProperty<ColorSwatch<int>>? color,
    WidgetProperty<Size>? minSize,
    WidgetProperty<EdgeInsets>? margin,
    WidgetProperty<EdgeInsets>? padding,
    WidgetProperty<Color>? surfaceColor,
    WidgetProperty<Color>? contentColor,
    WidgetProperty<Color>? borderColor,
    WidgetProperty<BorderRadius>? borderRadius,
    WidgetProperty<TextStyle>? titleStyle,
    WidgetProperty<TextStyle>? descriptionStyle,
  }) {
    return AlertBannerThemeData(
      color: color ?? this.color,
      minSize: minSize ?? this.minSize,
      margin: margin ?? this.margin,
      padding: padding ?? this.padding,
      surfaceColor: surfaceColor ?? this.surfaceColor,
      contentColor: contentColor ?? this.contentColor,
      borderColor: borderColor ?? this.borderColor,
      borderRadius: borderRadius ?? this.borderRadius,
      titleStyle: titleStyle ?? this.titleStyle,
      descriptionStyle: descriptionStyle ?? this.descriptionStyle,
    );
  }

  /// Merges the given [other] [AlertBannerThemeData] with this [AlertBannerThemeData].
  AlertBannerThemeData merge(AlertBannerThemeData? other) {
    if (other == null) {
      return this;
    }
    return copyWith(
      color: color ?? other.color,
      minSize: minSize ?? other.minSize,
      margin: margin ?? other.margin,
      padding: padding ?? other.padding,
      surfaceColor: surfaceColor ?? other.surfaceColor,
      contentColor: contentColor ?? other.contentColor,
      borderColor: borderColor ?? other.borderColor,
      borderRadius: borderRadius ?? other.borderRadius,
      titleStyle: titleStyle ?? other.titleStyle,
      descriptionStyle: descriptionStyle ?? other.descriptionStyle,
    );
  }

  /// Linearly interpolates between this [AlertBannerThemeData] and another.
  @override
  AlertBannerThemeData lerp(
    covariant ThemeExtension<AlertBannerThemeData>? other,
    double t,
  ) {
    if (other is! AlertBannerThemeData) return this;
    return AlertBannerThemeData(
      color: t < 0.5 ? color : other.color,
      minSize: t < 0.5 ? minSize : other.minSize,
      margin: t < 0.5 ? margin : other.margin,
      padding: t < 0.5 ? padding : other.padding,
      surfaceColor: t < 0.5 ? surfaceColor : other.surfaceColor,
      contentColor: t < 0.5 ? contentColor : other.contentColor,
      borderColor: t < 0.5 ? borderColor : other.borderColor,
      borderRadius: t < 0.5 ? borderRadius : other.borderRadius,
      titleStyle: t < 0.5 ? titleStyle : other.titleStyle,
      descriptionStyle: t < 0.5 ? descriptionStyle : other.descriptionStyle,
    );
  }

  @override
  int get hashCode => Object.hash(
    color,
    minSize,
    margin,
    padding,
    surfaceColor,
    contentColor,
    borderColor,
    borderRadius,
    titleStyle,
    descriptionStyle,
  );

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    return other is AlertBannerThemeData &&
        other.color == color &&
        other.minSize == minSize &&
        other.margin == margin &&
        other.padding == padding &&
        other.surfaceColor == surfaceColor &&
        other.contentColor == contentColor &&
        other.borderColor == borderColor &&
        other.borderRadius == borderRadius &&
        other.titleStyle == titleStyle &&
        other.descriptionStyle == descriptionStyle;
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties.add(
      DiagnosticsProperty<WidgetProperty<ColorSwatch<int>>>(
        'color',
        color,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<WidgetProperty<Size>>(
        'minSize',
        minSize,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<WidgetProperty<EdgeInsetsGeometry>>(
        'margin',
        margin,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<WidgetProperty<EdgeInsetsGeometry>>(
        'padding',
        padding,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<WidgetProperty<Color>>(
        'surfaceColor',
        surfaceColor,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<WidgetProperty<Color>>(
        'contentColor',
        contentColor,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<WidgetProperty<Color>>(
        'borderColor',
        borderColor,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<WidgetProperty<BorderRadius>>(
        'borderRadius',
        borderRadius,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<WidgetProperty<TextStyle>>(
        'titleStyle',
        titleStyle,
        defaultValue: null,
      ),
    );
    properties.add(
      DiagnosticsProperty<WidgetProperty<TextStyle>>(
        'descriptionStyle',
        descriptionStyle,
        defaultValue: null,
      ),
    );
  }
}

/// An inherited widget that overrides the default color style, and size
/// parameters for [AlertBanner]s in this widget's subtree.
///
/// Values specified here override the defaults for [AlertBanner] properties which
/// are not given an explicit non-null value.
class AlertBannerTheme extends InheritedTheme {
  /// Creates a theme that overrides the default properties for [AlertBanner]s
  /// in this widget's subtree.
  const AlertBannerTheme({super.key, required this.data, required super.child});

  /// Specifies the default property overrides for descendant [AlertBanner] widgets.
  final AlertBannerThemeData data;

  /// Retrieves the [AlertBannerThemeData] from the closest ancestor [AlertBannerTheme].
  ///
  /// If there is no enclosing [AlertBannerTheme] widget, then
  /// [ThemeData.alertBannerTheme] is used.
  ///
  /// Typical usage is as follows:
  ///
  /// ```dart
  /// AlertBannerThemeData theme = AlertBannerTheme.of(context);
  /// ```
  static AlertBannerThemeData of(BuildContext context) {
    final AlertBannerTheme? alertBannerTheme = context
        .dependOnInheritedWidgetOfExactType<AlertBannerTheme>();
    return alertBannerTheme?.data ?? context.themeData.alertBannerTheme;
  }

  @override
  Widget wrap(BuildContext context, Widget child) {
    return AlertBannerTheme(data: data, child: child);
  }

  @override
  bool updateShouldNotify(AlertBannerTheme oldWidget) => data != oldWidget.data;
}
