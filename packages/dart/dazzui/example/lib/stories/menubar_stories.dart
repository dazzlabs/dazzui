import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/story.dart';

/// `Components/Menubar` — several menus that behave as one.
class MenubarStories {
  MenubarStories._();

  static List<MenubarMenu> get menus => const [
    MenubarMenu(
      label: 'File',
      items: [
        MenuItem(label: 'New project', shortcut: '⌘N'),
        MenuItem(label: 'Open…', shortcut: '⌘O'),
        MenuItem(label: 'Save', shortcut: '⌘S'),
        MenuItem(label: 'Close window', shortcut: '⌘W'),
      ],
    ),
    MenubarMenu(
      label: 'Edit',
      items: [
        MenuItem(label: 'Undo', shortcut: '⌘Z'),
        MenuItem(label: 'Redo', shortcut: '⇧⌘Z', enabled: false),
        MenuItem(label: 'Find…', shortcut: '⌘F'),
      ],
    ),
    MenubarMenu(
      label: 'View',
      items: [
        MenuItem(label: 'Sidebar', shortcut: '⌘1'),
        MenuItem(label: 'Dock', shortcut: '⌘2'),
        MenuItem(label: 'Full screen', shortcut: '⌃⌘F'),
      ],
    ),
  ];

  static StoryPage get page => StoryPage('Menubar', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Bare', (context) => Menubar(menus: menus)),
    Story('With checks', (context) => const _Density()),
    Story(
      'Disabled',
      (context) => Menubar(
        menus: [
          ...menus,
          const MenubarMenu(
            label: 'Window',
            enabled: false,
            items: [MenuItem(label: 'Minimise')],
          ),
        ],
      ),
    ),
  ]);
}

class _Density extends StatefulWidget {
  const _Density();

  @override
  State<_Density> createState() => _DensityState();
}

class _DensityState extends State<_Density> {
  String _density = 'Compact';

  @override
  Widget build(BuildContext context) {
    return Menubar(
      menus: [
        MenubarMenu(
          label: 'View',
          items: [
            for (final String name in ['Compact', 'Comfortable'])
              MenuItem(
                label: name,
                checked: _density == name,
                onSelect: () => setState(() => _density = name),
              ),
          ],
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
      'Menubar',
      description:
          'The application menu bar: several menus that behave as one. That is '
          'the whole reason it exists rather than a row of Menus — once one of '
          'them is open, moving the pointer along the bar opens the next '
          'without a second click, and the arrow keys walk the bar as well as '
          'the rows. It is the behaviour every desktop menu bar has and none '
          'of them documents.',
      children: [
        DocSection(
          'The bar',
          description:
              'Each panel is a MenuPanel of MenuRows, so the drawing is the '
              'same everywhere; the bar adds only its triggers. They are '
              'chrome rather than controls — no box at rest, and a wash only '
              'under the pointer or while their own menu is open.',
          children: [Menubar(menus: MenubarStories.menus)],
        ),
        const DocSection(
          'Checked rows',
          description:
              'A menu that reports a choice reveals the leading column on '
              'every row of the group. Set `checked` on all of them, including '
              'the unchecked ones, so the labels stay on one edge.',
          children: [_Density()],
        ),
      ],
    );
  }
}
