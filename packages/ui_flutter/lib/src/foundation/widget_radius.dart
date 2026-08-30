import 'package:flutter/widgets.dart';

/// The named radius of a widget.
enum NamedRadius {
  /// The radius of the widget is none.
  none,

  /// The radius of the widget is tiny.
  tiny,

  /// The radius of the widget is small.
  small,

  /// The radius of the widget is medium.
  medium,

  /// The radius of the widget is large.
  large,

  /// The radius of the widget is big.
  big,

  /// The radius of the widget is full.
  full,
}

/// The radius of a widget.
class WidgetRadius extends BorderRadius {
  const WidgetRadius(this.namedRadius) : super.all(Radius.zero);

  /// The name of the radius.
  final NamedRadius namedRadius;

  @override
  int get hashCode => Object.hash(
    super.topLeft,
    super.topRight,
    super.bottomLeft,
    super.bottomRight,
    namedRadius,
  );

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    if (other.runtimeType != runtimeType) return false;
    return other is WidgetRadius && other.namedRadius == namedRadius;
  }

  /// The radius of the widget is none.
  static const WidgetRadius none = WidgetRadius(NamedRadius.none);

  /// The radius of the widget is tiny.
  static const WidgetRadius tiny = WidgetRadius(NamedRadius.tiny);

  /// The radius of the widget is small.
  static const WidgetRadius small = WidgetRadius(NamedRadius.small);

  /// The radius of the widget is medium.
  static const WidgetRadius medium = WidgetRadius(NamedRadius.medium);

  /// The radius of the widget is large.
  static const WidgetRadius large = WidgetRadius(NamedRadius.large);

  /// The radius of the widget is big.
  static const WidgetRadius big = WidgetRadius(NamedRadius.big);

  /// The radius of the widget is full.
  static const WidgetRadius full = WidgetRadius(NamedRadius.full);
}
