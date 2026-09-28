import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Meter` — a reading inside a known range.
class MeterStories {
  MeterStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Meter', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Bare',
      (context) => const SizedBox(width: 288, child: Meter(value: 0.62)),
    ),
    Story(
      'With value',
      (context) => const SizedBox(
        width: 320,
        child: Meter(
          value: 0.62,
          label: Text('Storage'),
          valueLabel: '62%',
        ),
      ),
    ),
    Story(
      'Sizes',
      (context) => Col(
        width: 320,
        children: [
          for (final size in sizes) Meter(value: 0.62, size: size),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        width: 320,
        children: [
          for (final tint in MeterTint.values) Meter(value: 0.62, tint: tint),
        ],
      ),
    ),
    Story('In a quota list', (context) => const _Quotas()),
  ]);
}

class _Quotas extends StatelessWidget {
  const _Quotas();

  @override
  Widget build(BuildContext context) {
    return const Col(
      width: 352,
      spacing: 16,
      children: [
        Meter(value: 0.62, label: Text('Storage'), valueLabel: '62%'),
        Meter(
          value: 0.41,
          label: Text('Bandwidth'),
          valueLabel: '41%',
          tint: MeterTint.info,
        ),
        Meter(
          value: 0.88,
          label: Text('Build minutes'),
          valueLabel: '88%',
          tint: MeterTint.warning,
        ),
        Meter(
          value: 1,
          label: Text('Seats'),
          valueLabel: '100%',
          tint: MeterTint.danger,
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
      'Meter',
      description:
          'A reading inside a known range — disk used of disk there, spend '
          'against a budget, an API quota. It is not a Progress, and the '
          'difference is not how they look: progress reports on something '
          'running, so it moves on its own, it can be indeterminate, and it '
          'ends. A meter measures something that already has a value, so it '
          'never animates and never has none — which is why it prints its '
          'number and progress does not.',
      children: [
        const DocSection(
          'Sizes',
          description:
              'The same groove a Progress bar is cut into — the sunken '
              'surface, the soft square corner — because they are the same '
              'hole in the page. What differs is everything above it.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'small',
                  children: [
                    SizedBox(
                      width: 240,
                      child: Meter(value: 0.62, size: WidgetSize.small),
                    ),
                  ],
                ),
                Specimen(
                  'medium',
                  children: [SizedBox(width: 240, child: Meter(value: 0.62))],
                ),
                Specimen(
                  'large',
                  children: [
                    SizedBox(
                      width: 240,
                      child: Meter(value: 0.62, size: WidgetSize.large),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        const DocSection(
          'Label and value',
          description:
              'The number takes tabular figures, so a column of meters keeps '
              'its numbers on one edge. The label is the meter’s '
              'accessible name as well as its heading.',
          children: [_Quotas()],
        ),
        DocSection(
          'Against a Progress',
          description:
              'Set side by side the rule is easy to keep: if the number would '
              'change on its own while the reader watched, it is progress. If '
              'it is a fact about right now, it is a meter.',
          children: [
            const Col(
              width: 320,
              children: [
                Meter(
                  value: 0.62,
                  label: Text('Disk used'),
                  valueLabel: '62%',
                ),
                Progress(value: 0.62),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
