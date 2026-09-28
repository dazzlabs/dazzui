import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Tag` — a Badge you can act on.
class TagStories {
  TagStories._();

  static const List<String> labels = <String>[
    'bug',
    'good first issue',
    'needs design',
    'blocked',
  ];

  static StoryPage get page => StoryPage('Tag', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Selectable', (context) => const _Filters()),
    Story('Removable', (context) => const _Removable()),
    Story(
      'Sizes',
      (context) => RowWrap(
        children: <Widget>[
          for (final WidgetSize size in <WidgetSize>[
            WidgetSize.small,
            WidgetSize.medium,
            WidgetSize.large,
          ])
            Tag(size: size, label: Text(size.namedSize.name), onRemove: _noop),
        ],
      ),
    ),
    Story(
      'Variants',
      (context) => RowWrap(
        children: <Widget>[
          for (final TagVariant variant in TagVariant.values)
            Tag(variant: variant, label: Text(variant.name)),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => RowWrap(
        children: <Widget>[
          for (final TagTint tint in TagTint.values)
            Tag(tint: tint, label: Text(tint.name), onRemove: _noop),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => RowWrap(
        children: <Widget>[
          Tag(
            enabled: false,
            selected: false,
            onSelectedChanged: (_) {},
            label: const Text('unavailable'),
          ),
          Tag(
            enabled: false,
            onRemove: _noop,
            label: const Text('pinned'),
          ),
        ],
      ),
    ),
    Story('A filter bar', (context) => const _FilterBar()),
  ]);

  static void _noop() {}
}

class _Filters extends StatefulWidget {
  const _Filters();

  @override
  State<_Filters> createState() => _FiltersState();
}

class _FiltersState extends State<_Filters> {
  final Set<String> _on = <String>{'bug'};

  @override
  Widget build(BuildContext context) {
    return RowWrap(
      children: <Widget>[
        for (final String label in TagStories.labels)
          Tag(
            variant: TagVariant.outlined,
            selected: _on.contains(label),
            onSelectedChanged: (bool next) => setState(() {
              if (next) {
                _on.add(label);
              } else {
                _on.remove(label);
              }
            }),
            label: Text(label),
          ),
      ],
    );
  }
}

class _Removable extends StatefulWidget {
  const _Removable();

  @override
  State<_Removable> createState() => _RemovableState();
}

class _RemovableState extends State<_Removable> {
  late final List<String> _labels = <String>[...TagStories.labels];

  @override
  Widget build(BuildContext context) {
    return RowWrap(
      children: <Widget>[
        for (final String label in _labels)
          Tag(
            removeSemanticsLabel: 'Remove $label',
            onRemove: () => setState(() => _labels.remove(label)),
            label: Text(label),
          ),
      ],
    );
  }
}

class _FilterBar extends StatelessWidget {
  const _FilterBar();

  @override
  Widget build(BuildContext context) {
    return const Col(
      width: 416,
      children: <Widget>[_Filters(), _Removable()],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Tag',
      description:
          'A Badge you can act on. The two are one drawing and two widgets '
          'rather than one widget with an interactive flag, because the '
          'difference is not cosmetic: a badge reads its recipes at rest and '
          'stays out of the shared state machine, since a label that moves '
          'under the pointer reads as something you can press. A tag is '
          'something you can press — and taking the state machine is the whole '
          'reason it exists.',
      children: [
        DocSection(
          'Beside a Badge',
          description:
              'They share the capsule — the same box at the same three '
              'densities — and nothing else. A tag that is neither selectable '
              'nor removable should be a Badge.',
          children: [
            Specimens(
              children: [
                Specimen('Badge', children: [Badge(child: Text('v1.7.0'))]),
                Specimen(
                  'Tag, selectable',
                  children: [
                    Tag(
                      selected: true,
                      variant: TagVariant.outlined,
                      label: Text('bug'),
                    ),
                  ],
                ),
                Specimen(
                  'Tag, removable',
                  children: [
                    Tag(
                      onRemove: TagStories._noop,
                      label: Text('needs design'),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Selected',
          description:
              'Drawn twice over: the tinted fill the widget swaps the variant '
              'to, and the tint’s edge. One alone is not enough — a tinted tag '
              'that was already tinted would look untouched, and an edge alone '
              'is a pixel to spot in a row of forty.',
          children: [_Filters()],
        ),
        DocSection(
          'Dismissing',
          description:
              'The dismiss button is its own control rather than a second job '
              'for the label: a chip that removes itself when tapped has no '
              'way to be selected, and one that does both from the same target '
              'removes things by accident. It is faint at rest and only '
              'resolves under the pointer — an ✕ at full strength on every '
              'chip turns a list of tags into a list of buttons.',
          children: [_Removable()],
        ),
      ],
    );
  }
}
