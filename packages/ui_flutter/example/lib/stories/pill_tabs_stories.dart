import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/PillTabs` — the filter row over a list.
class PillTabsStories {
  PillTabsStories._();

  static StoryPage get page => StoryPage('PillTabs', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Default', (context) => const _Tabs(counts: false)),
    Story('With counts', (context) => const _Tabs(counts: true)),
    Story(
      'Disabled',
      (context) => PillTabs<String>(
        value: 'all',
        onChanged: (_) {},
        items: const [
          PillTabItem(value: 'all', label: 'All', count: 24),
          PillTabItem(value: 'open', label: 'Open', count: 3),
          PillTabItem(value: 'archived', label: 'Archived', enabled: false),
        ],
      ),
    ),
    Story(
      'In a pane header',
      (context) => const Frame(
        width: 560,
        height: 260,
        children: [
          Pane(
            children: [Text('Builds'), _Tabs(counts: true), Divider()],
          ),
        ],
      ),
    ),
  ]);
}

class _Tabs extends StatefulWidget {
  const _Tabs({required this.counts});

  final bool counts;

  @override
  State<_Tabs> createState() => _TabsState();
}

class _TabsState extends State<_Tabs> {
  String _tab = 'all';

  @override
  Widget build(BuildContext context) {
    return PillTabs<String>(
      value: _tab,
      onChanged: (value) => setState(() => _tab = value),
      items: [
        PillTabItem(
          value: 'all',
          label: 'All',
          count: widget.counts ? 24 : null,
        ),
        PillTabItem(
          value: 'open',
          label: 'Open',
          count: widget.counts ? 3 : null,
        ),
        PillTabItem(
          value: 'done',
          label: 'Done',
          count: widget.counts ? 21 : null,
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
      'PillTabs',
      description:
          'A run of pills that filters the list under it. Unlike a segmented '
          'control it has no track: the pills sit on the page, so the row can '
          'wrap and can carry counts without turning into a toolbar.',
      children: [
        DocSection(
          'Selection',
          description:
              'The active pill takes the strong label face as well as the '
              'tint’s wash — the same rule the nav row follows, and the reason '
              'the row still reads when the theme’s accent is quiet.',
          children: [
            Specimens(
              children: [
                const Specimen('plain', children: [_Tabs(counts: false)]),
                const Specimen('counts', children: [_Tabs(counts: true)]),
                Specimen(
                  'disabled',
                  children: [
                    PillTabs<String>(
                      value: 'all',
                      onChanged: (_) {},
                      items: const [
                        PillTabItem(value: 'all', label: 'All', count: 24),
                        PillTabItem(
                          value: 'archived',
                          label: 'Archived',
                          enabled: false,
                        ),
                      ],
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        const DocSection(
          'In a pane header',
          description:
              'The row sits over the list it filters, above the rule that '
              'separates the two.',
          children: [
            Frame(
              width: 560,
              height: 220,
              children: [
                Pane(
                  children: [Text('Builds'), _Tabs(counts: true), Divider()],
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
