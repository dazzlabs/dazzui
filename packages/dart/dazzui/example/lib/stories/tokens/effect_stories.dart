import 'package:dazzui/dazzui.dart';

import '../../storybook/gallery.dart';
import '../../storybook/story.dart';

/// `Tokens/Effect` — shadow, and the motion curve.
class EffectStories {
  EffectStories._();

  static StoryPage get page => StoryPage('Effect', [
    Story('Shadow', (context) => const _Shadow(), layout: StoryLayout.page),
  ]);
}

class _Shadow extends StatelessWidget {
  const _Shadow();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    final List<(String, List<BoxShadow>)> steps = [
      ('shadow.2xs', vars.shadow2xs),
      ('shadow.xs', vars.shadowXs),
      ('shadow.sm', vars.shadowSm),
      ('shadow.md', vars.shadowMd),
      ('shadow.lg', vars.shadowLg),
      ('shadow.xl', vars.shadowXl),
      ('shadow.2xl', vars.shadow2xl),
    ];

    return DocPage(
      'Effect',
      description:
          'Seven shadow steps and one motion curve. A shadow is stored as its '
          'layers rather than as a finished blur, because a token describes a '
          'CSS shadow — blur *diameter* and all — and each platform draws it '
          'in its own units.',
      children: [
        DocSection(
          'Shadow',
          description:
              'The hairline lift a filled button carries is the smallest step; '
              'a popup takes the middle of the scale; a dialog the top of it. '
              'Nothing in the system uses a shadow to say something is '
              'interactive — that is the recipe’s job.',
          children: [
            TileGrid(
              min: 200,
              children: [
                for (final (name, shadows) in steps)
                  Tile(
                    name,
                    child: Container(
                      height: vars.spacing12,
                      margin: EdgeInsets.all(vars.spacing2),
                      decoration: BoxDecoration(
                        color: vars.colorSurface,
                        borderRadius: BorderRadius.circular(vars.radiusMedium),
                        border: Border.all(
                          color: vars.colorBorder,
                          width: vars.spacingPx,
                        ),
                        boxShadow: shadows,
                      ),
                    ),
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Motion',
          description:
              'One duration and one curve, which every state change animates '
              'on. A widget that wanted its own timing would be saying its '
              'states mean something different from every other widget’s.',
          children: [
            TokenTable([
              TokenRow(
                'motion.duration',
                '${vars.motionDuration.inMilliseconds}ms',
              ),
              TokenRow('motion.easing', vars.motionEasing.toString()),
            ]),
          ],
        ),
      ],
    );
  }
}
