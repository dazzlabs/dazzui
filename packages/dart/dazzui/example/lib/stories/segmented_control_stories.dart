import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/SegmentedControl` — one choice, on a track.
class SegmentedControlStories {
  SegmentedControlStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('SegmentedControl', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Raised',
      (context) => const _Modes(style: SegmentedActiveStyle.raised),
    ),
    Story(
      'Filled',
      (context) => const _Modes(style: SegmentedActiveStyle.filled),
    ),
    Story(
      'Sizes',
      (context) => Col(
        children: [
          for (final size in sizes)
            _Modes(style: SegmentedActiveStyle.raised, size: size),
        ],
      ),
    ),
    Story(
      'Stretch',
      (context) => const SizedBox(
        width: 380,
        child: _Modes(style: SegmentedActiveStyle.raised, stretch: true),
      ),
    ),
    Story(
      'Disabled',
      (context) => SegmentedControl<String>(
        value: 'design',
        onChanged: (_) {},
        items: const [
          SegmentedItem(value: 'design', label: 'Design'),
          SegmentedItem(value: 'code', label: 'Code'),
          SegmentedItem(value: 'preview', label: 'Preview', enabled: false),
        ],
      ),
    ),
    Story(
      'Two segments',
      (context) => SegmentedControl<String>(
        value: 'light',
        onChanged: (_) {},
        items: const [
          SegmentedItem(value: 'light', label: 'Light'),
          SegmentedItem(value: 'dark', label: 'Dark'),
        ],
      ),
    ),
    Story(
      'In a form',
      (context) => const Col(
        width: 380,
        children: [
          FormField(
            label: 'Editor',
            child: _Modes(style: SegmentedActiveStyle.raised, stretch: true),
          ),
        ],
      ),
    ),
  ]);
}

class _Modes extends StatefulWidget {
  const _Modes({
    required this.style,
    this.size = WidgetSize.small,
    this.stretch = false,
  });

  final SegmentedActiveStyle style;
  final WidgetSize size;
  final bool stretch;

  @override
  State<_Modes> createState() => _ModesState();
}

class _ModesState extends State<_Modes> {
  String _mode = 'design';

  @override
  Widget build(BuildContext context) {
    return SegmentedControl<String>(
      value: _mode,
      onChanged: (value) => setState(() => _mode = value),
      size: widget.size,
      activeStyle: widget.style,
      stretch: widget.stretch,
      items: const [
        SegmentedItem(value: 'design', label: 'Design'),
        SegmentedItem(value: 'code', label: 'Code'),
        SegmentedItem(value: 'preview', label: 'Preview'),
      ],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'SegmentedControl',
      description:
          'One choice out of a few, on a sunken track. Where pill tabs filter '
          'a list under them, this switches what a view *is* — a mode, an '
          'appearance, a unit — which is why it keeps the track: the unchosen '
          'options stay visible as part of one control.',
      children: [
        DocSection(
          'Active style',
          description:
              '`raised` is the AppKit shape: the chosen segment is a paper '
              'chip lifted off the track, and the label goes to the strong '
              'face. `filled` takes the tint instead, for a control that has '
              'to say which mode it is in from across the window.',
          children: [
            Specimens(
              children: [
                for (final style in SegmentedActiveStyle.values)
                  Specimen(style.name, children: [_Modes(style: style)]),
              ],
            ),
          ],
        ),
        DocSection(
          'Sizes',
          description:
              'Three heights off the control profile. The track itself is a '
              'surface role and never moves.',
          children: [
            Specimens(
              children: [
                for (final size in SegmentedControlStories.sizes)
                  Specimen(
                    size.namedSize.name,
                    children: [
                      _Modes(style: SegmentedActiveStyle.raised, size: size),
                      _Modes(style: SegmentedActiveStyle.filled, size: size),
                    ],
                  ),
              ],
            ),
          ],
        ),
        const DocSection(
          'Stretch',
          description:
              'By default a segment is as wide as its label. `stretch` divides '
              'the width evenly instead, which is what a control inside a form '
              'field wants so it lines up with the fields above it.',
          children: [
            Col(
              width: 420,
              children: [
                _Modes(style: SegmentedActiveStyle.raised),
                _Modes(style: SegmentedActiveStyle.raised, stretch: true),
              ],
            ),
          ],
        ),
        DocSection(
          'Tokens',
          description: 'The track’s own inset.',
          children: [
            TokenTable([
              TokenRow.length(
                'segmented-control.inset',
                vars.segmentedControlInset,
              ),
            ]),
          ],
        ),
      ],
    );
  }
}
