import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';
import '../storybook/story_index.dart';

/// `Components/Rail` — the second navigation column.
class RailStories {
  RailStories._();

  static StoryPage get page => StoryPage('Rail', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'A column',
      (context) => const Frame(
        width: 520,
        height: 300,
        children: [
          Rail(children: [_Rows()]),
          Pane(children: [Text('Studio Light')]),
        ],
      ),
    ),
    Story(
      'With an action',
      (context) => Frame(
        width: 520,
        height: 300,
        children: [
          Rail(
            children: [
              const _Rows(),
              NavItem(
                label: 'New theme',
                size: WidgetSize.small,
                onPressed: () {},
              ),
            ],
          ),
          const Pane(children: [Text('Studio Light')]),
        ],
      ),
    ),
    Story(
      'Beside a sidebar',
      (context) => Frame(
        width: 780,
        height: 320,
        children: [
          Sidebar(
            children: [
              SidebarGroup(
                label: 'Library',
                children: [
                  NavItem(
                    label: 'Design system',
                    icon: FluentIcons.paint_brush_16_regular,
                    current: true,
                    onPressed: () {},
                  ),
                  NavItem(
                    label: 'Examples',
                    icon: FluentIcons.folder_16_regular,
                    onPressed: () {},
                  ),
                ],
              ),
            ],
          ),
          const Rail(children: [_Rows()]),
          const Pane(children: [Text('Studio Light')]),
        ],
      ),
    ),
    Story(
      'Scrolling',
      (context) => Frame(
        width: 420,
        height: 220,
        children: [
          Rail(
            children: [
              for (final name in StoryIndex.componentNames)
                NavItem(
                  label: name,
                  size: WidgetSize.small,
                  current: name == 'Rail',
                  onPressed: () {},
                ),
            ],
          ),
          const Pane(children: [Text('Rail')]),
        ],
      ),
    ),
  ]);
}

class _Rows extends StatefulWidget {
  const _Rows();

  @override
  State<_Rows> createState() => _RowsState();
}

class _RowsState extends State<_Rows> {
  String _selected = 'studio-light';

  static const List<(String, String)> _themes = [
    ('studio-light', 'Studio Light'),
    ('studio-dark', 'Studio Dark'),
    ('bright-light', 'Bright'),
    ('bright-dark', 'Bright Dark'),
  ];

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisSize: MainAxisSize.min,
      children: [
        for (final (id, name) in _themes)
          NavItem(
            label: name,
            size: WidgetSize.small,
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
    return const DocPage(
      'Rail',
      description:
          'The column between the sidebar and the content: the groups, books '
          'or pages inside whatever the sidebar selected. It is lighter than '
          'the sidebar because it sits closer to the content — the two columns '
          'are not the same grey, and which is which is what tells you where '
          'you are.',
      children: [
        DocSection(
          'Rows',
          description:
              'A rail’s rows are `NavItem`s at the `small` step, a size '
              'tighter than the sidebar’s on both axes.',
          children: [
            Frame(
              width: 620,
              height: 300,
              children: [
                Rail(children: [_Rows()]),
                Pane(children: [Text('Studio Light')]),
              ],
            ),
          ],
        ),
        DocSection(
          'Three columns',
          description:
              'Sidebar, rail, content — and the aside on the far edge when the '
              'selection has details worth carrying. Each column’s width is a '
              '`frame.*` metric rather than a value the widget decides.',
          children: [
            Frame(
              width: 820,
              height: 320,
              children: [
                Sidebar(
                  children: [
                    SidebarGroup(
                      label: 'Library',
                      children: [
                        NavItem(
                          label: 'Design system',
                          icon: FluentIcons.paint_brush_16_regular,
                          current: true,
                          onPressed: null,
                        ),
                      ],
                    ),
                  ],
                ),
                Rail(children: [_Rows()]),
                Pane(children: [Text('Studio Light')]),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
