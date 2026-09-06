import 'package:dazzui/dazzui.dart';

import '../../playground_theme.dart';
import '../../storybook/gallery.dart';
import '../../storybook/stage.dart';
import '../../storybook/story.dart';
import 'color_stories.dart';

/// `Tokens/Themes` — the four, and the argument for how they are carried.
class ThemeStories {
  ThemeStories._();

  static StoryPage get page => StoryPage('Themes', [
    Story(
      'Four at once',
      (context) => const _FourAtOnce(),
      layout: StoryLayout.page,
    ),
    Story('Ramps', (context) => const _Ramps(), layout: StoryLayout.page),
  ]);
}

class _FourAtOnce extends StatelessWidget {
  const _FourAtOnce();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'Themes',
      description:
          'Four themes, one widget tree. A theme here is a value scoped to a '
          'subtree with the package’s own `Theme` widget rather than a '
          'process-wide mode, so the four cards below are the same widget '
          'rendered four times, each resolving against its own variables.',
      children: [
        DocSection(
          'The four',
          description:
              'Studio Light is the baseline the others are modes over: Studio '
              'Dark is the same skeleton on a near-black canvas, Bright is '
              'warm paper with ink navy and an acid marker, and Bright Dark '
              'takes that acid green as the accent itself.',
          children: [
            TileGrid(
              min: 300,
              children: [
                for (final theme in PlaygroundTheme.values)
                  Theme(
                    data: theme.data,
                    // Raised rather than outlined: the card has to bring its
                    // own theme’s paper with it, or a dark theme’s ink would
                    // be drawn on the light page underneath.
                    child: Card(child: _Sampler(name: theme)),
                  ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}

/// One theme's own little window: the surfaces stacked, a control on each, and
/// the accent that separates the families.
class _Sampler extends StatelessWidget {
  const _Sampler({required this.name});

  final PlaygroundTheme name;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing3,
      children: [
        Row(
          children: [
            Text(
              name.title,
              style: vars.labelLarge.copyWith(color: vars.colorContent),
            ),
            const Spacer(),
            Badge(
              size: WidgetSize.small,
              variant: BadgeVariant.tinted,
              child: Text(name.name),
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
            KeyCap('⌘S'),
          ],
        ),
        const Progress(value: 0.6, gradient: true),
      ],
    );
  }
}

class _Ramps extends StatelessWidget {
  const _Ramps();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables outer = Theme.of(context).vars;

    return DocPage(
      'Ramps in four themes',
      description:
          'The same six ramps, under each theme in turn. This is the page that '
          'says why a widget must never name a shade: the ladder under the '
          'recipe is a different ladder in every block below, and the recipe '
          'table above them is untouched.',
      children: [
        for (final theme in PlaygroundTheme.values)
          DocSection(
            theme.title,
            children: [
              Theme(
                data: theme.data,
                child: Builder(
                  builder: (context) {
                    final ThemeVariables vars = Theme.of(context).vars;

                    return Container(
                      padding: EdgeInsets.all(outer.spacing4),
                      decoration: BoxDecoration(
                        color: vars.colorSurface,
                        borderRadius: BorderRadius.circular(
                          outer.radiusMedium,
                        ),
                      ),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        mainAxisSize: MainAxisSize.min,
                        spacing: outer.spacing4,
                        children: [
                          for (final (name, swatch) in ColorStories.ramps(vars))
                            Ramp(name, swatch),
                        ],
                      ),
                    );
                  },
                ),
              ),
            ],
          ),
      ],
    );
  }
}
