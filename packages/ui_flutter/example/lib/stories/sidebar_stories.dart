import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Sidebar` — the first navigation column.
class SidebarStories {
  SidebarStories._();

  static StoryPage get page => StoryPage('Sidebar', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'A column',
      (context) => const Frame(
        width: 560,
        height: 320,
        children: [
          Sidebar(children: [_Library()]),
          Pane(children: [Text('Projects')]),
        ],
      ),
    ),
    Story(
      'Groups',
      (context) => Frame(
        width: 560,
        height: 320,
        children: [
          Sidebar(
            children: [
              const _Library(),
              SidebarGroup(
                label: 'Recent',
                children: [
                  NavItem(label: 'aurora', onPressed: () {}),
                  NavItem(label: 'beacon', onPressed: () {}),
                ],
              ),
            ],
          ),
          const Pane(children: [Text('Projects')]),
        ],
      ),
    ),
    Story(
      'With a header',
      (context) => const Frame(
        width: 560,
        height: 320,
        children: [
          Sidebar(
            header: Badge(
              size: WidgetSize.small,
              variant: BadgeVariant.tinted,
              child: Text('DazzUI'),
            ),
            children: [_Library()],
          ),
          Pane(children: [Text('Projects')]),
        ],
      ),
    ),
    Story('With a card', (context) => const _WithACard()),
    Story(
      'With an action',
      (context) => Frame(
        width: 560,
        height: 320,
        children: [
          Sidebar(
            children: [
              const _Library(),
              NavItem(label: 'New project', onPressed: () {}),
            ],
          ),
          const Pane(children: [Text('Projects')]),
        ],
      ),
    ),
    Story(
      'A fixed width',
      (context) => const Frame(
        width: 560,
        height: 320,
        children: [
          Sidebar(width: 200, children: [_Library()]),
          Pane(children: [Text('The column can be given a width of its own.')]),
        ],
      ),
    ),
  ]);
}

class _WithACard extends StatelessWidget {
  const _WithACard();

  @override
  Widget build(BuildContext context) {
    return const Frame(
      width: 560,
      height: 320,
      children: [
        Sidebar(
          children: [
            _Library(),
            SidebarCard(
              label: 'Build',
              children: [Progress(value: 0.75), Text('3 of 4 packages')],
            ),
          ],
        ),
        Pane(children: [Text('Projects')]),
      ],
    );
  }
}

class _Library extends StatefulWidget {
  const _Library();

  @override
  State<_Library> createState() => _LibraryState();
}

class _LibraryState extends State<_Library> {
  String _selected = 'projects';

  static const List<(String, String, IconData)> _rows = [
    ('projects', 'Projects', FluentIcons.folder_16_regular),
    ('archive', 'Archive', FluentIcons.archive_16_regular),
    ('settings', 'Settings', FluentIcons.settings_16_regular),
  ];

  @override
  Widget build(BuildContext context) {
    return SidebarGroup(
      label: 'Library',
      children: [
        for (final (id, name, icon) in _rows)
          NavItem(
            label: name,
            icon: icon,
            current: _selected == id,
            onPressed: () => setState(() => _selected = id),
          ),
      ],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Sidebar',
      description:
          'The first navigation column: what the window is *about*, in groups '
          'of rows. It carries the muted surface — the same one the aside '
          'wears — so a window reads as content between two quieter edges.',
      children: [
        DocSection(
          'Groups',
          description:
              'Rows are broken into groups rather than stacked. The gap does '
              'most of the work and the label only names it, which is why a '
              'first group can go unlabelled where the window’s title already '
              'says what it is.',
          children: [
            Frame(
              width: 620,
              height: 300,
              children: [
                Sidebar(
                  children: [
                    const _Library(),
                    SidebarGroup(
                      label: 'Recent',
                      children: [
                        NavItem(label: 'aurora', onPressed: () {}),
                        NavItem(label: 'beacon', onPressed: () {}),
                      ],
                    ),
                  ],
                ),
                const Pane(children: [Text('Projects')]),
              ],
            ),
          ],
        ),
        const DocSection(
          'Header and card',
          description:
              'The header sits on the titlebar’s own height, so it lines up '
              'with whatever is across the top of the window. A pinned '
              '`SidebarCard` stays at the foot while the rows take the space '
              'above it.',
          children: [_WithACard()],
        ),
        DocSection(
          'Tokens',
          description: 'The column widths a shell is built on.',
          children: [
            TokenTable([
              TokenRow.length('frame.sidebar-width', vars.frameSidebarWidth),
              TokenRow.length('frame.rail-width', vars.frameRailWidth),
              TokenRow.length('frame.aside-width', vars.frameAsideWidth),
              TokenRow.length('frame.titlebar-size', vars.frameTitlebarSize),
              TokenRow.length('frame.nav-gap', vars.frameNavGap),
            ]),
          ],
        ),
      ],
    );
  }
}
