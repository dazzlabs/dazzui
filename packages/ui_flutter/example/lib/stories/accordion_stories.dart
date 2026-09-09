import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/story.dart';

/// `Components/Accordion` — several disclosures that know about each other.
class AccordionStories {
  AccordionStories._();

  static const List<(String, String, String)> faq = [
    (
      'where',
      'Where do the values come from?',
      'Every colour, length and face is a token in packages/ds. A widget '
          'reads the variable; it never writes a value of its own.',
    ),
    (
      'themes',
      'How many themes are there?',
      'Twelve, as six families over one default. The modes compose, so a '
          'value shared along the light/dark axis is written once.',
    ),
    (
      'ports',
      'What keeps the platforms in line?',
      'The parity pipeline measures the React build in a browser and holds '
          'every port to those numbers.',
    ),
  ];

  static List<AccordionItem> get items => [
    for (final (String value, String title, String body) in faq)
      AccordionItem(
        value: value,
        title: Text(title),
        child: Text(body),
      ),
  ];

  static StoryPage get page => StoryPage('Accordion', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('One at a time', (context) => const _Sections()),
    Story('Multiple', (context) => const _Sections(multiple: true)),
    Story('Divided', (context) => const _Sections(divided: true)),
    Story(
      'Disabled',
      (context) => SizedBox(
        width: 448,
        child: Accordion(
          divided: true,
          initiallyOpen: const ['where'],
          items: [
            items.first,
            const AccordionItem(
              value: 'locked',
              enabled: false,
              title: Text('Managed by your admin'),
              child: Text('Nothing here can be changed on this device.'),
            ),
          ],
        ),
      ),
    ),
  ]);
}

class _Sections extends StatefulWidget {
  const _Sections({this.multiple = false, this.divided = false});

  final bool multiple;
  final bool divided;

  @override
  State<_Sections> createState() => _SectionsState();
}

class _SectionsState extends State<_Sections> {
  late Set<String> _open = widget.multiple ? {'where', 'themes'} : {'where'};

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 448,
      child: Accordion(
        multiple: widget.multiple,
        divided: widget.divided,
        items: AccordionStories.items,
        open: _open,
        onOpenChanged: (value) => setState(() => _open = value),
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Accordion',
      description:
          'Several disclosures that know about each other. That knowledge is '
          'the whole difference from a run of Collapsibles: by default opening '
          'one shuts the last, so a reader is always looking at one section '
          'rather than at a page that grew while they were reading it.',
      children: [
        DocSection(
          'One at a time',
          description:
              'The default, and the one to reach for when the sections are '
              'alternatives — a FAQ, a set of steps, anything where two open '
              'at once would push the third off the screen.',
          children: [_Sections()],
        ),
        DocSection(
          'Several at once',
          description:
              '`multiple` is for the case where the reader is comparing rather '
              'than choosing — two log sections side by side, two diffs. It is '
              'a deliberate opt-in, because a list that can be opened all the '
              'way is a list that can be scrolled past.',
          children: [_Sections(multiple: true)],
        ),
        DocSection(
          'Divided',
          description:
              'Undivided, the sections sit on one run and the rows are '
              'separated by their own hover wash alone. `divided` draws the '
              'hairline, for the case where they are genuinely a list and the '
              'reader is counting them.',
          children: [_Sections(divided: true)],
        ),
      ],
    );
  }
}
