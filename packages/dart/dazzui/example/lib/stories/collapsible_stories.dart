import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/story.dart';

/// `Components/Collapsible` — one thing that opens.
class CollapsibleStories {
  CollapsibleStories._();

  static const String log =
      '\$ pnpm test --filter core\n'
      '  ✓ resolves a relative import  (4ms)\n'
      '  ✓ resolves a bare specifier   (2ms)\n'
      '  ✓ falls back to the index     (3ms)';

  static StoryPage get page => StoryPage('Collapsible', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Shut',
      (context) => const SizedBox(
        width: 352,
        child: Collapsible(
          title: Text('Environment'),
          child: Text('Node 24.20, pnpm 10.4, macOS 15.5'),
        ),
      ),
    ),
    Story(
      'Open',
      (context) => const SizedBox(
        width: 352,
        child: Collapsible(
          initiallyOpen: true,
          title: Text('Environment'),
          child: Text('Node 24.20, pnpm 10.4, macOS 15.5'),
        ),
      ),
    ),
    Story('With meta', (context) => const _WithMeta()),
    Story(
      'Disabled',
      (context) => const SizedBox(
        width: 352,
        child: Collapsible(
          enabled: false,
          title: Text('Environment'),
          child: Text('Node 24.20, pnpm 10.4, macOS 15.5'),
        ),
      ),
    ),
    Story('Interactive', (context) => const _Live()),
  ]);
}

class _WithMeta extends StatelessWidget {
  const _WithMeta();

  @override
  Widget build(BuildContext context) {
    return const SizedBox(
      width: 416,
      child: Collapsible(
        initiallyOpen: true,
        icon: FluentIcons.wrench_16_regular,
        title: Text('Ran 3 tests'),
        meta: Badge(tint: BadgeTint.success, child: Text('passed')),
        child: Text(CollapsibleStories.log),
      ),
    );
  }
}

class _Live extends StatefulWidget {
  const _Live();

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  bool _open = false;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 384,
      child: Collapsible(
        open: _open,
        onOpenChanged: (value) => setState(() => _open = value),
        title: const Text('Request headers'),
        meta: Text(_open ? 'hide' : 'show'),
        child: const Text(
          'accept: application/json\nauthorization: Bearer ****',
        ),
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Collapsible',
      description:
          'One thing that opens. The header row is the affordance and the '
          'panel is what it discloses — a tool call’s output, a stack '
          'trace, the rest of a long description. It is the smallest of the '
          'three ways this system hides something: a collapsible keeps its '
          'subject in place and only folds it, where a Popover moves it off '
          'the page and a Dialog stops the page.',
      children: [
        DocSection(
          'Shut and open',
          description:
              'The chevron is the only moving part: one glyph turned rather '
              'than two swapped, so the turn is the animation. The panel '
              'animates its own height, which is the one thing a disclosure '
              'cannot do declaratively — the content has to be laid out before '
              'it has one.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'shut',
                  children: [
                    SizedBox(
                      width: 352,
                      child: Collapsible(
                        title: Text('Environment'),
                        child: Text('Node 24.20, pnpm 10.4, macOS 15.5'),
                      ),
                    ),
                  ],
                ),
                Specimen(
                  'open',
                  children: [
                    SizedBox(
                      width: 352,
                      child: Collapsible(
                        initiallyOpen: true,
                        title: Text('Environment'),
                        child: Text('Node 24.20, pnpm 10.4, macOS 15.5'),
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'The header row',
          description:
              'A glyph before the title and an aside after it. The aside is '
              'what makes a shut row worth reading — a count, a duration, a '
              'status — so the reader can decide whether to open it at all.',
          children: [_WithMeta()],
        ),
        DocSection(
          'Against an Accordion',
          description:
              'Several of these that know about each other are an Accordion, '
              'where opening one shuts the last. A run of separate '
              'Collapsibles does not, which is right when the sections are '
              'unrelated and wrong when they are one list.',
          children: [
            SizedBox(
              width: 416,
              child: Collapsible(
                icon: FluentIcons.folder_16_regular,
                title: Text('Advanced'),
                child: Text(
                  'Nothing here depends on anything else being shut.',
                ),
              ),
            ),
          ],
        ),
      ],
    );
  }
}
