import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Drawer` — a sheet that comes in from an edge.
class DrawerStories {
  DrawerStories._();

  static const List<DrawerSide> sides = [
    DrawerSide.end,
    DrawerSide.start,
    DrawerSide.top,
    DrawerSide.bottom,
  ];

  static StoryPage get page => StoryPage('Drawer', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Bare',
      (context) => const Drawer(
        title: 'Filters',
        description: 'Narrow what the list is showing.',
        child: _Filters(),
      ),
      layout: StoryLayout.fullscreen,
    ),
    Story('Sides', (context) => const _Sides()),
    Story(
      'With footer',
      (context) => const _Remote(),
      layout: StoryLayout.fullscreen,
    ),
    Story('Controlled', (context) => const _Controlled()),
    Story(
      'Wide',
      (context) => const Drawer(
        size: 512,
        title: 'Review',
        description: 'A panel that needs more room than the token gives it.',
        child: Text(
          'The width is a token, and `size` is the override for a panel that '
          'genuinely needs more.',
        ),
      ),
      layout: StoryLayout.fullscreen,
    ),
  ]);
}

class _Filters extends StatelessWidget {
  const _Filters();

  @override
  Widget build(BuildContext context) {
    return const Col(
      children: [
        Checkbox(value: true, label: Text('Open')),
        Checkbox(value: false, label: Text('Merged')),
        Checkbox(value: false, label: Text('Closed')),
      ],
    );
  }
}

class _Remote extends StatelessWidget {
  const _Remote();

  @override
  Widget build(BuildContext context) {
    return Drawer(
      title: 'Edit remote',
      description: 'origin · git@github.com:acme/web.git',
      footer: [
        Button(
          variant: ButtonVariant.filled,
          onPressed: () {},
          child: const Text('Save'),
        ),
        Button(onPressed: () {}, child: const Text('Cancel')),
      ],
      child: const Col(
        children: [
          FormField(
            label: 'Name',
            child: TextField(placeholder: 'origin'),
          ),
          FormField(
            label: 'URL',
            hint: 'SSH or HTTPS',
            child: TextField(placeholder: 'git@github.com:acme/web.git'),
          ),
        ],
      ),
    );
  }
}

/// The four edges, each opened over the window rather than drawn in place: a
/// drawer that did not reach the Navigator would be bounded by whatever holds
/// it, which is the one thing it must not be.
class _Sides extends StatelessWidget {
  const _Sides();

  @override
  Widget build(BuildContext context) {
    return RowWrap(
      children: [
        for (final DrawerSide side in DrawerStories.sides)
          Button(
            size: WidgetSize.small,
            onPressed: () => showDrawer<void>(
              context: context,
              side: side,
              builder: (context) => const Drawer(
                title: 'Filters',
                description: 'Narrow what the list is showing.',
                child: _Filters(),
              ),
            ),
            child: Text(side.name),
          ),
      ],
    );
  }
}

class _Controlled extends StatelessWidget {
  const _Controlled();

  @override
  Widget build(BuildContext context) {
    return Button(
      variant: ButtonVariant.filled,
      onPressed: () => showDrawer<void>(
        context: context,
        builder: (context) => Drawer(
          title: 'Detail',
          description: 'Driven by the caller rather than by a trigger.',
          footer: [
            Button(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('Done'),
            ),
          ],
          child: const Text(
            'Nothing here opened it; the button beside it did.',
          ),
        ),
      ),
      child: const Text('Open'),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Drawer',
      description:
          'A sheet that comes in from an edge and can be pushed back out. It '
          'is the Dialog’s sibling, and the choice between them is about '
          'where the reader’s attention has to go rather than about the '
          'screen it is on: a dialog takes the middle of the view and asks a '
          'question; a drawer takes a side of it and holds a panel the reader '
          'dips into — filters, a detail pane — while what they were reading '
          'stays where it was.',
      children: [
        DocSection(
          'Sides',
          description:
              'It keeps a corner only on the two sides still inside the view. '
              'A sheet flush to an edge with all four corners rounded reads as '
              'floating just off the screen rather than as attached to it.',
          children: [_Sides()],
        ),
        DocSection(
          'The three rows',
          description:
              'Header, body and footer are properties here rather than the '
              'separate widgets a Dialog has, because a DialogHeader inside a '
              'drawer would be the same three rows drawn twice. Only the body '
              'scrolls, so a long panel keeps its heading and its actions in '
              'view.',
          children: [
            SizedBox(height: 320, child: _Remote()),
          ],
        ),
      ],
    );
  }
}
