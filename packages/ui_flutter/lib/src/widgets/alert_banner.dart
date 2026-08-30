import 'package:flutter/widgets.dart';

import '../foundation/widget_radius.dart';
import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../foundation/widget_variant.dart';
import '../generated/theme_variables.dart';
import '../painting/widget_property.dart';
import '../theme/theme.dart';
import 'alert_banner_theme.dart';

/// The tint of an alert banner.
enum AlertBannerTint with WidgetTint {
  /// A primary banner.
  primary,

  /// A neutral banner.
  neutral,

  /// An info banner.
  info,

  /// A success banner.
  success,

  /// A warning banner.
  warning,

  /// A danger banner.
  danger,
}

/// The variant of an alert banner.
enum AlertBannerVariant with WidgetVariant {
  /// A filled alert banner.
  filled,

  /// A tinted alert banner.
  tinted,

  /// An outlined alert banner.
  outlined,
}

/// A banner widget.
class AlertBanner extends StatefulWidget {
  const AlertBanner({
    super.key,
    this.color,
    this.tint = AlertBannerTint.primary,
    this.variant = AlertBannerVariant.filled,
    this.radius,
    this.icon,
    this.title,
    this.message,
    this.actions,
  });

  /// The color of the alert banner's seed color.
  ///
  /// Defaults to null.
  final Color? color;

  /// The tint of the alert banner.
  final AlertBannerTint tint;

  /// The variant of the alert banner.
  final AlertBannerVariant variant;

  /// The radius of the alert banner's corners when it has a background color.
  final BorderRadius? radius;

  /// The icon of the alert banner.
  final Widget? icon;

  /// The title of the alert banner.
  final Widget? title;

  /// The message of the alert banner.
  final Widget? message;

  /// The set of actions that are displayed for the user to select.
  final List<Widget>? actions;

  @override
  State<AlertBanner> createState() => _AlertBannerState();
}

class _AlertBannerState extends State<AlertBanner> {
  @override
  Widget build(BuildContext context) {
    final themeDefaults = _AlertBannerDefaults(context);
    final alertBannerTheme = AlertBannerTheme.of(context).merge(themeDefaults);

    T resolve<T>(
      WidgetProperty<T> Function(AlertBannerThemeData theme) getProperty,
    ) {
      Color? seedColor =
          widget.color ?? alertBannerTheme.color?.tinted(widget.tint);
      return getProperty(alertBannerTheme).resolveWith(
        {},
        tint: widget.tint,
        variant: widget.variant,
        size: WidgetSize.medium,
        radius: widget.radius is WidgetRadius
            ? widget.radius as WidgetRadius
            : WidgetRadius.medium,
        extra: {
          'seedColor': seedColor,
        }..removeWhere((key, value) => value == null),
      );
    }

    EdgeInsets padding = resolve<EdgeInsets>((t) => t.padding!);
    Color backgroundColor = resolve<Color>((t) => t.surfaceColor!);
    Color foregroundColor = resolve<Color>((t) => t.contentColor!);
    Color borderColor = resolve<Color>((t) => t.borderColor!);
    BorderRadius borderRadius = resolve<BorderRadius>((t) => t.borderRadius!);
    TextStyle titleStyle = resolve<TextStyle>((t) => t.titleStyle!);
    TextStyle descriptionStyle = resolve<TextStyle>((t) => t.descriptionStyle!);

    return Container(
      width: double.infinity,
      padding: padding,
      foregroundDecoration: BoxDecoration(
        border: Border.all(
          color: borderColor,
          width: 1,
        ),
        borderRadius: borderRadius,
      ),
      decoration: BoxDecoration(
        color: backgroundColor,
        borderRadius: borderRadius,
      ),
      child: Row(
        spacing: 8,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (widget.icon != null)
            IconTheme(
              data: IconThemeData(
                color: foregroundColor,
                size: 18,
              ),
              child: widget.icon!,
            ),
          Expanded(
            child: Column(
              spacing: 4,
              mainAxisSize: MainAxisSize.max,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (widget.title != null)
                  DefaultTextStyle(
                    style: titleStyle.copyWith(color: foregroundColor),
                    child: widget.title!,
                  ),
                if (widget.message != null)
                  DefaultTextStyle(
                    style: descriptionStyle.copyWith(color: foregroundColor),
                    child: widget.message!,
                  ),
                if ((widget.actions ?? []).isNotEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 4),
                    child: Row(spacing: 8, children: widget.actions!),
                  ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _AlertBannerDefaults extends AlertBannerThemeData {
  _AlertBannerDefaults(this.context) : super();

  final BuildContext context;
  late final ThemeData _theme = Theme.of(context);
  late final ThemeVariables _vars = _theme.vars;

  @override
  WidgetProperty<ColorSwatch<int>>? get color {
    return _vars.controlColor;
  }

  @override
  WidgetProperty<Size>? get minSize {
    return SizedWidgetProperty<Size>(
      small: Size.square(_vars.controlSmallPaddingInline),
      medium: Size.square(_vars.controlMediumPaddingInline),
      large: Size.square(_vars.controlLargePaddingInline),
    );
  }

  @override
  WidgetProperty<EdgeInsets>? get margin {
    return WidgetPropertyAll<EdgeInsets>(EdgeInsets.zero);
  }

  @override
  WidgetProperty<EdgeInsets>? get padding {
    return SizedWidgetProperty<EdgeInsets>(
      small: EdgeInsets.symmetric(
        horizontal: _vars.controlSmallPaddingInline,
        vertical: _vars.controlSmallPaddingBlock,
      ),
      medium: EdgeInsets.symmetric(
        horizontal: _vars.controlMediumPaddingInline,
        vertical: _vars.controlMediumPaddingBlock,
      ),
      large: EdgeInsets.symmetric(
        horizontal: _vars.controlLargePaddingInline,
        vertical: _vars.controlLargePaddingBlock,
      ),
    );
  }

  @override
  WidgetProperty<Color>? get surfaceColor {
    return _vars.controlColorSurface;
  }

  @override
  WidgetProperty<Color>? get contentColor {
    return _vars.controlColorContent;
  }

  @override
  WidgetProperty<Color>? get borderColor {
    return _vars.controlColorBorder;
  }

  @override
  WidgetProperty<BorderRadius>? get borderRadius {
    return SizedWidgetProperty<BorderRadius>(
      small: BorderRadius.circular(_vars.radiusSmall),
      medium: BorderRadius.circular(_vars.radiusMedium),
      large: BorderRadius.circular(_vars.radiusLarge),
    );
  }

  @override
  WidgetProperty<TextStyle>? get titleStyle {
    return SizedWidgetProperty<TextStyle>(
      small: _vars.titleSmall,
      medium: _vars.titleMedium,
      large: _vars.titleLarge,
    );
  }

  @override
  WidgetProperty<TextStyle>? get descriptionStyle {
    return SizedWidgetProperty<TextStyle>(
      small: _vars.bodySmall,
      medium: _vars.bodyMedium,
      large: _vars.bodyLarge,
    );
  }
}
