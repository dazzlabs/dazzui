import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Loader` — work in flight, with no end in sight.
///
/// The React and SwiftUI packages call this `Spinner`; the Flutter one keeps
/// the name the widget shipped under.
class LoaderStories {
  LoaderStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Loader', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Default', (context) => const Loader()),
    Story(
      'Sizes',
      (context) => const RowWrap(
        children: [
          Loader(size: WidgetSize.small),
          Loader(),
          Loader(size: WidgetSize.large),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => RowWrap(
        children: [for (final tint in LoaderTint.values) Loader(tint: tint)],
      ),
    ),
    Story(
      'On an accent',
      (context) => Button(
        variant: ButtonVariant.filled,
        onPressed: () {},
        child: Row(
          mainAxisSize: MainAxisSize.min,
          spacing: Theme.of(context).vars.spacing1,
          children: const [
            Loader(size: WidgetSize.small, onAccent: true),
            Text('Translating…'),
          ],
        ),
      ),
    ),
    Story(
      'In a button',
      (context) => Button(
        variant: ButtonVariant.filled,
        onPressed: null,
        child: Row(
          mainAxisSize: MainAxisSize.min,
          spacing: Theme.of(context).vars.spacing1,
          children: const [
            Loader(size: WidgetSize.small),
            Text('Translating…'),
          ],
        ),
      ),
    ),
    Story('In a callout', (context) => const _InACallout()),
    Story('In a panel', (context) => const _InAPanel()),
  ]);
}

class _InACallout extends StatelessWidget {
  const _InACallout();

  @override
  Widget build(BuildContext context) {
    return const SizedBox(
      width: 420,
      child: Callout(
        tint: CalloutTint.info,
        icon: Loader(size: WidgetSize.small, tint: LoaderTint.info),
        title: Text('Rebuilding'),
        message: Text('Four platform packages are being rewritten.'),
      ),
    );
  }
}

class _InAPanel extends StatelessWidget {
  const _InAPanel();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return SizedBox(
      width: 320,
      child: Card(
        child: Row(
          mainAxisSize: MainAxisSize.min,
          spacing: vars.spacing3,
          children: const [
            Loader(),
            Flexible(child: Text('Resolving packages…')),
          ],
        ),
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'Loader',
      description:
          'The indeterminate mark. Where a `Progress` bar without a value '
          'sweeps along a track, a loader has no track at all — it is what a '
          'label, a button or a toast carries while it waits.',
      children: [
        DocSection(
          'Sizes and tints',
          description:
              'Three sizes; the track is the tint at a quarter, which is the '
              'one value the React stylesheet holds as a literal and this port '
              'keeps as a named constant beside the same reason.',
          children: [
            Specimens(
              children: [
                for (final size in LoaderStories.sizes)
                  Specimen(
                    size.namedSize.name,
                    children: [
                      for (final tint in LoaderTint.values)
                        Loader(size: size, tint: tint),
                    ],
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'On an accent',
          description:
              '`onAccent` is the loader drawn to sit on a filled surface '
              'rather than on paper. A loader inside a *disabled* filled '
              'button is on the groove instead, so it takes the ordinary '
              'colours — which is why this is a flag rather than something the '
              'widget guesses.',
          children: [
            RowWrap(
              children: [
                Button(
                  variant: ButtonVariant.filled,
                  onPressed: () {},
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    spacing: Theme.of(context).vars.spacing1,
                    children: const [
                      Loader(size: WidgetSize.small, onAccent: true),
                      Text('Translating…'),
                    ],
                  ),
                ),
                Button(
                  variant: ButtonVariant.filled,
                  onPressed: null,
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    spacing: Theme.of(context).vars.spacing1,
                    children: const [
                      Loader(size: WidgetSize.small),
                      Text('Translating…'),
                    ],
                  ),
                ),
              ],
            ),
          ],
        ),
        const DocSection(
          'Where it lands',
          description:
              'In a button’s label, in a callout’s leading slot, in a toast, '
              'or on its own in a panel that is still filling.',
          children: [
            Col(children: [_InACallout(), _InAPanel()]),
          ],
        ),
      ],
    );
  }
}
