import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Card` — a surface, and nothing that answers the pointer.
class CardStories {
  CardStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Card', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    for (final variant in CardVariant.values)
      Story(label(variant.name), (context) => _Sample(variant: variant)),
    Story(
      'Variants',
      (context) => RowWrap(
        alignment: WrapCrossAlignment.start,
        children: [
          for (final variant in CardVariant.values) _Sample(variant: variant),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => RowWrap(
        children: [
          for (final tint in CardTint.values)
            Card(
              variant: CardVariant.tinted,
              tint: tint,
              child: SizedBox(width: 96, child: Text(label(tint.name))),
            ),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => RowWrap(
        alignment: WrapCrossAlignment.start,
        children: [
          for (final size in sizes)
            Card(
              size: size,
              child: SizedBox(
                width: 96,
                child: Text(label(size.namedSize.name)),
              ),
            ),
        ],
      ),
    ),
    Story(
      'With an action',
      (context) => SizedBox(
        width: 320,
        child: Card(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: Theme.of(context).vars.spacing3,
            children: [
              const _Blurb(
                title: 'Tokens are stale',
                note: 'packages/ds changed after the last build.',
              ),
              ActionBar(
                align: ActionBarAlign.end,
                children: [
                  Button(
                    variant: ButtonVariant.filled,
                    onPressed: () {},
                    child: const Text('Rebuild'),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    ),
    Story(
      'As a grid',
      (context) => const SizedBox(
        width: 640,
        child: TileGrid(
          min: 200,
          children: [
            Card(
              child: _Blurb(title: 'ui_react', note: 'Reads what ds wrote.'),
            ),
            Card(
              child: _Blurb(title: 'ui_flutter', note: 'Reads what ds wrote.'),
            ),
            Card(
              child: _Blurb(title: 'ui_gpui', note: 'Reads what ds wrote.'),
            ),
            Card(
              child: _Blurb(title: 'ui_swiftui', note: 'Reads what ds wrote.'),
            ),
          ],
        ),
      ),
    ),
    Story(
      'As a well',
      (context) => SizedBox(
        width: 320,
        child: Card(
          variant: CardVariant.sunken,
          child: Text(
            'flutter analyze',
            style: mono(Theme.of(context).vars.bodySmall),
          ),
        ),
      ),
    ),
    Story('Nested', (context) => const _Nested()),
  ]);
}

/// One card of each variant, with the same thing inside it.
class _Sample extends StatelessWidget {
  const _Sample({required this.variant});

  final CardVariant variant;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 180,
      child: Card(
        variant: variant,
        child: _Blurb(title: label(variant.name), note: 'A surface.'),
      ),
    );
  }
}

class _Nested extends StatelessWidget {
  const _Nested();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return SizedBox(
      width: 340,
      child: Card(
        variant: CardVariant.sunken,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          spacing: vars.spacing3,
          children: const [
            _Blurb(title: 'Queue', note: 'Two jobs waiting.'),
            Card(
              child: _Blurb(title: 'aurora', note: 'main · 2m 14s'),
            ),
            Card(
              child: _Blurb(title: 'beacon', note: 'release/2.1 · queued'),
            ),
          ],
        ),
      ),
    );
  }
}

class _Blurb extends StatelessWidget {
  const _Blurb({required this.title, required this.note});

  final String title;
  final String note;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing1,
      children: [
        Text(title, style: vars.labelMedium.copyWith(color: vars.colorContent)),
        Text(
          note,
          style: vars.captionSmall.copyWith(color: vars.colorContentMuted),
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
      'Card',
      description:
          'A surface with a corner and a pad. It is not interactive and stays '
          'out of the shared state machine: a panel that lit up under the '
          'pointer would read as something you can press. Where a card has to '
          'be selectable, that is an `OptionCard`.',
      children: [
        DocSection(
          'Variants',
          description:
              '`raised` is the paper card on the canvas; `sunken` is the '
              'groove a well or a code block sits in; `outlined` is the drawn '
              'edge with no fill; `tinted` is the wash that carries a status.',
          children: [
            RowWrap(
              alignment: WrapCrossAlignment.start,
              children: [
                for (final variant in CardVariant.values)
                  _Sample(variant: variant),
              ],
            ),
          ],
        ),
        DocSection(
          'Tints',
          description:
              'Only `tinted` reads a ramp — the others are the theme’s own '
              'surface roles, so a tint leaves them where they are. The wash '
              'is an alpha of the tint rather than an opaque step, which is '
              'what lets the same card sit correctly on paper and on a '
              'near-black canvas.',
          children: [
            RowWrap(
              children: [
                for (final tint in CardTint.values)
                  Card(
                    variant: CardVariant.tinted,
                    tint: tint,
                    child: SizedBox(width: 88, child: Text(label(tint.name))),
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Compositions',
          description:
              'A card holds a heading, its detail and at most one action row. '
              'A sunken card inside a raised one is the well pattern — the '
              'groove reads as inside the panel rather than as a second panel.',
          children: [
            const RowWrap(
              alignment: WrapCrossAlignment.start,
              children: [_Nested()],
            ),
          ],
        ),
      ],
    );
  }
}
