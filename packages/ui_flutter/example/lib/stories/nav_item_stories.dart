import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/NavItem` — the row a sidebar and a rail are made of.
class NavItemStories {
  NavItemStories._();

  static StoryPage get page => StoryPage('NavItem', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Idle',
      (context) => SizedBox(
        width: 200,
        child: NavItem(label: 'Projects', onPressed: () {}),
      ),
    ),
    Story(
      'Active',
      (context) => SizedBox(
        width: 200,
        child: NavItem(label: 'Projects', current: true, onPressed: () {}),
      ),
    ),
    Story(
      'Disabled',
      (context) => const SizedBox(
        width: 200,
        child: NavItem(label: 'Projects', onPressed: null),
      ),
    ),
    Story(
      'States',
      (context) => Col(
        width: 200,
        spacing: 0,
        children: [
          NavItem(label: 'Idle', onPressed: () {}),
          NavItem(label: 'Active', current: true, onPressed: () {}),
          const NavItem(label: 'Disabled', onPressed: null),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => Col(
        width: 200,
        spacing: 0,
        children: [
          NavItem(
            label: 'Medium — the sidebar row',
            current: true,
            onPressed: () {},
          ),
          NavItem(
            label: 'Small — the rail row',
            size: WidgetSize.small,
            onPressed: () {},
          ),
        ],
      ),
    ),
    Story(
      'With an icon',
      (context) => Col(
        width: 200,
        spacing: 0,
        children: [
          NavItem(
            label: 'Projects',
            icon: FluentIcons.folder_16_regular,
            current: true,
            onPressed: () {},
          ),
          NavItem(
            label: 'Archive',
            icon: FluentIcons.archive_16_regular,
            onPressed: () {},
          ),
          NavItem(
            label: 'Settings',
            icon: FluentIcons.settings_16_regular,
            onPressed: () {},
          ),
        ],
      ),
    ),
    Story(
      'With a count',
      (context) => Col(
        width: 200,
        spacing: 0,
        children: [
          NavItem(
            label: 'Inbox',
            icon: FluentIcons.mail_16_regular,
            trailing: const Badge(
              size: WidgetSize.small,
              variant: BadgeVariant.tinted,
              child: Text('12'),
            ),
            onPressed: () {},
          ),
          NavItem(
            label: 'Archive',
            icon: FluentIcons.archive_16_regular,
            onPressed: () {},
          ),
        ],
      ),
    ),
    Story('Selecting', (context) => const _Selecting()),
    Story(
      'In a sidebar',
      (context) => const Frame(
        width: 520,
        height: 300,
        children: [
          Sidebar(children: [_Library()]),
          Pane(children: [Text('Projects')]),
        ],
      ),
    ),
  ]);
}

class _Selecting extends StatefulWidget {
  const _Selecting();

  @override
  State<_Selecting> createState() => _SelectingState();
}

class _SelectingState extends State<_Selecting> {
  String _selected = 'projects';

  static const List<(String, String)> _rows = [
    ('projects', 'Projects'),
    ('archive', 'Archive'),
    ('settings', 'Settings'),
  ];

  @override
  Widget build(BuildContext context) {
    return Col(
      width: 200,
      spacing: 0,
      children: [
        for (final (id, name) in _rows)
          NavItem(
            label: name,
            current: _selected == id,
            onPressed: () => setState(() => _selected = id),
          ),
      ],
    );
  }
}

class _Library extends StatelessWidget {
  const _Library();

  @override
  Widget build(BuildContext context) {
    return SidebarGroup(
      label: 'Library',
      children: [
        NavItem(
          label: 'Projects',
          icon: FluentIcons.folder_16_regular,
          current: true,
          onPressed: () {},
        ),
        NavItem(
          label: 'Archive',
          icon: FluentIcons.archive_16_regular,
          onPressed: () {},
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
      'NavItem',
      description:
          'One navigable row. Selection shifts weight as well as colour: a row '
          'rests at the quiet label face and the selected one takes the filled '
          'recipe, so a column reads as one selection rather than as a list of '
          'differently coloured rows.',
      children: [
        DocSection(
          'States',
          description:
              'Idle rows sit in the navigation ink, a step back from body '
              'copy. Hover brings the row to full ink — the hover rule '
              'outranks the row’s own colour, so both kinds of row answer the '
              'pointer the same way. Active takes the filled recipe.',
          children: [
            Col(
              width: 220,
              spacing: 0,
              children: [
                NavItem(label: 'Idle', onPressed: () {}),
                NavItem(label: 'Active', current: true, onPressed: () {}),
                const NavItem(label: 'Disabled', onPressed: null),
              ],
            ),
          ],
        ),
        DocSection(
          'Sizes',
          description:
              '`medium` is the sidebar row — 28px, the regular control height. '
              '`small` is the rail row, a step tighter on both axes because it '
              'sits closer to the content.',
          children: [
            Col(
              width: 220,
              spacing: 0,
              children: [
                NavItem(label: 'Medium', current: true, onPressed: () {}),
                NavItem(
                  label: 'Small',
                  size: WidgetSize.small,
                  onPressed: () {},
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'The glyph and the trailing slot',
          description:
              'An absent icon stays absent rather than becoming an empty one: '
              'the glyph slot is a fixed width, and a row without an icon '
              'would otherwise be indented past a row with one. The trailing '
              'slot takes a count or a state, never a second action.',
          children: [
            Col(
              width: 220,
              spacing: 0,
              children: [
                NavItem(
                  label: 'Inbox',
                  icon: FluentIcons.mail_16_regular,
                  trailing: const Badge(
                    size: WidgetSize.small,
                    variant: BadgeVariant.tinted,
                    child: Text('12'),
                  ),
                  onPressed: () {},
                ),
                NavItem(label: 'Archive', onPressed: () {}),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
