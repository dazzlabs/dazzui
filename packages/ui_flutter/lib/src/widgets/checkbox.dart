import 'package:flutter/cupertino.dart' show CupertinoColors;
import 'package:flutter/widgets.dart';

import '../foundation/widget_radius.dart';
import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../foundation/widget_variant.dart';
import '../generated/colors.dart';
import '../generated/theme_variables.dart';
import '../painting/widget_property.dart';
import '../theme/theme.dart';
import '../widgets/checkbox_theme.dart';

/// The tint of a checkbox.
enum CheckboxTint with WidgetTint {
  /// A checkbox indicating a primary.
  primary,

  /// A checkbox tinted with the neutral ramp.
  neutral,

  /// A checkbox indicating an info.
  info,

  /// A checkbox indicating a success.
  success,

  /// A checkbox indicating a warning.
  warning,

  /// A checkbox indicating a danger.
  danger,
}

/// The variant of a checkbox.
enum CheckboxVariant with WidgetVariant {
  /// A filled checkbox.
  filled,

  /// A tinted checkbox.
  tinted,

  /// An outlined checkbox.
  outlined,

  /// A muted checkbox.
  muted,

  /// A cleared checkbox.
  cleared,
}

// The relative values needed to transform a color to it's equivalent focus
// outline color.
const double _kCupertinoFocusColorOpacity = 0.80;
const double _kCupertinoFocusColorBrightness = 0.69;
const double _kCupertinoFocusColorSaturation = 0.835;

/// A macOS style checkbox.
///
/// The checkbox itself does not maintain any state. Instead, when the state of
/// the checkbox changes, the widget calls the [onChanged] callback. Most
/// widgets that use a checkbox will listen for the [onChanged] callback and
/// rebuild the checkbox with a new [value] to update the visual appearance of
/// the checkbox.
///
/// The checkbox can optionally display three values - true, false, and null -
/// if [tristate] is true. When [value] is null a dash is displayed. By default
/// [tristate] is false and the checkbox's [value] must be true or false.
///
/// In the Apple Human Interface Guidelines (HIG), checkboxes are encouraged for
/// use on macOS, but is silent about their use on iOS. If a multi-selection
/// component is needed on iOS, the HIG encourages the developer to use switches
/// ([Switch] in Flutter) instead, or to find a creative custom
/// solution.
///
/// See also:
///
///  * [Checkbox], the Material Design equivalent.
///  * [Switch], a widget with semantics similar to [Checkbox].
///  * [Slider], for selecting a value in a range.
///  * <https://developer.apple.com/design/human-interface-guidelines/components/selection-and-input/toggles/>
class Checkbox extends StatefulWidget {
  /// Creates a macOS-styled checkbox.
  ///
  /// The checkbox itself does not maintain any state. Instead, when the state of
  /// the checkbox changes, the widget calls the [onChanged] callback. Most
  /// widgets that use a checkbox will listen for the [onChanged] callback and
  /// rebuild the checkbox with a new [value] to update the visual appearance of
  /// the checkbox.
  ///
  /// The following arguments are required:
  ///
  /// * [value], which determines whether the checkbox is checked. The [value]
  ///   can only be null if [tristate] is true.
  /// * [onChanged], which is called when the value of the checkbox should
  ///   change. It can be set to null to disable the checkbox.
  const Checkbox({
    super.key,
    required this.value,
    this.tristate = false,
    required this.onChanged,
    this.color,
    this.size = WidgetSize.medium,
    this.radius = WidgetRadius.small,
    this.interactiveSize,
    this.focusNode,
    this.autofocus = false,
  }) : assert(tristate || value != null);

  /// Whether this checkbox is checked.
  ///
  /// When [tristate] is true, a value of null corresponds to the mixed state.
  /// When [tristate] is false, this value must not be null. This is asserted in
  /// debug mode.
  final bool? value;

  /// Called when the value of the checkbox should change.
  ///
  /// The checkbox passes the new value to the callback but does not actually
  /// change state until the parent widget rebuilds the checkbox with the new
  /// value.
  ///
  /// If this callback is null, the checkbox will be displayed as disabled
  /// and will not respond to input gestures.
  ///
  /// When the checkbox is tapped, if [tristate] is false (the default) then
  /// the [onChanged] callback will be applied to `!value`. If [tristate] is
  /// true this callback cycle from false to true to null and back to false
  /// again.
  ///
  /// The callback provided to [onChanged] should update the state of the parent
  /// [StatefulWidget] using the [State.setState] method, so that the parent
  /// gets rebuilt; for example:
  ///
  /// ```dart
  /// Checkbox(
  ///   value: _throwShotAway,
  ///   onChanged: (bool? newValue) {
  ///     setState(() {
  ///       _throwShotAway = newValue!;
  ///     });
  ///   },
  /// )
  /// ```
  final ValueChanged<bool?>? onChanged;

  /// If true, the checkbox's [value] can be true, false, or null.
  ///
  /// [Checkbox] displays a dash when its value is null.
  ///
  /// When a tri-state checkbox ([tristate] is true) is tapped, its [onChanged]
  /// callback will be applied to true if the current value is false, to null if
  /// value is true, and to false if value is null (i.e. it cycles through false
  /// => true => null => false when tapped).
  ///
  /// If tristate is false (the default), [value] must not be null, and
  /// [onChanged] will only toggle between true and false.
  final bool tristate;

  /// The color of the checkbox's seed color.
  ///
  /// Defaults to null.
  final Color? color;

  /// The size of the checkbox.
  ///
  /// Defaults to [WidgetSize.medium].
  final Size size;

  /// The radius of the checkbox.
  ///
  /// Defaults to [WidgetRadius.medium].
  final BorderRadius radius;

  /// The size of the checkbox when it is interactive.
  ///
  /// Defaults to null.
  final Size? interactiveSize;

  /// {@macro flutter.widgets.Focus.focusNode}
  final FocusNode? focusNode;

  /// {@macro flutter.widgets.Focus.autofocus}
  final bool autofocus;

  @override
  State<Checkbox> createState() => _CheckboxState();
}

class _CheckboxState extends State<Checkbox>
    with TickerProviderStateMixin, ToggleableStateMixin {
  final _CheckboxPainter _painter = _CheckboxPainter();
  bool? _previousValue;

  bool focused = false;

  @override
  void initState() {
    super.initState();
    _previousValue = widget.value;
  }

  @override
  void didUpdateWidget(Checkbox oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.value != widget.value) {
      _previousValue = oldWidget.value;
    }
  }

  @override
  void dispose() {
    _painter.dispose();
    super.dispose();
  }

  @override
  ValueChanged<bool?>? get onChanged => widget.onChanged;

  @override
  bool get tristate => widget.tristate;

  @override
  bool? get value => widget.value;

  void onFocusChange(bool value) {
    if (focused != value) {
      focused = value;
    }
  }

  @override
  Widget build(BuildContext context) {
    final themeDefaults = _CheckboxDefaults(context);
    final checkboxTheme = CheckboxTheme.of(context).merge(themeDefaults);

    final ThemeData theme = Theme.of(context);

    final Color effectiveActiveColor = theme.vars.colorPrimary;
    const Color effectiveInactiveColor = CupertinoColors.inactiveGray;

    final Color effectiveFocusOverlayColor =
        HSLColor.fromColor(
              effectiveActiveColor.withValues(
                alpha: _kCupertinoFocusColorOpacity,
              ),
            )
            .withLightness(_kCupertinoFocusColorBrightness)
            .withSaturation(_kCupertinoFocusColorSaturation)
            .toColor();

    final Size interactiveSize =
        widget.interactiveSize ??
        checkboxTheme.interactiveSize!.sized(widget.size);
    final Size size = checkboxTheme.size!.sized(widget.size);
    final BorderRadius borderRadius = checkboxTheme.borderRadius!.rounded(
      widget.radius,
    );
    final Color borderColor = checkboxTheme.borderColor!.tinted(
      CheckboxTint.primary,
    );

    return Semantics(
      checked: widget.value ?? false,
      mixed: widget.tristate ? widget.value == null : null,
      child: buildToggleable(
        focusNode: widget.focusNode,
        autofocus: widget.autofocus,
        onFocusChange: onFocusChange,
        size: interactiveSize,
        painter: _painter
          ..dimension = size.width
          ..focusColor = effectiveFocusOverlayColor
          ..isFocused = focused
          ..downPosition = downPosition
          ..activeColor = effectiveActiveColor
          ..inactiveColor = effectiveInactiveColor
          ..checkColor = Colors.white
          ..value = value
          ..previousValue = _previousValue
          ..isActive = widget.onChanged != null
          ..shape = RoundedRectangleBorder(
            borderRadius: borderRadius,
          )
          ..side = BorderSide(color: borderColor),
      ),
    );
  }
}

class _CheckboxPainter extends ToggleablePainter {
  double _dimension = 16.0;

  set dimension(double value) {
    if (_dimension == value) {
      return;
    }
    _dimension = value;
    notifyListeners();
  }

  Color get checkColor => _checkColor!;
  Color? _checkColor;
  set checkColor(Color value) {
    if (_checkColor == value) {
      return;
    }
    _checkColor = value;
    notifyListeners();
  }

  bool? get value => _value;
  bool? _value;
  set value(bool? value) {
    if (_value == value) {
      return;
    }
    _value = value;
    notifyListeners();
  }

  bool? get previousValue => _previousValue;
  bool? _previousValue;
  set previousValue(bool? value) {
    if (_previousValue == value) {
      return;
    }
    _previousValue = value;
    notifyListeners();
  }

  OutlinedBorder get shape => _shape!;
  OutlinedBorder? _shape;
  set shape(OutlinedBorder value) {
    if (_shape == value) {
      return;
    }
    _shape = value;
    notifyListeners();
  }

  BorderSide? get side => _side;
  BorderSide? _side;
  set side(BorderSide? value) {
    if (_side == value) {
      return;
    }
    _side = value;
    notifyListeners();
  }

  Rect _outerRectAt(Offset origin) {
    final Rect rect = Rect.fromLTWH(
      origin.dx,
      origin.dy,
      _dimension,
      _dimension,
    );
    return rect;
  }

  // The checkbox's border color if value == false, or its fill color when
  // value == true or null.
  Color _colorAt(bool value) {
    return value && isActive ? activeColor : inactiveColor;
  }

  // White stroke used to paint the check and dash.
  Paint _createStrokePaint() {
    return Paint()
      ..color = checkColor
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2.4
      ..strokeCap = StrokeCap.round;
  }

  void _drawBox(
    Canvas canvas,
    Rect outer,
    Paint paint,
    BorderSide? side,
    bool fill,
  ) {
    if (fill) {
      canvas.drawPath(shape.getOuterPath(outer), paint);
    }
    if (side != null) {
      shape.copyWith(side: side).paint(canvas, outer);
    }
  }

  void _drawCheck(Canvas canvas, Offset origin, Paint paint) {
    final Path path = Path();
    // The ratios for the offsets below were found from looking at the checkbox
    // examples on in the HIG docs. The distance from the needed point to the
    // edge was measured, then divided by the total width.
    final Offset start = Offset(_dimension * 0.28, _dimension * 0.5);
    final Offset mid = Offset(_dimension * 0.43, _dimension * 0.65);
    final Offset end = Offset(_dimension * 0.72, _dimension * 0.35);
    path.moveTo(origin.dx + start.dx, origin.dy + start.dy);
    path.lineTo(origin.dx + mid.dx, origin.dy + mid.dy);
    canvas.drawPath(path, paint);
    path.moveTo(origin.dx + mid.dx, origin.dy + mid.dy);
    path.lineTo(origin.dx + end.dx, origin.dy + end.dy);
    canvas.drawPath(path, paint);
  }

  void _drawDash(Canvas canvas, Offset origin, Paint paint) {
    // From measuring the checkbox example in the HIG docs, the dash was found
    // to be half the total width, centered in the middle.
    final Offset start = Offset(_dimension * 0.28, _dimension * 0.5);
    final Offset end = Offset(_dimension * 0.72, _dimension * 0.5);
    canvas.drawLine(origin + start, origin + end, paint);
  }

  @override
  void paint(Canvas canvas, Size size) {
    final Paint strokePaint = _createStrokePaint();
    final Offset origin = size / 2.0 - Size.square(_dimension) / 2.0 as Offset;

    final Rect outer = _outerRectAt(origin);
    final Paint paint = Paint()..color = _colorAt(value ?? true);

    if (value == false) {
      final BorderSide border = side ?? BorderSide(color: paint.color);
      _drawBox(canvas, outer, paint, border, false);
    } else {
      _drawBox(canvas, outer, paint, side, true);
      if (value ?? false) {
        _drawCheck(canvas, origin, strokePaint);
      } else {
        _drawDash(canvas, origin, strokePaint);
      }
    }

    if (isFocused) {
      final Rect focusOuter = outer.inflate(1);

      final Paint borderPaint = Paint()
        ..color = focusColor
        ..style = PaintingStyle.stroke
        ..strokeWidth = 3.5;

      _drawBox(canvas, focusOuter, borderPaint, side, true);
    }
  }
}

class _CheckboxDefaults extends CheckboxThemeData {
  _CheckboxDefaults(this.context) : super();

  final BuildContext context;
  late final ThemeData _theme = Theme.of(context);
  late final ThemeVariables _vars = _theme.vars;

  @override
  WidgetProperty<ColorSwatch<int>>? get color {
    return _vars.controlColor;
  }

  @override
  WidgetProperty<Size>? get interactiveSize {
    return SizedWidgetProperty<Size>(
      small: Size.square(_vars.controlSmallSize),
      medium: Size.square(_vars.controlMediumSize),
      large: Size.square(_vars.controlLargeSize),
    );
  }

  @override
  WidgetProperty<Size>? get size {
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
  WidgetProperty<Color>? get borderColor {
    return WidgetPropertyAll<Color>(_vars.colorBorder);
  }

  @override
  WidgetProperty<BorderRadius>? get borderRadius {
    return RoundedWidgetProperty<BorderRadius>(
      none: BorderRadius.circular(_vars.radiusNone),
      tiny: BorderRadius.circular(_vars.radiusTiny),
      small: BorderRadius.circular(_vars.radiusSmall),
      medium: BorderRadius.circular(_vars.radiusMedium),
      large: BorderRadius.circular(_vars.radiusLarge),
      big: BorderRadius.circular(_vars.radiusBig),
      full: BorderRadius.circular(_vars.radiusFull),
    );
  }
}
