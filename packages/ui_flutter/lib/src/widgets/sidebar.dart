import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show Tooltip;
import 'package:flutter/widgets.dart';

import '../theme/theme.dart';
import 'icon_button.dart' as ff;
import 'sidebar_theme.dart';

/// Sidebar collapsible behavior.
enum SidebarCollapsible {
  /// Sidebar does not collapse.
  none,

  /// Sidebar collapses to an icon-only rail.
  icon,
}

class SidebarProvider extends StatefulWidget {
  const SidebarProvider({
    super.key,
    required this.child,
    this.defaultOpen = true,
    this.open,
    this.onOpenChange,
  });

  final Widget child;

  /// Uncontrolled initial open state.
  final bool defaultOpen;

  /// Controlled open state. When provided, [SidebarProvider] becomes controlled.
  final bool? open;

  /// Called when open state changes.
  final ValueChanged<bool>? onOpenChange;

  static SidebarController of(BuildContext context) {
    final _SidebarProviderScope? scope =
        context.dependOnInheritedWidgetOfExactType<_SidebarProviderScope>();
    assert(scope != null, 'SidebarProvider.of() called with no SidebarProvider.');
    return scope!.controller;
  }

  @override
  State<SidebarProvider> createState() => _SidebarProviderState();
}

/// The controller exposed by [SidebarProvider].
@immutable
class SidebarController {
  const SidebarController({
    required this.open,
    required this.setOpen,
    required this.toggle,
  });

  final bool open;
  final ValueChanged<bool> setOpen;
  final VoidCallback toggle;
}

class _SidebarProviderState extends State<SidebarProvider> {
  late bool _open;

  bool get _isControlled => widget.open != null;

  bool get open => _isControlled ? widget.open! : _open;

  @override
  void initState() {
    super.initState();
    _open = widget.defaultOpen;
  }

  void _setOpen(bool value) {
    if (_isControlled) {
      widget.onOpenChange?.call(value);
      return;
    }
    if (_open == value) return;
    setState(() {
      _open = value;
    });
    widget.onOpenChange?.call(value);
  }

  void _toggle() => _setOpen(!open);

  @override
  Widget build(BuildContext context) {
    return _SidebarProviderScope(
      controller: SidebarController(
        open: open,
        setOpen: _setOpen,
        toggle: _toggle,
      ),
      child: widget.child,
    );
  }
}

class _SidebarProviderScope extends InheritedWidget {
  const _SidebarProviderScope({
    required this.controller,
    required super.child,
  });

  final SidebarController controller;

  @override
  bool updateShouldNotify(_SidebarProviderScope oldWidget) {
    return controller.open != oldWidget.controller.open;
  }
}

class Sidebar extends StatelessWidget {
  const Sidebar({
    super.key,
    required this.child,
    this.collapsible = SidebarCollapsible.icon,
  });

  final Widget child;
  final SidebarCollapsible collapsible;

  @override
  Widget build(BuildContext context) {
    final SidebarThemeData theme = SidebarTheme.of(context).withDefaults(context);

    final SidebarController? controller = context
        .dependOnInheritedWidgetOfExactType<_SidebarProviderScope>()
        ?.controller;

    final bool collapsed =
        collapsible == SidebarCollapsible.icon && (controller?.open == false);

    final double width = collapsed ? theme.collapsedWidth! : theme.width!;
    final double borderWidth = 1.0 / MediaQuery.devicePixelRatioOf(context);

    return _SidebarStateScope(
      collapsed: collapsed,
      theme: theme,
      child: AnimatedContainer(
        duration: theme.animationDuration!,
        width: width,
        decoration: BoxDecoration(
          color: theme.backgroundColor,
          border: Border(
            right: BorderSide(
              color: theme.borderColor!,
              width: borderWidth,
            ),
          ),
        ),
        child: Padding(
          padding: theme.padding!,
          child: DefaultTextStyle(
            style: DefaultTextStyle.of(context).style.copyWith(
                  color: theme.foregroundColor,
                ),
            child: IconTheme(
              data: IconTheme.of(context).copyWith(
                color: theme.foregroundColor,
              ),
              child: child,
            ),
          ),
        ),
      ),
    );
  }
}

class _SidebarStateScope extends InheritedWidget {
  const _SidebarStateScope({
    required this.collapsed,
    required this.theme,
    required super.child,
  });

  final bool collapsed;
  final SidebarThemeData theme;

  static _SidebarStateScope of(BuildContext context) {
    final _SidebarStateScope? scope =
        context.dependOnInheritedWidgetOfExactType<_SidebarStateScope>();
    assert(scope != null, 'Sidebar widgets must be used within a Sidebar.');
    return scope!;
  }

  @override
  bool updateShouldNotify(_SidebarStateScope oldWidget) {
    return collapsed != oldWidget.collapsed || theme != oldWidget.theme;
  }
}

class SidebarHeader extends StatelessWidget {
  const SidebarHeader({super.key, required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final _SidebarStateScope scope = _SidebarStateScope.of(context);
    return Padding(
      padding: scope.theme.headerPadding!,
      child: child,
    );
  }
}

class SidebarFooter extends StatelessWidget {
  const SidebarFooter({super.key, required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final _SidebarStateScope scope = _SidebarStateScope.of(context);
    return Padding(
      padding: scope.theme.footerPadding!,
      child: child,
    );
  }
}

class SidebarContent extends StatelessWidget {
  const SidebarContent({
    super.key,
    required this.child,
    this.padding,
  });

  final Widget child;
  final EdgeInsets? padding;

  @override
  Widget build(BuildContext context) {
    final EdgeInsets resolvedPadding = padding ?? EdgeInsets.zero;
    return Expanded(
      child: SingleChildScrollView(
        padding: resolvedPadding,
        child: child,
      ),
    );
  }
}

class SidebarGroup extends StatelessWidget {
  const SidebarGroup({super.key, required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: child,
    );
  }
}

class SidebarGroupLabel extends StatelessWidget {
  const SidebarGroupLabel({super.key, required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final _SidebarStateScope scope = _SidebarStateScope.of(context);
    if (scope.collapsed) return const SizedBox.shrink();

    return Padding(
      padding: const EdgeInsets.only(bottom: 6),
      child: DefaultTextStyle(
        style: scope.theme.groupLabelStyle!,
        child: child,
      ),
    );
  }
}

class SidebarGroupContent extends StatelessWidget {
  const SidebarGroupContent({super.key, required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) => child;
}

class SidebarMenu extends StatelessWidget {
  const SidebarMenu({super.key, required this.children});
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: children,
    );
  }
}

class SidebarMenuItem extends StatelessWidget {
  const SidebarMenuItem({super.key, required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) => child;
}

class SidebarMenuButton extends StatefulWidget {
  const SidebarMenuButton({
    super.key,
    required this.icon,
    required this.label,
    this.trailing,
    this.selected = false,
    this.onPressed,
    this.tooltip,
  });

  final Widget icon;
  final Widget label;
  final Widget? trailing;
  final bool selected;
  final VoidCallback? onPressed;

  /// Tooltip text shown when sidebar is collapsed (icon-only).
  final String? tooltip;

  bool get enabled => onPressed != null;

  @override
  State<SidebarMenuButton> createState() => _SidebarMenuButtonState();
}

class _SidebarMenuButtonState extends State<SidebarMenuButton> {
  late bool _isHovered;
  late bool _isFocused;
  late bool _isPressed;

  Set<WidgetState> get states => <WidgetState>{
        if (_isHovered) WidgetState.hovered,
        if (_isFocused) WidgetState.focused,
        if (_isPressed) WidgetState.pressed,
        if (widget.selected) WidgetState.selected,
        if (!widget.enabled) WidgetState.disabled,
      };

  @override
  void initState() {
    super.initState();
    _isHovered = false;
    _isFocused = false;
    _isPressed = false;
  }

  void _handleTapDown(TapDownDetails _) {
    if (!_isPressed) setState(() => _isPressed = true);
  }

  void _handleTapUp(TapUpDetails _) {
    if (_isPressed) setState(() => _isPressed = false);
  }

  void _handleTapCancel() {
    if (_isPressed) setState(() => _isPressed = false);
  }

  void _onShowFocusHighlight(bool show) => setState(() => _isFocused = show);
  void _onShowHoverHighlight(bool show) => setState(() => _isHovered = show);

  @override
  Widget build(BuildContext context) {
    final _SidebarStateScope scope = _SidebarStateScope.of(context);
    final SidebarThemeData theme = scope.theme;

    final Color bg = theme.menuItemBackgroundColor!.resolveWith(states);
    final Color fg = theme.menuItemForegroundColor!.resolveWith(states);

    final Widget content = Container(
      decoration: BoxDecoration(
        color: bg,
        borderRadius: theme.menuItemBorderRadius,
      ),
      padding: theme.menuItemPadding,
      child: scope.collapsed
          ? Center(
              child: IconTheme(
                data: IconTheme.of(context).copyWith(
                  size: theme.menuItemIconSize,
                  color: fg,
                ),
                child: widget.icon,
              ),
            )
          : Row(
              children: [
                IconTheme(
                  data: IconTheme.of(context).copyWith(
                    size: theme.menuItemIconSize,
                    color: fg,
                  ),
                  child: widget.icon,
                ),
                SizedBox(width: theme.menuItemGap),
                Expanded(
                  child: DefaultTextStyle(
                    style: DefaultTextStyle.of(context).style.copyWith(
                          color: fg,
                        ),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    child: widget.label,
                  ),
                ),
                if (widget.trailing != null) ...[
                  const SizedBox(width: 8),
                  IconTheme(
                    data: IconTheme.of(context).copyWith(color: fg),
                    child: widget.trailing!,
                  ),
                ],
              ],
            ),
    );

    return FocusableActionDetector(
      enabled: widget.enabled,
      onShowFocusHighlight: _onShowFocusHighlight,
      onShowHoverHighlight: _onShowHoverHighlight,
      mouseCursor: widget.enabled && kIsWeb
          ? SystemMouseCursors.click
          : MouseCursor.defer,
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTapDown: widget.enabled ? _handleTapDown : null,
        onTapUp: widget.enabled ? _handleTapUp : null,
        onTapCancel: widget.enabled ? _handleTapCancel : null,
        onTap: widget.onPressed,
        child: scope.collapsed && (widget.tooltip?.isNotEmpty ?? false)
            ? Tooltip(message: widget.tooltip!, child: content)
            : content,
      ),
    );
  }
}

class SidebarTrigger extends StatelessWidget {
  const SidebarTrigger({
    super.key,
    this.icon,
    this.tooltip = 'Toggle sidebar',
  });

  final IconData? icon;
  final String tooltip;

  @override
  Widget build(BuildContext context) {
    final SidebarController controller = SidebarProvider.of(context);
    final ThemeData theme = Theme.of(context);
    final IconData resolvedIcon = icon ??
        (controller.open ? theme.iconLibrary.chevronLeft : theme.iconLibrary.chevronRight);

    return Tooltip(
      message: tooltip,
      child: ff.IconButton(
        icon: resolvedIcon,
        variant: ff.IconButtonVariant.plain,
        onPressed: controller.toggle,
      ),
    );
  }
}

