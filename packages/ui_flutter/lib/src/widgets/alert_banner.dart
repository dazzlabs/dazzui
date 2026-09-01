import 'package:flutter/widgets.dart';

import '../foundation/color_descriptor.dart';
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
    this.tint = AlertBannerTint.neutral,
    this.size = WidgetSize.medium,
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

  /// The size of the alert banner.
  ///
  /// One shape, three densities: the type never changes, only the box around
  /// it. A banner that grew its type would compete with the content it
  /// interrupts.
  final WidgetSize size;

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
    final ThemeVariables vars = Theme.of(context).vars;
    final bool hasDescription = widget.message != null;

    T resolve<T>(
      WidgetProperty<T> Function(AlertBannerThemeData theme) getProperty,
    ) {
      Color? seedColor =
          widget.color ?? alertBannerTheme.color?.tinted(widget.tint);
      return getProperty(alertBannerTheme).resolveWith(
        {},
        tint: widget.tint,
        variant: AlertBannerVariant.tinted,
        size: widget.size,
        extra: {
          'seedColor': seedColor,
        }..removeWhere((key, value) => value == null),
      );
    }

    EdgeInsets padding = resolve<EdgeInsets>((t) => t.padding!);
    BorderRadius borderRadius = resolve<BorderRadius>((t) => t.borderRadius!);
    TextStyle titleStyle = resolve<TextStyle>((t) => t.titleStyle!);
    TextStyle descriptionStyle = resolve<TextStyle>((t) => t.descriptionStyle!);

    // Fill and edge are washes of the tint rather than opaque steps, so the
    // banner sits as correctly on a card as on the paper. The alphas are the
    // shared surface wash rather than the `tinted` recipe's: that recipe
    // fills a chip at 12%, which is right for something the size of a word
    // and twice too heavy spread across a full width.
    final ColorSwatch<int> ramp = alertBannerTheme.color!.tinted(widget.tint);
    final Color backgroundColor = ramp[600]!.withValues(
      alpha: vars.washSurface,
    );
    final Color borderColor = ramp[600]!.withValues(alpha: vars.washEdge);

    // The text grades of the tint, not its fill: a notice is read, not
    // pressed. The title takes the hovered grade, a step darker than the
    // description's resting one.
    final ColorDescriptor tintedContent = vars.controlColorTintedContent;
    final Color titleColor = ramp[tintedContent.hoveredShade!]!;
    final Color descriptionColor = ramp[tintedContent.normalShade!]!;

    return Container(
      width: double.infinity,
      padding: padding,
      decoration: BoxDecoration(
        color: backgroundColor,
        border: Border.all(color: borderColor, width: context.hairlineWidth),
        borderRadius: borderRadius,
      ),
      child: Row(
        spacing: vars.spacing25,
        // Once a description wraps, the icon belongs beside the first line
        // rather than halfway down the block.
        crossAxisAlignment: hasDescription
            ? CrossAxisAlignment.start
            : CrossAxisAlignment.center,
        children: [
          if (widget.icon != null)
            IconTheme(
              data: IconThemeData(
                color: ramp[600],
                size: vars.spacing4,
              ),
              child: widget.icon!,
            ),
          Expanded(
            child: Column(
              // The title/description pair is one block of text, so it is set
              // tighter than the gap separating it from the icon and the
              // action.
              spacing: vars.spacing1,
              mainAxisSize: MainAxisSize.max,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (widget.title != null)
                  DefaultTextStyle(
                    style: titleStyle.copyWith(color: titleColor),
                    child: widget.title!,
                  ),
                if (widget.message != null)
                  DefaultTextStyle(
                    style: descriptionStyle.copyWith(color: descriptionColor),
                    child: widget.message!,
                  ),
                if ((widget.actions ?? []).isNotEmpty)
                  Padding(
                    padding: EdgeInsets.only(top: vars.spacing1),
                    child: Row(
                      spacing: vars.spacing2,
                      children: widget.actions!,
                    ),
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
        horizontal: _vars.spacing3,
        vertical: _vars.spacing2,
      ),
      medium: EdgeInsets.symmetric(
        horizontal: _vars.spacing35,
        vertical: _vars.spacing3,
      ),
      large: EdgeInsets.all(_vars.spacing4),
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
      small: BorderRadius.circular(_vars.radiusMedium),
      medium: BorderRadius.circular(_vars.radiusLarge),
      large: BorderRadius.circular(_vars.radiusLarge),
    );
  }

  /// The type never changes with the size — only the box does.
  @override
  WidgetProperty<TextStyle>? get titleStyle {
    return WidgetPropertyAll<TextStyle>(
      _vars.labelMedium.copyWith(height: 1.4),
    );
  }

  @override
  WidgetProperty<TextStyle>? get descriptionStyle {
    return WidgetPropertyAll<TextStyle>(
      _vars.labelQuiet.copyWith(
        fontWeight: _vars.captionSmall.fontWeight,
        height: 1.5,
      ),
    );
  }
}
