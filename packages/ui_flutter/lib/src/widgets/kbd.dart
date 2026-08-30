import 'package:flutter/widgets.dart';

import '../foundation/widget_size.dart';
import '../generated/theme_variables.dart';
import '../painting/widget_property.dart';
import '../theme/theme.dart';
import 'kbd_theme.dart';

/// Display keyboard button or keys combination
class Kbd extends StatefulWidget {
  const Kbd(
    this.label, {
    super.key,
    this.size = WidgetSize.medium,
  });

  final String label;

  final WidgetSize size;

  @override
  State<Kbd> createState() => _KbdState();
}

class _KbdState extends State<Kbd> {
  @override
  Widget build(BuildContext context) {
    final KbdThemeData kbdTheme = KbdTheme.of(context);
    final KbdThemeData defaults = _KbdDefaults(context);

    final padding = (kbdTheme.padding ?? defaults.padding)
        ?.sized<EdgeInsetsGeometry>(widget.size);

    // Legacy implementation, will be removed in the future
    final ThemeData themeData = Theme.of(context);

    final Color backgroundColor = themeData.vars.colorSurface;
    final Color borderColor = themeData.vars.colorBorder;
    final Color labelColor = themeData.vars.colorContent;

    final TextStyle textStyle = (themeData.vars.labelMedium).copyWith(
      color: labelColor,
      fontWeight: FontWeight.w700,
      fontFamily: 'JetBrains Mono',
      fontFamilyFallback: ['Roboto Mono', 'monospace'],
      fontSize: 12,
    );

    return Container(
      constraints: const BoxConstraints(
        minWidth: 0,
        minHeight: 22,
      ),
      decoration: BoxDecoration(
        color: backgroundColor,
        border: BorderDirectional(
          start: BorderSide(
            color: borderColor,
            width: 1,
          ),
          top: BorderSide(
            color: borderColor,
            width: 1,
          ),
          end: BorderSide(
            color: borderColor,
            width: 1,
          ),
          bottom: BorderSide(
            color: borderColor,
            width: 3,
          ),
        ),
        borderRadius: const BorderRadius.all(Radius.circular(4.0)),
      ),
      child: Padding(
        padding: padding ?? EdgeInsets.zero,
        child: DefaultTextStyle(
          style: textStyle,
          child: Text(widget.label),
        ),
      ),
    );
  }
}

class _KbdDefaults extends KbdThemeData {
  _KbdDefaults(this.context) : super();

  final BuildContext context;
  late final ThemeData _theme = Theme.of(context);
  late final ThemeVariables _vars = _theme.vars;

  @override
  SizedWidgetProperty<EdgeInsetsGeometry>? get padding {
    return SizedWidgetProperty<EdgeInsetsGeometry>(
      small: EdgeInsets.symmetric(
        horizontal: _vars.spacing2,
        vertical: _vars.spacing2,
      ),
      medium: EdgeInsets.symmetric(
        horizontal: _vars.spacing2,
        vertical: _vars.spacing2,
      ),
      large: EdgeInsets.symmetric(
        horizontal: _vars.spacing2,
        vertical: _vars.spacing2,
      ),
    );
  }
}
