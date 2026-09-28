import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/KeyCap` — a key, printed.
class KeyCapStories {
  KeyCapStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('KeyCap', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    for (final variant in KeyCapVariant.values)
      Story(label(variant.name), (context) => KeyCap('⌘K', variant: variant)),
    Story(
      'All variants',
      (context) => RowWrap(
        children: [
          for (final variant in KeyCapVariant.values)
            KeyCap('⌘K', variant: variant),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => RowWrap(
        children: [for (final size in sizes) KeyCap('⌘K', size: size)],
      ),
    ),
    Story(
      'A chord',
      (context) => const RowWrap(
        children: [KeyCap('⇧'), KeyCap('⌘'), KeyCap('P')],
      ),
    ),
    Story('A joined chord', (context) => const KeyCap('⇧⌘P')),
    Story(
      'Word keys',
      (context) => const RowWrap(
        children: [
          KeyCap('Esc', variant: KeyCapVariant.key),
          KeyCap('Tab', variant: KeyCapVariant.key),
          KeyCap('Enter', variant: KeyCapVariant.key),
          KeyCap('Space', variant: KeyCapVariant.key),
        ],
      ),
    ),
    Story('As a search hint', (context) => const _SearchHint()),
    Story(
      'In a settings row',
      (context) => const SizedBox(
        width: 460,
        child: PreferenceSection(
          label: 'Shortcuts',
          children: [
            PreferenceRow(title: 'Toggle the palette', trailing: KeyCap('⌘K')),
            PreferenceRow(title: 'Rebuild tokens', trailing: KeyCap('⇧⌘B')),
          ],
        ),
      ),
    ),
  ]);
}

class _SearchHint extends StatelessWidget {
  const _SearchHint();

  @override
  Widget build(BuildContext context) {
    return const SizedBox(
      width: 320,
      child: SearchField(
        placeholder: 'Search components',
        hint: KeyCap('⌘K', size: WidgetSize.small),
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'KeyCap',
      description:
          'The printed form of a key. It is not a control and never reacts to '
          'the pointer — it is a label saying which key does the thing beside '
          'it. Its colours are already in the token graph, so it takes no tint.',
      children: [
        DocSection(
          'Variants',
          description:
              '`hint` is the quiet one that sits inside a field or beside a '
              'menu row. `strong` is the one in prose, where it has to hold '
              'its own against body copy. `key` is the drawn cap, for a word '
              'key that has to read as a physical key rather than as a symbol.',
          children: [
            Specimens(
              children: [
                for (final variant in KeyCapVariant.values)
                  Specimen(
                    variant.name,
                    children: [
                      KeyCap('⌘K', variant: variant),
                      KeyCap('⇧⌘P', variant: variant),
                      KeyCap('Esc', variant: variant),
                    ],
                  ),
              ],
            ),
          ],
        ),
        const DocSection(
          'Chords',
          description:
              'A chord can be one cap with the symbols joined, or a cap per '
              'key. One cap is the usual form — it is one shortcut, and three '
              'boxes read as three keys to press in turn.',
          children: [
            Specimens(
              children: [
                Specimen('joined', children: [KeyCap('⇧⌘P')]),
                Specimen(
                  'separate',
                  children: [KeyCap('⇧'), KeyCap('⌘'), KeyCap('P')],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Where it lands',
          description:
              'The trailing hint in a search field, the shortcut column of a '
              'menu, the trailing slot of a settings row. Inside a button the '
              'shortcut is *not* a cap — it is the button’s own colour at a '
              'lowered opacity, because a cap inside a control reads as a '
              'second control.',
          children: [
            Specimens(
              children: [
                const Specimen('field', children: [_SearchHint()]),
                const Specimen(
                  'settings',
                  children: [
                    SizedBox(
                      width: 420,
                      child: PreferenceSection(
                        label: 'Shortcuts',
                        children: [
                          PreferenceRow(
                            title: 'Toggle the palette',
                            trailing: KeyCap('⌘K'),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
                Specimen(
                  'button',
                  children: [
                    Button(
                      variant: ButtonVariant.filled,
                      shortcut: const Text('⌘S'),
                      onPressed: () {},
                      child: const Text('Translate'),
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
