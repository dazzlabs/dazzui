import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/EmptyState` — what stands in for content that is not there.
class EmptyStateStories {
  EmptyStateStories._();

  static StoryPage get page => StoryPage('EmptyState', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'On its own',
      (context) => const EmptyState(title: 'No components match this filter'),
    ),
    Story('With an action', (context) => const _WithAnAction()),
    Story('With two actions', (context) => const _WithTwoActions()),
    Story(
      'Nothing matched',
      (context) => const EmptyState(title: 'Nothing matched “tokens”'),
    ),
    Story(
      'In a well',
      (context) => const SizedBox(
        width: 460,
        child: Card(variant: CardVariant.sunken, child: _WithAnAction()),
      ),
    ),
    Story(
      'In a pane',
      (context) => const Frame(
        width: 560,
        height: 300,
        children: [
          Pane(children: [_WithAnAction()]),
        ],
      ),
    ),
  ]);
}

class _WithAnAction extends StatelessWidget {
  const _WithAnAction();

  @override
  Widget build(BuildContext context) {
    return EmptyState(
      title: 'No components match this filter',
      actions: [
        Button(
          variant: ButtonVariant.tinted,
          onPressed: () {},
          child: const Text('Clear the filter'),
        ),
      ],
    );
  }
}

class _WithTwoActions extends StatelessWidget {
  const _WithTwoActions();

  @override
  Widget build(BuildContext context) {
    return EmptyState(
      title: 'This project has no glossary yet',
      actions: [
        Button(onPressed: () {}, child: const Text('Import')),
        Button(
          variant: ButtonVariant.filled,
          onPressed: () {},
          child: const Text('Create one'),
        ),
      ],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'EmptyState',
      description:
          'A line saying what is missing, and at most the one action that '
          'would fix it. It is the pane’s content rather than a notice about '
          'the pane, so it centres in the space it was given instead of '
          'hanging from the top of it.',
      children: [
        DocSection(
          'Shapes',
          description:
              'The title alone is enough where nothing can be done — a filter '
              'that matched nothing is fixed by changing the filter, not by a '
              'button. Where there is something to do, the action slot takes '
              'it; two actions sit on the control rhythm.',
          children: [
            RowWrap(
              alignment: WrapCrossAlignment.start,
              children: [
                EmptyState(title: 'Nothing matched “tokens”'),
                _WithAnAction(),
                _WithTwoActions(),
              ],
            ),
          ],
        ),
        DocSection(
          'Where it lands',
          description:
              'A well, when the empty thing is a section of a page; the pane '
              'itself, when the empty thing is the whole view.',
          children: [
            Frame(
              width: 620,
              height: 260,
              children: [
                Pane(children: [_WithAnAction()]),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
