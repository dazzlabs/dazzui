import 'package:flutter/widgets.dart';

import '../theme/theme.dart';

// Margin on top of the list section.
const double _kMarginTop = 22.0;

// Standard header margin.
const EdgeInsetsDirectional _kDefaultHeaderMargin = EdgeInsetsDirectional.fromSTEB(
  20.0,
  0.0,
  20.0,
  6.0,
);

// Header margin for inset grouped variant.
const EdgeInsetsDirectional _kInsetGroupedDefaultHeaderMargin = EdgeInsetsDirectional.fromSTEB(
  20.0,
  16.0,
  20.0,
  6.0,
);

// Standard footer margin.
const EdgeInsetsDirectional _kDefaultFooterMargin = EdgeInsetsDirectional.fromSTEB(
  20.0,
  0.0,
  20.0,
  0.0,
);

// Footer margin for inset grouped variant.
const EdgeInsetsDirectional _kInsetGroupedDefaultFooterMargin = EdgeInsetsDirectional.fromSTEB(
  20.0,
  0.0,
  20.0,
  10.0,
);

// Margin around children in edge-to-edge variant.
const EdgeInsets _kDefaultRowsMargin = EdgeInsets.only(bottom: 8.0);

// Used for "Inset Grouped" margin.
const EdgeInsetsDirectional _kDefaultInsetGroupedRowsMargin = EdgeInsetsDirectional.fromSTEB(
  20.0,
  20.0,
  20.0,
  10.0,
);

// Used for "Inset Grouped" margin with header.
const EdgeInsetsDirectional _kDefaultInsetGroupedRowsMarginWithHeader =
    EdgeInsetsDirectional.fromSTEB(20.0, 0.0, 20.0, 10.0);

// Used for "Inset Grouped" border radius.
const BorderRadius _kDefaultInsetGroupedBorderRadius = BorderRadius.all(Radius.circular(10.0));

// The margin of divider used in base list section.
const double _kBaseDividerMargin = 20.0;

// Additional margin of divider used in base list section with list tiles with
// leading widgets.
const double _kBaseAdditionalDividerMargin = 44.0;

// The margin of divider used in inset grouped version of list section.
const double _kInsetDividerMargin = 14.0;

// Additional margin of divider used in inset grouped version of list section.
const double _kInsetAdditionalDividerMargin = 42.0;

// Additional margin of divider used in inset grouped version of list section
// when there is no leading widgets.
const double _kInsetAdditionalDividerMarginWithoutLeading = 14.0;

/// Denotes what type of the list section a [ListSection] is.
///
/// This is for internal use only.
enum ListSectionType {
  /// A basic form of [ListSection].
  base,

  /// An inset-grouped style of [ListSection].
  insetGrouped,
}

/// A list section widget.
///
/// The [ListSection] is a container for children widgets. These are
/// most often list tile widgets.
///
/// The base constructor for [ListSection] constructs an
/// edge-to-edge style section which includes a header, the dividers
/// between rows, and borders on top and bottom of the rows.
///
/// The [ListSection.insetGrouped] constructor creates a round-edged
/// and padded section. It creates a header, and the dividers between rows.
/// Does not create borders on top and bottom of the rows.
///
/// The section [header] lies above the [children] rows, with margins and style
/// that match the design system.
///
/// The section [footer] lies below the [children] rows and is used to provide
/// additional information for current list section.
///
/// The [children] is the list of widgets to be displayed in this list section.
///
/// The [margin] is used to provide spacing around the content area of the
/// section encapsulating [children].
///
/// The [decoration] of [children] specifies how they should be decorated. If it
/// is not provided in constructor, the background color of [children] defaults
/// to the surface color and border radius of children group defaults to 10.0
/// circular radius when constructing with [ListSection.insetGrouped]. Defaults
/// to zero radius for the standard [ListSection] constructor.
///
/// The [dividerMargin] and [additionalDividerMargin] specify the starting
/// margin of the divider between list tiles. The [dividerMargin] is always
/// present, but [additionalDividerMargin] is only added to the [dividerMargin]
/// if `hasLeading` is set to true in the constructor, which is the default
/// value.
///
/// The [backgroundColor] of the section defaults to the background color from theme.
///
/// {@macro flutter.material.Material.clipBehavior}
class ListSection extends StatelessWidget {
  /// Creates a list section.
  ///
  /// The base constructor for [ListSection] constructs an
  /// edge-to-edge style section which includes a header, the dividers
  /// between rows, and borders on top and bottom of the rows.
  ///
  /// The [header] parameter sets the form section header. The section header
  /// lies above the [children] rows, with margins that match the design system.
  ///
  /// The [footer] parameter sets the form section footer. The section footer
  /// lies below the [children] rows.
  ///
  /// The [children] parameter is required and sets the list of rows shown in
  /// the section. The [children] parameter takes a list, as opposed to a more
  /// efficient builder function that lazy builds, because sections are intended to
  /// be short in row count.
  ///
  /// The [margin] parameter sets the spacing around the content area of the
  /// section encapsulating [children], and defaults to zero padding.
  ///
  /// The [decoration] parameter sets the decoration around [children].
  /// If null, defaults to the surface color from theme.
  ///
  /// The [backgroundColor] parameter sets the background color behind the
  /// section. If null, defaults to the background color from theme.
  ///
  /// The [dividerMargin] parameter sets the starting offset of the divider
  /// between rows.
  ///
  /// The [additionalDividerMargin] parameter adds additional margin to existing
  /// [dividerMargin] when [hasLeading] is set to true. By default, it offsets
  /// for the width of leading and space between leading and title, but it can
  /// be overwritten for custom look.
  ///
  /// The [hasLeading] parameter specifies whether children widgets contain
  /// leading or not. Used for calculating correct starting margin for the
  /// divider between rows.
  ///
  /// The [topMargin] is used to specify the margin above the list section. It
  /// matches the design system by default.
  ///
  /// {@macro flutter.material.Material.clipBehavior}
  const ListSection({
    super.key,
    this.children,
    this.header,
    this.footer,
    this.margin = _kDefaultRowsMargin,
    this.backgroundColor,
    this.decoration,
    this.clipBehavior = Clip.none,
    this.dividerMargin = _kBaseDividerMargin,
    double? additionalDividerMargin,
    this.topMargin = _kMarginTop,
    bool hasLeading = true,
    this.separatorColor,
  })  : type = ListSectionType.base,
        additionalDividerMargin =
            additionalDividerMargin ?? (hasLeading ? _kBaseAdditionalDividerMargin : 0.0);

  /// Creates an inset grouped list section.
  ///
  /// The [ListSection.insetGrouped] constructor creates a round-edged
  /// and padded section. It creates a header, and the dividers between rows.
  /// Does not create borders on top and bottom of the rows.
  ///
  /// The [header] parameter sets the form section header. The section header
  /// lies above the [children] rows, with margins that match the design system.
  ///
  /// The [footer] parameter sets the form section footer. The section footer
  /// lies below the [children] rows.
  ///
  /// The [children] parameter is required and sets the list of rows shown in
  /// the section. The [children] parameter takes a list, as opposed to a more
  /// efficient builder function that lazy builds, because sections are intended to
  /// be short in row count.
  ///
  /// The [margin] parameter sets the spacing around the content area of the
  /// section encapsulating [children], and defaults to the standard
  /// notched-style padding.
  ///
  /// The [decoration] parameter sets the decoration around [children].
  /// If null, defaults to the surface color from theme with 10.0 circular radius.
  ///
  /// The [backgroundColor] parameter sets the background color behind the
  /// section. If null, defaults to the background color from theme.
  ///
  /// The [dividerMargin] parameter sets the starting offset of the divider
  /// between rows.
  ///
  /// The [additionalDividerMargin] parameter adds additional margin to existing
  /// [dividerMargin] when [hasLeading] is set to true. By default, it offsets
  /// for the width of leading and space between leading and title, but it can
  /// be overwritten for custom look.
  ///
  /// The [hasLeading] parameter specifies whether children widgets contain
  /// leading or not. Used for calculating correct starting margin for the
  /// divider between rows.
  ///
  /// {@macro flutter.material.Material.clipBehavior}
  const ListSection.insetGrouped({
    super.key,
    this.children,
    this.header,
    this.footer,
    EdgeInsetsGeometry? margin,
    this.backgroundColor,
    this.decoration,
    this.clipBehavior = Clip.hardEdge,
    this.dividerMargin = _kInsetDividerMargin,
    double? additionalDividerMargin,
    this.topMargin,
    bool hasLeading = true,
    this.separatorColor,
  })  : type = ListSectionType.insetGrouped,
        additionalDividerMargin = additionalDividerMargin ??
            (hasLeading
                ? _kInsetAdditionalDividerMargin
                : _kInsetAdditionalDividerMarginWithoutLeading),
        margin = margin ??
            (header == null
                ? _kDefaultInsetGroupedRowsMargin
                : _kDefaultInsetGroupedRowsMarginWithHeader);

  /// The type of list section, either base or inset grouped.
  ///
  /// This member is public for testing purposes only and cannot be set
  /// manually. Instead, use a corresponding constructors.
  @visibleForTesting
  final ListSectionType type;

  /// Sets the form section header. The section header lies above the [children]
  /// rows. Usually a [Text] widget.
  final Widget? header;

  /// Sets the form section footer. The section footer lies below the [children]
  /// rows. Usually a [Text] widget.
  final Widget? footer;

  /// Margin around the content area of the section encapsulating [children].
  ///
  /// Defaults to zero padding if constructed with standard
  /// [ListSection] constructor. Defaults to the standard notched-style
  /// margin when constructing with [ListSection.insetGrouped].
  final EdgeInsetsGeometry margin;

  /// The list of rows in the section.
  ///
  /// This takes a list, as opposed to a more efficient builder function that
  /// lazy builds, because such lists are intended to be short in row count.
  final List<Widget>? children;

  /// Sets the decoration around [children].
  ///
  /// If null, background color defaults to the surface color from theme.
  ///
  /// If null, border radius defaults to 10.0 circular radius when constructing
  /// with [ListSection.insetGrouped]. Defaults to zero radius for the
  /// standard [ListSection] constructor.
  final BoxDecoration? decoration;

  /// Sets the background color behind the section.
  ///
  /// Defaults to the background color from theme.
  final Color? backgroundColor;

  /// {@macro flutter.material.Material.clipBehavior}
  ///
  /// Defaults to [Clip.hardEdge].
  final Clip clipBehavior;

  /// The starting offset of a margin between two list tiles.
  final double dividerMargin;

  /// Additional starting inset of the divider used between rows. This is used
  /// when adding a leading icon to children and a divider should start at the
  /// text inset instead of the icon.
  final double additionalDividerMargin;

  /// Margin above the list section. Only used in edge-to-edge variant and it
  /// matches the design system by default.
  final double? topMargin;

  /// Sets the color for the dividers between rows, and borders on top and
  /// bottom of the rows.
  ///
  /// If null, defaults to the border color from theme.
  final Color? separatorColor;

  @override
  Widget build(BuildContext context) {
    assert(
      (children != null && children!.isNotEmpty) || header != null,
      'ListSection must have either children or a header.',
    );
    
    final ThemeData theme = Theme.of(context);
    final Color dividerColor = separatorColor ?? theme.vars.colorBorder;
    final double dividerHeight = 1.0 / MediaQuery.devicePixelRatioOf(context);

    // Long divider is used for wrapping the top and bottom of rows.
    // Only used in ListSectionType.base mode.
    final Widget longDivider = Container(color: dividerColor, height: dividerHeight);

    // Short divider is used between rows.
    final Widget shortDivider = Container(
      margin: EdgeInsetsDirectional.only(start: dividerMargin + additionalDividerMargin),
      color: dividerColor,
      height: dividerHeight,
    );

    TextStyle style = DefaultTextStyle.of(context).style;

    Widget? headerWidget, footerWidget;
    switch (type) {
      case ListSectionType.base:
        style = style.merge(
          TextStyle(
            fontSize: 13.0,
            color: theme.vars.colorContentMuted,
          ),
        );
        if (header != null) {
          headerWidget = DefaultTextStyle(style: style, child: header!);
        }
        if (footer != null) {
          footerWidget = DefaultTextStyle(style: style, child: footer!);
        }
      case ListSectionType.insetGrouped:
        if (header != null) {
          headerWidget = DefaultTextStyle(
            style: style.merge(const TextStyle(fontSize: 20.0, fontWeight: FontWeight.bold)),
            child: header!,
          );
        }
        if (footer != null) {
          footerWidget = DefaultTextStyle(style: style, child: footer!);
        }
    }

    Widget? decoratedChildrenGroup;
    if (children != null && children!.isNotEmpty) {
      // We construct childrenWithDividers as follows:
      // Insert a short divider between all rows.
      // If it is a `ListSectionType.base` type, add a long divider
      // to the top and bottom of the rows.
      final childrenWithDividers = <Widget>[];

      if (type == ListSectionType.base) {
        childrenWithDividers.add(longDivider);
      }

      children!.sublist(0, children!.length - 1).forEach((Widget widget) {
        childrenWithDividers.add(widget);
        childrenWithDividers.add(shortDivider);
      });

      childrenWithDividers.add(children!.last);
      if (type == ListSectionType.base) {
        childrenWithDividers.add(longDivider);
      }

      final BorderRadius childrenGroupBorderRadius = switch (type) {
        ListSectionType.insetGrouped => _kDefaultInsetGroupedBorderRadius,
        ListSectionType.base => BorderRadius.zero,
      };

      decoratedChildrenGroup = DecoratedBox(
        decoration: decoration ??
            BoxDecoration(
              color: theme.vars.colorSurface,
              borderRadius: childrenGroupBorderRadius,
            ),
        child: Column(children: childrenWithDividers),
      );

      decoratedChildrenGroup = Padding(
        padding: margin,
        child: clipBehavior == Clip.none
            ? decoratedChildrenGroup
            : ClipRRect(
                borderRadius: childrenGroupBorderRadius,
                clipBehavior: clipBehavior,
                child: decoratedChildrenGroup,
              ),
      );
    }

    return DecoratedBox(
      decoration: BoxDecoration(color: backgroundColor ?? theme.vars.colorSurfaceMuted),
      child: Column(
        children: <Widget>[
          if (type == ListSectionType.base) SizedBox(height: topMargin),
          if (headerWidget != null)
            Align(
              alignment: AlignmentDirectional.centerStart,
              child: Padding(
                padding: type == ListSectionType.base
                    ? _kDefaultHeaderMargin
                    : _kInsetGroupedDefaultHeaderMargin,
                child: headerWidget,
              ),
            ),
          ?decoratedChildrenGroup,
          if (footerWidget != null)
            Align(
              alignment: AlignmentDirectional.centerStart,
              child: Padding(
                padding: type == ListSectionType.base
                    ? _kDefaultFooterMargin
                    : _kInsetGroupedDefaultFooterMargin,
                child: footerWidget,
              ),
            ),
        ],
      ),
    );
  }
}

