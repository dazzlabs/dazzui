import 'package:flutter/widgets.dart';

/// Validation supplied by a form field, separately from a control's tint.
class FieldValidity extends InheritedWidget {
  const FieldValidity({super.key, required this.invalid, required super.child});

  final bool invalid;

  static bool of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<FieldValidity>()?.invalid ??
      false;

  @override
  bool updateShouldNotify(FieldValidity oldWidget) =>
      invalid != oldWidget.invalid;
}
