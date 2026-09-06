import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Button` — the Flutter half of
/// `apps/storybook/src/stories/button.stories.tsx`.
class ButtonStories {
  ButtonStories._();

  /// The recipes that read a ramp. `recessed` and `normal` are the neutral
  /// controls: they read the theme's surface and ink roles rather than a shade
  /// off a ramp, so a tint does nothing to them — a neutral chip that could
  /// turn red would no longer be neutral.
  static const List<ButtonVariant> tintedVariants = [
    ButtonVariant.filled,
    ButtonVariant.tinted,
    ButtonVariant.outlined,
    ButtonVariant.plain,
  ];

  /// The four size profiles. `WidgetSize` is a value rather than an enum, so
  /// the list is written out here instead of being read off `values`.
  static const List<WidgetSize> sizes = [
    WidgetSize.tiny,
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Button', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    for (final variant in ButtonVariant.values)
      Story(
        label(variant.name),
        (context) => Button(
          variant: variant,
          onPressed: () {},
          child: const Text('Save changes'),
        ),
      ),
    Story(
      'All variants',
      (context) => RowWrap(
        children: [
          for (final variant in ButtonVariant.values)
            Button(
              variant: variant,
              onPressed: () {},
              child: Text(label(variant.name)),
            ),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => RowWrap(
        children: [
          for (final variant in ButtonVariant.values)
            Button(
              variant: variant,
              onPressed: null,
              child: Text(label(variant.name)),
            ),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => RowWrap(
        children: [
          for (final size in sizes)
            Button(
              size: size,
              variant: ButtonVariant.filled,
              onPressed: () {},
              child: Text(label(size.namedSize.name)),
            ),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        children: [
          for (final variant in tintedVariants)
            RowWrap(
              children: [
                for (final tint in ButtonTint.values)
                  Button(
                    variant: variant,
                    tint: tint,
                    onPressed: () {},
                    child: Text(label(tint.name)),
                  ),
              ],
            ),
        ],
      ),
    ),
    Story(
      'With an icon',
      (context) => Button(
        variant: ButtonVariant.filled,
        onPressed: () {},
        child: Row(
          mainAxisSize: MainAxisSize.min,
          spacing: Theme.of(context).vars.spacing1,
          children: const [
            Icon(FluentIcons.arrow_download_16_regular),
            Text('Export'),
          ],
        ),
      ),
    ),
    Story(
      'With a shortcut',
      (context) => RowWrap(
        children: [
          Button(
            variant: ButtonVariant.filled,
            shortcut: const Text('⌘S'),
            onPressed: () {},
            child: const Text('Translate'),
          ),
          Button(
            shortcut: const Text('⇧⌘S'),
            onPressed: () {},
            child: const Text('Translate all'),
          ),
        ],
      ),
    ),
    Story(
      'Busy',
      (context) => Button(
        variant: ButtonVariant.filled,
        onPressed: null,
        child: Row(
          mainAxisSize: MainAxisSize.min,
          spacing: Theme.of(context).vars.spacing1,
          children: const [
            Spinner(size: WidgetSize.small),
            Text('Translating…'),
          ],
        ),
      ),
    ),
    Story(
      'Full width',
      (context) => Col(
        width: 280,
        children: [
          Button(
            variant: ButtonVariant.filled,
            expand: true,
            onPressed: () {},
            child: const Text('Continue'),
          ),
          Button(
            expand: true,
            onPressed: () {},
            child: const Text('Not now'),
          ),
        ],
      ),
    ),
    Story(
      'Destructive',
      (context) => Button(
        variant: ButtonVariant.filled,
        tint: ButtonTint.danger,
        onPressed: () {},
        child: const Text('Delete glossary'),
      ),
    ),
    Story(
      'In an action bar',
      (context) => SizedBox(
        width: 320,
        child: ActionBar(
          align: ActionBarAlign.end,
          children: [
            Button(onPressed: () {}, child: const Text('Cancel')),
            Button(
              variant: ButtonVariant.filled,
              onPressed: () {},
              child: const Text('Export'),
            ),
          ],
        ),
      ),
    ),
  ]);
}

/// The documented page. Every story after this one is the widget alone.
class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'Button',
      description:
          'The six control recipes, each on any ramp. Hover, press and '
          'disabled are not written here at all — the shared state machine '
          'puts the recipe’s own states on the box, so a button is only the '
          'box.',
      children: [
        DocSection(
          'Variants',
          description:
              '`normal` is the neutral control on the paper, edged by a '
              'hairline; `recessed` is the same control one step into it — the '
              'workaday grey chip a toolbar or a footer is full of, and the '
              'default. `filled` is the one action a view points at, and the '
              'only one that carries the hairline lift; `outlined` is its '
              'drawn twin; `tinted` a quiet accent chip; `plain` looks like '
              'text at rest but keeps the box, because its recipe washes on '
              'hover.',
          children: [
            Specimens(
              children: [
                for (final variant in ButtonVariant.values)
                  Specimen(
                    variant.name,
                    children: [
                      Button(
                        variant: variant,
                        onPressed: () {},
                        child: const Text('Save changes'),
                      ),
                      Button(
                        variant: variant,
                        onPressed: null,
                        child: const Text('Disabled'),
                      ),
                    ],
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Tints',
          description:
              'Nothing below sets a colour: each row is one variant read '
              'against six ramps, and the hover and press states come from the '
              'same recipe, so a danger button darkens the way a primary one '
              'does. `recessed` and `normal` are absent on purpose — they are '
              'the neutral controls, and a tint leaves them untouched.',
          children: [
            Specimens(
              children: [
                for (final variant in ButtonStories.tintedVariants)
                  Specimen(
                    variant.name,
                    children: [
                      for (final tint in ButtonTint.values)
                        Button(
                          variant: variant,
                          tint: tint,
                          onPressed: () {},
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
              'Button keeps no per-size block at all: the size profile carries '
              'the height, the padding, the radius and the type face, and the '
              'widget reads them once. `tiny` is the icon button’s step — a '
              'labelled button rarely wants it.',
          children: [
            Specimens(
              children: [
                for (final size in ButtonStories.sizes)
                  Specimen(
                    size.namedSize.name,
                    children: [
                      Button(
                        size: size,
                        variant: ButtonVariant.filled,
                        onPressed: () {},
                        child: const Text('Export'),
                      ),
                      Button(
                        size: size,
                        onPressed: () {},
                        child: const Text('Cancel'),
                      ),
                      Button(
                        size: size,
                        variant: ButtonVariant.plain,
                        onPressed: () {},
                        child: const Text('Learn more'),
                      ),
                    ],
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Content',
          description:
              'An icon is a child rather than a prop, so it costs the widget '
              'no API and takes the button’s own colour. A shortcut is the one '
              'trailing slot, set in that colour at a lowered opacity — a key '
              'cap inside a button would read as a second control. There is no '
              '`loading` flag either: work in flight is a loader in the label '
              'and an unusable button.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'icon',
                  children: [
                    Button(
                      variant: ButtonVariant.filled,
                      onPressed: () {},
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        spacing: Theme.of(context).vars.spacing1,
                        children: const [
                          Icon(FluentIcons.arrow_download_16_regular),
                          Text('Export'),
                        ],
                      ),
                    ),
                  ],
                ),
                Specimen(
                  'shortcut',
                  children: [
                    Button(
                      variant: ButtonVariant.filled,
                      shortcut: const Text('⌘S'),
                      onPressed: () {},
                      child: const Text('Translate'),
                    ),
                    Button(
                      shortcut: const Text('⇧⌘S'),
                      onPressed: () {},
                      child: const Text('Translate all'),
                    ),
                  ],
                ),
                Specimen(
                  'busy',
                  children: [
                    Button(
                      variant: ButtonVariant.filled,
                      onPressed: null,
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        spacing: Theme.of(context).vars.spacing1,
                        children: const [
                          Spinner(size: WidgetSize.small),
                          Text('Translating…'),
                        ],
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'In a row',
          description:
              'Where more than one button sits together the row is an '
              '`ActionBar`, so the gap comes from the control scale. The '
              'confirming action goes last and is the only filled one: two '
              'filled buttons side by side ask the reader to choose which is '
              'the default.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'confirming',
                  children: [
                    SizedBox(
                      width: 320,
                      child: ActionBar(
                        align: ActionBarAlign.end,
                        children: [
                          Button(onPressed: () {}, child: const Text('Cancel')),
                          Button(
                            variant: ButtonVariant.filled,
                            onPressed: () {},
                            child: const Text('Export'),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
                Specimen(
                  'destructive',
                  children: [
                    SizedBox(
                      width: 320,
                      child: ActionBar(
                        align: ActionBarAlign.end,
                        children: [
                          Button(onPressed: () {}, child: const Text('Keep')),
                          Button(
                            variant: ButtonVariant.filled,
                            tint: ButtonTint.danger,
                            onPressed: () {},
                            child: const Text('Delete glossary'),
                          ),
                        ],
                      ),
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
