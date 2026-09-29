import 'package:dazzui/dazzui.dart';

import 'package:dazzui_example/lib/scene.dart';
import 'layout.dart';

/*
 * The four stage components a story arranges with — Row, Col, Frame, Text —
 * read the way `example/src/components/stage.tsx` draws them on the web.
 */

/// A spacing step as the web writes it (`'3'`, `'0-5'`), in this theme.
double spacingOf(ThemeVariables vars, String? step) => switch (step) {
  null => vars.spacing3,
  '0' => vars.spacing0,
  'px' => vars.spacingPx,
  '0-5' => vars.spacing05,
  '1' => vars.spacing1,
  '1-5' => vars.spacing15,
  '2' => vars.spacing2,
  '2-5' => vars.spacing25,
  '3' => vars.spacing3,
  '3-5' => vars.spacing35,
  '4' => vars.spacing4,
  '5' => vars.spacing5,
  '6' => vars.spacing6,
  '7' => vars.spacing7,
  '8' => vars.spacing8,
  '9' => vars.spacing9,
  '10' => vars.spacing10,
  '11' => vars.spacing11,
  '12' => vars.spacing12,
  '14' => vars.spacing14,
  '16' => vars.spacing16,
  '20' => vars.spacing20,
  _ => throw FlutterError('No spacing step $step'),
};

/// A length as the web writes it: a number of pixels, `rem` against the
/// 16px root every port shares, or a percentage of the room there is.
sealed class Length {
  const Length();

  static Length? parse(Object? value) {
    if (value == null) return null;
    if (value is num) return Px(value.toDouble());
    final String text = '$value';
    if (text.endsWith('rem')) {
      return Px(double.parse(text.substring(0, text.length - 3)) * 16);
    }
    if (text.endsWith('px')) {
      return Px(double.parse(text.substring(0, text.length - 2)));
    }
    if (text.endsWith('%')) {
      return Fraction(double.parse(text.substring(0, text.length - 1)) / 100);
    }
    return Px(double.parse(text));
  }
}

class Px extends Length {
  const Px(this.value);
  final double value;
}

class Fraction extends Length {
  const Fraction(this.value);
  final double value;
}

/// [child] at [width], and never past the room it is given — the web's
/// `width` with `max-width: 100%`.
Widget sized(Widget child, {Length? width, Length? height}) {
  if (width == null && height == null) return child;
  return LayoutBuilder(
    builder: (context, constraints) {
      double? resolve(Length? length, double max) => switch (length) {
        null => null,
        Px(:final value) => max.isFinite ? value.clamp(0, max) : value,
        Fraction(:final value) => max.isFinite ? max * value : null,
      };
      return SizedBox(
        width: resolve(width, constraints.maxWidth),
        height: resolve(height, constraints.maxHeight),
        child: child,
      );
    },
  );
}

CrossAxisAlignment _cross(String? align, CrossAxisAlignment fallback) =>
    switch (align) {
      'start' => CrossAxisAlignment.start,
      'center' => CrossAxisAlignment.center,
      'end' => CrossAxisAlignment.end,
      'stretch' => CrossAxisAlignment.stretch,
      _ => fallback,
    };

Widget stageRow(Node node) {
  final ThemeVariables vars = Theme.of(node.context).vars;
  final double gap = spacingOf(vars, node.string('gap'));
  final String? align = node.string('align');
  final List<Widget> kids = node.kids;
  final Widget row;
  if (node.string('justify') == 'between') {
    // Each may shrink, as a flex item may: a label wraps rather than
    // pushing its control out of the row.
    row = Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      crossAxisAlignment: _cross(align, CrossAxisAlignment.center),
      spacing: gap,
      children: [for (final kid in kids) Flexible(child: kid)],
    );
  } else if (align == 'stretch') {
    row = StretchRow(gap: gap, children: kids);
  } else {
    row = Wrap(
      spacing: gap,
      runSpacing: gap,
      crossAxisAlignment: switch (align) {
        'start' => WrapCrossAlignment.start,
        'end' => WrapCrossAlignment.end,
        _ => WrapCrossAlignment.center,
      },
      children: kids,
    );
  }
  return sized(row, width: Length.parse(node['width']));
}

Widget stageCol(Node node) {
  final ThemeVariables vars = Theme.of(node.context).vars;
  final Length? width = Length.parse(node['width']);
  final CrossAxisAlignment cross = _cross(
    node.string('align'),
    CrossAxisAlignment.stretch,
  );
  final double gap = spacingOf(vars, node.string('gap'));
  // Without a width a stretched column is as wide as its widest child, the
  // way a shrink-to-fit flex box is on the web.
  if (width == null && cross == CrossAxisAlignment.stretch) {
    return StretchColumn(gap: gap, children: node.kids);
  }
  return sized(
    Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: cross,
      spacing: gap,
      children: node.kids,
    ),
    width: width,
  );
}

Widget stageFrame(Node node) {
  final ThemeVariables vars = Theme.of(node.context).vars;
  final BorderRadius radius = BorderRadius.circular(vars.radiusLarge);
  return sized(
    DecoratedBox(
      position: DecorationPosition.foreground,
      decoration: BoxDecoration(
        border: Border.all(
          color: vars.colorBorder,
          width: node.context.hairlineWidth,
        ),
        borderRadius: radius,
      ),
      child: ClipRRect(
        borderRadius: radius,
        child: ColoredBox(
          color: vars.colorSurface,
          child: node.kids.single,
        ),
      ),
    ),
    width: Length.parse(node['width'] ?? '36rem'),
    height: Length.parse(node['height'] ?? '16rem'),
  );
}

/// A run of text that may hold components inline, as a paragraph would.
Widget stageText(Node node) {
  final ThemeVariables vars = Theme.of(node.context).vars;
  final String? tint = node.string('tint');
  final Color? color = tint != null
      ? tintRamp(vars, tint)[500]
      : node.string('tone') == 'secondary'
      ? vars.colorContentSecondary
      : null;
  final TextStyle base = DefaultTextStyle.of(node.context).style;
  final TextStyle style = base.copyWith(
    color: color,
    fontWeight: node.flag('strong') ? vars.labelStrong.fontWeight : null,
  );
  final List<InlineSpan> spans = [
    for (final child in node.children)
      if (child is String)
        TextSpan(text: child)
      else if (isLive(child))
        TextSpan(text: '${resolve(node.context, child) ?? ''}')
      else
        WidgetSpan(
          alignment: PlaceholderAlignment.middle,
          child: buildNode(node.context, child),
        ),
  ];
  return IconTheme.merge(
    data: IconThemeData(color: color ?? style.color),
    child: Text.rich(TextSpan(children: spans), style: style),
  );
}

/// A tint's ramp in this theme.
ColorSwatch<int> tintRamp(ThemeVariables vars, String tint) => switch (tint) {
  'primary' => vars.colorPrimary,
  'neutral' => vars.colorNeutral,
  'info' => vars.colorInfo,
  'success' => vars.colorSuccess,
  'warning' => vars.colorWarning,
  'danger' => vars.colorDanger,
  _ => throw FlutterError('No tint $tint'),
};
