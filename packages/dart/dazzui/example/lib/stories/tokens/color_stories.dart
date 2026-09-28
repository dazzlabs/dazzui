import 'package:dazzui/dazzui.dart';

import '../../storybook/gallery.dart';
import '../../storybook/story.dart';

/// `Tokens/Color` — the ramps, and the roles that alias them.
class ColorStories {
  ColorStories._();

  static StoryPage get page => StoryPage('Color', [
    Story('Palette', (context) => const _Palette(), layout: StoryLayout.page),
    Story('Roles', (context) => const _Roles(), layout: StoryLayout.page),
  ]);

  /// The six ramps of the active theme, by the name a tint carries.
  static List<(String, ColorSwatch<int>)> ramps(ThemeVariables vars) => [
    ('primary', vars.colorPrimary),
    ('neutral', vars.colorNeutral),
    ('info', vars.colorInfo),
    ('success', vars.colorSuccess),
    ('warning', vars.colorWarning),
    ('danger', vars.colorDanger),
  ];
}

class _Palette extends StatelessWidget {
  const _Palette();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Palette',
      description:
          'Six ramps, eleven steps each, in the theme selected above. A ramp '
          'is a **role ladder, not a lightness ramp** — read Bright’s primary '
          'before assuming otherwise: its 700 is *lighter* than its 600, '
          'because that theme’s primary action is ink navy whose hover lifts. '
          'A step is only ever reached by a recipe, so what a step means is '
          'the role that recipe gives it.',
      children: [
        DocSection(
          'Ramps',
          description:
              'A widget never names a step. This page is the one place they '
              'are printed, and it is a reference rather than a menu: a shade '
              'written into a widget would be a lie rather than merely a '
              'duplicate, because the next theme re-points the ladder '
              'underneath it.',
          children: [
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing5,
              children: [
                for (final (name, swatch) in ColorStories.ramps(vars))
                  Ramp(name, swatch),
              ],
            ),
          ],
        ),
      ],
    );
  }
}

class _Roles extends StatelessWidget {
  const _Roles();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Roles',
      description:
          'The named colours a widget actually reads. Each theme decides its '
          'own paper, ink and edges — those are the one kind of raw value the '
          'semantic layer holds — and everything else aliases a ramp.',
      children: [
        DocSection(
          'Surfaces',
          description:
              'The canvas the window sits on, the paper a card is, and the '
              'grooves under it. Disabled controls flatten to '
              '`surface-sunken`, which is why it has to read as a hole rather '
              'than as a quieter card.',
          children: [
            TileGrid(
              min: 200,
              children: [
                Swatch('canvas', vars.colorCanvas, note: 'The window’s ground'),
                Swatch('surface', vars.colorSurface, note: 'Paper'),
                Swatch(
                  'surface-muted',
                  vars.colorSurfaceMuted,
                  note: 'Sidebar, titlebar',
                ),
                Swatch(
                  'surface-sunken',
                  vars.colorSurfaceSunken,
                  note: 'The groove',
                ),
                Swatch('surface-subtle', vars.colorSurfaceSubtle),
                Swatch('surface-inset', vars.colorSurfaceInset),
                Swatch(
                  'surface-raised',
                  vars.colorSurfaceRaised,
                  note: 'Popups',
                ),
                Swatch('surface-chrome', vars.colorSurfaceChrome),
              ],
            ),
          ],
        ),
        DocSection(
          'Ink',
          description:
              'The ladder body copy, labels and hints are set in. Every step '
              'is held to a WCAG contrast floor in every theme by a check in '
              'the token build, which fails rather than shipping a role that '
              'landed under it.',
          children: [
            TileGrid(
              min: 200,
              children: [
                Swatch('content', vars.colorContent, note: 'Body'),
                Swatch('content-secondary', vars.colorContentSecondary),
                Swatch(
                  'content-nav',
                  vars.colorContentNav,
                  note: 'Idle nav rows',
                ),
                Swatch('content-muted', vars.colorContentMuted, note: 'Hints'),
                Swatch('content-subtle', vars.colorContentSubtle),
                Swatch(
                  'content-faint',
                  vars.colorContentFaint,
                  note: 'Disabled',
                ),
                Swatch(
                  'on-accent',
                  vars.colorOnAccent,
                  note: 'On a filled surface',
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Edges',
          description:
              'One hairline, one stronger, one quieter. A separator draws '
              '`stroke.hairline` and halves on Retina; a control outline draws '
              '`stroke.control` and stays put.',
          children: [
            TileGrid(
              min: 200,
              children: [
                Swatch('border', vars.colorBorder),
                Swatch('border-strong', vars.colorBorderStrong),
                Swatch('border-muted', vars.colorBorderMuted),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
