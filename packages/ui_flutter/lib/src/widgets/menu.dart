import 'package:flutter/widgets.dart';

import '../generated/theme_variables.dart';
import '../theme/theme.dart';
import 'pressable.dart';

/// The floating panel a menu's rows sit in.
class MenuPanel extends StatelessWidget {
  const MenuPanel({super.key, required this.children});

  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return ConstrainedBox(
      constraints: BoxConstraints(minWidth: vars.menuMinWidth),
      child: Container(
        clipBehavior: Clip.antiAlias,
        padding: EdgeInsets.all(vars.spacing1),
        decoration: BoxDecoration(
          color: vars.colorSurface,
          border: Border.all(
            color: vars.colorBorderStrong,
            width: context.hairlineWidth,
          ),
          borderRadius: BorderRadius.circular(vars.radiusLarge),
          boxShadow: vars.shadowLg,
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: children,
        ),
      ),
    );
  }
}

/// One menu row.
///
/// The quiet label on a row `menu.item-padding` deep — the odd 7px that
/// squares a 12px label onto the design's menu-row height. The row's corner
/// is the panel's minus the panel's own padding, so the hover wash stays
/// concentric with the panel edge.
class MenuItem extends StatelessWidget {
  const MenuItem({
    super.key,
    required this.label,
    this.icon,
    this.shortcut,
    this.checked,
    required this.onPressed,
  });

  final String label;
  final IconData? icon;

  /// A faint hint in the display cut — never a boxed key cap; a cap in a menu
  /// row would be furniture.
  final String? shortcut;

  /// Set on a row that participates in a checked group. The column holds its
  /// width whether or not the row is checked, so the labels of a radio group
  /// stay on one left edge.
  final bool? checked;

  final VoidCallback? onPressed;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final bool enabled = onPressed != null;
    final BorderRadius radius = BorderRadius.circular(
      vars.radiusLarge - vars.spacing1,
    );

    return Pressable(
      onPressed: onPressed,
      enabled: enabled,
      checked: checked,
      borderRadius: radius,
      builder: (context, states) {
        // The `normal` recipe, not `plain`. Both have no resting fill, but
        // `plain` is the quiet *accent* control — its wash comes off the
        // tinted ramp, which on a row would land heavier than a tinted
        // selection sitting one step lower. A row's wash is neutral, and it
        // holds while pressed: a menu item has no pressed look of its own.
        final bool washed =
            enabled &&
            (states.contains(WidgetState.hovered) ||
                states.contains(WidgetState.pressed));
        final Color content = enabled
            ? vars.colorContent
            : vars.controlColorNormalContent.disabledColor!;

        return AnimatedContainer(
          duration: vars.motionDuration,
          curve: vars.motionEasing,
          padding: EdgeInsets.symmetric(
            vertical: vars.menuItemPadding,
            horizontal: vars.spacing2,
          ),
          decoration: BoxDecoration(
            color: washed ? vars.colorSurfaceSubtle : null,
            borderRadius: radius,
          ),
          child: Row(
            spacing: vars.spacing2,
            children: [
              if (checked != null)
                SizedBox(
                  width: vars.spacing3,
                  child: Opacity(
                    opacity: checked! ? 1 : 0,
                    child: Icon(
                      _kCheck,
                      size: vars.labelSmall.fontSize,
                      color: content,
                    ),
                  ),
                ),
              if (icon != null)
                Icon(
                  icon,
                  size: vars.spacing35,
                  color: enabled ? vars.colorContentMuted : content,
                ),
              Expanded(
                child: Text(
                  label,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: vars.labelQuiet.copyWith(height: 1, color: content),
                ),
              ),
              if (shortcut != null)
                Text(
                  shortcut!,
                  style: vars.labelStrong.copyWith(
                    fontSize: vars.labelSmall.fontSize,
                    fontWeight: vars.labelSmall.fontWeight,
                    height: 1,
                    color: vars.colorContentFaint,
                  ),
                ),
            ],
          ),
        );
      },
    );
  }
}

/// The check glyph, from the icon library the package already ships.
const IconData _kCheck = IconData(0xe5ca, fontFamily: 'MaterialIcons');
