import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Skeleton` — the shape of something that has not arrived.
class SkeletonStories {
  SkeletonStories._();

  static StoryPage get page => StoryPage('Skeleton', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Shapes',
      (context) => const RowWrap(
        children: [
          Skeleton(shape: SkeletonShape.circle, width: 40),
          Skeleton(shape: SkeletonShape.rect, width: 80, height: 48),
        ],
      ),
    ),
    Story(
      'Text',
      (context) => const Col(width: 320, children: [SkeletonText()]),
    ),
    Story(
      'Lines',
      (context) => const Col(width: 320, children: [SkeletonText(lines: 5)]),
    ),
    Story('A list row', (context) => const _Placeholder()),
    Story('In a card', (context) => const _InACard()),
  ]);
}

/// An avatar and two lines: the shape a list row will land in.
class _Placeholder extends StatelessWidget {
  const _Placeholder();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return SizedBox(
      width: 320,
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        spacing: vars.spacing3,
        children: <Widget>[
          const Skeleton(shape: SkeletonShape.circle, width: 40),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing2,
              children: const <Widget>[
                FractionallySizedBox(
                  alignment: AlignmentDirectional.centerStart,
                  widthFactor: 0.7,
                  child: Skeleton(),
                ),
                FractionallySizedBox(
                  alignment: AlignmentDirectional.centerStart,
                  widthFactor: 0.45,
                  child: Skeleton(),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _InACard extends StatelessWidget {
  const _InACard();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return SizedBox(
      width: 352,
      child: Card(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisSize: MainAxisSize.min,
          spacing: vars.spacing3,
          children: const <Widget>[_Placeholder(), SkeletonText(lines: 2)],
        ),
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Skeleton',
      description:
          'The shape of something that has not arrived. A Spinner says the '
          'app is busy; a skeleton says what is coming, and where it will be — '
          'which is why it belongs on the first load of a view whose layout is '
          'already known, and a spinner belongs on an action whose result is '
          'not. Drawing one for content that then lands somewhere else is '
          'worse than drawing nothing.',
      children: [
        DocSection(
          'Shapes',
          description:
              'text is a bar on the line box of the type it stands in for, so '
              'it needs no height. circle is an avatar or a glyph and squares '
              'itself from its width alone. rect is everything with a box of '
              'its own — a thumbnail, a chart, a card.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'text',
                  children: [SizedBox(width: 80, child: Skeleton())],
                ),
                Specimen(
                  'circle',
                  children: [
                    Skeleton(shape: SkeletonShape.circle, width: 40),
                  ],
                ),
                Specimen(
                  'rect',
                  children: [
                    Skeleton(shape: SkeletonShape.rect, width: 80, height: 48),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Lines',
          description:
              'A paragraph’s worth of bars, with the last one short. The '
              'ragged edge is most of what makes a block of bars legible as '
              'text at all — a full-width final line reads as a table.',
          children: [SizedBox(width: 320, child: SkeletonText())],
        ),
        DocSection(
          'Materials',
          description:
              'The fill is surface-sunken — the same dead step a disabled '
              'control flattens to, which is exactly what a skeleton is. The '
              'sweep over it is surface-subtle, the ink wash the whole system '
              'already hovers with, so the highlight moves paper, card and '
              'column by the same amount without a per-theme value. Unlike the '
              'spinner it takes no reduced-motion exception: a still spinner '
              'reads as a hung app, where a still skeleton still says '
              'everything it has to say.',
          children: [_InACard()],
        ),
      ],
    );
  }
}
