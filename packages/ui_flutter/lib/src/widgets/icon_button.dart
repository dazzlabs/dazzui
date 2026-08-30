import 'package:collection/collection.dart';
import 'package:flutter/widgets.dart';

import '../foundation/widget_radius.dart';
import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../foundation/widget_variant.dart';
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
    this.size = WidgetSize.medium,
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
  @override
  Widget build(BuildContext context) {
    return Button(
      color: widget.color,
      tint: ButtonTint.values.firstWhereOrNull(
        (e) => e.namedTint == widget.tint?.namedTint,
      ),
      variant: ButtonVariant.values.firstWhereOrNull(
        (e) => e.namedVariant == widget.variant?.namedVariant,
      ),
      size: widget.size,
      radius: widget.radius,
      padding: widget.padding,
      iconSize: widget.iconSize,
      expand: widget.expand,
      focusNode: widget.focusNode,
      autofocus: widget.autofocus,
      onFocusChange: widget.onFocusChange,
      onHover: widget.onHover,
      onPressed: widget.onPressed,
      child: Icon(widget.icon),
    );
  }
}
