import 'package:flutter/widgets.dart';

/// The presenter used by ordinary dropdowns. Search suggestions stay custom.
enum MenuPresentation { custom, native }

/// Configures Menu, Select, ContextMenu and Menubar for a subtree.
///
/// Native menus use the desktop system's appearance and behavior. Unsupported
/// platforms and unavailable native backends fall back to custom menus.
class MenuConfiguration extends InheritedTheme {
  const MenuConfiguration({
    super.key,
    required this.presentation,
    required super.child,
  });

  final MenuPresentation presentation;

  static MenuPresentation of(BuildContext context) =>
      context
          .dependOnInheritedWidgetOfExactType<MenuConfiguration>()
          ?.presentation ??
      MenuPresentation.custom;

  @override
  bool updateShouldNotify(MenuConfiguration oldWidget) =>
      presentation != oldWidget.presentation;

  @override
  Widget wrap(BuildContext context, Widget child) =>
      MenuConfiguration(presentation: presentation, child: child);
}
