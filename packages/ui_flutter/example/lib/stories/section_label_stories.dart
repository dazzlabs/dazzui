import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/SectionLabel` — the heading over a run of rows.
class SectionLabelStories {
  SectionLabelStories._();

  static StoryPage get page => StoryPage('SectionLabel', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Default', (context) => const SectionLabel('Library')),
    Story(
      'Faint',
      (context) => const SectionLabel('Library', tone: SectionLabelTone.faint),
    ),
    Story(
      'Tints',
      (context) => Col(
        children: [
          for (final tint in SectionLabelTint.values)
            SectionLabel(label(tint.name), tint: tint),
        ],
      ),
    ),
    Story('In a panel', (context) => const _InAPanel()),
    Story(
      'In a column',
      (context) => Frame(
        width: 520,
        height: 280,
        children: [
          Sidebar(
            children: [
              SidebarGroup(
                label: 'Library',
                children: [
                  NavItem(label: 'Projects', current: true, onPressed: () {}),
                  NavItem(label: 'Archive', onPressed: () {}),
                ],
              ),
              SidebarGroup(
                label: 'Recent',
                children: [NavItem(label: 'aurora', onPressed: () {})],
              ),
            ],
          ),
          const Pane(children: [Text('Projects')]),
        ],
      ),
    ),
  ]);
}

class _InAPanel extends StatelessWidget {
  const _InAPanel();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return SizedBox(
      width: 260,
      child: Card(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          spacing: vars.spacing2,
          children: const [
            SectionLabel('Details'),
            Text('aurora'),
            Divider(),
            SectionLabel('Timing'),
            Text('2m 14s'),
          ],
        ),
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'SectionLabel',
      description:
          'The quiet heading that names a run of rows. A sidebar group already '
          'wears one; this is the same label reached directly, for a panel or '
          'a column that is not a sidebar.',
      children: [
        DocSection(
          'Tone',
          description:
              'It takes the subtle ink by default and the faint one where the '
              'label is only there to break a long run. A tint colours it '
              'where a section carries a meaning of its own — a danger zone in '
              'a settings page.',
          children: [
            Specimens(
              children: [
                const Specimen('default', children: [SectionLabel('Library')]),
                const Specimen(
                  'faint',
                  children: [
                    SectionLabel('Library', tone: SectionLabelTone.faint),
                  ],
                ),
                Specimen(
                  'tinted',
                  children: [
                    for (final tint in SectionLabelTint.values)
                      SectionLabel(label(tint.name), tint: tint),
                  ],
                ),
              ],
            ),
          ],
        ),
        const DocSection(
          'Where it lands',
          description:
              'Over a run of rows in a panel or an aside. In a sidebar the '
              'group draws it, so a label written by hand there would be a '
              'second heading over the same rows.',
          children: [_InAPanel()],
        ),
      ],
    );
  }
}
