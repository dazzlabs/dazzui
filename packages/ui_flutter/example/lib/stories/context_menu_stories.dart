import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/ContextMenu` — the menu a right-click opens.
class ContextMenuStories {
  ContextMenuStories._();

  static List<MenuItem> get items => const [
    MenuItem(label: 'Open', shortcut: '↩'),
    MenuItem(label: 'Rename', shortcut: 'F2'),
    MenuItem(label: 'Duplicate', shortcut: '⌘D'),
    MenuItem(label: 'Reveal in Finder'),
    MenuItem(label: 'Delete', enabled: false),
  ];

  static StoryPage get page => StoryPage('ContextMenu', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Bare',
      (context) => ContextMenu(items: items, child: const _Target()),
    ),
    Story('Checked', (context) => const _Sorted()),
    Story('On a row', (context) => const _Rows()),
  ]);
}

/// The region the menu belongs to. It draws nothing the menu needs — the
/// trigger is a listener on whatever it is given.
class _Target extends StatelessWidget {
  const _Target({this.label});

  final String? label;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Card(
      child: SizedBox(
        width: 352,
        height: 144,
        child: Center(
          child: Text(
            label ?? 'Right-click anywhere in here',
            style: vars.labelQuiet.copyWith(color: vars.colorContentSubtle),
          ),
        ),
      ),
    );
  }
}

class _Sorted extends StatefulWidget {
  const _Sorted();

  @override
  State<_Sorted> createState() => _SortedState();
}

class _SortedState extends State<_Sorted> {
  String _sort = 'name';

  @override
  Widget build(BuildContext context) {
    return ContextMenu(
      items: [
        for (final String key in ['name', 'date', 'size'])
          MenuItem(
            label: 'Sort by $key',
            checked: _sort == key,
            onSelect: () => setState(() => _sort = key),
          ),
      ],
      child: _Target(label: 'Sorted by $_sort'),
    );
  }
}

class _Rows extends StatelessWidget {
  const _Rows();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Col(
      width: 352,
      spacing: vars.spacing1,
      children: [
        for (final String branch in ['main', 'release/4.2', 'fix/token-drift'])
          ContextMenu(
            items: [
              const MenuItem(label: 'Check out'),
              const MenuItem(label: 'Rename', shortcut: 'F2'),
              MenuItem(label: 'Delete', enabled: branch != 'main'),
            ],
            child: Container(
              padding: EdgeInsets.symmetric(
                vertical: vars.spacing2,
                horizontal: vars.spacing3,
              ),
              decoration: BoxDecoration(
                color: vars.colorSurfaceMuted,
                borderRadius: BorderRadius.circular(vars.radiusMedium),
              ),
              child: Text(
                branch,
                style: vars.labelQuiet.copyWith(color: vars.colorContent),
              ),
            ),
          ),
      ],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'ContextMenu',
      description:
          'The menu a right-click opens, over the thing that was clicked. It '
          'is the same menu a button opens — the panel and the rows are the '
          'MenuPanel and the MenuRow, so this shares the drawing rather than '
          'resembling it. What is its own is where it opens: at the pointer '
          'rather than against a trigger’s edge, and on a long press as '
          'well as a secondary click, so it is reachable on a touch screen.',
      children: [
        DocSection(
          'A region that has a menu',
          description:
              'The trigger is a listener on what you give it, not a wrapper '
              'around it, so a row, a card or a canvas keeps its own layout. '
              'Every affordance in a context menu must also be reachable '
              'somewhere a pointer can see — a right-click is a shortcut, '
              'never the only door.',
          children: [
            ContextMenu(
              items: ContextMenuStories.items,
              child: const _Target(),
            ),
          ],
        ),
        const DocSection(
          'Checked rows',
          description:
              'Set `checked` on every row of a group, including the unchecked '
              'ones, so the leading column holds its width and the labels stay '
              'on one left edge.',
          children: [_Sorted()],
        ),
      ],
    );
  }
}
