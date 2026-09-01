import 'package:collection/collection.dart';
import 'package:flutter/widgets.dart';

import '../foundation/widget_radius.dart';
import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../foundation/widget_variant.dart';
import '../generated/theme_variables.dart';
import '../theme/theme.dart';
import 'button.dart';

/// The tint of a icon button.
enum IconButtonTint with WidgetTint {
  /// A button indicating a primary action.
  primary,

  /// A button indicating a neutral action.
  neutral,

  /// A button indicating an info.
  info,

  /// A button indicating a success.
  success,

  /// A button indicating a warning.
  warning,

  /// A button indicating a danger.
  danger,
}

enum IconButtonVariant with WidgetVariant {
  normal,
  recessed,
  filled,
  tinted,
  outlined,
  plain,
}

class IconButton extends StatefulWidget {
  const IconButton({
    super.key,
    required this.icon,
    this.color,
    this.tint,
    this.variant,
    this.active = false,
    this.size = WidgetSize.tiny,
    this.radius = WidgetRadius.medium,
    this.padding,
    this.iconSize,
    this.expand = false,
    this.focusNode,
    this.autofocus = false,
    this.onFocusChange,
    this.onHover,
    required this.onPressed,
  });

  /// The widget below this widget in the tree.
  final IconData icon;

  /// The color of the button's seed color.
  ///
  /// Defaults to null.
  final Color? color;

  /// The tint of the button.
  ///
  /// Defaults to null.
  final IconButtonTint? tint;

  /// The variant of the button.
  ///
  /// Defaults to null.
  final IconButtonVariant? variant;

  /// A persistent on-state — a pin held down, a panel kept open. The glyph
  /// takes the accent's text grade and the fill goes: "this is on" is a
  /// colour statement, not a chip.
  final bool active;

  /// The size of the button.
  ///
  /// Defaults to [WidgetSize.medium].
  final Size size;

  /// The radius of the button.
  ///
  /// Defaults to [WidgetRadius.medium].
  final BorderRadius radius;

  /// The padding of the button.
  ///
  /// Defaults to null.
  final EdgeInsets? padding;

  /// The icon size of the button.
  ///
  /// Defaults to null.
  final double? iconSize;

  /// Whether the button should expand to fill its container.
  ///
  /// Defaults to false.
  final bool expand;

  /// {@macro flutter.widgets.Focus.focusNode}
  final FocusNode? focusNode;

  /// {@macro flutter.widgets.Focus.autofocus}
  final bool autofocus;

  /// Handler called when the focus changes.
  ///
  /// Called with true if this widget's node gains focus, and false if it loses
  /// focus.
  final ValueChanged<bool>? onFocusChange;

  /// Handler called when the hover state changes.
  ///
  /// Called with true if this widget's node gains hover, and false if it loses
  /// hover.
  final ValueChanged<bool>? onHover;

  /// The callback that is called when the button is tapped or otherwise activated.
  ///
  /// If this is set to null, the button will be disabled.
  final VoidCallback? onPressed;

  /// Whether the button is enabled or disabled. Buttons are disabled by default. To
  /// enable a button, set its [onPressed] property to a non-null value.
  bool get enabled => onPressed != null;

  @override
  State<IconButton> createState() => _IconButtonState();
}

class _IconButtonState extends State<IconButton> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final IconButtonVariant variant = widget.variant ?? IconButtonVariant.plain;
    final IconButtonTint tint = widget.tint ?? IconButtonTint.neutral;

    // The default posture — plain variant, neutral tint — carries its own
    // quiet chrome: a subtle-ink glyph that darkens to full ink under the
    // pointer, because the recipes' neutral ramp is too flat a grey for a
    // toolbar row that has to read as chrome until touched.
    final bool quietChrome =
        variant == IconButtonVariant.plain &&
        tint == IconButtonTint.neutral &&
        widget.color == null;

    Color? color = widget.color;
    Color? surface;
    if (widget.active) {
      // "This is on" is a colour statement, not a chip: the glyph takes the
      // accent's text grade and the fill goes.
      color = vars.colorPrimary[700];
      surface = const Color(0x00000000);
    } else if (quietChrome && widget.onPressed != null) {
      color = _isHovered ? vars.colorContent : vars.colorContentSubtle;
      // A toolbar glyph washes neutral, not accent — the row has to read as
      // chrome until it is touched.
      surface = _isHovered ? vars.colorSurfaceSubtle : const Color(0x00000000);
    }

    return Button(
      color: color,
      tint: ButtonTint.values.firstWhereOrNull(
        (e) => e.namedTint == tint.namedTint,
      ),
      variant: ButtonVariant.values.firstWhereOrNull(
        (e) => e.namedVariant == variant.namedVariant,
      ),
      size: widget.size,
      square: true,
      surfaceColor: surface,
      // A toolbar glyph is smaller than the square it sits in; without this
      // it would inherit the ambient IconTheme, which is half again as large.
      iconSize: widget.iconSize ?? vars.spacing35,
      radius: widget.radius,
      padding: widget.padding,
      expand: widget.expand,
      focusNode: widget.focusNode,
      autofocus: widget.autofocus,
      onFocusChange: widget.onFocusChange,
      onHover: (hovered) {
        widget.onHover?.call(hovered);
        if (quietChrome && mounted) setState(() => _isHovered = hovered);
      },
      onPressed: widget.onPressed,
      child: Icon(widget.icon),
    );
  }
}
