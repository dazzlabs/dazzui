import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show ThemeExtension;
import 'package:flutter/widgets.dart';

import '../foundation/color_descriptor.dart';
import '../generated/theme_variables.dart';
import '../painting/widget_property.dart';
import '../theme/theme.dart';

/// Overrides the default properties values for descendant Sidebar widgets.
@immutable
class SidebarThemeData extends ThemeExtension<SidebarThemeData>
    with Diagnosticable {
  const SidebarThemeData({
    this.width,
    this.collapsedWidth,
    this.padding,
    this.headerPadding,
    this.footerPadding,
    this.backgroundColor,
    this.foregroundColor,
    this.borderColor,
    this.accentColor,
    this.accentForegroundColor,
    this.groupLabelStyle,
    this.menuItemPadding,
    this.menuItemGap,
    this.menuItemIconSize,
    this.menuItemBorderRadius,
    this.menuItemBackgroundColor,
    this.menuItemForegroundColor,
    this.animationDuration,
  });

  /// Expanded sidebar width.
  final double? width;

  /// Collapsed (icon-only) sidebar width.
  final double? collapsedWidth;

  /// Padding for the sidebar container.
  final EdgeInsets? padding;

  /// Padding for [SidebarHeader].
  final EdgeInsets? headerPadding;

  /// Padding for [SidebarFooter].
  final EdgeInsets? footerPadding;

  /// Sidebar background color.
  final Color? backgroundColor;

  /// Default text/icon color inside sidebar.
  final Color? foregroundColor;

  /// Sidebar border color.
  final Color? borderColor;

  /// Accent background for hovered/selected items.
  final Color? accentColor;

  /// Foreground color on accent background.
  final Color? accentForegroundColor;

  /// Style for [SidebarGroupLabel].
  final TextStyle? groupLabelStyle;

  /// Padding for [SidebarMenuButton].
  final EdgeInsets? menuItemPadding;

  /// Horizontal gap between icon and label in [SidebarMenuButton].
  final double? menuItemGap;

  /// Icon size in [SidebarMenuButton].
  final double? menuItemIconSize;

  /// Border radius for [SidebarMenuButton] background.
  final BorderRadius? menuItemBorderRadius;

  /// Background color for [SidebarMenuButton], resolved by widget states.
  final WidgetProperty<Color>? menuItemBackgroundColor;

  /// Foreground color for [SidebarMenuButton], resolved by widget states.
  final WidgetProperty<Color>? menuItemForegroundColor;

  /// Animation duration for collapsing/expanding.
  final Duration? animationDuration;

  @override
  SidebarThemeData copyWith({
    double? width,
    double? collapsedWidth,
    EdgeInsets? padding,
    EdgeInsets? headerPadding,
    EdgeInsets? footerPadding,
    Color? backgroundColor,
    Color? foregroundColor,
    Color? borderColor,
    Color? accentColor,
    Color? accentForegroundColor,
    TextStyle? groupLabelStyle,
    EdgeInsets? menuItemPadding,
    double? menuItemGap,
    double? menuItemIconSize,
    BorderRadius? menuItemBorderRadius,
    WidgetProperty<Color>? menuItemBackgroundColor,
    WidgetProperty<Color>? menuItemForegroundColor,
    Duration? animationDuration,
  }) {
    return SidebarThemeData(
      width: width ?? this.width,
      collapsedWidth: collapsedWidth ?? this.collapsedWidth,
      padding: padding ?? this.padding,
      headerPadding: headerPadding ?? this.headerPadding,
      footerPadding: footerPadding ?? this.footerPadding,
      backgroundColor: backgroundColor ?? this.backgroundColor,
      foregroundColor: foregroundColor ?? this.foregroundColor,
      borderColor: borderColor ?? this.borderColor,
      accentColor: accentColor ?? this.accentColor,
      accentForegroundColor:
          accentForegroundColor ?? this.accentForegroundColor,
      groupLabelStyle: groupLabelStyle ?? this.groupLabelStyle,
      menuItemPadding: menuItemPadding ?? this.menuItemPadding,
      menuItemGap: menuItemGap ?? this.menuItemGap,
      menuItemIconSize: menuItemIconSize ?? this.menuItemIconSize,
      menuItemBorderRadius: menuItemBorderRadius ?? this.menuItemBorderRadius,
      menuItemBackgroundColor:
          menuItemBackgroundColor ?? this.menuItemBackgroundColor,
      menuItemForegroundColor:
          menuItemForegroundColor ?? this.menuItemForegroundColor,
      animationDuration: animationDuration ?? this.animationDuration,
    );
  }

  SidebarThemeData merge(SidebarThemeData? other) {
    if (other == null) return this;
    return copyWith(
      width: width ?? other.width,
      collapsedWidth: collapsedWidth ?? other.collapsedWidth,
      padding: padding ?? other.padding,
      headerPadding: headerPadding ?? other.headerPadding,
      footerPadding: footerPadding ?? other.footerPadding,
      backgroundColor: backgroundColor ?? other.backgroundColor,
      foregroundColor: foregroundColor ?? other.foregroundColor,
      borderColor: borderColor ?? other.borderColor,
      accentColor: accentColor ?? other.accentColor,
      accentForegroundColor:
          accentForegroundColor ?? other.accentForegroundColor,
      groupLabelStyle: groupLabelStyle ?? other.groupLabelStyle,
      menuItemPadding: menuItemPadding ?? other.menuItemPadding,
      menuItemGap: menuItemGap ?? other.menuItemGap,
      menuItemIconSize: menuItemIconSize ?? other.menuItemIconSize,
      menuItemBorderRadius: menuItemBorderRadius ?? other.menuItemBorderRadius,
      menuItemBackgroundColor:
          menuItemBackgroundColor ?? other.menuItemBackgroundColor,
      menuItemForegroundColor:
          menuItemForegroundColor ?? other.menuItemForegroundColor,
      animationDuration: animationDuration ?? other.animationDuration,
    );
  }

  @override
  SidebarThemeData lerp(
    covariant ThemeExtension<SidebarThemeData>? other,
    double t,
  ) {
    if (other is! SidebarThemeData) return this;
    return SidebarThemeData(
      width: t < 0.5 ? width : other.width,
      collapsedWidth: t < 0.5 ? collapsedWidth : other.collapsedWidth,
      padding: t < 0.5 ? padding : other.padding,
      headerPadding: t < 0.5 ? headerPadding : other.headerPadding,
      footerPadding: t < 0.5 ? footerPadding : other.footerPadding,
      backgroundColor: t < 0.5 ? backgroundColor : other.backgroundColor,
      foregroundColor: t < 0.5 ? foregroundColor : other.foregroundColor,
      borderColor: t < 0.5 ? borderColor : other.borderColor,
      accentColor: t < 0.5 ? accentColor : other.accentColor,
      accentForegroundColor: t < 0.5
          ? accentForegroundColor
          : other.accentForegroundColor,
      groupLabelStyle: t < 0.5 ? groupLabelStyle : other.groupLabelStyle,
      menuItemPadding: t < 0.5 ? menuItemPadding : other.menuItemPadding,
      menuItemGap: t < 0.5 ? menuItemGap : other.menuItemGap,
      menuItemIconSize: t < 0.5 ? menuItemIconSize : other.menuItemIconSize,
      menuItemBorderRadius: t < 0.5
          ? menuItemBorderRadius
          : other.menuItemBorderRadius,
      menuItemBackgroundColor: t < 0.5
          ? menuItemBackgroundColor
          : other.menuItemBackgroundColor,
      menuItemForegroundColor: t < 0.5
          ? menuItemForegroundColor
          : other.menuItemForegroundColor,
      animationDuration: t < 0.5 ? animationDuration : other.animationDuration,
    );
  }
}

/// An inherited widget that overrides the default values for sidebars
/// in this widget's subtree.
class SidebarTheme extends InheritedTheme {
  const SidebarTheme({super.key, required this.data, required super.child});

  final SidebarThemeData data;

  static SidebarThemeData of(BuildContext context) {
    final SidebarTheme? sidebarTheme = context
        .dependOnInheritedWidgetOfExactType<SidebarTheme>();
    return sidebarTheme?.data ?? context.themeData.sidebarTheme;
  }

  @override
  Widget wrap(BuildContext context, Widget child) {
    return SidebarTheme(data: data, child: child);
  }

  @override
  bool updateShouldNotify(SidebarTheme oldWidget) => data != oldWidget.data;
}

class _SidebarDefaults extends SidebarThemeData {
  _SidebarDefaults(this.context) : super();

  final BuildContext context;

  late final ThemeData _theme = Theme.of(context);
  late final ThemeVariables _vars = _theme.vars;

  /// The column's width is a token rather than a layout choice because the
  /// columns of a desktop shell have to agree with each other.
  @override
  double? get width => _vars.frameSidebarWidth;

  @override
  double? get collapsedWidth => _vars.controlLargeSize + _vars.spacing6;

  @override
  EdgeInsets? get padding => EdgeInsets.symmetric(
    horizontal: _vars.spacing25,
    vertical: _vars.spacing35,
  );

  /// The header strip is exactly the titlebar's height, so the sidebar and
  /// the toolbar beside it meet on one line.
  @override
  EdgeInsets? get headerPadding =>
      EdgeInsets.symmetric(horizontal: _vars.spacing4);

  @override
  EdgeInsets? get footerPadding => EdgeInsets.symmetric(
    horizontal: _vars.spacing2,
    vertical: _vars.spacing2,
  );

  @override
  Color? get backgroundColor => _vars.colorSurfaceMuted;

  @override
  Color? get foregroundColor => _vars.colorContent;

  @override
  Color? get borderColor => _vars.colorBorder;

  @override
  Color? get accentColor => _vars.colorSurface;

  @override
  Color? get accentForegroundColor => _vars.colorContent;

  @override
  TextStyle? get groupLabelStyle => _vars.labelSmall.copyWith(
    color: _vars.colorContentSubtle,
  );

  /// The sidebar row: 12px on 8px pads — 28px, the regular control height.
  @override
  EdgeInsets? get menuItemPadding => EdgeInsets.symmetric(
    horizontal: _vars.spacing25,
    vertical: _vars.spacing2,
  );

  @override
  double? get menuItemGap => _vars.spacing15;

  @override
  double? get menuItemIconSize => _vars.spacing4;

  @override
  BorderRadius? get menuItemBorderRadius =>
      BorderRadius.circular(_vars.radiusSmall);

  /// The idle row rests transparent, its hover is the accent at the `plain`
  /// recipe's wash — a preview of the filled selection sitting far below
  /// it — and the selected row takes the filled accent outright. A menu item
  /// or a table row hovers neutral instead; the difference is what the
  /// selected state is.
  @override
  WidgetProperty<Color>? get menuItemBackgroundColor {
    final ColorDescriptor plain = _vars.controlColorPlainSurface;
    final ColorDescriptor filled = _vars.controlColorFilledSurface;
    final Color selected = _vars.colorPrimary[filled.normalShade!]!;
    return StatedWidgetProperty<Color>(
      normal: const Color(0x00000000),
      hovered: _vars.colorPrimary[plain.hoveredShade!]!.withValues(
        alpha: plain.hoveredOpacity,
      ),
      pressed: _vars.colorPrimary[plain.pressedShade!]!.withValues(
        alpha: plain.pressedOpacity,
      ),
      selected: selected,
      focused: const Color(0x00000000),
      disabled: const Color(0x00000000),
    );
  }

  /// A column of full-ink rows would leave the selected one nothing to win
  /// by, so the idle row rests in the navigation ink and only the selected
  /// one takes the accent's contrasting print.
  @override
  WidgetProperty<Color>? get menuItemForegroundColor =>
      StatedWidgetProperty<Color>(
        normal: _vars.colorContentNav,
        hovered: _vars.colorContent,
        pressed: _vars.colorContent,
        selected: _vars.colorOnAccent,
        focused: _vars.colorContentNav,
        disabled: _vars.controlColorNormalContent.disabledColor!,
      );

  @override
  Duration? get animationDuration => _vars.motionDuration;
}

extension SidebarThemeDefaultsX on SidebarThemeData {
  SidebarThemeData withDefaults(BuildContext context) {
    return merge(_SidebarDefaults(context));
  }
}
