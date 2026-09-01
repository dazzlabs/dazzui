import 'package:flutter/widgets.dart';

import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../generated/theme_variables.dart';
import '../theme/theme.dart';
import 'oval_loader.dart';

/// The tint a [Loader] draws in.
enum LoaderTint with WidgetTint {
  primary,
  neutral,
  info,
  success,
  warning,
  danger,
}

/// A ring with one segment picked out: the track is the tint at quarter
/// strength, the head the tint itself, and the whole thing turns once a
/// second — fast enough to read as work, slow enough not to read as alarm.
class Loader extends StatelessWidget {
  const Loader({
    super.key,
    this.size = WidgetSize.medium,
    this.tint = LoaderTint.primary,
    this.color,
    this.onAccent = false,
  });

  final WidgetSize size;

  final LoaderTint tint;

  final Color? color;

  /// On a filled control the ring goes to the accent's contrasting ink — a
  /// tinted spinner on its own tint's fill is invisible.
  final bool onAccent;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    final Color resolvedColor =
        color ??
        (onAccent
            ? vars.colorOnAccent
            : switch (tint) {
                LoaderTint.primary => vars.colorPrimary[600]!,
                LoaderTint.neutral => vars.colorNeutral[600]!,
                LoaderTint.info => vars.colorInfo[600]!,
                LoaderTint.success => vars.colorSuccess[600]!,
                LoaderTint.warning => vars.colorWarning[600]!,
                LoaderTint.danger => vars.colorDanger[600]!,
              });

    final double dimension = switch (size.namedSize) {
      NamedSize.large => vars.spacing5,
      NamedSize.medium => vars.spacing4,
      _ => vars.spacing35,
    };

    return SizedBox(
      width: dimension,
      height: dimension,
      child: OvalLoader(
        size: dimension,
        color: resolvedColor,
        // A wash rather than a ramp step, so the track sits as correctly over
        // a tinted chip or a dark canvas as over the paper.
        trackAlpha: onAccent ? 0.35 : 0.25,
        strokeWidth: vars.spacing05,
      ),
    );
  }
}
