import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Callout` — a tinted notice that stays.
class CalloutStories {
  CalloutStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Callout', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Info',
      (context) => const SizedBox(
        width: 460,
        child: Callout(
          tint: CalloutTint.info,
          title: Text('A new version is available'),
          message: Text('Restart to finish updating to 2.1.0.'),
        ),
      ),
    ),
    Story(
      'Success',
      (context) => const SizedBox(
        width: 460,
        child: Callout(
          tint: CalloutTint.success,
          title: Text('Tokens rebuilt'),
          message: Text('Four platform packages were refreshed.'),
        ),
      ),
    ),
    Story(
      'Warning',
      (context) => const SizedBox(
        width: 460,
        child: Callout(
          tint: CalloutTint.warning,
          title: Text('Ramp moves only part-way'),
          message: Text('color.warning cannot move whole under bright-dark.'),
        ),
      ),
    ),
    Story(
      'Danger',
      (context) => const SizedBox(
        width: 460,
        child: Callout(
          tint: CalloutTint.danger,
          title: Text('The build failed'),
          message: Text('dazzui: 4 errors in sidebar.dart.'),
        ),
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        width: 460,
        children: [
          for (final tint in CalloutTint.values)
            Callout(
              tint: tint,
              title: Text(label(tint.name)),
              message: const Text('The notice reads on any ramp.'),
            ),
        ],
      ),
    ),
    Story(
      'Without a title',
      (context) => const SizedBox(
        width: 460,
        child: Callout(
          tint: CalloutTint.info,
          message: Text('The glossary has not been indexed yet.'),
        ),
      ),
    ),
    Story(
      'With an icon',
      (context) => const SizedBox(
        width: 460,
        child: Callout(
          tint: CalloutTint.info,
          icon: Icon(FluentIcons.info_16_regular),
          title: Text('A new version is available'),
          message: Text('Restart to finish updating to 2.1.0.'),
        ),
      ),
    ),
    Story('With an action', (context) => const _Actioned()),
    Story(
      'Sizes',
      (context) => Col(
        width: 460,
        children: [
          for (final size in sizes)
            Callout(
              size: size,
              tint: CalloutTint.info,
              title: Text(label(size.namedSize.name)),
              message: const Text('The box changes; the type never does.'),
            ),
        ],
      ),
    ),
    Story(
      'In a pane',
      (context) => const Frame(
        width: 560,
        height: 260,
        children: [
          Pane(
            children: [
              Callout(
                tint: CalloutTint.warning,
                title: Text('Stale build'),
                message: Text(
                  'Rebuild the tokens before this page will be right.',
                ),
              ),
              Text('Components'),
            ],
          ),
        ],
      ),
    ),
  ]);
}

class _Actioned extends StatelessWidget {
  const _Actioned();

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 460,
      child: Callout(
        tint: CalloutTint.warning,
        title: const Text('Tokens are stale'),
        message: const Text('packages/ds changed after the last build.'),
        actions: [
          Button(
            variant: ButtonVariant.tinted,
            tint: ButtonTint.warning,
            onPressed: () {},
            child: const Text('Rebuild'),
          ),
        ],
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'Callout',
      description:
          'A notice that stays on the page, where a toast passes. It always '
          'wears a tinted treatment — a notice that needed a fill louder than '
          'this would be a dialog — so the tint is the only colour axis it has.',
      children: [
        DocSection(
          'Tints',
          description:
              'The fill and the edge are washes of the tint rather than opaque '
              'steps, which is what lets the same callout sit correctly on '
              'paper, on a card and on a near-black canvas without per-theme '
              'values.',
          children: [
            Col(
              width: proseMeasure,
              children: [
                for (final tint in CalloutTint.values)
                  Callout(
                    tint: tint,
                    title: Text(label(tint.name)),
                    message: const Text('The notice reads on any ramp.'),
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Shapes',
          description:
              'A title is optional: without one the message carries the whole '
              'notice. The leading slot takes a glyph, and the action slot '
              'takes at most one action — tinted to match, so the notice reads '
              'as one object rather than as a banner with a button on it.',
          children: [
            const Col(
              width: proseMeasure,
              children: [
                Callout(
                  tint: CalloutTint.info,
                  message: Text('The glossary has not been indexed yet.'),
                ),
                Callout(
                  tint: CalloutTint.info,
                  icon: Icon(FluentIcons.info_16_regular),
                  title: Text('A new version is available'),
                  message: Text('Restart to finish updating to 2.1.0.'),
                ),
                _Actioned(),
              ],
            ),
          ],
        ),
        DocSection(
          'Sizes',
          description:
              'One shape, three densities. The type never changes, only the '
              'box around it — a callout that grew its type would be competing '
              'with the content it interrupts.',
          children: [
            Col(
              width: proseMeasure,
              children: [
                for (final size in CalloutStories.sizes)
                  Callout(
                    size: size,
                    tint: CalloutTint.info,
                    title: Text(label(size.namedSize.name)),
                    message: const Text(
                      'The box changes; the type never does.',
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
