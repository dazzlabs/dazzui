import 'package:dazzui/dazzui.dart';

import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/EmptyState` — what stands in for content that is not there.
class EmptyStateStories {
  EmptyStateStories._();

  static StoryPage get page => StoryPage('EmptyState', [
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
