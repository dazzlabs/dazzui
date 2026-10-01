import 'package:flutter/foundation.dart';

import '../generated/theme_variables.dart';
import 'toml.dart';

/// The sizes Omarchy's shell sets its text and lays out its controls at, as
/// a machine's `shell.toml` decides them.
///
/// The shell (`shell/Commons/Style.qml`) derives every size from two roots.
/// Type comes from `[font] base-size`, 12 by default: each step is
/// `round(base ×` its multiplier`)`, and any step can be pinned in px under
/// its own key. Lengths — margins, gaps, padding, control heights, panel
/// dimensions — are the px they are at the default, times `[spacing] scale`,
/// and times `base-size / 12` too while `scale-with-font` is on, as it is
/// by default; `control-height` and `control-padding-x` can be pinned in px.
///
/// The generated `omarchy-*` themes carry all of it at the default, from
/// the `omarchy` metrics in `ds/themes/omarchy.yaml`; this follows a machine
/// that moved it. The shell's type steps land on DazzUI's the way that file
/// states: `caption` on caption, `body-small` on small, `body` on body,
/// `subtitle` on emphasis, `title` on title, and a control's label at
/// `body`, which is what the shell sets a button in.
@immutable
class OmarchyShellStyle {
  const OmarchyShellStyle({
    this.fontBaseSize = 12,
    this.fontOverrides = const {},
    this.spacingScale = 1,
    this.spacingScaleWithFont = true,
    this.spacingOverrides = const {},
  });

  /// The style out of the `shell.toml` files the shell reads, in the order
  /// it reads them — the theme's, then the user's — a later file's key
  /// winning over an earlier one's, as the shell's does.
  factory OmarchyShellStyle.fromShellToml(Iterable<String> sources) {
    final values = <String, String>{
      for (final source in sources) ...readOmarchyToml(source),
    };
    var fontBaseSize = 12;
    final fontOverrides = <String, int>{};
    var spacingScale = 1.0;
    var spacingScaleWithFont = true;
    final spacingOverrides = <String, double>{};
    for (final MapEntry(:key, :value) in values.entries) {
      if (key.startsWith('font.')) {
        final n = _parseInt(value);
        if (n == null) continue;
        final step = key.substring('font.'.length);
        if (step == 'base-size') {
          fontBaseSize = n;
        } else if (n > 0) {
          fontOverrides[step] = n;
        }
      } else if (key.startsWith('spacing.')) {
        final token = key.substring('spacing.'.length);
        if (token == 'scale-with-font') {
          spacingScaleWithFont = _parseBool(value) ?? spacingScaleWithFont;
          continue;
        }
        final n = double.tryParse(value.trim());
        if (n == null || !n.isFinite) continue;
        if (token == 'scale') {
          spacingScale = n;
        } else if (n >= 0) {
          spacingOverrides[token] = n;
        }
      }
    }
    return OmarchyShellStyle(
      fontBaseSize: fontBaseSize < 1 ? 1 : fontBaseSize,
      fontOverrides: fontOverrides,
      spacingScale: spacingScale < 0 ? 1 : spacingScale,
      spacingScaleWithFont: spacingScaleWithFont,
      spacingOverrides: spacingOverrides,
    );
  }

  /// `[font] base-size`, the root every type step is a multiple of.
  final int fontBaseSize;

  /// Type steps pinned in px, by the shell's key (`caption`, `body-small`,
  /// …).
  final Map<String, int> fontOverrides;

  /// `[spacing] scale`.
  final double spacingScale;

  /// `[spacing] scale-with-font`: whether lengths grow with the type too.
  final bool spacingScaleWithFont;

  /// Lengths pinned in px, by the shell's key (`control-height`, …).
  final Map<String, double> spacingOverrides;

  /// What every length is multiplied by, as the shell's
  /// `effectiveSpacingScale` is.
  double get lengthScale =>
      spacingScale * (spacingScaleWithFont ? fontBaseSize / 12 : 1);

  int _step(String key, double multiplier) =>
      fontOverrides[key] ?? _max1(_round(fontBaseSize * multiplier));

  int get caption => _step('caption', 0.833);
  int get bodySmall => _step('body-small', 0.917);
  int get body => _step('body', 1.0);
  int get subtitle => _step('subtitle', 1.083);
  int get title => _step('title', 1.167);

  /// [vars] — an Omarchy theme, at the shell's default sizes — at this
  /// machine's.
  ///
  /// Every face set at one of the type steps moves to it, keeping its
  /// family and weight; the headlines are not the chrome's and are left
  /// alone. A face that never wraps keeps its leading at its size, and those
  /// at the small step are a control's label.
  ///
  /// Every length is scaled as the shell's `space(px)` scales one, and with
  /// the spacing ladder go the paddings, gaps and boxes DazzUI derives from
  /// it. Corners, strokes and the focus ring are not lengths the shell
  /// scales, and stay.
  ThemeVariables applyTo(ThemeVariables vars) {
    final double caption = this.caption.toDouble();
    final double small = bodySmall.toDouble();
    final double body = this.body.toDouble();
    final double emphasis = subtitle.toDouble();
    final double title = this.title.toDouble();
    final double label = body;
    final double scale = lengthScale;
    double space(double px) =>
        px <= 0 ? 0 : _max1(_round(px * scale)).toDouble();
    double token(String key, double px) {
      final pinned = spacingOverrides[key];
      return pinned != null ? _round(pinned).toDouble() : space(px);
    }

    return vars.copyWith(
      titleSmallFontSize: body,
      titleSmallLineHeight: _leading(body),
      titleMediumFontSize: emphasis,
      titleMediumLineHeight: _leading(emphasis),
      titleLargeFontSize: title,
      titleLargeLineHeight: _leading(title),
      bodySmallFontSize: small,
      bodySmallLineHeight: _leading(small),
      bodyMediumFontSize: body,
      bodyMediumLineHeight: _leading(body),
      bodyLargeFontSize: emphasis,
      bodyLargeLineHeight: _leading(emphasis),
      labelQuietFontSize: label,
      labelQuietLineHeight: label,
      labelStrongFontSize: label,
      labelStrongLineHeight: label,
      labelSmallFontSize: caption,
      labelSmallLineHeight: caption,
      labelMediumFontSize: label,
      labelMediumLineHeight: label,
      labelLargeFontSize: body,
      labelLargeLineHeight: body,
      captionSmallFontSize: caption,
      captionSmallLineHeight: _leading(caption),
      captionMediumFontSize: small,
      captionMediumLineHeight: _leading(small),
      captionLargeFontSize: body,
      captionLargeLineHeight: _leading(body),
      spacingPx: space(vars.spacingPx),
      spacing05: space(vars.spacing05),
      spacing1: space(vars.spacing1),
      spacing15: space(vars.spacing15),
      spacing2: space(vars.spacing2),
      spacing25: space(vars.spacing25),
      spacing3: space(vars.spacing3),
      spacing35: space(vars.spacing35),
      spacing4: space(vars.spacing4),
      spacing5: space(vars.spacing5),
      spacing6: space(vars.spacing6),
      spacing7: space(vars.spacing7),
      spacing8: space(vars.spacing8),
      spacing9: space(vars.spacing9),
      spacing10: space(vars.spacing10),
      spacing11: space(vars.spacing11),
      spacing12: space(vars.spacing12),
      spacing14: space(vars.spacing14),
      spacing16: space(vars.spacing16),
      spacing20: space(vars.spacing20),
      iconSmall: space(vars.iconSmall),
      iconMedium: space(vars.iconMedium),
      iconLarge: space(vars.iconLarge),
      controlTinySize: space(vars.controlTinySize),
      controlSmallSize: space(vars.controlSmallSize),
      controlMediumSize: token('control-height', vars.controlMediumSize),
      controlLargeSize: space(vars.controlLargeSize),
      controlMediumPaddingInline: token(
        'control-padding-x',
        vars.controlMediumPaddingInline,
      ),
      frameTitlebarSize: space(vars.frameTitlebarSize),
      frameSidebarWidth: space(vars.frameSidebarWidth),
      frameSidebarIconWidth: space(vars.frameSidebarIconWidth),
      frameRailWidth: space(vars.frameRailWidth),
      frameAsideWidth: space(vars.frameAsideWidth),
      frameNavGap: space(vars.frameNavGap),
      dialogWidth: space(vars.dialogWidth),
      drawerSize: space(vars.drawerSize),
      menuMinWidth: space(vars.menuMinWidth),
      menuItemPadding: space(vars.menuItemPadding),
      preferencesWidth: space(vars.preferencesWidth),
      previewCardWidth: space(vars.previewCardWidth),
      segmentedControlInset: space(vars.segmentedControlInset),
      shortcutRecorderWidth: space(vars.shortcutRecorderWidth),
      switchMediumWidth: space(vars.switchMediumWidth),
      switchMediumHeight: space(vars.switchMediumHeight),
      switchMediumThumb: space(vars.switchMediumThumb),
      toastMaxWidth: space(vars.toastMaxWidth),
    );
  }

  @override
  bool operator ==(Object other) =>
      other is OmarchyShellStyle &&
      other.fontBaseSize == fontBaseSize &&
      mapEquals(other.fontOverrides, fontOverrides) &&
      other.spacingScale == spacingScale &&
      other.spacingScaleWithFont == spacingScaleWithFont &&
      mapEquals(other.spacingOverrides, spacingOverrides);

  @override
  int get hashCode => Object.hash(
    fontBaseSize,
    _hashMap(fontOverrides),
    spacingScale,
    spacingScaleWithFont,
    _hashMap(spacingOverrides),
  );

  @override
  String toString() =>
      'OmarchyShellStyle(font: $fontBaseSize $fontOverrides, '
      'spacing: ×$spacingScale${spacingScaleWithFont ? ' with font' : ''} '
      '$spacingOverrides)';
}

int _hashMap(Map<String, Object> map) => Object.hashAllUnordered(
  map.entries.map((e) => Object.hash(e.key, e.value)),
);

/// The shell leaves leading to Qt; DazzUI's is a third over the size,
/// rounded — the same rule the scale in `omarchy.yaml` is written by.
double _leading(double size) => _round(size * 4 / 3).toDouble();

/// JavaScript's `Math.round`, which the shell rounds with.
int _round(double v) => (v + 0.5).floor();

int _max1(int v) => v < 1 ? 1 : v;

/// QML's `parseInt`: the leading integer, so `12.5` reads as 12.
int? _parseInt(String value) {
  final match = RegExp(r'^\s*([+-]?\d+)').firstMatch(value);
  return match == null ? null : int.parse(match[1]!);
}

/// The shell's `boolToken`.
bool? _parseBool(String value) => switch (value.trim().toLowerCase()) {
  'true' || '1' || 'yes' || 'on' => true,
  'false' || '0' || 'no' || 'off' => false,
  _ => null,
};
