import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Menu` — the floating sheet, and the rows in it.
class MenuStories {
  MenuStories._();

  static StoryPage get page => StoryPage('Menu', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Default',
      (context) => Menu(
        items: [
          MenuItem(label: 'Rename', onSelect: () {}),
          MenuItem(label: 'Duplicate', onSelect: () {}),
          MenuItem(label: 'Delete', onSelect: () {}),
        ],
        trigger: _actionsTrigger,
      ),
    ),
    Story(
      'With shortcuts',
      (context) => Menu(
        items: [
          MenuItem(label: 'Rename', shortcut: '⌘R', onSelect: () {}),
          MenuItem(label: 'Duplicate', shortcut: '⇧⌘D', onSelect: () {}),
          MenuItem(label: 'Delete', shortcut: '⌫', onSelect: () {}),
        ],
        trigger: _actionsTrigger,
      ),
    ),
    Story(
      'With icons',
      (context) => Menu(
        items: [
          MenuItem(
            label: 'Rename',
            icon: FluentIcons.rename_16_regular,
            shortcut: '⌘R',
            onSelect: () {},
          ),
          MenuItem(
            label: 'Duplicate',
            icon: FluentIcons.square_multiple_16_regular,
            onSelect: () {},
          ),
          MenuItem(
            label: 'Delete',
            icon: FluentIcons.delete_16_regular,
            onSelect: () {},
          ),
        ],
        trigger: _actionsTrigger,
      ),
    ),
    Story('Checked', (context) => const _Checked()),
    Story(
      'Disabled',
      (context) => Menu(
        items: [
          MenuItem(
            label: 'Rename',
            icon: FluentIcons.rename_16_regular,
            onSelect: () {},
          ),
          const MenuItem(
            label: 'Delete',
            icon: FluentIcons.delete_16_regular,
            enabled: false,
          ),
        ],
        trigger: _actionsTrigger,
      ),
    ),
    Story('Alignment', (context) => const _Alignments()),
    Story(
      'From an icon button',
      (context) => Menu(
        items: [
          MenuItem(
            label: 'Rename',
            icon: FluentIcons.rename_16_regular,
            onSelect: () {},
          ),
          MenuItem(
            label: 'Duplicate',
            icon: FluentIcons.square_multiple_16_regular,
            onSelect: () {},
          ),
        ],
        trigger: (context, state) => IconButton(
          icon: Icon(FluentIcons.more_horizontal_16_regular),
          onPressed: state.toggle,
        ),
      ),
    ),
  ]);

  /// The trigger every plain menu story wears: the `▾` button a menu hangs
  /// from.
  static Widget _actionsTrigger(BuildContext context, MenuTriggerState state) {
    return Button(
      shortcut: const Text('▾'),
      onPressed: state.toggle,
      child: const Text('Actions'),
    );
  }
}

/// The tick column: a menu that carries any checked item reserves the column
/// for all of them, so the labels stay on one edge.
class _Checked extends StatefulWidget {
  const _Checked();

  @override
  State<_Checked> createState() => _CheckedState();
}

class _CheckedState extends State<_Checked> {
  bool _hidden = true;
  bool _sidebar = false;

  @override
  Widget build(BuildContext context) {
    return Menu(
      items: [
        MenuItem(
          label: 'Show hidden files',
          checked: _hidden,
          onSelect: () => setState(() => _hidden = !_hidden),
        ),
        MenuItem(
          label: 'Show the sidebar',
          checked: _sidebar,
          onSelect: () => setState(() => _sidebar = !_sidebar),
        ),
      ],
      trigger: (context, state) => Button(
        shortcut: const Text('▾'),
        onPressed: state.toggle,
        child: const Text('View'),
      ),
    );
  }
}

/// Both edges the sheet can hang from, far enough apart to see which one it
/// chose.
class _Alignments extends StatelessWidget {
  const _Alignments();

  @override
  Widget build(BuildContext context) {
    return RowWrap(
      spacing: Theme.of(context).vars.spacing20,
      children: [
        for (final align in MenuAlign.values)
          Menu(
            align: align,
            items: [
              MenuItem(label: 'Rename', onSelect: () {}),
              MenuItem(label: 'Duplicate', onSelect: () {}),
            ],
            trigger: (context, state) => Button(
              shortcut: const Text('▾'),
              onPressed: state.toggle,
              child: Text(label(align.name)),
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
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Menu',
      description:
          'A trigger and a floating sheet of rows. The sheet is one drawing '
          'shared with the tooltip and the select’s list; the menu adds only '
          'what is genuinely its own — the icon slot, the shortcut column, and '
          'the tick that appears once any row is checkable.',
      children: [
        DocSection(
          'Rows',
          description:
              'A row can carry a leading glyph, a trailing shortcut, a check, '
              'and a disabled state. The shortcut is a faint hint in the '
              'display cut rather than a key cap: a cap in a menu row would be '
              'furniture.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'plain',
                  children: [
                    Menu(
                      items: [
                        MenuItem(label: 'Rename', onSelect: () {}),
                        MenuItem(label: 'Duplicate', onSelect: () {}),
                      ],
                      trigger: MenuStories._actionsTrigger,
                    ),
                  ],
                ),
                Specimen(
                  'icons',
                  children: [
                    Menu(
                      items: [
                        MenuItem(
                          label: 'Rename',
                          icon: FluentIcons.rename_16_regular,
                          shortcut: '⌘R',
                          onSelect: () {},
                        ),
                        const MenuItem(
                          label: 'Delete',
                          icon: FluentIcons.delete_16_regular,
                          enabled: false,
                        ),
                      ],
                      trigger: MenuStories._actionsTrigger,
                    ),
                  ],
                ),
                const Specimen('checked', children: [_Checked()]),
              ],
            ),
          ],
        ),
        const DocSection(
          'Alignment',
          description:
              'The sheet hangs from the trigger’s trailing edge by default, '
              'which is where a `▾` button wants it. `start` hangs it from the '
              'leading edge instead. The gap between the trigger and the sheet '
              'is a token rather than a number passed in code.',
          children: [_Alignments()],
        ),
        DocSection(
          'Tokens',
          description: 'The sheet’s own metrics.',
          children: [
            TokenTable([
              TokenRow.length('menu.min-width', vars.menuMinWidth),
              TokenRow.length('menu.item.padding', vars.menuItemPadding),
            ]),
          ],
        ),
      ],
    );
  }
}
