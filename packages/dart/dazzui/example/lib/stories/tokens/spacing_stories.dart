import 'package:dazzui/dazzui.dart';

import '../../storybook/gallery.dart';
import '../../storybook/story.dart';

/// `Tokens/Spacing` — the grid every gap and pad comes off.
class SpacingStories {
  SpacingStories._();

  static StoryPage get page => StoryPage('Spacing', [
    Story('Scale', (context) => const _Scale(), layout: StoryLayout.page),
  ]);
}

class _Scale extends StatelessWidget {
  const _Scale();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    final List<(String, double)> steps = [
      ('spacing.px', vars.spacingPx),
      ('spacing.0', vars.spacing0),
      ('spacing.0-5', vars.spacing05),
      ('spacing.1', vars.spacing1),
      ('spacing.1-5', vars.spacing15),
      ('spacing.2', vars.spacing2),
      ('spacing.2-5', vars.spacing25),
      ('spacing.3', vars.spacing3),
      ('spacing.3-5', vars.spacing35),
      ('spacing.4', vars.spacing4),
      ('spacing.5', vars.spacing5),
      ('spacing.6', vars.spacing6),
      ('spacing.7', vars.spacing7),
      ('spacing.8', vars.spacing8),
      ('spacing.9', vars.spacing9),
      ('spacing.10', vars.spacing10),
      ('spacing.11', vars.spacing11),
      ('spacing.12', vars.spacing12),
      ('spacing.14', vars.spacing14),
      ('spacing.16', vars.spacing16),
      ('spacing.20', vars.spacing20),
    ];

    return DocPage(
      'Spacing',
      description:
          'The one scale, and the one part of the semantic layer that holds '
          'values directly rather than aliasing a primitive: these names *are* '
          'the grid steps, and a base twin under the same names would only be '
          'duplication.',
      children: [
        DocSection(
          'Steps',
          description:
              'A value the design lands between two steps is composed out of '
              'them rather than written down — the 18px a dialog body pads by '
              'is `spacing.4 + spacing.0-5`. A step and a half says what it '
              'is, where `18px` only says what it measures.',
          children: [
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing2,
              children: [
                for (final (name, length) in steps) Bar(name, length),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
