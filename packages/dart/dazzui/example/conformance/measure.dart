import 'dart:math' as math;

import 'package:dazzui/dazzui.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/rendering.dart';

/*
 * A specimen's readings, in the conformance vocabulary rather than
 * Flutter's: a `BorderRadius` is `corner`, a `BoxDecoration.color` is
 * `fill`, and every colour is sRGB hex. See specs/conformance.md.
 *
 * The web reads one element's computed style; a Flutter component is a
 * stack of render objects. What stands for the element is the part of that
 * stack the component's own box is: the decorations and the padding the
 * size of its root, the first run of text and the first flex inside it —
 * never inside a component nested in it, which is a specimen of its own.
 */

double _round(double n) => (n * 100).round() / 100;

String hex(Color colour) {
  final int argb = colour.toARGB32();
  String byte(int n) => n.toRadixString(16).padLeft(2, '0');
  final int a = argb >> 24 & 0xff;
  if (a == 0) return '#00000000';
  final String rgb =
      '#${byte(argb >> 16 & 0xff)}${byte(argb >> 8 & 0xff)}${byte(argb & 0xff)}';
  return a == 0xff ? rgb : '$rgb${byte(a)}';
}

/// A shadow, by the token it is, so it reads the same as React's.
String _shadow(List<BoxShadow>? shadow, ThemeVariables vars) {
  if (shadow == null || shadow.isEmpty) return 'none';
  final Map<String, List<BoxShadow>> tokens = {
    '2xs': vars.shadow2xs,
    'xs': vars.shadowXs,
    'sm': vars.shadowSm,
    'md': vars.shadowMd,
    'lg': vars.shadowLg,
    'xl': vars.shadowXl,
    '2xl': vars.shadow2xl,
  };
  for (final MapEntry(:key, :value) in tokens.entries) {
    if (listEquals(value, shadow)) return key;
  }
  return shadow.toString();
}

/// The render objects a specimen owns: its subtree, less its nested
/// specimens' subtrees.
List<RenderObject> _own(RenderObject root, Set<RenderObject> nested) {
  final List<RenderObject> list = [];
  void visit(RenderObject object) {
    if (!identical(object, root) && nested.contains(object)) return;
    list.add(object);
    object.visitChildren(visit);
  }

  visit(root);
  return list;
}

bool _sameSize(RenderObject object, Size size) =>
    object is RenderBox &&
    object.hasSize &&
    (object.size.width - size.width).abs() < 0.5 &&
    (object.size.height - size.height).abs() < 0.5;

Map<String, Object?> measure(
  RenderBox root,
  Set<RenderObject> nested,
  ThemeVariables vars,
) {
  final Size size = root.size;
  final List<RenderObject> own = _own(root, nested);

  // The surface: a decoration the size of the component's box.
  BoxDecoration? background;
  BoxDecoration? foreground;
  for (final RenderObject object in own) {
    if (object is! RenderDecoratedBox || !_sameSize(object, size)) continue;
    final Decoration decoration = object.decoration;
    if (decoration is! BoxDecoration) continue;
    if (object.position == DecorationPosition.background) {
      background ??= decoration;
    } else {
      foreground ??= decoration;
    }
  }
  // A surface painted by hand cannot be read back; one painted by nothing
  // is clear.
  final bool painted =
      background == null &&
      foreground == null &&
      own.any((o) => o is RenderCustomPaint && o.painter != null);

  final BoxBorder? border = background?.border ?? foreground?.border;
  final BorderSide? side = border is Border ? border.top : null;
  final double stroke = side == null || side.style == BorderStyle.none
      ? 0
      : side.width;
  final BorderRadiusGeometry? radius =
      background?.borderRadius ?? foreground?.borderRadius;
  // A radius past half the short side draws a capsule either way.
  final double corner = math.min(
    radius == null
        ? (background?.shape == BoxShape.circle ? size.shortestSide / 2 : 0)
        : radius.resolve(TextDirection.ltr).topLeft.x,
    size.shortestSide / 2,
  );

  // A glyph is a paragraph in an icon font on Flutter and an image on the
  // web: it has an ink, but no type to compare.
  bool glyph(RenderParagraph p) =>
      p.text.style?.fontFamily?.contains('FluentSystemIcons') ?? false;
  final List<RenderParagraph> paragraphs = own
      .whereType<RenderParagraph>()
      .toList();
  final TextStyle? text = paragraphs
      .where((p) => !glyph(p))
      .firstOrNull
      ?.text
      .style;
  final Color? ink = (text ?? paragraphs.firstOrNull?.text.style)?.color;
  final double? fontSize = text?.fontSize;

  final RenderFlex? flex = own.whereType<RenderFlex>().firstOrNull;

  // The padding is the inset from the box's edge to its content, less the
  // stroke — what CSS calls padding. Flutter spreads it over nested
  // paddings, and a decoration's border insets as well, so it is summed
  // from the root down to where the content starts: its first flex, or its
  // first run of text.
  EdgeInsets insets = EdgeInsets.zero;
  final RenderObject? content = flex ?? paragraphs.firstOrNull;
  for (RenderObject? object = content; object != null; object = object.parent) {
    if (object is RenderPadding && !identical(object, content)) {
      insets += object.padding.resolve(TextDirection.ltr);
    }
    if (identical(object, root)) break;
  }

  return {
    'box': {'width': _round(size.width), 'height': _round(size.height)},
    'values': {
      'corner': painted ? null : _round(corner),
      'stroke': painted ? null : _round(stroke),
      'strokeColor': painted || stroke == 0 ? null : hex(side!.color),
      'fill': painted
          ? null
          : hex(background?.color ?? const Color(0x00000000)),
      'ink': ink == null ? null : hex(ink),
      'fontSize': fontSize == null ? null : _round(fontSize),
      'fontWeight': text?.fontWeight?.value ?? (text == null ? null : 400),
      'lineHeight': fontSize == null || text?.height == null
          ? null
          : _round(fontSize * text!.height!),
      'paddingX': painted ? null : _round(math.max(0, insets.left - stroke)),
      'paddingY': painted ? null : _round(math.max(0, insets.top - stroke)),
      'gap': flex == null ? null : _round(flex.spacing),
      'shadow': painted ? null : _shadow(background?.boxShadow, vars),
    },
  };
}
