import 'package:flutter/widgets.dart';

import '../foundation/widget_radius.dart';
import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../foundation/widget_variant.dart';
import '../generated/theme_variables.dart';
import '../painting/widget_property.dart';
import '../theme/theme.dart';
import 'badge_theme.dart';

/// An enum to define the tint of a badge.
enum BadgeTint with WidgetTint {
  /// A badge indicating a primary action.
  primary,

  /// A badge indicating a neutral action.
  neutral,

  /// A badge indicating an info.
  info,

  /// A badge indicating a success.
  success,

  /// A badge indicating a warning.
  warning,

  /// A badge indicating a danger.
  danger,
}

enum BadgeVariant with WidgetVariant {
  filled,
  tinted,
  outlined,
  plain,
}

/// A badge widget.
///
/// Takes in a text or an icon that fades out and in on touch. May optionally have a
/// background.
///
/// The [padding] defaults to 16.0 pixels. When using a [Badge] within
/// a fixed height parent, like a [CupertinoNavigationBar], a smaller, or even
/// [EdgeInsets.zero], should be used to prevent clipping larger [child]
/// widgets.
///
/// Preserves any parent [IconThemeData] but overwrites its [IconThemeData.color]
/// with the [CupertinoThemeData.primaryColor] (or
/// [CupertinoThemeData.primaryContrastingColor] if the badge is disabled).
///
/// {@tool dartpad}
/// This sample shows produces an enabled and disabled [Badge] and
/// [Badge.filled].
///
/// ** See code in examples/api/lib/cupertino/badge/cupertino_badge.0.dart **
/// {@end-tool}
///
/// See also:
///
///  * <https://developer.apple.com/design/human-interface-guidelines/badges/>
class Badge extends StatefulWidget {
  /// Creates an iOS-style badge.
  const Badge({
    super.key,
    required this.child,
    this.color,
    this.variant,
    this.tint,
    this.size = WidgetSize.medium,
    this.radius = WidgetRadius.full,
    this.padding,
    this.alignment = Alignment.center,
  });

  /// The widget below this widget in the tree.
  ///
  /// Typically a [Text] widget.
  final Widget child;

  /// The color of the badge's seed color.
  ///
  /// Defaults to null.
  final Color? color;

  /// The tint of the badge.
  ///
  /// Defaults to null.
  final BadgeTint? tint;

  /// The variant of the badge.
  ///
  /// Defaults to null.
  final BadgeVariant? variant;

  /// The size of the badge.
  ///
  /// Defaults to [WidgetSize.medium].
  final Size size;

  /// The radius of the badge's corners when it has a background color.
  ///
  /// Defaults to round corners of 8 logical pixels.
  final BorderRadius? radius;

  /// The amount of space to surround the child inside the bounds of the badge.
  ///
  /// Defaults to 16.0 pixels.
  final EdgeInsetsGeometry? padding;

  /// The alignment of the badge's [child].
  ///
  /// Typically badges are sized to be just big enough to contain the child and its
  /// [padding]. If the badge's size is constrained to a fixed size, for example by
  /// enclosing it with a [SizedBox], this property defines how the child is aligned
  /// within the available space.
  ///
  /// Always defaults to [Alignment.center].
  final AlignmentGeometry alignment;

  @override
  State<Badge> createState() => _BadgeState();
}

class _BadgeState extends State<Badge> with SingleTickerProviderStateMixin {
  @override
  Widget build(BuildContext context) {
    final themeDefaults = _BadgeDefaults(context);
    final badgeTheme = BadgeTheme.of(context).merge(themeDefaults);

    // Resolve the property for the given theme.
    T resolve<T>(
      WidgetProperty<T> Function(BadgeThemeData theme) getProperty,
    ) {
      Color? seedColor =
          widget.color ??
          badgeTheme.color?.tinted(widget.tint ?? BadgeTint.primary);
      return getProperty(badgeTheme).resolveWith(
        {},
        tint: widget.tint,
        size: widget.size is WidgetSize ? widget.size as WidgetSize : null,
        variant: widget.variant,
        radius: widget.radius is WidgetRadius
            ? widget.radius as WidgetRadius
            : WidgetRadius.medium,
        extra: {
          'seedColor': seedColor,
        }..removeWhere((key, value) => value == null),
      );
    }

    Size minSize = resolve<Size>((t) => t.minSize!);
    // EdgeInsets margin = resolve<EdgeInsets>((t) => t.margin!);
    EdgeInsets padding = resolve<EdgeInsets>((t) => t.padding!);
    Color backgroundColor = resolve<Color>((t) => t.surfaceColor!);
    Color foregroundColor = resolve<Color>((t) => t.contentColor!);
    Color borderColor = resolve<Color>((t) => t.borderColor!);
    BorderRadius borderRadius = resolve<BorderRadius>((t) => t.borderRadius!);
    TextStyle labelStyle = resolve<TextStyle>((t) => t.labelStyle!);

    return ConstrainedBox(
      constraints: BoxConstraints(
        minWidth: minSize.width,
        minHeight: minSize.height,
      ),
      child: Container(
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
        child: Padding(
          padding: padding,
          child: Align(
            alignment: widget.alignment,
            widthFactor: 1.0,
            heightFactor: 1.0,
            child: DefaultTextStyle(
              style: labelStyle.copyWith(
                color: foregroundColor,
              ),
              child: IconTheme(
                data: IconThemeData(
                  color: foregroundColor,
                ),
                child: widget.child,
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _BadgeDefaults extends BadgeThemeData {
  _BadgeDefaults(this.context) : super();

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
      small: Size.square(_vars.controlTinyPaddingInline),
      medium: Size.square(_vars.controlSmallPaddingInline),
      large: Size.square(_vars.controlMediumPaddingInline),
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
        horizontal: _vars.controlTinyPaddingInline,
        vertical: _vars.controlTinyPaddingBlock,
      ),
      medium: EdgeInsets.symmetric(
        horizontal: _vars.controlSmallPaddingInline,
        vertical: _vars.controlSmallPaddingBlock,
      ),
      large: EdgeInsets.symmetric(
        horizontal: _vars.controlMediumPaddingInline,
        vertical: _vars.controlMediumPaddingBlock,
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
  WidgetProperty<TextStyle>? get labelStyle {
    return SizedWidgetProperty<TextStyle>(
      small: _vars.captionSmall,
      medium: _vars.captionMedium,
      large: _vars.captionLarge,
    );
  }
}
