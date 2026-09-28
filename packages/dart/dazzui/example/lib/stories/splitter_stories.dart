import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/story.dart';

/// `Components/Splitter` — regions the reader can re-proportion.
class SplitterStories {
  SplitterStories._();

  static StoryPage get page => StoryPage('Splitter', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Horizontal',
      (context) => const _Box(width: 544, height: 192, child: _Pair()),
    ),
    Story(
      'Vertical',
      (context) => const _Box(width: 480, height: 224, child: _Stack()),
    ),
    Story(
      'Three panels',
      (context) => const _Box(width: 608, height: 224, child: _Window()),
    ),
    Story(
      'Constrained',
      (context) => const _Box(width: 544, height: 192, child: _Constrained()),
    ),
    Story(
      'Nested',
      (context) => const _Box(width: 608, height: 256, child: _Nested()),
    ),
  ]);
}

/// A region with a name in it, so a seam can be seen to move something.
class _Region extends StatelessWidget {
  const _Region(this.label);

  final String label;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Padding(
      padding: EdgeInsets.all(vars.spacing3),
      child: Align(
        alignment: AlignmentDirectional.topStart,
        child: SectionLabel(label),
      ),
    );
  }
}

/// The window the specimens sit in — a bordered box, since a splitter paints
/// nothing of its own and would otherwise be three seams in mid-air.
class _Box extends StatelessWidget {
  const _Box({
    required this.width,
    required this.height,
    required this.child,
  });

  final double width;
  final double height;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Container(
      width: width,
      height: height,
      clipBehavior: Clip.antiAlias,
      decoration: BoxDecoration(
        color: vars.colorSurface,
        border: Border.all(
          color: vars.colorBorder,
          width: vars.strokeHairline,
        ),
        borderRadius: BorderRadius.circular(vars.radiusMedium),
      ),
      child: child,
    );
  }
}

class _Pair extends StatelessWidget {
  const _Pair();

  @override
  Widget build(BuildContext context) {
    return const Splitter(
      panels: <SplitterPanel>[
        SplitterPanel(defaultSize: 50, child: _Region('Before')),
        SplitterPanel(defaultSize: 50, child: _Region('After')),
      ],
    );
  }
}

class _Stack extends StatelessWidget {
  const _Stack();

  @override
  Widget build(BuildContext context) {
    return const Splitter(
      orientation: SplitterOrientation.vertical,
      panels: <SplitterPanel>[
        SplitterPanel(defaultSize: 40, minSize: 20, child: _Region('Editor')),
        SplitterPanel(defaultSize: 60, minSize: 20, child: _Region('Output')),
      ],
    );
  }
}

class _Window extends StatelessWidget {
  const _Window();

  @override
  Widget build(BuildContext context) {
    return const Splitter(
      panels: <SplitterPanel>[
        SplitterPanel(
          defaultSize: 24,
          minSize: 15,
          maxSize: 40,
          child: _Region('Files'),
        ),
        SplitterPanel(
          defaultSize: 52,
          minSize: 30,
          child: _Region('Document'),
        ),
        SplitterPanel(
          defaultSize: 24,
          minSize: 15,
          maxSize: 40,
          child: _Region('Inspector'),
        ),
      ],
    );
  }
}

class _Constrained extends StatelessWidget {
  const _Constrained();

  @override
  Widget build(BuildContext context) {
    return const Splitter(
      panels: <SplitterPanel>[
        SplitterPanel(
          defaultSize: 30,
          minSize: 25,
          maxSize: 45,
          child: _Region('25–45%'),
        ),
        SplitterPanel(defaultSize: 70, child: _Region('the rest')),
      ],
    );
  }
}

class _Nested extends StatelessWidget {
  const _Nested();

  @override
  Widget build(BuildContext context) {
    return const Splitter(
      panels: <SplitterPanel>[
        SplitterPanel(defaultSize: 30, minSize: 20, child: _Region('Files')),
        SplitterPanel(
          defaultSize: 70,
          child: Splitter(
            orientation: SplitterOrientation.vertical,
            panels: <SplitterPanel>[
              SplitterPanel(defaultSize: 65, child: _Region('Editor')),
              SplitterPanel(defaultSize: 35, child: _Region('Terminal')),
            ],
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
    return const DocPage(
      'Splitter',
      description:
          'Two or more regions the reader can re-proportion. A desktop window '
          'is mostly this — a column beside a document beside an inspector — '
          'and which of the three deserves the room is the reader’s decision '
          'rather than the designer’s.',
      children: [
        DocSection(
          'Beside a Sidebar',
          description:
              'Where a column has one right width and the reader only opens or '
              'shuts it, that is Sidebar instead: it carries its own width '
              'token and its own rail, and drawing it with a splitter would '
              'offer a choice the design has already made. Reach for this '
              'where there is no right answer — where the reader’s screen, and '
              'what they are doing on it, decide.',
          children: [_Box(width: 608, height: 208, child: _Window())],
        ),
        DocSection(
          'The seam',
          description:
              'The handle is drawn from the space between two panels rather '
              'than added to it: the hit target is a whole step of the scale, '
              'so grabbing a seam does not demand pixel accuracy, while the '
              'rule itself stays the hairline every other separator in the '
              'system is. Under the pointer it takes the accent and swells — '
              'without reflowing, since a seam that pushed its neighbours '
              'aside on the way to a drag would be a layout jump.',
          children: [_Box(width: 544, height: 160, child: _Pair())],
        ),
        DocSection(
          'From the keyboard',
          description:
              'Each seam is a focus stop that reports its share: the arrow '
              'keys move it one percent, shift ten, and Home and End push it '
              'to the panel’s own floor and ceiling. Double-tapping a seam '
              'splits that pair evenly, which is what a reader means by the '
              'gesture.',
          children: [_Box(width: 480, height: 176, child: _Stack())],
        ),
      ],
    );
  }
}
