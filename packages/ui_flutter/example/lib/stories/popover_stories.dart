import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Popover` — a panel of anything, anchored to what opened it.
class PopoverStories {
  PopoverStories._();

  static const List<PopoverSide> sides = [
    PopoverSide.top,
    PopoverSide.bottom,
    PopoverSide.left,
    PopoverSide.right,
  ];

  static Widget about({PopoverSide side = PopoverSide.bottom, String? label}) {
    return Popover(
      side: side,
      title: const Text('Build 4.2.1'),
      description: const Text('Published 3 days ago from main.'),
      trigger: (context, state) => Button(
        onPressed: state.toggle,
        child: Text(label ?? 'About this build'),
      ),
    );
  }

  static StoryPage get page => StoryPage('Popover', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Bare', (context) => about()),
    Story(
      'Sides',
      (context) => RowWrap(
        children: [
          for (final side in sides) about(side: side, label: side.name),
        ],
      ),
    ),
    Story('With controls', (context) => const _Filter()),
    Story('A form', (context) => const _Rename()),
    Story('In a toolbar', (context) => const _Toolbar()),
  ]);
}

class _Filter extends StatefulWidget {
  const _Filter();

  @override
  State<_Filter> createState() => _FilterState();
}

class _FilterState extends State<_Filter> {
  final Set<String> _states = {'Open'};

  @override
  Widget build(BuildContext context) {
    return Popover(
      align: PopoverAlign.start,
      width: 256,
      title: const Text('Filter'),
      trigger: (context, state) => IconButton(
        icon: const Icon(FluentIcons.filter_16_regular),
        semanticsLabel: 'Filter',
        onPressed: state.toggle,
      ),
      child: Col(
        children: [
          for (final String name in ['Open', 'Merged', 'Closed'])
            Checkbox(
              value: _states.contains(name),
              label: Text(name),
              onChanged: (value) => setState(() {
                value ? _states.add(name) : _states.remove(name);
              }),
            ),
        ],
      ),
    );
  }
}

class _Rename extends StatelessWidget {
  const _Rename();

  @override
  Widget build(BuildContext context) {
    return Popover(
      width: 320,
      title: const Text('Rename branch'),
      description: const Text('The remote keeps its own name until you push.'),
      trigger: (context, state) => Button(
        variant: ButtonVariant.filled,
        onPressed: state.toggle,
        child: const Text('Rename branch'),
      ),
      child: const Col(
        children: [
          FormField(
            label: 'Branch name',
            child: TextField(placeholder: 'release/4.2'),
          ),
        ],
      ),
    );
  }
}

class _Toolbar extends StatelessWidget {
  const _Toolbar();

  @override
  Widget build(BuildContext context) {
    return RowWrap(
      children: [
        const _Filter(),
        Popover(
          align: PopoverAlign.end,
          title: const Text('About'),
          description: const Text('42 pull requests, 8 of them yours.'),
          trigger: (context, state) => IconButton(
            icon: const Icon(FluentIcons.info_16_regular),
            semanticsLabel: 'About',
            onPressed: state.toggle,
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
      'Popover',
      description:
          'A panel of anything, anchored to what opened it. It is the sibling '
          'of Menu rather than of Dialog: it wears the same kind of floating '
          'sheet and is positioned against its trigger, but it holds content '
          'rather than rows. Reach for a dialog instead when the reader has to '
          'finish before doing anything else — a popover closes on an outside '
          'press, which makes it the wrong home for something half-typed.',
      children: [
        DocSection(
          'A heading and a line',
          description:
              'The title names the panel and the description describes it, to '
              'a screen reader as well. The gap between the panel and its '
              'trigger is padding on the follower rather than a number passed '
              'in, so it stays a token.',
          children: [PopoverStories.about()],
        ),
        DocSection(
          'Sides',
          description:
              'Anchored to whichever edge of the trigger it is told, and '
              'aligned to whichever end.',
          children: [
            Specimens(
              children: [
                for (final side in PopoverStories.sides)
                  Specimen(
                    side.name,
                    children: [
                      PopoverStories.about(side: side, label: side.name),
                    ],
                  ),
              ],
            ),
          ],
        ),
        const DocSection(
          'A panel with controls in it',
          description:
              'This is what separates it from a Tooltip: a popover can be '
              'reached and typed into. A tooltip must carry nothing a reader '
              'needs, because a pointer never gets there.',
          children: [_Filter()],
        ),
      ],
    );
  }
}
