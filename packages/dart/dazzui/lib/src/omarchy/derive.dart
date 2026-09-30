import 'dart:math' as math;

import 'package:flutter/widgets.dart';

import '../generated/theme_variables.dart';

/// `ds/scripts/themes.mjs`'s `derive`, in Dart: every colour an imported
/// theme has, from the three its source decides everything else from.
///
/// The build derives Omarchy's shipped themes once, into the generated
/// `themeVariablesOmarchy*` instances; this does the same at run time, for
/// whatever palette the machine is wearing now — a user's own theme as much
/// as a built-in one. It is a transcription, step for step, rather than a
/// second reading, and `test/omarchy_test.dart` holds it to the generated
/// instances to the digit. A change to the script is a change here.
///
/// Everything that is not a colour — Hyprland's square corner, the dark
/// shadows, the shades a recipe picks — is the same for every Omarchy theme
/// of one scheme, so it is taken from a generated Omarchy instance of that
/// scheme and only the derived colours are laid over it.
ThemeVariables deriveOmarchyThemeVariables({
  required Color background,
  required Color foreground,
  required Color accent,
  required bool dark,
}) {
  final paper = _rgb(background);
  final ink = _parse(foreground, paper);
  final accentRgb = _parse(accent, paper);
  final steps = dark ? _ladderDark : _ladderLight;
  final anchor = dark ? 500 : 600;
  final ramp = _rampThrough(accentRgb, anchor, paper, dark);

  _Rgb off(double dL) => _stepOff(paper, ink, dL)!;
  _Rgb under(double dL) => _stepOff(paper, const [0, 0, 0], dL) ?? off(dL);

  final muted = off(steps.muted);
  final column = dark ? under(steps.column) : off(steps.column);
  final inset = off(steps.inset);
  final sunken = off(steps.sunken);
  final raised = dark ? off(steps.raised) : paper;
  final canvas = dark ? under(steps.canvas) : off(steps.canvas);
  final overlay = dark ? muted : paper;

  final washOver = [
    (paper, dark ? 5.5 : 4.5),
    (muted, dark ? 5.0 : 4.0),
    (column, dark ? 5.0 : 4.0),
    (overlay, dark ? 5.0 : 4.5),
  ];
  final subtle = washOver
      .map((e) => _alphaFor(ink, e.$1, e.$2))
      .fold<double>(dark ? 0.07 : 0.06, math.max);

  final faint = _mix(paper, ink, 0.36);
  final subtleInk = _mix(paper, ink, 0.5);
  final mutedT = _legible(paper, ink, 0.64, 4.6);
  final navT = math.max(_legible(paper, ink, 0.72, 4.6), mutedT);
  final secondaryT = math.max(_legible(paper, ink, 0.82, 4.6), navT);

  final separator = dark ? 0.16 : 0.13;

  final onAccent =
      [paper, ink].where((c) => _contrast(c, accentRgb) >= 3).firstOrNull ??
      paper;

  final swatch = ColorSwatch<int>(_color(ramp[600]!).toARGB32(), {
    for (final step in _steps) step: _color(ramp[step]!),
  });

  final template = dark
      ? themeVariablesOmarchyTokyoNightDark
      : themeVariablesOmarchyTokyoNightLight;
  return template.copyWith(
    colorCanvas: _color(canvas),
    colorSurface: _color(paper),
    colorSurfaceMuted: _color(muted),
    colorSurfaceSunken: _color(sunken),
    colorSurfaceSubtle: _color(ink, subtle),
    colorSurfaceInset: _color(inset),
    colorSurfaceRaised: _color(raised),
    colorSurfaceOverlay: _color(overlay),
    colorSurfaceChrome: _color(muted),
    colorSurfaceColumn: _color(column),
    colorContent: _color(ink),
    colorContentSecondary: _color(_mix(paper, ink, secondaryT)),
    colorContentNav: _color(_mix(paper, ink, navT)),
    colorContentMuted: _color(_mix(paper, ink, mutedT)),
    colorContentSubtle: _color(subtleInk),
    colorContentFaint: _color(faint),
    colorBorder: _color(ink, separator),
    colorBorderStrong: _color(ink, separator + 0.04),
    colorBorderMuted: _color(_mix(paper, ink, 0.38)),
    colorOnAccent: _color(onAccent),
    colorPrimary: swatch,
    progressGradientFrom: swatch[anchor],
    progressGradientTo: swatch[300],
  );
}

/// `bothModes`' other half: the foreground becomes the paper, lifted towards
/// white or pressed towards black, the paper becomes the ink, and the accent
/// is pressed or lifted so a label printed in it still reads.
({Color background, Color foreground, Color accent}) oppositeOmarchyColors({
  required Color background,
  required Color foreground,
  required Color accent,
  required bool dark,
}) {
  const white = Color(0xFFFFFFFF);
  const black = Color(0xFF000000);
  return dark
      ? (
          background: _mixHex(foreground, white, 0.78),
          foreground: background,
          accent: _mixHex(accent, black, 0.38),
        )
      : (
          background: _mixHex(foreground, black, 0.55),
          foreground: background,
          accent: _mixHex(accent, white, 0.38),
        );
}

typedef _Rgb = List<double>;

const _steps = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

typedef _Ladder = ({
  double muted,
  double column,
  double inset,
  double sunken,
  double raised,
  double canvas,
});

const _Ladder _ladderLight = (
  muted: 2.5,
  column: 1.5,
  inset: 5.5,
  sunken: 10.5,
  raised: 0,
  canvas: 6,
);

const _Ladder _ladderDark = (
  muted: 3,
  column: 1.5,
  inset: 6,
  sunken: 10,
  raised: 20,
  canvas: 2,
);

/// `a` mixed `t` of the way towards `b`, in sRGB bytes.
Color _mixHex(Color a, Color b, double t) {
  int channel(double from, double to) {
    final f = (from * 255).round();
    return _round(f + ((to * 255).round() - f) * t);
  }

  return Color.fromARGB(
    255,
    channel(a.r, b.r),
    channel(a.g, b.g),
    channel(a.b, b.b),
  );
}

/* The ramp ----------------------------------------------------------------- */

const _shape = <int, (double, double)>{
  50: (0.975, 0.12),
  100: (0.945, 0.25),
  200: (0.89, 0.45),
  300: (0.82, 0.7),
  400: (0.72, 0.9),
  500: (0.64, 1),
  600: (0.56, 1),
  700: (0.49, 0.92),
  800: (0.42, 0.82),
  900: (0.36, 0.7),
  950: (0.27, 0.55),
};

Map<int, _Rgb> _rampThrough(_Rgb accent, int anchor, _Rgb paper, bool dark) {
  final [L, C, h] = _oklch(accent);
  final (aL, aC) = _shape[anchor]!;
  final light = _shape[50]!.$1;
  final deep = _shape[950]!.$1;
  double lightnessOf(int step) {
    final sL = _shape[step]!.$1;
    return sL > aL
        ? L + ((math.max(light, L) - L) * (sL - aL)) / (light - aL)
        : L - ((L - math.min(deep, L)) * (aL - sL)) / (aL - deep);
  }

  final ramp = <int, _Rgb>{
    for (final step in _steps)
      step: step == anchor
          ? accent
          : _inGamut(lightnessOf(step), (C * _shape[step]!.$2) / aC, h),
  };
  for (final step in dark ? const [300, 200] : const [700, 800]) {
    var sL = _oklch(ramp[step]!)[0];
    final chroma = (C * _shape[step]!.$2) / aC;
    while (_contrast(ramp[step]!, paper) < 4.5 && sL > 0.02 && sL < 0.99) {
      sL += dark ? 0.01 : -0.01;
      ramp[step] = _inGamut(sL, chroma, h);
    }
  }
  return ramp;
}

_Rgb _inGamut(double L, double C, double h) {
  var c = C;
  for (var i = 0; i < 40; i++) {
    final rgb = _fromOklch(L, c, h);
    if (rgb.every((v) => v >= -1e-4 && v <= 1 + 1e-4)) {
      return rgb.map(_clamp01).toList();
    }
    c *= 0.9;
  }
  return _fromOklch(L, 0, h).map(_clamp01).toList();
}

/* The ladders -------------------------------------------------------------- */

_Rgb? _stepOff(_Rgb from, _Rgb to, double dL) {
  final l0 = _lightness(from);
  if ((_lightness(to) - l0).abs() < dL) return null;
  var lo = 0.0;
  var hi = 1.0;
  for (var i = 0; i < 40; i++) {
    final t = (lo + hi) / 2;
    if ((_lightness(_mix(from, to, t)) - l0).abs() < dL) {
      lo = t;
    } else {
      hi = t;
    }
  }
  return _mix(from, to, hi);
}

double _alphaFor(_Rgb ink, _Rgb ground, double dL) {
  final l0 = _lightness(ground);
  var lo = 0.0;
  var hi = 1.0;
  for (var i = 0; i < 40; i++) {
    final a = (lo + hi) / 2;
    if ((_lightness(_mix(ground, ink, a)) - l0).abs() < dL) {
      lo = a;
    } else {
      hi = a;
    }
  }
  return (hi * 255).ceil() / 255;
}

double _legible(_Rgb paper, _Rgb ink, double t, double min) {
  var u = t;
  while (u < 1 && _contrast(_mix(paper, ink, u), paper) < min) {
    u += 0.01;
  }
  return math.min(u, 1);
}

/* Colour ------------------------------------------------------------------- */

_Rgb _rgb(Color c) => [
  (c.r * 255).round() / 255,
  (c.g * 255).round() / 255,
  (c.b * 255).round() / 255,
];

/// A colour; one with an alpha is composited over `ground` first.
_Rgb _parse(Color c, _Rgb ground) {
  final rgb = _rgb(c);
  final alpha = (c.a * 255).round() / 255;
  return [for (var i = 0; i < 3; i++) rgb[i] * alpha + ground[i] * (1 - alpha)];
}

/// Hex precision is what ships, so every value is rounded to it here, as
/// the script's `hex` does.
Color _color(_Rgb rgb, [double alpha = 1]) {
  int byte(double v) => _round(_clamp01(v) * 255);
  return Color.fromARGB(
    alpha < 1 ? byte(alpha) : 255,
    byte(rgb[0]),
    byte(rgb[1]),
    byte(rgb[2]),
  );
}

_Rgb _mix(_Rgb a, _Rgb b, double t) => [
  for (var i = 0; i < 3; i++) _round((a[i] + (b[i] - a[i]) * t) * 255) / 255,
];

/// JavaScript's `Math.round`: halves go up, never away from zero.
int _round(double v) => (v + 0.5).floor();

double _clamp01(double v) => math.min(1, math.max(0, v));

double _cbrt(double v) =>
    v < 0 ? -math.pow(-v, 1 / 3).toDouble() : math.pow(v, 1 / 3).toDouble();

double _linear(double v) =>
    v <= 0.04045 ? v / 12.92 : math.pow((v + 0.055) / 1.055, 2.4).toDouble();

double _gamma(double v) => v <= 0.0031308
    ? 12.92 * v
    : 1.055 * v.sign * math.pow(v.abs(), 1 / 2.4) - 0.055;

double _luminance(_Rgb rgb) =>
    0.2126 * _linear(rgb[0]) +
    0.7152 * _linear(rgb[1]) +
    0.0722 * _linear(rgb[2]);

double _lightness(_Rgb rgb) {
  final y = _luminance(rgb);
  return y > 0.008856 ? 116 * _cbrt(y) - 16 : 903.3 * y;
}

double _contrast(_Rgb a, _Rgb b) {
  final la = _luminance(a);
  final lb = _luminance(b);
  return (math.max(la, lb) + 0.05) / (math.min(la, lb) + 0.05);
}

List<double> _oklch(_Rgb rgb) {
  final r = _linear(rgb[0]);
  final g = _linear(rgb[1]);
  final b = _linear(rgb[2]);
  final l = _cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b);
  final m = _cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b);
  final s = _cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b);
  final L = 0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s;
  final A = 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s;
  final B = 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s;
  return [L, math.sqrt(A * A + B * B), math.atan2(B, A)];
}

_Rgb _fromOklch(double L, double C, double h) {
  final A = C * math.cos(h);
  final B = C * math.sin(h);
  double cube(double v) => v * v * v;
  final l = cube(L + 0.3963377774 * A + 0.2158037573 * B);
  final m = cube(L - 0.1055613458 * A - 0.0638541728 * B);
  final s = cube(L - 0.0894841775 * A - 1.291485548 * B);
  return [
    4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
    -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
    -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s,
  ].map(_gamma).toList();
}
