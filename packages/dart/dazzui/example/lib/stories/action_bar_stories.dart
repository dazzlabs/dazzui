import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/ActionBar` — the row a dialog footer, a toolbar and a form's
/// last line are all the same thing.
class ActionBarStories {
  ActionBarStories._();

  static StoryPage get page => StoryPage('ActionBar', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    for (final align in ActionBarAlign.values)
      Story(label(align.name), (context) => _Bar(align: align)),
    Story(
      'As a toolbar',
      (context) => SizedBox(
        width: 320,
        child: ActionBar(
          children: [
            IconButton(
              icon: Icon(FluentIcons.chevron_left_16_regular),
              onPressed: () {},
            ),
            IconButton(
              icon: Icon(FluentIcons.chevron_right_16_regular),
              onPressed: () {},
            ),
            const ToolbarSeparator(),
            IconButton(
              icon: Icon(FluentIcons.arrow_clockwise_16_regular),
              onPressed: () {},
            ),
            IconButton(
              icon: Icon(FluentIcons.share_16_regular),
              onPressed: () {},
            ),
          ],
        ),
      ),
    ),
    Story(
      'An action row',
      (context) => SizedBox(
        width: 380,
        child: ActionBar(
          align: ActionBarAlign.end,
          children: [
            Button(onPressed: () {}, child: const Text('Cancel')),
            Button(
              variant: ButtonVariant.filled,
              onPressed: () {},
              child: const Text('Save'),
            ),
          ],
        ),
      ),
    ),
    Story(
      'With a key hint',
      (context) => SizedBox(
        width: 380,
        child: ActionBar(
          align: ActionBarAlign.between,
          children: [
            const KeyCap('⌘↩', size: WidgetSize.small),
            Button(
              variant: ButtonVariant.filled,
              shortcut: const Text('⌘↩'),
              onPressed: () {},
              child: const Text('Send'),
            ),
          ],
        ),
      ),
    ),
    Story(
      'Mixed controls',
      (context) => SizedBox(
        width: 420,
        child: ActionBar(
          align: ActionBarAlign.between,
          children: [
            const Badge(
              size: WidgetSize.small,
              variant: BadgeVariant.tinted,
              tint: BadgeTint.warning,
              child: Text('Draft'),
            ),
            RowWrap(
              children: [
                Button(
                  variant: ButtonVariant.plain,
                  tint: ButtonTint.danger,
                  onPressed: () {},
                  child: const Text('Discard'),
                ),
                Button(
                  variant: ButtonVariant.filled,
                  onPressed: () {},
                  child: const Text('Publish'),
                ),
              ],
            ),
          ],
        ),
      ),
    ),
  ]);
}

/// The three distributions, over the same three controls.
class _Bar extends StatelessWidget {
  const _Bar({required this.align});

  final ActionBarAlign align;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 420,
      child: ActionBar(
        align: align,
        children: [
          Button(
            variant: ButtonVariant.plain,
            onPressed: () {},
            child: const Text('Reset'),
          ),
          Button(onPressed: () {}, child: const Text('Cancel')),
          Button(
            variant: ButtonVariant.filled,
            onPressed: () {},
            child: const Text('Apply'),
          ),
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
      'ActionBar',
      description:
          'A row of controls kept on one rhythm. It owns no colour and no box '
          '— only the gap — so it takes the shared control gap rather than a '
          'token of its own: the small profile’s, whatever the children’s '
          'size, since the design draws every action row on that one step.',
      children: [
        DocSection(
          'Alignment',
          description:
              '`start` is the default. `end` is a dialog footer’s usual shape, '
              'with the confirming action last. `between` hands the slack to '
              'the gaps *between* the children, which is what puts a '
              'destructive action opposite the pair that confirms.',
          children: [
            Specimens(
              children: [
                for (final align in ActionBarAlign.values)
                  Specimen(align.name, children: [_Bar(align: align)]),
              ],
            ),
          ],
        ),
        DocSection(
          'What it holds',
          description:
              'Anything on the control rhythm: buttons, icon buttons with a '
              'vertical rule between runs, a key hint, a badge that says what '
              'the row is about. The bar never colours or sizes what it holds.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'toolbar',
                  children: [
                    SizedBox(
                      width: 240,
                      child: ActionBar(
                        children: [
                          IconButton(
                            icon: Icon(FluentIcons.chevron_left_16_regular),
                            onPressed: () {},
                          ),
                          IconButton(
                            icon: Icon(FluentIcons.chevron_right_16_regular),
                            onPressed: () {},
                          ),
                          const ToolbarSeparator(),
                          IconButton(
                            icon: Icon(FluentIcons.arrow_clockwise_16_regular),
                            onPressed: () {},
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
                Specimen(
                  'confirming',
                  children: [
                    SizedBox(
                      width: 300,
                      child: ActionBar(
                        align: ActionBarAlign.end,
                        children: [
                          Button(onPressed: () {}, child: const Text('Cancel')),
                          Button(
                            variant: ButtonVariant.filled,
                            onPressed: () {},
                            child: const Text('Save'),
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
