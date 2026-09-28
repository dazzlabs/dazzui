import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Aside` — the trailing information column.
class AsideStories {
  AsideStories._();

  static StoryPage get page => StoryPage('Aside', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Beside a pane',
      (context) => const Frame(
        width: 640,
        children: [
          Pane(children: [Text('The selection')]),
          Aside(children: [_Inspector()]),
        ],
      ),
    ),
    Story(
      'An inspector',
      (context) => const Frame(
        width: 420,
        height: 320,
        children: [
          Aside(children: [_Inspector()]),
        ],
      ),
    ),
    Story('With an action', (context) => const _WithAction()),
    Story(
      'In a three-column window',
      (context) => Frame(
        width: 900,
        height: 380,
        children: [
          Sidebar(
            children: [
              SidebarGroup(
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
              ),
            ],
          ),
          const Pane(children: [Text('aurora')]),
          const Aside(children: [_Inspector()]),
        ],
      ),
    ),
  ]);
}

class _WithAction extends StatelessWidget {
  const _WithAction();

  @override
  Widget build(BuildContext context) {
    return Frame(
      width: 420,
      height: 320,
      children: [
        Aside(
          children: [
            const _Inspector(),
            Button(
              expand: true,
              onPressed: () {},
              child: const Text('Reveal'),
            ),
          ],
        ),
      ],
    );
  }
}

/// What the current selection *is* — a run of labelled facts rather than rows
/// you can navigate to.
class _Inspector extends StatelessWidget {
  const _Inspector();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing3,
      children: const [
        SectionLabel('Details'),
        _Fact(name: 'Branch', value: 'main'),
        _Fact(name: 'Commit', value: '785f949'),
        _Fact(
          name: 'Status',
          accessory: Badge(
            size: WidgetSize.small,
            variant: BadgeVariant.tinted,
            tint: BadgeTint.success,
            child: Text('Passed'),
          ),
        ),
        SectionLabel('Timing'),
        _Fact(name: 'Started', value: '09:41'),
        _Fact(name: 'Duration', value: '2m 14s'),
      ],
    );
  }
}

class _Fact extends StatelessWidget {
  const _Fact({required this.name, this.value, this.accessory});

  final String name;
  final String? value;
  final Widget? accessory;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing05,
      children: [
        Text(
          name,
          style: vars.captionSmall.copyWith(color: vars.colorContentSubtle),
        ),
        if (value != null)
          Text(
            value!,
            style: vars.labelMedium.copyWith(color: vars.colorContent),
          ),
        ?accessory,
      ],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Aside',
      description:
          'The trailing information column. It carries what the current '
          'selection *is*, where the sidebar and the rail carry what you can '
          'navigate to — which is why it hangs off the opposite edge and takes '
          'no rows.',
      children: [
        DocSection(
          'Where it sits',
          description:
              'It shares the sidebar’s muted surface rather than the rail’s, '
              'so a window reads as content between two quieter edges rather '
              'than as three different greys. Its width is the `frame.aside` '
              'metric, not a value this widget decides.',
          children: [
            Frame(
              width: 720,
              children: [
                Pane(children: [Text('The selection')]),
                Aside(children: [_Inspector()]),
              ],
            ),
          ],
        ),
        DocSection(
          'What goes in it',
          description:
              'Labelled facts, a badge where a fact has a state, and at most '
              'one action at the foot. A row you can select belongs in the '
              'sidebar or the rail instead.',
          children: [_WithAction()],
        ),
      ],
    );
  }
}
