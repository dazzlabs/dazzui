import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';

import '../foundation/widget_radius.dart';
import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../foundation/widget_variant.dart';
import '../generated/theme_variables.dart';
import '../painting/widget_property.dart';
import '../theme/theme.dart';
import '../widgets/button_theme.dart';

/// The tint of a button.
enum ButtonTint with WidgetTint {
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

/// The variant of a button.
enum ButtonVariant with WidgetVariant {
  /// A filled button.
  filled,

  /// A tinted button.
  tinted,

  /// An outlined button.
  outlined,

  /// A plain button.
  plain,
}

/// A button widget.
///
/// Takes in a text or an icon that fades out and in on touch. May optionally have a
/// background.
class Button extends StatefulWidget {
  /// Creates an iOS-style button.
  const Button({
    super.key,
    required this.child,
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
  ///
  /// Typically a [Text] widget.
  final Widget child;

  /// The color of the button's seed color.
  ///
  /// Defaults to null.
  final Color? color;

  /// The tint of the button.
  ///
  /// Defaults to null.
  final ButtonTint? tint;

  /// The variant of the button.
  ///
  /// Defaults to null.
  final ButtonVariant? variant;

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
  State<Button> createState() => _ButtonState();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties.add(
      FlagProperty('enabled', value: enabled, ifFalse: 'disabled'),
    );
  }
}

class _ButtonState extends State<Button> with SingleTickerProviderStateMixin {
  late bool _isHovered;
  late bool _isFocused;
  late bool _isPressed;

  Set<WidgetState> get states {
    return <WidgetState>{
      if (_isHovered) WidgetState.hovered,
      if (_isFocused) WidgetState.focused,
      if (_isPressed) WidgetState.pressed,
      if (!widget.enabled) WidgetState.disabled,
    };
  }

  @override
  void initState() {
    super.initState();
    _isHovered = false;
    _isFocused = false;
    _isPressed = false;
  }

  void _handleTapDown(TapDownDetails event) {
    if (!_isPressed) {
      _isPressed = true;
      setState(() {});
    }
  }

  void _handleTapUp(TapUpDetails event) {
    if (_isPressed) {
      _isPressed = false;
      setState(() {});
    }
  }

  void _handleTapCancel() {
    if (_isPressed) {
      _isPressed = false;
      setState(() {});
    }
  }

  void _onShowFocusHighlight(bool showHighlight) {
    setState(() {
      _isFocused = showHighlight;
    });
  }

  void _onShowHoverHighlight(bool showHighlight) {
    if (widget.onHover != null) {
      widget.onHover?.call(showHighlight);
    }
    setState(() {
      _isHovered = showHighlight;
    });
  }

  @override
  Widget build(BuildContext context) {
    final ButtonThemeData buttonTheme = ButtonTheme.of(
      context,
    ).merge(_ButtonDefaults(context));

    // Resolve the property for the given theme.
    T resolve<T>(
      WidgetProperty<T> Function(ButtonThemeData theme) getProperty,
    ) {
      Color? seedColor =
          widget.color ??
          buttonTheme.color?.tinted(widget.tint ?? ButtonTint.primary);
      return getProperty(buttonTheme).resolveWith(
        states,
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

    return FocusableActionDetector(
      enabled: widget.enabled,
      focusNode: widget.focusNode,
      autofocus: widget.autofocus,
      onFocusChange: widget.onFocusChange,
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
        child: Semantics(
          button: true,
          child: ConstrainedBox(
            constraints: BoxConstraints(
              minWidth: minSize.width,
              minHeight: minSize.height,
            ),
            child: Container(
              width: widget.expand ? double.infinity : null,
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
              padding: padding,
              child: Align(
                alignment: Alignment.center,
                widthFactor: 1.0,
                heightFactor: 1.0,
                child: DefaultTextStyle(
                  style: labelStyle.copyWith(
                    color: foregroundColor,
                  ),
                  child: IconTheme(
                    data: IconTheme.of(context).copyWith(
                      size: widget.iconSize,
                      color: foregroundColor,
                    ),
                    child: widget.child,
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _ButtonDefaults extends ButtonThemeData {
  _ButtonDefaults(this.context) : super();

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
      small: Size.square(_vars.controlSmallSize),
      medium: Size.square(_vars.controlMediumSize),
      large: Size.square(_vars.controlLargeSize),
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
        horizontal: _vars.controlSmallPaddingInline,
        vertical: _vars.controlSmallPaddingBlock,
      ),
      medium: EdgeInsets.symmetric(
        horizontal: _vars.controlMediumPaddingInline,
        vertical: _vars.controlMediumPaddingBlock,
      ),
      large: EdgeInsets.symmetric(
        horizontal: _vars.controlLargePaddingInline,
        vertical: _vars.controlLargePaddingBlock,
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
      small: BorderRadius.circular(_vars.controlSmallRadius),
      medium: BorderRadius.circular(_vars.controlMediumRadius),
      large: BorderRadius.circular(_vars.controlLargeRadius),
    );
  }

  @override
  WidgetProperty<TextStyle>? get labelStyle {
    return SizedWidgetProperty<TextStyle>(
      small: _vars.labelSmall,
      medium: _vars.labelMedium,
      large: _vars.labelLarge,
    );
  }
}
