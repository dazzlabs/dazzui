import 'package:dazzui/dazzui.dart';

import '../../storybook/gallery.dart';
import '../../storybook/story.dart';

/// `Tokens/Radius` — one corner scale, and where each step lands.
class RadiusStories {
  RadiusStories._();

  static StoryPage get page => StoryPage('Radius', [
    Story('Scale', (context) => const _Scale(), layout: StoryLayout.page),
  ]);
}

class _Scale extends StatelessWidget {
  const _Scale();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    final List<(String, double, String)> steps = [
      ('radius.none', vars.radiusNone, 'Square'),
      ('radius.tiny', vars.radiusTiny, 'A badge, a swatch'),
      ('radius.small', vars.radiusSmall, 'Controls, nav rows'),
      ('radius.medium', vars.radiusMedium, 'Cards, popups'),
      ('radius.large', vars.radiusLarge, 'The large control'),
      ('radius.big', vars.radiusBig, 'A window'),
      ('radius.full', vars.radiusFull, 'A pill'),
    ];

    return DocPage(
      'Radius',
      description:
          'One scale per axis, so there is no second radius to alias by '
          'accident. The corners are the AppKit metrics the design is drawn to '
          '— which is why Bright’s pill controls are a theme re-pointing this '
          'scale rather than a widget choosing a different one.',
      children: [
        DocSection(
          'Steps',
          description:
              'Each drawn on the surface a widget of that corner sits on.',
          children: [
            TileGrid(
              min: 180,
              children: [
                for (final (name, radius, note) in steps)
                  Tile(
                    name,
                    note: note,
                    child: Container(
                      height: vars.spacing16,
                      decoration: BoxDecoration(
                        color: vars.colorPrimary[500],
                        borderRadius: BorderRadius.circular(radius),
                      ),
                    ),
                  ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
