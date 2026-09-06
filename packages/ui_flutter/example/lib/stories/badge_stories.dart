import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Badge` — an annotation, not a control.
class BadgeStories {
  BadgeStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Badge', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    for (final variant in BadgeVariant.values)
      Story(
        label(variant.name),
        (context) => Badge(variant: variant, child: const Text('Beta')),
      ),
    Story(
      'All variants',
      (context) => RowWrap(
        children: [
          for (final variant in BadgeVariant.values)
            Badge(variant: variant, child: Text(label(variant.name))),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        children: [
          for (final variant in BadgeVariant.values)
            RowWrap(
              children: [
                for (final tint in BadgeTint.values)
                  Badge(
                    variant: variant,
                    tint: tint,
                    child: Text(label(tint.name)),
                  ),
              ],
            ),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => RowWrap(
        children: [
          for (final size in sizes)
            Badge(size: size, child: Text(label(size.namedSize.name))),
        ],
      ),
    ),
    Story(
      'States',
      (context) => const RowWrap(
        children: [
          Badge(
            variant: BadgeVariant.tinted,
            tint: BadgeTint.success,
            child: Text('Passed'),
          ),
          Badge(
            variant: BadgeVariant.tinted,
            tint: BadgeTint.info,
            child: Text('Running'),
          ),
          Badge(
            variant: BadgeVariant.tinted,
            tint: BadgeTint.neutral,
            child: Text('Queued'),
          ),
          Badge(
            variant: BadgeVariant.tinted,
            tint: BadgeTint.danger,
            child: Text('Failed'),
          ),
          Badge(
            variant: BadgeVariant.tinted,
            tint: BadgeTint.warning,
            child: Text('Deprecated'),
          ),
        ],
      ),
    ),
    Story(
      'Counts',
      (context) => const RowWrap(
        children: [
          Badge(
            size: WidgetSize.small,
            variant: BadgeVariant.filled,
            child: Text('3'),
          ),
          Badge(
            size: WidgetSize.small,
            variant: BadgeVariant.tinted,
            child: Text('12'),
          ),
          Badge(
            size: WidgetSize.small,
            variant: BadgeVariant.outlined,
            child: Text('99+'),
          ),
        ],
      ),
    ),
    Story('With a glyph', (context) => const _Verified()),
    Story(
      'In a table',
      (context) => const SizedBox(
        width: 380,
        child: Table(
          children: [
            TableHead(
              children: [
                TableCell(head: true, child: Text('Build')),
                TableCell(
                  head: true,
                  align: TableCellAlign.end,
                  child: Text('Status'),
                ),
              ],
            ),
            TableRow(
              children: [
                TableCell(child: Text('aurora')),
                TableCell(
                  align: TableCellAlign.end,
                  child: Badge(
                    size: WidgetSize.small,
                    variant: BadgeVariant.tinted,
                    tint: BadgeTint.success,
                    child: Text('Passed'),
                  ),
                ),
              ],
            ),
            TableRow(
              children: [
                TableCell(child: Text('cinder')),
                TableCell(
                  align: TableCellAlign.end,
                  child: Badge(
                    size: WidgetSize.small,
                    variant: BadgeVariant.tinted,
                    tint: BadgeTint.danger,
                    child: Text('Failed'),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    ),
    Story(
      'On a card',
      (context) => const Card(
        variant: CardVariant.sunken,
        child: SizedBox(
          width: 260,
          child: Row(
            children: [
              Text('Tokens'),
              Spacer(),
              Badge(
                size: WidgetSize.small,
                variant: BadgeVariant.raised,
                child: Text('Generated'),
              ),
            ],
          ),
        ),
      ),
    ),
  ]);
}

/// A badge whose label is a glyph and a word. The label is a child, so the
/// glyph costs the widget no API and takes the badge's own colour.
class _Verified extends StatelessWidget {
  const _Verified();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Badge(
      variant: BadgeVariant.tinted,
      tint: BadgeTint.success,
      child: Row(
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing1,
        children: [
          Icon(
            FluentIcons.checkmark_12_regular,
            size: vars.labelSmall.fontSize,
          ),
          const Text('Verified'),
        ],
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'Badge',
      description:
          'A tag: the display cut at label weight in a tight chip-cornered '
          'capsule. A badge is not interactive, so it deliberately skips the '
          'shared state machine and reads the resting recipes directly — a '
          'label that changed colour under the pointer would read as something '
          'you can press.',
      children: [
        DocSection(
          'Variants',
          description:
              'Four of them are the control recipes read at rest. `raised` is '
              'the one a badge owns: paper fill and quiet ink, the chip that '
              'has to stay legible on a grey card where a tinted wash would '
              'disappear.',
          children: [
            Specimens(
              children: [
                for (final variant in BadgeVariant.values)
                  Specimen(
                    variant.name,
                    children: [
                      for (final tint in BadgeTint.values)
                        Badge(
                          variant: variant,
                          tint: tint,
                          child: Text(label(tint.name)),
                        ),
                    ],
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Sizes',
          description:
              'A badge is an annotation rather than a control, so it has three '
              'sizes of its own and never reaches control height. The face and '
              'the weight never soften: only the box and, at `large`, one step '
              'of type.',
          children: [
            Specimens(
              children: [
                for (final size in BadgeStories.sizes)
                  Specimen(
                    size.namedSize.name,
                    children: [
                      Badge(size: size, child: const Text('Beta')),
                      Badge(
                        size: size,
                        variant: BadgeVariant.tinted,
                        tint: BadgeTint.success,
                        child: const Text('Passed'),
                      ),
                      Badge(
                        size: size,
                        variant: BadgeVariant.filled,
                        child: const Text('12'),
                      ),
                    ],
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Where it lands',
          description:
              'A status in a table row, a count beside a label, a state on a '
              'card. Its tint carries the meaning, which is why the status '
              'ramps are the ones it is usually read against.',
          children: [
            const Specimens(
              children: [
                Specimen(
                  'states',
                  children: [
                    Badge(
                      variant: BadgeVariant.tinted,
                      tint: BadgeTint.success,
                      child: Text('Passed'),
                    ),
                    Badge(
                      variant: BadgeVariant.tinted,
                      tint: BadgeTint.info,
                      child: Text('Running'),
                    ),
                    Badge(
                      variant: BadgeVariant.tinted,
                      tint: BadgeTint.danger,
                      child: Text('Failed'),
                    ),
                  ],
                ),
                Specimen('glyph', children: [_Verified()]),
                Specimen(
                  'counts',
                  children: [
                    Badge(
                      size: WidgetSize.small,
                      variant: BadgeVariant.filled,
                      child: Text('3'),
                    ),
                    Badge(
                      size: WidgetSize.small,
                      variant: BadgeVariant.outlined,
                      child: Text('99+'),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
