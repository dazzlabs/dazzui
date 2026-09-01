import 'package:dazzui/dazzui.dart';

import '../gallery.dart';
import '../playground_theme.dart';

/// The four themes at once.
///
/// This is the page that argues for the design of the thing: a theme is a
/// value scoped to a subtree with the package's own [Theme] widget, not a
/// process-wide mode, so four of them can sit side by side and every
/// component inside each one resolves against its own variables. The cards
/// below are the same widget rendered four times.
class ThemesPage extends StatelessWidget {
  const ThemesPage({super.key});

  /// Mirrors the SwiftUI grid's adaptive minimum column width.
  static const _cardWidth = 300.0;

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing6,
      children: [
        const PageTitle(
          'Themes',
          'Four themes, one widget tree — each card scopes its own with '
              'Theme(data:).',
        ),
        Wrap(
          spacing: vars.spacing4,
          runSpacing: vars.spacing4,
          children: [
            for (final theme in PlaygroundTheme.values)
              Theme(
                data: theme.data,
                child: SizedBox(
                  width: _cardWidth,
                  child: _ThemeCard(theme: theme),
                ),
              ),
          ],
        ),
        Specimen(
          'Ramps',
          note:
              'a role ladder, not a lightness ramp — read the ink ladder '
              'before assuming',
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing2,
            children: const [
              _RampRow('primary'),
              _RampRow('neutral'),
              _RampRow('info'),
              _RampRow('success'),
              _RampRow('warning'),
              _RampRow('danger'),
            ],
          ),
        ),
        SizedBox(height: vars.spacing8),
      ],
    );
  }
}

/// One theme's own little window: the surfaces stacked, a control on each,
/// and the accent that separates the families.
class _ThemeCard extends StatelessWidget {
  const _ThemeCard({required this.theme});

  final PlaygroundTheme theme;

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Container(
      padding: EdgeInsets.all(vars.spacing4),
      decoration: BoxDecoration(
        color: vars.colorSurface,
        borderRadius: BorderRadius.circular(vars.frameWindowRadius),
        border: Border.all(color: vars.colorBorder, width: vars.spacingPx),
        boxShadow: vars.shadowSm,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing3,
        children: [
          Row(
            children: [
              Expanded(
                child: Text(
                  theme.title,
                  overflow: TextOverflow.ellipsis,
                  style: vars.labelLarge.copyWith(color: vars.colorContent),
                ),
              ),
              Badge(
                size: WidgetSize.small,
                variant: BadgeVariant.tinted,
                child: Text(theme.name),
              ),
            ],
          ),
          Text(
            'The quick brown fox jumps over the lazy dog.',
            style: vars.bodySmall.copyWith(color: vars.colorContentMuted),
          ),
          RowWrap(
            children: [
              Button(
                variant: ButtonVariant.filled,
                onPressed: () {},
                child: const Text('Primary'),
              ),
              Button(
                variant: ButtonVariant.plain,
                onPressed: () {},
                child: const Text('Quiet'),
              ),
            ],
          ),
          const Divider(),
          const RowWrap(
            children: [
              Badge(
                size: WidgetSize.small,
                variant: BadgeVariant.tinted,
                tint: BadgeTint.success,
                child: Text('Passed'),
              ),
              Badge(
                size: WidgetSize.small,
                variant: BadgeVariant.tinted,
                tint: BadgeTint.danger,
                child: Text('Failed'),
              ),
              Kbd('⌘S'),
            ],
          ),
        ],
      ),
    );
  }
}

/// The eleven steps of one ramp, in the active theme.
class _RampRow extends StatelessWidget {
  const _RampRow(this.name);

  final String name;

  static const _shades = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;
    final ramp = switch (name) {
      'primary' => vars.colorPrimary,
      'neutral' => vars.colorNeutral,
      'info' => vars.colorInfo,
      'success' => vars.colorSuccess,
      'warning' => vars.colorWarning,
      _ => vars.colorDanger,
    };

    return Row(
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing1,
      children: [
        SizedBox(
          width: vars.spacing16,
          child: Text(
            name,
            style: vars.captionSmall.copyWith(color: vars.colorContentMuted),
          ),
        ),
        for (final shade in _shades)
          Container(
            width: vars.spacing8,
            height: vars.spacing6,
            decoration: BoxDecoration(
              color: ramp[shade],
              borderRadius: BorderRadius.circular(vars.radiusTiny),
              border: Border.all(
                color: vars.colorBorder,
                width: vars.spacingPx,
              ),
            ),
          ),
      ],
    );
  }
}
